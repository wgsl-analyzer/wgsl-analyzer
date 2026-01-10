mod builtin;
mod eval;
mod unify;

use std::{fmt, ops::Index};

use either::Either;
use hir_def::{
    HasSource as _,
    body::{BindingId, Body},
    data::{FieldId, FunctionData, GlobalConstantData, GlobalVariableData, OverrideData},
    database::{
        DefinitionWithBodyId, GlobalConstantId, GlobalVariableId, Lookup as _, ModuleDefinitionId,
        OverrideId, StructId,
    },
    expression::{
        ArithmeticOperation, BinaryOperation, ComparisonOperation, Expression, ExpressionId,
        Statement, StatementId, SwitchCaseSelector, UnaryOperator,
    },
    expression_store::{ExpressionStore, ExpressionStoreSource},
    module_data::Name,
    resolver::{ResolveKind, Resolver},
    type_ref::{self, VecDimensionality},
    type_specifier::{IdentExpression, TypeSpecifierId},
};
use la_arena::ArenaMap;
use rustc_hash::FxHashMap;
use triomphe::Arc;
use wgsl_types::syntax::{AccessMode, AddressSpace, Enumerant};

use crate::{
    builtins::{Builtin, BuiltinId, BuiltinOverload, BuiltinOverloadId},
    database::HirDatabase,
    function::{FunctionDetails, ResolvedFunctionId},
    infer::{
        eval::{TemplateParameter, TemplateParameters},
        unify::{UnificationTable, unify},
    },
    ty::{
        ArraySize, ArrayType, AtomicType, MatrixType, Pointer, Reference, ScalarType,
        TextureDimensionality, TextureKind, TextureType, Type, TypeKind, VecSize, VectorType,
    },
};

/// Infers the type of a global item.
/// For `const`s and co, it first uses the specified type,
/// and then uses the body (expression) to infer the return type.
pub fn infer_query(
    database: &dyn HirDatabase,
    definition: DefinitionWithBodyId,
) -> Arc<InferenceResult> {
    let resolver = definition.resolver(database);
    let body = database.body(definition);
    let mut context = InferenceContext::new(database, definition.into(), resolver);

    match definition {
        DefinitionWithBodyId::Function(function) => {
            let data = database.function_data(function).0;
            let return_type = context.collect_fn(&data, &body);
            context.infer_body(&body, return_type, AbstractHandling::Concretize);
        },
        DefinitionWithBodyId::GlobalVariable(variable) => {
            let data = database.global_var_data(variable).0;
            let return_type = context.collect_global_variable(&data, &body);
            context.infer_body(&body, return_type, AbstractHandling::Concretize);
            context.infer_global_variable(&data, &body);
        },
        DefinitionWithBodyId::GlobalConstant(constant) => {
            let data = database.global_constant_data(constant).0;
            let return_type = context.collect_global_constant(&data, &body);
            context.infer_body(&body, return_type, AbstractHandling::Abstract);
        },
        DefinitionWithBodyId::Override(override_declaration) => {
            let data = database.override_data(override_declaration).0;
            let return_type = context.collect_override(&data, &body);
            context.infer_body(&body, return_type, AbstractHandling::Concretize);
        },
        DefinitionWithBodyId::GlobalAssertStatement(_global_assert_statement) => {
            let expression = body.root.and_then(Either::right);

            if let Some(expression) = expression {
                let expected_type = &TypeExpectation::from_type(
                    database.intern_type(TypeKind::Scalar(ScalarType::Bool)),
                );
                context.infer_expression_expect(expression, expected_type, &body.store);
            }
        },
    }

    Arc::new(context.resolve_all())
}

#[expect(clippy::trivially_copy_pass_by_ref, reason = "must match salsa")]
pub fn infer_cycle_result(
    database: &dyn HirDatabase,
    _cycle: &[String],
    definition: &DefinitionWithBodyId,
) -> Arc<InferenceResult> {
    let mut inference_result = InferenceResult::new(database);
    let (name, range) = get_name_and_range(database, ModuleDefinitionId::from(*definition));

    inference_result
        .diagnostics
        .push(InferenceDiagnostic::CyclicType { name, range });

    Arc::new(inference_result)
}

/// Infers the type of a global item's signature.
///
/// The [`InferenceResult`] will contain [`ExpressionId`]s from the signature expression store.
/// The return type is purposefully left as the error type.
/// For example, for `const a = 3` it depends on the initializer, which we do not access here.
pub fn infer_signature_query(
    database: &dyn HirDatabase,
    definition: ModuleDefinitionId,
) -> Option<Arc<InferenceResult>> {
    let resolver = definition.resolver(database);
    let context = InferenceContext::new(database, definition, resolver);
    // TODO: Match the definition and deal with the generic types in the signature.
    // Those can contain expressions, which need to land in the inference results.
    // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/657

    let result = context.resolve_all();
    if result.is_empty() {
        None
    } else {
        Some(Arc::new(result))
    }
}

#[expect(clippy::trivially_copy_pass_by_ref, reason = "must match salsa")]
#[expect(clippy::unnecessary_wraps, reason = "must match salsa")]
pub fn infer_signature_cycle_result(
    database: &dyn HirDatabase,
    _cycle: &[String],
    definition: &ModuleDefinitionId,
) -> Option<Arc<InferenceResult>> {
    let mut inference_result = InferenceResult::new(database);
    let (name, range) = get_name_and_range(database, *definition);
    inference_result
        .diagnostics
        .push(InferenceDiagnostic::CyclicType { name, range });

    Some(Arc::new(inference_result))
}

fn get_name_and_range(
    database: &dyn HirDatabase,
    definition: ModuleDefinitionId,
) -> (Name, base_db::TextRange) {
    match definition {
        ModuleDefinitionId::Function(id) => (
            database.function_data(id).0.name.clone(),
            id.lookup(database)
                .source(database)
                .original_file_range(database)
                .range,
        ),
        ModuleDefinitionId::GlobalVariable(id) => (
            database.global_var_data(id).0.name.clone(),
            id.lookup(database)
                .source(database)
                .original_file_range(database)
                .range,
        ),
        ModuleDefinitionId::GlobalConstant(id) => (
            database.global_constant_data(id).0.name.clone(),
            id.lookup(database)
                .source(database)
                .original_file_range(database)
                .range,
        ),
        ModuleDefinitionId::Override(id) => (
            database.override_data(id).0.name.clone(),
            id.lookup(database)
                .source(database)
                .original_file_range(database)
                .range,
        ),
        ModuleDefinitionId::Struct(id) => (
            database.struct_data(id).0.name.clone(),
            id.lookup(database)
                .source(database)
                .original_file_range(database)
                .range,
        ),
        ModuleDefinitionId::TypeAlias(id) => (
            database.type_alias_data(id).0.name.clone(),
            id.lookup(database)
                .source(database)
                .original_file_range(database)
                .range,
        ),
        ModuleDefinitionId::GlobalAssertStatement(id) => (
            Name::from("const_assert"),
            id.lookup(database)
                .source(database)
                .original_file_range(database)
                .range,
        ),
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum InferenceDiagnostic {
    AssignmentNotAReference {
        left_side: ExpressionId,
        actual: Type,
    },
    TypeMismatch {
        expression: ExpressionId,
        expected: TypeExpectation,
        actual: Type,
    },
    NoSuchField {
        expression: ExpressionId,
        name: Name,
        r#type: Type,
    },
    ArrayAccessInvalidType {
        expression: ExpressionId,
        r#type: Type,
    },
    UnresolvedName {
        expression: ExpressionId,
        name: Name,
    },
    InvalidConstructionType {
        expression: ExpressionId,
        r#type: Type,
    },
    FunctionCallArgCountMismatch {
        expression: ExpressionId,
        n_expected: usize,
        n_actual: usize,
    },
    NoBuiltinOverload {
        expression: ExpressionId,
        builtin: BuiltinId,
        name: Option<&'static str>,
        parameters: Vec<Type>,
    },
    NoConstructor {
        expression: ExpressionId,
        builtins: BuiltinId,
        r#type: Type,
        parameters: Vec<Type>,
    },
    AddressOfNotReference {
        expression: ExpressionId,
        actual: Type,
    },
    DerefNotAPointer {
        expression: ExpressionId,
        actual: Type,
    },
    InvalidType {
        source: ExpressionStoreSource,
        error: TypeLoweringError,
    },
    CyclicType {
        name: Name,
        range: base_db::TextRange,
    },
    UnexpectedTemplateArgument {
        expression: ExpressionId,
    },
    WgslError {
        expression: ExpressionId,
        message: String,
    },
    ExpectedLoweredKind {
        expression: ExpressionId,
        expected: LoweredKind,
        actual: LoweredKind,
        name: Name,
    },
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TypeContainer {
    Expression(ExpressionId),
    TypeSpecifier(TypeSpecifierId),
}

impl From<ExpressionId> for TypeContainer {
    fn from(id: ExpressionId) -> Self {
        Self::Expression(id)
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum ResolvedCall {
    Function(ResolvedFunctionId),
    OtherTypeInitializer(Type),
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct InternedStandardTypes {
    unknown: Type,
}

impl InternedStandardTypes {
    fn new(database: &dyn HirDatabase) -> Self {
        Self {
            unknown: TypeKind::Error.intern(database),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct InferenceResult {
    pub(crate) type_of_expression: ArenaMap<ExpressionId, Type>,
    pub(crate) type_of_binding: ArenaMap<BindingId, Type>,
    diagnostics: Vec<InferenceDiagnostic>,
    return_type: Type,
    call_resolutions: FxHashMap<ExpressionId, ResolvedCall>,
    field_resolutions: FxHashMap<ExpressionId, FieldId>,
    standard_types: InternedStandardTypes,
}

impl InferenceResult {
    fn new(database: &dyn HirDatabase) -> Self {
        Self {
            type_of_expression: ArenaMap::default(),
            type_of_binding: ArenaMap::default(),
            diagnostics: Vec::default(),
            return_type: TypeKind::Error.intern(database),
            call_resolutions: FxHashMap::default(),
            field_resolutions: FxHashMap::default(),
            standard_types: InternedStandardTypes::new(database),
        }
    }

    #[must_use]
    pub fn field_resolution(
        &self,
        expression: ExpressionId,
    ) -> Option<FieldId> {
        self.field_resolutions.get(&expression).copied()
    }

    #[must_use]
    pub fn call_resolution(
        &self,
        expression: ExpressionId,
    ) -> Option<ResolvedCall> {
        self.call_resolutions.get(&expression).copied()
    }

    #[must_use]
    pub fn diagnostics(&self) -> &[InferenceDiagnostic] {
        &self.diagnostics
    }

    #[must_use]
    pub const fn return_type(&self) -> Type {
        self.return_type
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.type_of_expression.values().next().is_none()
            && self.type_of_binding.values().next().is_none()
            && self.diagnostics.is_empty()
            && self.call_resolutions.is_empty()
            && self.field_resolutions.is_empty()
    }
}

impl Index<ExpressionId> for InferenceResult {
    type Output = Type;

    fn index(
        &self,
        index: ExpressionId,
    ) -> &Type {
        self.type_of_expression
            .get(index)
            .unwrap_or(&self.standard_types.unknown)
    }
}

impl Index<BindingId> for InferenceResult {
    type Output = Type;

    fn index(
        &self,
        index: BindingId,
    ) -> &Type {
        self.type_of_binding
            .get(index)
            .unwrap_or(&self.standard_types.unknown)
    }
}

/// Runs inference for items that have a body, such as functions
pub struct InferenceContext<'database> {
    database: &'database dyn HirDatabase,
    owner: ModuleDefinitionId,
    /// Root resolver for the entire module
    resolver: Resolver,
    result: InferenceResult, // set in collect_* calls
    return_type: Type,
}

impl<'database> InferenceContext<'database> {
    pub fn new(
        database: &'database dyn HirDatabase,
        owner: ModuleDefinitionId,
        resolver: Resolver,
    ) -> Self {
        Self {
            database,
            owner,
            resolver,
            result: InferenceResult::new(database),
            return_type: TypeKind::Error.intern(database),
        }
    }

    // pub fn with_store<T>(
    //     &mut self,
    //     store: &'database ExpressionStore,
    //     f: impl FnOnce(&mut InferenceContext<'_>) -> T,
    // ) -> T {
    //     let old_store = std::mem::replace(&mut self.store, store);
    //     let result = f(self);
    //     self.store = old_store;
    //     result
    // }

    fn set_expression_type(
        &mut self,
        expression: ExpressionId,
        r#type: Type,
    ) {
        self.result.type_of_expression.insert(expression, r#type);
    }

    fn set_binding_type(
        &mut self,
        binding: BindingId,
        r#type: Type,
    ) {
        self.result.type_of_binding.insert(binding, r#type);
    }

    fn bind_return_type(
        &mut self,
        r#type: Option<Type>,
        body: &Body,
    ) {
        if let Some(r#type) = r#type
            && let Some(binding) = body.main_binding
        {
            self.set_binding_type(binding, r#type);
        }

        self.return_type = r#type.unwrap_or_else(|| self.error_type());
    }

    fn set_field_resolution(
        &mut self,
        expression: ExpressionId,
        field: FieldId,
    ) {
        self.result.field_resolutions.insert(expression, field);
    }

    fn push_diagnostic(
        &mut self,
        diagnostic: InferenceDiagnostic,
    ) {
        self.result.diagnostics.push(diagnostic);
    }

    fn push_lowering_diagnostics(
        &mut self,
        diagnostics: &mut Vec<TypeLoweringError>,
        store: &ExpressionStore,
    ) {
        for diagnostic in diagnostics.drain(..) {
            self.push_diagnostic(InferenceDiagnostic::InvalidType {
                source: store.store_source,
                error: diagnostic,
            });
        }
    }

    fn resolve_all(mut self) -> InferenceResult {
        self.result.return_type = self.return_type;
        self.result
    }

    fn collect_global_variable(
        &mut self,
        variable: &GlobalVariableData,
        body: &Body,
    ) -> Option<Type> {
        let r#type = variable
            .r#type
            .map(|r#type| self.lower_type(r#type, &self.resolver.clone(), &variable.store));

        self.bind_return_type(r#type, body);
        r#type
    }

    fn infer_global_variable(
        &mut self,
        variable: &GlobalVariableData,
        body: &Body,
    ) {
        let (address_space, access_mode) =
            self.infer_variable_template(&variable.template_parameters, &variable.store);
        if address_space == AddressSpace::Function {
            // Function address space is not allowed at the module level
            self.push_diagnostic(InferenceDiagnostic::UnexpectedTemplateArgument {
                expression: variable.template_parameters[0],
            });
        }

        self.bind_return_type(
            Some(self.make_ref(self.return_type, address_space, access_mode)),
            body,
        );
    }

    fn infer_variable_template(
        &mut self,
        template: &[ExpressionId],
        store: &ExpressionStore,
    ) -> (AddressSpace, AccessMode) {
        let mut context = TypeLoweringContext::new(self.database, &self.resolver, store);
        let template_args: Vec<_> = template
            .iter()
            .map(|argument| context.evaluate_template_argument(*argument))
            .collect();
        self.push_lowering_diagnostics(&mut context.diagnostics, store);

        let default_address_space = match store.store_source {
            ExpressionStoreSource::Body => AddressSpace::Function,
            ExpressionStoreSource::Signature => AddressSpace::Handle,
        };

        let address_space = match template_args.first() {
            Some(TemplateParameter::Enumerant(Enumerant::AddressSpace(address_space))) => {
                *address_space
            },
            None => default_address_space,
            _ => {
                self.push_diagnostic(InferenceDiagnostic::UnexpectedTemplateArgument {
                    expression: template[0],
                });
                default_address_space
            },
        };
        let access_mode = match template_args.get(1) {
            Some(TemplateParameter::Enumerant(Enumerant::AccessMode(access_mode))) => {
                if address_space == AddressSpace::Storage {
                    *access_mode
                } else {
                    // Only the storage address space allows for an access mode
                    self.push_diagnostic(InferenceDiagnostic::UnexpectedTemplateArgument {
                        expression: template[0],
                    });
                    address_space.default_access_mode()
                }
            },
            None => address_space.default_access_mode(),
            _ => {
                self.push_diagnostic(InferenceDiagnostic::UnexpectedTemplateArgument {
                    expression: template[0],
                });
                address_space.default_access_mode()
            },
        };

        // Mark extra template arguments as errors
        if template.len() > 2 {
            for expression in &template[2..] {
                self.push_diagnostic(InferenceDiagnostic::UnexpectedTemplateArgument {
                    expression: *expression,
                });
            }
        }
        (address_space, access_mode)
    }

    fn collect_global_constant(
        &mut self,
        constant: &GlobalConstantData,
        body: &Body,
    ) -> Option<Type> {
        let r#type = constant
            .r#type
            .map(|r#type| self.lower_type(r#type, &self.resolver.clone(), &constant.store));

        self.bind_return_type(r#type, body);
        r#type
    }

    fn collect_override(
        &mut self,
        override_data: &OverrideData,
        body: &Body,
    ) -> Option<Type> {
        let r#type = override_data
            .r#type
            .map(|r#type| self.lower_type(r#type, &self.resolver.clone(), &override_data.store));

        self.bind_return_type(r#type, body);
        r#type
    }

    fn collect_fn(
        &mut self,
        function_data: &FunctionData,
        body: &Body,
    ) -> Option<Type> {
        for ((_, parameter), &binding_id) in function_data.parameters.iter().zip(&body.parameters) {
            let parameter_type = self.lower_type(
                parameter.r#type,
                &self.resolver.clone(),
                &function_data.store,
            );
            self.set_binding_type(binding_id, parameter_type);
        }
        let r#type = function_data.return_type.map(|type_ref| {
            self.lower_type(type_ref, &self.resolver.clone(), &function_data.store)
        });
        self.return_type = r#type.unwrap_or_else(|| self.error_type());
        r#type
    }

    /// Runs type inference on the body and infer the type for `const`s, `var`s and `override`s
    fn infer_body(
        &mut self,
        body: &Body,
        return_type: Option<Type>,
        abstract_handling: AbstractHandling,
    ) {
        match body.root {
            Some(Either::Left(statement)) => {
                self.infer_statement(statement, body);
            },
            Some(Either::Right(expression)) => {
                let r#type =
                    self.infer_initializer(body, Some(expression), return_type, abstract_handling);

                if return_type.is_none() {
                    self.bind_return_type(Some(r#type), body);
                }
            },
            None => (),
        }
    }

    fn resolver_for_expression(
        &self,
        expression: ExpressionId,
    ) -> Option<Resolver> {
        let ModuleDefinitionId::Function(function) = self.owner else {
            return None;
        };
        let expression_scopes = self
            .database
            .expression_scopes(DefinitionWithBodyId::Function(function));

        let scope_id = expression_scopes.scope_for_expression(expression)?;

        Some(
            self.resolver
                .clone()
                .push_expression_scope(function, expression_scopes, scope_id),
        )
    }

    fn resolver_for_statement(
        &self,
        statement: StatementId,
    ) -> Resolver {
        let ModuleDefinitionId::Function(function) = self.owner else {
            return self.resolver.clone();
        };

        let expression_scopes = self
            .database
            .expression_scopes(DefinitionWithBodyId::Function(function));

        if let Some(scope_id) = expression_scopes.scope_for_statement(statement) {
            self.resolver
                .clone()
                .push_expression_scope(function, expression_scopes, scope_id)
        } else {
            self.resolver.clone()
        }
    }

    #[expect(clippy::too_many_lines, reason = "match with many small cases")]
    fn infer_statement(
        &mut self,
        statement: StatementId,
        body: &Body,
    ) {
        let resolver = self.resolver_for_statement(statement);

        match &body.statements[statement] {
            Statement::Compound { statements } => {
                for statement in statements {
                    self.infer_statement(*statement, body);
                }
            },
            Statement::Variable {
                binding_id,
                type_ref,
                initializer,
                template_parameters,
            } => {
                let r#type = type_ref.map(|r#type| self.lower_type(r#type, &resolver, body));
                let r#type = self.infer_initializer(
                    body,
                    *initializer,
                    r#type,
                    AbstractHandling::Concretize,
                );

                let (address_space, access_mode) =
                    self.infer_variable_template(template_parameters, body);
                if address_space != AddressSpace::Function {
                    // Only function address space is allowed
                    self.push_diagnostic(InferenceDiagnostic::UnexpectedTemplateArgument {
                        expression: template_parameters[0],
                    });
                }
                let reference_type = self.make_ref(r#type, address_space, access_mode);
                self.set_binding_type(*binding_id, reference_type);
            },
            Statement::Const {
                binding_id,
                type_ref,
                initializer,
                ..
            } => {
                let r#type = type_ref.map(|r#type| self.lower_type(r#type, &resolver, body));
                let r#type =
                    self.infer_initializer(body, *initializer, r#type, AbstractHandling::Abstract);
                self.set_binding_type(*binding_id, r#type);
            },
            Statement::Let {
                binding_id,
                type_ref,
                initializer,
                ..
            } => {
                let r#type = type_ref.map(|r#type| self.lower_type(r#type, &resolver, body));
                let r#type = self.infer_initializer(
                    body,
                    *initializer,
                    r#type,
                    AbstractHandling::Concretize,
                );
                self.set_binding_type(*binding_id, r#type);
            },

            Statement::Return { expression } => {
                if let Some(expression) = expression {
                    self.infer_expression_expect(
                        *expression,
                        &TypeExpectation::from_type(self.return_type),
                        body,
                    );
                }
            },
            Statement::Assignment {
                left_side,
                right_side,
            } => {
                let left_type = self.infer_expression(*left_side, body);

                let kind = left_type.kind(self.database);
                let left_inner = if let TypeKind::Reference(reference) = kind {
                    reference.inner
                } else {
                    self.push_diagnostic(InferenceDiagnostic::AssignmentNotAReference {
                        left_side: *left_side,
                        actual: left_type,
                    });
                    self.error_type()
                };

                self.infer_expression_expect(
                    *right_side,
                    &TypeExpectation::from_type(left_inner),
                    body,
                );
            },
            Statement::CompoundAssignment {
                left_side,
                right_side,
                operator,
            } => {
                let left_type = self.infer_expression(*left_side, body);

                let left_kind = left_type.kind(self.database);
                let left_inner = if let TypeKind::Reference(reference) = left_kind {
                    reference.inner
                } else {
                    self.push_diagnostic(InferenceDiagnostic::AssignmentNotAReference {
                        left_side: *left_side,
                        actual: left_type,
                    });
                    self.error_type()
                };

                let r#type = self.infer_binary_op(
                    *right_side,
                    *left_side,
                    *right_side,
                    (*operator).into(),
                    body,
                );

                if !r#type.is_convertible_to(left_inner, self.database) {
                    self.push_diagnostic(InferenceDiagnostic::TypeMismatch {
                        expression: *right_side,
                        actual: r#type,
                        expected: TypeExpectation::Type(TypeExpectationInner::Exact(left_inner)),
                    });
                }
            },
            Statement::PhonyAssignment { right_side } => {
                self.infer_expression(*right_side, body);
            },
            Statement::IncrDecr { expression, .. } => {
                let left_type = self.infer_expression(*expression, body);

                let left_kind = left_type.kind(self.database);
                let left_inner = if let TypeKind::Reference(reference) = left_kind {
                    reference.inner
                } else {
                    self.push_diagnostic(InferenceDiagnostic::AssignmentNotAReference {
                        left_side: *expression,
                        actual: left_type,
                    });
                    self.error_type()
                };

                if self
                    .expect_type_inner(left_inner, &TypeExpectationInner::IntegerScalar)
                    .is_err()
                {
                    self.push_diagnostic(InferenceDiagnostic::TypeMismatch {
                        expression: *expression,
                        actual: left_inner,
                        expected: TypeExpectation::Type(TypeExpectationInner::IntegerScalar),
                    });
                }
            },
            Statement::If {
                condition,
                block,
                else_if_blocks,
                else_block,
            } => {
                self.infer_statement(*block, body);
                for else_if_block in else_if_blocks {
                    self.infer_statement(*else_if_block, body);
                }
                if let Some(else_block) = else_block {
                    self.infer_statement(*else_block, body);
                }
                self.infer_expression_expect(
                    *condition,
                    &TypeExpectation::from_type(self.bool_type()),
                    body,
                );
            },
            Statement::While { condition, block } => {
                self.infer_statement(*block, body);
                self.infer_expression_expect(
                    *condition,
                    &TypeExpectation::from_type(self.bool_type()),
                    body,
                );
            },
            Statement::Switch {
                expression,
                case_blocks,
            } => {
                let r#type = self
                    .infer_expression(*expression, body)
                    .unref(self.database);

                for (selectors, case) in case_blocks {
                    for selector in selectors {
                        if let SwitchCaseSelector::Expression(selector) = selector {
                            self.infer_expression_expect(
                                *selector,
                                &TypeExpectation::from_type(r#type),
                                body,
                            );
                        }
                    }
                    self.infer_statement(*case, body);
                }
            },
            Statement::For {
                initializer,
                condition,
                continuing_part,
                block,
            } => {
                if let Some(init) = initializer {
                    self.infer_statement(*init, body);
                }
                if let Some(cont) = continuing_part {
                    self.infer_statement(*cont, body);
                }

                if let Some(condition) = condition {
                    self.infer_expression_expect(
                        *condition,
                        &TypeExpectation::from_type(self.bool_type()),
                        body,
                    );
                }

                self.infer_statement(*block, body);
            },
            Statement::Loop { body: loop_body } => {
                self.infer_statement(*loop_body, body);
            },
            Statement::Assert { expression } => {
                self.infer_expression_expect(
                    *expression,
                    &TypeExpectation::from_type(self.bool_type()),
                    body,
                );
            },
            Statement::Discard | Statement::Break | Statement::Continue | Statement::Missing => {},
            Statement::Continuing { block } => self.infer_statement(*block, body),
            Statement::BreakIf { condition } => {
                self.infer_expression_expect(
                    *condition,
                    &TypeExpectation::from_type(self.bool_type()),
                    body,
                );
            },
            Statement::Expression { expression } => {
                self.infer_expression(*expression, body);
            },
        }
    }

    fn infer_initializer(
        &mut self,
        store: &ExpressionStore,
        initializer: Option<ExpressionId>,
        r#type: Option<Type>,
        abstract_handling: AbstractHandling,
    ) -> Type {
        match (r#type, initializer) {
            (Some(r#type), Some(initializer)) => {
                self.infer_expression_expect(
                    initializer,
                    &TypeExpectation::from_type(r#type),
                    store,
                );
                r#type
            },
            (Some(r#type), None) => r#type,
            (None, Some(initializer)) => {
                let r#type = self
                    .infer_expression(initializer, store)
                    .unref(self.database);
                if abstract_handling == AbstractHandling::Concretize {
                    r#type.concretize(self.database)
                } else {
                    r#type
                }
            },
            (None, None) => self.error_type(),
        }
    }

    fn expect_type_inner(
        &self,
        r#type: Type,
        expectation: &TypeExpectationInner,
    ) -> Result<(), ()> {
        let type_kind = r#type.kind(self.database);
        if type_kind == TypeKind::Error {
            return Ok(());
        }

        match *expectation {
            TypeExpectationInner::Exact(expected_type) => {
                if expected_type.kind(self.database) == TypeKind::Error
                    || r#type.is_convertible_to(expected_type, self.database)
                {
                    Ok(())
                } else {
                    Err(())
                }
            },
            TypeExpectationInner::IntegerScalar => {
                if let TypeKind::Scalar(ScalarType::I32 | ScalarType::U32) =
                    r#type.kind(self.database).unref(self.database).as_ref()
                {
                    Ok(())
                } else {
                    Err(())
                }
            },
        }
    }

    fn infer_expression_expect(
        &mut self,
        expression: ExpressionId,
        expected: &TypeExpectation,
        store: &ExpressionStore,
    ) -> Type {
        let r#type = self
            .infer_expression(expression, store)
            .unref(self.database);

        match expected {
            TypeExpectation::Type(expected_type) => {
                if self.expect_type_inner(r#type, expected_type) != Ok(()) {
                    self.push_diagnostic(InferenceDiagnostic::TypeMismatch {
                        expression,
                        actual: r#type,
                        expected: expected.clone(),
                    });
                }
            },
            TypeExpectation::Any => {},
        }
        r#type
    }

    #[expect(clippy::too_many_lines, reason = "match with many small cases")]
    fn infer_expression(
        &mut self,
        expression: ExpressionId,
        store: &ExpressionStore,
    ) -> Type {
        let r#type = match &store[expression] {
            Expression::Missing => self.error_type(), // this would be a parser error
            Expression::BinaryOperation {
                left_side,
                right_side,
                operation,
            } => self.infer_binary_op(expression, *left_side, *right_side, *operation, store),
            Expression::UnaryOperator {
                expression,
                operator,
            } => self.infer_unary_op(*expression, *operator, store),
            Expression::Field {
                expression: field_expression,
                name,
            } => {
                let expression_type = self.infer_expression(*field_expression, store);
                if expression_type.is_err(self.database) {
                    return self.error_type();
                }

                match expression_type
                    .kind(self.database)
                    .unref(self.database)
                    .as_ref()
                {
                    TypeKind::Struct(r#struct) => {
                        let struct_data = self.database.struct_data(*r#struct).0;
                        let field_types = &self.database.field_types(*r#struct).0;

                        if let Some(field) = struct_data.field(name) {
                            self.set_field_resolution(
                                expression,
                                FieldId {
                                    r#struct: *r#struct,
                                    field,
                                },
                            );

                            let field_type = field_types[field];
                            // TODO: correct Address Spaces/access mode
                            // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/650
                            self.make_ref(field_type, AddressSpace::Private, AccessMode::ReadWrite)
                        } else {
                            self.push_diagnostic(InferenceDiagnostic::NoSuchField {
                                expression: *field_expression,
                                name: name.clone(),
                                r#type: expression_type,
                            });
                            self.error_type()
                        }
                    },
                    TypeKind::Vector(vec_type) => {
                        if let Ok(r#type) = self.vec_swizzle(vec_type, name) {
                            r#type
                        } else {
                            self.push_diagnostic(InferenceDiagnostic::NoSuchField {
                                expression: *field_expression,
                                name: name.clone(),
                                r#type: expression_type,
                            });
                            self.error_type()
                        }
                    },
                    TypeKind::Error
                    | TypeKind::Scalar(_)
                    | TypeKind::Atomic(_)
                    | TypeKind::Matrix(_)
                    | TypeKind::Array(_)
                    | TypeKind::Texture(_)
                    | TypeKind::Sampler(_)
                    | TypeKind::Reference(_)
                    | TypeKind::Pointer(_)
                    | TypeKind::BoundVariable(_)
                    | TypeKind::StorageTypeOfTexelFormat(_) => {
                        self.push_diagnostic(InferenceDiagnostic::NoSuchField {
                            expression: *field_expression,
                            name: name.clone(),
                            r#type: expression_type,
                        });
                        self.error_type()
                    },
                }
            },
            Expression::Call {
                ident_expression,
                arguments,
            } => {
                let arguments: Vec<_> = arguments
                    .iter()
                    .map(|&argument| self.infer_expression(argument, store).unref(self.database))
                    .collect();
                self.infer_call(expression, ident_expression, arguments, store)
            },
            Expression::Index { left_side, index } => {
                let left_side = self.infer_expression(*left_side, store);
                let _index_expression = self.infer_expression(*index, store);
                // TODO: check that the type of the index expression makes sense. Can't index with a f32, for example.
                // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/671
                let left_kind = left_side.kind(self.database);
                let is_reference = matches!(left_kind, TypeKind::Reference(_));

                let left_inner = left_kind.unref(self.database);

                let r#type = match &*left_inner {
                    TypeKind::Vector(vec) => vec.component_type,
                    TypeKind::Matrix(matrix_type) => {
                        self.database.intern_type(TypeKind::Vector(VectorType {
                            size: matrix_type.rows,
                            component_type: matrix_type.inner,
                        }))
                    },
                    TypeKind::Array(array) => array.inner,
                    TypeKind::Error
                    | TypeKind::Scalar(_)
                    | TypeKind::Atomic(_)
                    | TypeKind::Struct(_)
                    | TypeKind::Texture(_)
                    | TypeKind::Sampler(_)
                    | TypeKind::Reference(_)
                    | TypeKind::Pointer(_)
                    | TypeKind::BoundVariable(_)
                    | TypeKind::StorageTypeOfTexelFormat(_) => {
                        self.push_diagnostic(InferenceDiagnostic::ArrayAccessInvalidType {
                            expression,
                            r#type: left_side,
                        });
                        self.error_type()
                    },
                };

                if is_reference {
                    self.make_ref(r#type, AddressSpace::Private, AccessMode::ReadWrite)
                } else {
                    r#type
                }
            },
            Expression::Literal(literal) => {
                use hir_def::expression::{BuiltinFloat, BuiltinInt, Literal};
                let type_kind = match literal {
                    Literal::Int(_, BuiltinInt::I32) => TypeKind::Scalar(ScalarType::I32),
                    Literal::Int(_, BuiltinInt::U32) => TypeKind::Scalar(ScalarType::U32),
                    Literal::Int(_, BuiltinInt::Abstract) => {
                        TypeKind::Scalar(ScalarType::AbstractInt)
                    },
                    Literal::Float(_, BuiltinFloat::F16) => TypeKind::Scalar(ScalarType::F16),
                    Literal::Float(_, BuiltinFloat::F32) => TypeKind::Scalar(ScalarType::F32),
                    Literal::Float(_, BuiltinFloat::Abstract) => {
                        TypeKind::Scalar(ScalarType::AbstractFloat)
                    },
                    Literal::Bool(_) => TypeKind::Scalar(ScalarType::Bool),
                };
                self.database.intern_type(type_kind)
            },
            Expression::IdentExpression(ident_expression) => {
                self.infer_ident_expression(expression, ident_expression, store)
            },
        };

        self.set_expression_type(expression, r#type);

        r#type
    }

    fn validate_function_call(
        &mut self,
        function: &FunctionDetails,
        arguments: &[Type],
        callee: ExpressionId,
        expression: ExpressionId,
    ) -> Type {
        if function.parameters.len() == arguments.len() {
            for (expected, actual) in function.parameters().zip(arguments.iter().copied()) {
                if !actual.is_convertible_to(expected, self.database) {
                    self.push_diagnostic(InferenceDiagnostic::TypeMismatch {
                        expression,
                        actual,
                        expected: TypeExpectation::Type(TypeExpectationInner::Exact(expected)),
                    });
                }
            }

            function.return_type.unwrap_or_else(|| self.error_type())
        } else {
            self.push_diagnostic(InferenceDiagnostic::FunctionCallArgCountMismatch {
                expression: callee,
                n_expected: function.parameters.len(),
                n_actual: arguments.len(),
            });
            self.error_type()
        }
    }

    fn infer_unary_op(
        &mut self,
        expression: ExpressionId,
        operator: UnaryOperator,
        store: &ExpressionStore,
    ) -> Type {
        let expression_type = self.infer_expression(expression, store);
        if expression_type.is_err(self.database) {
            return self.error_type();
        }

        let builtin = match operator {
            UnaryOperator::Negation => {
                Builtin::builtin_op_unary_minus(self.database).intern(self.database)
            },
            UnaryOperator::LogicalNegation => {
                Builtin::builtin_op_unary_not(self.database).intern(self.database)
            },
            UnaryOperator::BitwiseComplement => {
                Builtin::builtin_op_unary_bitnot(self.database).intern(self.database)
            },
            UnaryOperator::AddressOf => {
                if let TypeKind::Reference(reference) = expression_type.kind(self.database) {
                    return self.ref_to_pointer(&reference);
                }
                self.push_diagnostic(InferenceDiagnostic::AddressOfNotReference {
                    expression,
                    actual: expression_type,
                });
                return self.error_type();
            },
            UnaryOperator::Indirection => {
                let argument_type = expression_type.unref(self.database);
                if let TypeKind::Pointer(pointer) = argument_type.kind(self.database) {
                    return self.ptr_to_ref(&pointer);
                }
                self.push_diagnostic(InferenceDiagnostic::DerefNotAPointer {
                    expression,
                    actual: argument_type,
                });
                return self.error_type();
            },
        };

        let argument_type = expression_type.unref(self.database);
        self.call_builtin(
            expression,
            builtin,
            &[argument_type],
            Some(operator.symbol()),
        )
    }

    fn infer_binary_op(
        &mut self,
        expression: ExpressionId,
        left_side: ExpressionId,
        right_side: ExpressionId,
        operation: BinaryOperation,
        store: &ExpressionStore,
    ) -> Type {
        let left_type = self.infer_expression(left_side, store).unref(self.database);
        let rhs_type = self
            .infer_expression(right_side, store)
            .unref(self.database);

        if left_type.is_err(self.database) || rhs_type.is_err(self.database) {
            return self.error_type();
        }

        let builtin = match operation {
            BinaryOperation::Logical(_) => {
                Builtin::builtin_op_binary_bool(self.database).intern(self.database)
            },
            BinaryOperation::Arithmetic(operation) => match operation {
                ArithmeticOperation::BitwiseOr
                | ArithmeticOperation::BitwiseAnd
                | ArithmeticOperation::BitwiseXor => {
                    Builtin::builtin_op_binary_bitop(self.database).intern(self.database)
                },
                ArithmeticOperation::Multiplication => {
                    Builtin::builtin_op_binary_mul(self.database).intern(self.database)
                },
                ArithmeticOperation::Division => {
                    Builtin::builtin_op_binary_div(self.database).intern(self.database)
                },
                ArithmeticOperation::Addition
                | ArithmeticOperation::Subtraction
                | ArithmeticOperation::Remainder => {
                    Builtin::builtin_op_binary_number(self.database).intern(self.database)
                },
                ArithmeticOperation::ShiftLeft | ArithmeticOperation::ShiftRight => {
                    Builtin::builtin_op_binary_shift(self.database).intern(self.database)
                },
            },
            BinaryOperation::Comparison(cmp) => match cmp {
                ComparisonOperation::Equality | ComparisonOperation::Inequality => {
                    Builtin::builtin_op_eq(self.database).intern(self.database)
                },
                ComparisonOperation::LessThan
                | ComparisonOperation::LessThanEqual
                | ComparisonOperation::GreaterThan
                | ComparisonOperation::GreaterThanEqual => {
                    Builtin::builtin_op_cmp(self.database).intern(self.database)
                },
            },
        };

        self.call_builtin(
            expression,
            builtin,
            &[left_type, rhs_type],
            Some(operation.symbol()),
        )
    }

    fn infer_ident_expression(
        &mut self,
        expression: ExpressionId,
        ident_expression: &IdentExpression,
        store: &ExpressionStore,
    ) -> Type {
        let resolver = self.resolver_for_expression(expression);
        let mut context = TypeLoweringContext::new(
            self.database,
            resolver.as_ref().unwrap_or(&self.resolver),
            store,
        );
        let lowered = context.lower(
            TypeContainer::Expression(expression),
            &ident_expression.path,
            &ident_expression.template_parameters,
        );
        self.push_lowering_diagnostics(&mut context.diagnostics, store);

        match lowered {
            Lowered::GlobalConstant(id) => {
                self.database
                    .infer(DefinitionWithBodyId::GlobalConstant(id))
                    .return_type
            },
            Lowered::GlobalVariable(id) => {
                self.database
                    .infer(DefinitionWithBodyId::GlobalVariable(id))
                    .return_type
            },
            Lowered::Override(id) => {
                self.database
                    .infer(DefinitionWithBodyId::Override(id))
                    .return_type
            },
            Lowered::Local(id) => self.result.type_of_binding[id],
            Lowered::Type(_)
            | Lowered::TypeWithoutTemplate(_)
            | Lowered::Function(_)
            | Lowered::BuiltinFunction
            | Lowered::Enumerant(_) => {
                self.push_diagnostic(InferenceDiagnostic::ExpectedLoweredKind {
                    expression,
                    expected: LoweredKind::Variable,
                    actual: lowered.kind(),
                    name: ident_expression.path.clone(),
                });
                self.error_type()
            },
        }
    }

    fn builtin_vector_inferred_constructor(
        &self,
        size: VecDimensionality,
    ) -> BuiltinId {
        match size {
            VecDimensionality::Two => Builtin::builtin_op_vec2_constructor(self.database),
            VecDimensionality::Three => Builtin::builtin_op_vec3_constructor(self.database),
            VecDimensionality::Four => Builtin::builtin_op_vec4_constructor(self.database),
        }
        .intern(self.database)
    }

    fn builtin_matrix_inferred_constructor(
        &self,
        columns: VecDimensionality,
        rows: VecDimensionality,
    ) -> BuiltinId {
        use type_ref::VecDimensionality::{Four, Three, Two};
        match (columns, rows) {
            (Two, Two) => Builtin::builtin_op_mat2x2_constructor(self.database),
            (Two, Three) => Builtin::builtin_op_mat2x3_constructor(self.database),
            (Two, Four) => Builtin::builtin_op_mat2x4_constructor(self.database),
            (Three, Two) => Builtin::builtin_op_mat3x2_constructor(self.database),
            (Three, Three) => Builtin::builtin_op_mat3x3_constructor(self.database),
            (Three, Four) => Builtin::builtin_op_mat3x4_constructor(self.database),
            (Four, Two) => Builtin::builtin_op_mat4x2_constructor(self.database),
            (Four, Three) => Builtin::builtin_op_mat4x3_constructor(self.database),
            (Four, Four) => Builtin::builtin_op_mat4x4_constructor(self.database),
        }
        .intern(self.database)
    }

    fn type_from_vec_size(
        &self,
        inner: Type,
        vec_size: u8,
    ) -> Type {
        if vec_size == 1 {
            inner
        } else {
            let kind = vec_size
                .try_into()
                .map(|size| {
                    TypeKind::Vector(VectorType {
                        size,
                        component_type: inner,
                    })
                })
                .unwrap_or(TypeKind::Error);
            self.database.intern_type(kind)
        }
    }

    fn vec_swizzle(
        &self,
        vector_type: &VectorType,
        name: &Name,
    ) -> Result<Type, ()> {
        const SWIZZLES: [[char; 4]; 2] = [['x', 'y', 'z', 'w'], ['r', 'g', 'b', 'a']];
        let max_size = 4;
        let max_swizzle_index = vector_type.size.as_u8();

        if name.as_str().len() > max_size {
            return Err(());
        }

        for swizzle in &SWIZZLES {
            let allowed_chars = &swizzle[..(usize::from(max_swizzle_index))];
            if name
                .as_str()
                .chars()
                .all(|character| allowed_chars.contains(&character))
            {
                let r#type = self.type_from_vec_size(
                    vector_type.component_type,
                    u8::try_from(name.as_str().len()).unwrap(),
                );
                // TODO: check correctness
                // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/650
                let result_type =
                    self.make_ref(r#type, AddressSpace::Function, AccessMode::ReadWrite);
                return Ok(result_type);
            }
        }

        Err(())
    }

    fn call_builtin(
        &mut self,
        expression: ExpressionId,
        builtin_id: BuiltinId,
        arguments: &[Type],
        name: Option<&'static str>,
    ) -> Type {
        self.call_builtin_inner(expression, builtin_id, arguments, name)
    }

    fn call_builtin_inner(
        &mut self,
        expression: ExpressionId,
        builtin_id: BuiltinId,
        arguments: &[Type],
        name: Option<&'static str>,
    ) -> Type {
        if let Ok((return_type, overload_id)) = self.try_call_builtin(builtin_id, arguments) {
            let builtin = builtin_id.lookup(self.database);
            let resolved = builtin.overload(overload_id).r#type;
            self.result
                .call_resolutions
                .insert(expression, ResolvedCall::Function(resolved));
            return_type
        } else {
            self.push_diagnostic(InferenceDiagnostic::NoBuiltinOverload {
                expression,
                builtin: builtin_id,
                name,
                parameters: arguments.to_vec(),
            });
            self.error_type()
        }
    }

    fn try_call_builtin(
        &self,
        builtin_id: BuiltinId,
        arguments: &[Type],
    ) -> Result<(Type, BuiltinOverloadId), ()> {
        let builtin = builtin_id.lookup(self.database);
        for (overload_id, overload) in builtin.overloads() {
            // Hack: overload resolution algorithm is not implemented here or used
            // here because it is the same as just picking the first valid overload.
            if let Ok((r#type, _conversion_rank)) = self.call_builtin_overload(overload, arguments)
            {
                return Ok((r#type, overload_id));
            }
        }
        Err(())
    }

    fn call_builtin_overload(
        &self,
        signature: &BuiltinOverload,
        arguments: &[Type],
    ) -> Result<(Type, u32), ()> {
        let function_type = signature.r#type.lookup(self.database);

        if function_type.parameters.len() != arguments.len() {
            return Err(());
        }

        let conversion_rank = 0;
        let mut unification_table = UnificationTable::default();
        for (expected, &found) in function_type.parameters().zip(arguments.iter()) {
            unify(self.database, &mut unification_table, expected, found)?;
        }

        let return_type = function_type
            .return_type
            .map(|r#type| unification_table.resolve(self.database, r#type));

        Ok((
            return_type.unwrap_or_else(|| self.error_type()),
            conversion_rank,
        ))
    }

    fn infer_call(
        &mut self,
        expression: ExpressionId,
        callee: &IdentExpression,
        arguments: Vec<Type>,
        store: &ExpressionStore,
    ) -> Type {
        let resolver = self
            .resolver_for_expression(expression)
            .unwrap_or_else(|| self.resolver.clone());
        let mut context = TypeLoweringContext::new(self.database, &resolver, store);
        let lowered = context.lower(
            TypeContainer::Expression(expression),
            &callee.path,
            &callee.template_parameters,
        );
        self.push_lowering_diagnostics(&mut context.diagnostics, store);

        match lowered {
            Lowered::Type(r#type) => {
                self.call_templated_type_constructor(expression, r#type, arguments)
            },
            Lowered::TypeWithoutTemplate(r#type) => {
                self.call_type_without_template_constructor(expression, r#type, arguments)
            },
            Lowered::Function(id) => {
                let details = id.lookup(self.database);
                self.result
                    .call_resolutions
                    .insert(expression, ResolvedCall::Function(id));
                self.validate_function_call(&details, &arguments, expression, expression)
            },
            Lowered::BuiltinFunction => {
                let template_args = context.eval_template_args(
                    TypeContainer::Expression(expression),
                    &callee.template_parameters,
                );
                self.push_lowering_diagnostics(&mut context.diagnostics, store);
                self.call_builtin_function(expression, callee, template_args, &arguments)
            },
            Lowered::Enumerant(_)
            | Lowered::GlobalConstant(_)
            | Lowered::GlobalVariable(_)
            | Lowered::Override(_)
            | Lowered::Local(_) => {
                self.push_diagnostic(InferenceDiagnostic::ExpectedLoweredKind {
                    expression,
                    expected: LoweredKind::Function,
                    actual: lowered.kind(),
                    name: callee.path.clone(),
                });
                self.error_type()
            },
        }
    }

    fn call_builtin_function(
        &mut self,
        expression: ExpressionId,
        callee: &IdentExpression,
        mut template_parameters: TemplateParameters,
        arguments: &[Type],
    ) -> Type {
        let mut converter = WgslTypeConverter::new(self.database);
        let mut template_args = vec![];
        while let Some((template_parameter, _)) = template_parameters.next() {
            if let Some(template_parameter) =
                converter.template_parameter_to_wgsl_types(template_parameter)
            {
                template_args.push(template_parameter);
            } else {
                self.push_diagnostic(InferenceDiagnostic::WgslError {
                    expression,
                    message:
                        "internal error: wgsl-types did not align with wgsl-analyzer's type system"
                            .to_owned(),
                });
                return self.error_type();
            }
        }
        let template_args = if template_args.is_empty() {
            None
        } else {
            Some(template_args.as_slice())
        };

        let converted_arguments: Option<Vec<_>> = arguments
            .iter()
            .map(|r#type| converter.to_wgsl_types(*r#type))
            .collect();

        let Some(converted_arguments) = converted_arguments else {
            // One of the arguments had an error type
            return self.error_type();
        };

        let return_type = wgsl_types::builtin::type_builtin_fn(
            callee.path.as_str(),
            template_args,
            &converted_arguments,
        );

        match return_type {
            Ok(Some(r#type)) => converter.from_wgsl_types(r#type),
            Ok(None) => self.error_type(),
            Err(error) => {
                self.push_diagnostic(InferenceDiagnostic::WgslError {
                    expression,
                    message: error.to_string(),
                });
                self.error_type()
            },
        }
    }

    #[expect(
        clippy::too_many_lines,
        reason = "large bug not complex match expression"
    )]
    /// Constructor for a type with a fully specified template
    fn call_templated_type_constructor(
        &mut self,
        expression: ExpressionId,
        r#type: Type,
        arguments: Vec<Type>,
    ) -> Type {
        fn size_to_dimension(size: VecSize) -> VecDimensionality {
            match size {
                VecSize::Two => VecDimensionality::Two,
                VecSize::Three => VecDimensionality::Three,
                VecSize::Four => VecDimensionality::Four,
                #[expect(
                    clippy::unreachable,
                    reason = "this is by far the easiest way to handle it, at least for now"
                )]
                VecSize::BoundVariable(_) => {
                    unreachable!("Can never have unbound type at this point")
                },
            }
        }

        match r#type.kind(self.database) {
            TypeKind::Scalar(scalar_type) => {
                self.call_scalar_constructor(scalar_type, expression, r#type, arguments)
            },
            TypeKind::Array(array_type) => {
                for argument in &arguments {
                    if !argument.is_convertible_to(array_type.inner, self.database) {
                        self.push_diagnostic(InferenceDiagnostic::TypeMismatch {
                            expression,
                            expected: TypeExpectation::Type(TypeExpectationInner::Exact(
                                array_type.inner,
                            )),
                            actual: *argument,
                        });
                    }
                }
                #[expect(
                    clippy::as_conversions,
                    reason = "constructing an array with too many parameters is an error anyway"
                )]
                if let ArraySize::Constant(size) = array_type.size
                    && arguments.len() != size as usize
                {
                    self.push_diagnostic(InferenceDiagnostic::FunctionCallArgCountMismatch {
                        expression,
                        n_expected: size as usize,
                        n_actual: arguments.len(),
                    });
                }
                r#type
            },
            TypeKind::Vector(vec) => {
                if arguments.is_empty() {
                    return r#type;
                }
                let construction_builtin_id =
                    self.builtin_vector_inferred_constructor(size_to_dimension(vec.size));
                let construction_result =
                    self.try_call_builtin(construction_builtin_id, &arguments);

                if construction_result.is_ok() {
                    r#type
                } else {
                    self.push_diagnostic(InferenceDiagnostic::NoConstructor {
                        expression,
                        builtins: construction_builtin_id,
                        r#type,
                        parameters: arguments,
                    });
                    self.error_type()
                }
            },
            TypeKind::Matrix(matrix) => {
                if arguments.is_empty() {
                    return r#type;
                }
                let construction_builtin_id = self.builtin_matrix_inferred_constructor(
                    size_to_dimension(matrix.columns),
                    size_to_dimension(matrix.rows),
                );
                let construction_result =
                    self.try_call_builtin(construction_builtin_id, &arguments);
                if construction_result.is_ok() {
                    r#type
                } else {
                    self.push_diagnostic(InferenceDiagnostic::NoConstructor {
                        expression,
                        builtins: construction_builtin_id,
                        r#type,
                        parameters: arguments,
                    });
                    self.error_type()
                }
            },
            TypeKind::Struct(_) => {
                // TODO: Implement checking field types
                // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/674
                r#type
            },

            // Never constructible
            TypeKind::Texture(_)
            | TypeKind::Sampler(_)
            | TypeKind::Pointer(_)
            | TypeKind::Atomic(_)
            | TypeKind::StorageTypeOfTexelFormat(_)
            | TypeKind::BoundVariable(_)
            | TypeKind::Reference(_) => {
                self.push_diagnostic(InferenceDiagnostic::InvalidConstructionType {
                    expression,
                    r#type,
                });
                self.error_type()
            },
            TypeKind::Error => r#type,
        }
    }

    #[expect(
        clippy::too_many_lines,
        reason = "large bug not complex match expression"
    )]
    /// Constructor for just a type name
    fn call_type_without_template_constructor(
        &mut self,
        expression: ExpressionId,
        r#type: Type,
        arguments: Vec<Type>,
    ) -> Type {
        fn size_to_dimension(size: VecSize) -> VecDimensionality {
            #[expect(
                clippy::unreachable,
                reason = "this is by far the easiest way to handle it, at least for now"
            )]
            match size {
                VecSize::Two => VecDimensionality::Two,
                VecSize::Three => VecDimensionality::Three,
                VecSize::Four => VecDimensionality::Four,
                VecSize::BoundVariable(_) => {
                    unreachable!("Can never have unbound type at this point")
                },
            }
        }

        match r#type.kind(self.database) {
            TypeKind::Scalar(scalar_type) => {
                self.call_scalar_constructor(scalar_type, expression, r#type, arguments)
            },
            TypeKind::Array(array_type) => {
                let Some(mut expected_type) = arguments.first().copied() else {
                    self.push_diagnostic(InferenceDiagnostic::FunctionCallArgCountMismatch {
                        expression,
                        n_expected: 1,
                        n_actual: arguments.len(),
                    });
                    return self.error_type();
                };

                for argument_type in &arguments[1..] {
                    if argument_type.is_convertible_to(expected_type, self.database) {
                        // Everything is as intended
                    } else if expected_type.is_convertible_to(*argument_type, self.database) {
                        // Narrowing the expected type
                        expected_type = *argument_type;
                    } else {
                        self.push_diagnostic(InferenceDiagnostic::TypeMismatch {
                            expression,
                            expected: TypeExpectation::Type(TypeExpectationInner::Exact(
                                expected_type,
                            )),
                            actual: *argument_type,
                        });
                    }
                }
                if let Ok(validated_length) = u32::try_from(arguments.len()) {
                    TypeKind::Array(ArrayType {
                        inner: expected_type,
                        binding_array: array_type.binding_array,
                        size: ArraySize::Constant(validated_length),
                    })
                    .intern(self.database)
                } else {
                    self.push_diagnostic(InferenceDiagnostic::FunctionCallArgCountMismatch {
                        expression,
                        #[expect(clippy::as_conversions, reason = "usize always holds a u32")]
                        n_expected: ArraySize::MAX as usize,
                        n_actual: arguments.len(),
                    });
                    TypeKind::Array(ArrayType {
                        inner: expected_type,
                        binding_array: array_type.binding_array,
                        size: ArraySize::Constant(ArraySize::MAX),
                    })
                    .intern(self.database)
                }
            },
            TypeKind::Vector(vec) => {
                if arguments.is_empty() {
                    return TypeKind::Vector(VectorType {
                        size: vec.size,
                        component_type: TypeKind::Scalar(ScalarType::AbstractInt)
                            .intern(self.database),
                    })
                    .intern(self.database);
                }
                let construction_builtin_id =
                    self.builtin_vector_inferred_constructor(size_to_dimension(vec.size));
                let construction_result =
                    self.try_call_builtin(construction_builtin_id, &arguments);

                if let Ok((r#type, _)) = construction_result {
                    r#type
                } else {
                    self.push_diagnostic(InferenceDiagnostic::NoConstructor {
                        expression,
                        builtins: construction_builtin_id,
                        r#type,
                        parameters: arguments,
                    });
                    self.error_type()
                }
            },
            TypeKind::Matrix(matrix) => {
                if arguments.is_empty() {
                    self.push_diagnostic(InferenceDiagnostic::FunctionCallArgCountMismatch {
                        expression,
                        n_expected: 1,
                        n_actual: arguments.len(),
                    });
                    return self.error_type();
                }
                let construction_builtin_id = self.builtin_matrix_inferred_constructor(
                    size_to_dimension(matrix.columns),
                    size_to_dimension(matrix.rows),
                );
                let construction_result =
                    self.try_call_builtin(construction_builtin_id, &arguments);
                if let Ok((r#type, _)) = construction_result {
                    r#type
                } else {
                    self.push_diagnostic(InferenceDiagnostic::NoConstructor {
                        expression,
                        builtins: construction_builtin_id,
                        r#type,
                        parameters: arguments,
                    });
                    self.error_type()
                }
            },
            TypeKind::Struct(_) => {
                // TODO: Implement checking fields' types
                // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/674
                r#type
            },
            // Never constructible
            TypeKind::Texture(_)
            | TypeKind::Sampler(_)
            | TypeKind::Pointer(_)
            | TypeKind::Atomic(_)
            | TypeKind::StorageTypeOfTexelFormat(_)
            | TypeKind::BoundVariable(_)
            | TypeKind::Reference(_) => {
                self.push_diagnostic(InferenceDiagnostic::InvalidConstructionType {
                    expression,
                    r#type,
                });
                self.error_type()
            },
            TypeKind::Error => r#type,
        }
    }

    fn call_scalar_constructor(
        &mut self,
        scalar_type: ScalarType,
        expression: ExpressionId,
        r#type: Type,
        arguments: Vec<Type>,
    ) -> Type {
        if arguments.is_empty() {
            // Permit the zero value
            return r#type;
        }
        let construction_builtin_id = match scalar_type {
            ScalarType::Bool => {
                Builtin::builtin_op_bool_constructor(self.database).intern(self.database)
            },
            ScalarType::I32 => {
                Builtin::builtin_op_i32_constructor(self.database).intern(self.database)
            },
            ScalarType::U32 => {
                Builtin::builtin_op_u32_constructor(self.database).intern(self.database)
            },
            ScalarType::F32 => {
                Builtin::builtin_op_f32_constructor(self.database).intern(self.database)
            },
            ScalarType::F16 => {
                Builtin::builtin_op_f16_constructor(self.database).intern(self.database)
            },
            ScalarType::AbstractInt | ScalarType::AbstractFloat => {
                // Panic is correct here, since it should be impossible to enter this branch
                panic!("cannot construct abstract types")
            },
        };

        let construction_result = self.try_call_builtin(construction_builtin_id, &arguments);
        if let Ok((r#type, _)) = construction_result {
            r#type
        } else {
            self.push_diagnostic(InferenceDiagnostic::NoConstructor {
                expression,
                builtins: construction_builtin_id,
                r#type,
                parameters: arguments,
            });
            self.error_type()
        }
    }

    fn lower_type(
        &mut self,
        type_ref: TypeSpecifierId,
        resolver: &Resolver,
        store: &ExpressionStore,
    ) -> Type {
        let mut context = TypeLoweringContext::new(self.database, resolver, store);
        let r#type = context.lower_type(type_ref);
        self.push_lowering_diagnostics(&mut context.diagnostics, store);
        r#type
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TypeExpectationInner {
    Exact(Type),
    IntegerScalar,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TypeExpectation {
    Type(TypeExpectationInner),
    Any,
}

impl TypeExpectation {
    const fn from_option(option: Option<Type>) -> Self {
        match option {
            Some(r#type) => Self::Type(TypeExpectationInner::Exact(r#type)),
            None => Self::Any,
        }
    }

    const fn from_type(r#type: Type) -> Self {
        Self::Type(TypeExpectationInner::Exact(r#type))
    }
}

impl InferenceContext<'_> {
    fn make_ref(
        &self,
        r#type: Type,
        address_space: AddressSpace,
        access_mode: AccessMode,
    ) -> Type {
        self.database.intern_type(TypeKind::Reference(Reference {
            address_space,
            inner: r#type,
            access_mode,
        }))
    }

    fn ref_to_pointer(
        &self,
        reference: &Reference,
    ) -> Type {
        self.database.intern_type(TypeKind::Pointer(Pointer {
            address_space: reference.address_space,
            inner: reference.inner,
            access_mode: reference.access_mode,
        }))
    }

    fn ptr_to_ref(
        &self,
        pointer: &Pointer,
    ) -> Type {
        self.database.intern_type(TypeKind::Reference(Reference {
            address_space: pointer.address_space,
            inner: pointer.inner,
            access_mode: pointer.access_mode,
        }))
    }

    const fn error_type(&self) -> Type {
        self.result.standard_types.unknown
    }

    fn bool_type(&self) -> Type {
        self.database
            .intern_type(TypeKind::Scalar(ScalarType::Bool))
    }
}

/// Lowers types and expressions, the two are deeply intertwined.
pub struct TypeLoweringContext<'database> {
    database: &'database dyn HirDatabase,
    /// Make sure to set the correct resolver when going into function scopes
    resolver: &'database Resolver,
    store: &'database ExpressionStore,

    pub(crate) diagnostics: Vec<TypeLoweringError>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TypeLoweringError {
    pub container: TypeContainer,
    pub kind: TypeLoweringErrorKind,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TypeLoweringErrorKind {
    UnresolvedName(Name),
    UnexpectedTemplateArgument(String),
    MissingTemplateArgument(String),
    MissingTemplate,
    WrongNumberOfTemplateArguments {
        expected: std::ops::RangeInclusive<usize>,
        actual: usize,
    },
    // A value was provided where a type was expected.
    ExpectedType(Name),
    // A function was provided but not called.
    ExpectedFunctionToBeCalled(Name),
    // TODO: Change this to a strongly typed wgsl_types::Error
    // The challenge here is that wgsl_types::Error doesn't implement Eq,
    // However the inference result keeps track of all the diagnostics and is cached
    // wgsl_types::Error cannot trivially implement Eq, because the `Instance` would
    // need to implement Eq. And it would have to be eq where "floating point NaNs" are
    // prooobably equal, if their bits are equal?
    WgslError(String),
}

impl fmt::Display for TypeLoweringErrorKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::UnresolvedName(name) => {
                write!(formatter, "`{}` not found in scope", name.as_str())
            },
            Self::WgslError(error) => {
                write!(formatter, "{error}")
            },
            Self::UnexpectedTemplateArgument(expected) => {
                write!(
                    formatter,
                    "unexpected template argument, expected {expected}"
                )
            },
            Self::MissingTemplateArgument(expected) => {
                write!(formatter, "missing template argument, expected {expected}")
            },
            Self::MissingTemplate => {
                write!(formatter, "missing template arguments")
            },
            Self::WrongNumberOfTemplateArguments { expected, actual }
                if expected.start() == expected.end() =>
            {
                write!(
                    formatter,
                    "expected {} template arguments, but got {actual}",
                    expected.start()
                )
            },
            Self::WrongNumberOfTemplateArguments { expected, actual } => {
                write!(
                    formatter,
                    "expected {} to {} template arguments, but got {actual}",
                    expected.start(),
                    expected.end()
                )
            },
            Self::ExpectedType(name) => {
                write!(formatter, "{} is not a type", name.as_str())
            },
            Self::ExpectedFunctionToBeCalled(name) => {
                write!(
                    formatter,
                    "{0:} was written, write {0:}() instead",
                    name.as_str()
                )
            },
        }
    }
}

/// A lowered type, or the definition of an item
/// Also covers built-ins
pub enum Lowered {
    Type(Type),
    TypeWithoutTemplate(Type),
    Function(ResolvedFunctionId),
    GlobalConstant(GlobalConstantId),
    GlobalVariable(GlobalVariableId),
    Override(OverrideId),
    Local(BindingId),
    Enumerant(Enumerant),
    BuiltinFunction,
}

impl Lowered {
    #[must_use]
    pub const fn kind(&self) -> LoweredKind {
        match self {
            Self::Type(_) | Self::TypeWithoutTemplate(_) => LoweredKind::Type,
            Self::Function(_) | Self::BuiltinFunction => LoweredKind::Function,
            Self::GlobalConstant(_) => LoweredKind::Constant,
            Self::GlobalVariable(_) => LoweredKind::Variable,
            Self::Override(_) => LoweredKind::Override,
            Self::Local(_) => LoweredKind::Local,
            Self::Enumerant(_) => LoweredKind::Enumerant,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LoweredKind {
    Type,
    Function,
    Constant,
    Variable,
    Override,
    Local,
    Enumerant,
}

impl std::fmt::Display for LoweredKind {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Type => write!(f, "type"),
            Self::Function => write!(f, "function"),
            Self::Constant => write!(f, "constant"),
            Self::Variable => write!(f, "variable"),
            Self::Override => write!(f, "override"),
            Self::Local => write!(f, "local variable"),
            Self::Enumerant => write!(f, "enumerant"),
        }
    }
}

impl<'database> TypeLoweringContext<'database> {
    pub fn new(
        database: &'database dyn HirDatabase,
        resolver: &'database Resolver,
        store: &'database ExpressionStore,
    ) -> Self {
        Self {
            database,
            resolver,
            store,
            diagnostics: Vec::new(),
        }
    }

    pub fn lower(
        &mut self,
        type_container: TypeContainer,
        path: &Name,
        template_parameters: &[ExpressionId],
    ) -> Lowered {
        match self.try_lower(type_container, path, template_parameters) {
            Ok(lowered) => lowered,
            Err(error) => {
                self.diagnostics.push(error);
                Lowered::Type(self.database.intern_type(TypeKind::Error))
            },
        }
    }

    /// Will lower types, and resolve the definition of other items.
    pub fn try_lower(
        &mut self,
        type_container: TypeContainer,
        path: &Name,
        template_parameters: &[ExpressionId],
    ) -> Result<Lowered, TypeLoweringError> {
        let resolved_type = self.resolver.resolve(path);

        if resolved_type.is_some() {
            self.expect_no_template(template_parameters);
        }

        match resolved_type {
            Some(ResolveKind::TypeAlias(location)) => {
                let id = self.database.intern_type_alias(location);
                Ok(Lowered::Type(self.database.type_alias_type(id).0))
            },
            Some(ResolveKind::Struct(location)) => {
                let id = self.database.intern_struct(location);
                Ok(Lowered::Type(
                    self.database.intern_type(TypeKind::Struct(id)),
                ))
            },
            Some(ResolveKind::Function(location)) => {
                let id = self.database.intern_function(location);
                Ok(Lowered::Function(self.database.function_type(id)))
            },
            Some(ResolveKind::GlobalConstant(location)) => {
                let id = self.database.intern_global_constant(location);
                Ok(Lowered::GlobalConstant(id))
            },
            Some(ResolveKind::GlobalVariable(location)) => {
                let id = self.database.intern_global_variable(location);
                Ok(Lowered::GlobalVariable(id))
            },
            Some(ResolveKind::Override(location)) => {
                let id = self.database.intern_override(location);
                Ok(Lowered::Override(id))
            },
            Some(ResolveKind::Local(local)) => Ok(Lowered::Local(local)),
            None => self.lower_predeclared(type_container, path, template_parameters),
        }
    }

    fn expect_no_template(
        &mut self,
        template_parameters: &[ExpressionId],
    ) {
        if template_parameters.is_empty() {
            return;
        }
        for template_expression in template_parameters {
            self.diagnostics.push(TypeLoweringError {
                container: TypeContainer::Expression(*template_expression),
                kind: TypeLoweringErrorKind::UnexpectedTemplateArgument("nothing".to_owned()),
            });
        }
    }

    fn expect_n_templates(
        &mut self,
        template_parameters: &eval::TemplateParameters,
        expected: std::ops::RangeInclusive<usize>,
    ) -> bool {
        if expected.contains(&template_parameters.len()) {
            true
        } else {
            self.diagnostics.push(TypeLoweringError {
                container: *template_parameters.container(),
                kind: TypeLoweringErrorKind::WrongNumberOfTemplateArguments {
                    expected,
                    actual: template_parameters.len(),
                },
            });

            false
        }
    }

    pub fn lower_type(
        &mut self,
        type_specifier_id: TypeSpecifierId,
    ) -> Type {
        let type_specifier = &self.store[type_specifier_id];
        let lowered = self.try_lower(
            TypeContainer::TypeSpecifier(type_specifier_id),
            &type_specifier.path,
            &type_specifier.template_parameters,
        );
        match lowered {
            Ok(Lowered::Type(r#type)) => r#type,
            Ok(Lowered::TypeWithoutTemplate(_)) => {
                self.diagnostics.push(TypeLoweringError {
                    container: TypeContainer::TypeSpecifier(type_specifier_id),
                    kind: TypeLoweringErrorKind::MissingTemplate,
                });
                self.database.intern_type(TypeKind::Error)
            },
            Ok(
                Lowered::Enumerant(_)
                | Lowered::Function(_)
                | Lowered::BuiltinFunction
                | Lowered::GlobalConstant(_)
                | Lowered::GlobalVariable(_)
                | Lowered::Override(_)
                | Lowered::Local(_),
            ) => {
                self.diagnostics.push(TypeLoweringError {
                    container: TypeContainer::TypeSpecifier(type_specifier_id),
                    kind: TypeLoweringErrorKind::ExpectedType(type_specifier.path.clone()),
                });
                self.database.intern_type(TypeKind::Error)
            },
            Err(error) => {
                self.diagnostics.push(error);
                self.database.intern_type(TypeKind::Error)
            },
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum AbstractHandling {
    Concretize,
    Abstract,
}

struct WgslTypeConverter<'database> {
    database: &'database dyn HirDatabase,
    interned_structs: Vec<StructId>,
}

impl<'database> WgslTypeConverter<'database> {
    fn new(database: &'database dyn HirDatabase) -> Self {
        Self {
            database,
            interned_structs: Vec::default(),
        }
    }

    #[expect(
        clippy::wrong_self_convention,
        reason = "naming things is hard and this is probably changing in the future"
    )]
    fn to_wgsl_types(
        &mut self,
        r#type: Type,
    ) -> Option<wgsl_types::Type> {
        Some(match r#type.kind(self.database) {
            // TODO: This should not be necessary because the types should align 1:1
            // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/672
            TypeKind::Error
            | TypeKind::BoundVariable(_)
            | TypeKind::StorageTypeOfTexelFormat(_) => {
                return None;
            },
            TypeKind::Scalar(ScalarType::AbstractFloat) => wgsl_types::Type::AbstractFloat,
            TypeKind::Scalar(ScalarType::AbstractInt) => wgsl_types::Type::AbstractInt,
            TypeKind::Scalar(ScalarType::Bool) => wgsl_types::Type::Bool,
            TypeKind::Scalar(ScalarType::F16) => wgsl_types::Type::F16,
            TypeKind::Scalar(ScalarType::F32) => wgsl_types::Type::F32,
            TypeKind::Scalar(ScalarType::I32) => wgsl_types::Type::I32,
            TypeKind::Scalar(ScalarType::U32) => wgsl_types::Type::U32,
            TypeKind::Atomic(AtomicType { inner }) => {
                wgsl_types::Type::Atomic(Box::new(self.to_wgsl_types(inner)?))
            },
            TypeKind::Vector(VectorType {
                size,
                component_type,
            }) => {
                wgsl_types::Type::Vec(size.as_u8(), Box::new(self.to_wgsl_types(component_type)?))
            },
            TypeKind::Matrix(MatrixType {
                columns,
                rows,
                inner,
            }) => wgsl_types::Type::Mat(
                columns.as_u8(),
                rows.as_u8(),
                Box::new(self.to_wgsl_types(inner)?),
            ),
            TypeKind::Struct(struct_id) => {
                let data = self.database.struct_data(struct_id).0;
                let fields = &self.database.field_types(struct_id).0;
                let name = self.intern_struct(struct_id);
                wgsl_types::Type::Struct(Box::new(wgsl_types::ty::StructType {
                    name,
                    members: data
                        .fields
                        .iter()
                        .map(|(id, data)| {
                            Some(wgsl_types::ty::StructMemberType {
                                name: data.name.as_str().to_owned(),
                                // Skip broken struct fields
                                ty: self.to_wgsl_types(fields[id])?,
                                // Don't bother reconstructing the correct layout
                                size: None,
                                align: None,
                            })
                        })
                        .collect::<Option<Vec<_>>>()?,
                }))
            },
            TypeKind::Array(ArrayType {
                inner,
                binding_array: false,
                size,
            }) => wgsl_types::Type::Array(
                Box::new(self.to_wgsl_types(inner)?),
                match size {
                    #[expect(clippy::as_conversions, reason = "externally defined")]
                    ArraySize::Constant(size) => Some(size as usize),
                    ArraySize::Dynamic => None,
                },
            ),
            TypeKind::Array(ArrayType {
                inner,
                binding_array: true,
                size,
            }) => wgsl_types::Type::BindingArray(
                Box::new(self.to_wgsl_types(inner)?),
                match size {
                    #[expect(clippy::as_conversions, reason = "externally defined")]
                    ArraySize::Constant(size) => Some(size as usize),
                    ArraySize::Dynamic => None,
                },
            ),
            TypeKind::Texture(texture_type) => {
                wgsl_types::Type::Texture(self.to_wgsl_texture_type(texture_type))
            },
            TypeKind::Sampler(sampler_type) => wgsl_types::Type::Sampler(sampler_type),
            TypeKind::Reference(Reference {
                address_space,
                inner,
                access_mode,
            }) => wgsl_types::Type::Ref(
                address_space,
                Box::new(self.to_wgsl_types(inner)?),
                access_mode,
            ),
            TypeKind::Pointer(Pointer {
                address_space,
                inner,
                access_mode,
            }) => wgsl_types::Type::Ptr(
                address_space,
                Box::new(self.to_wgsl_types(inner)?),
                access_mode,
            ),
        })
    }

    /// Returns none if it is an error type
    fn template_parameter_to_wgsl_types(
        &mut self,
        param: eval::TemplateParameter,
    ) -> Option<wgsl_types::tplt::TpltParam> {
        Some(match param {
            eval::TemplateParameter::Type(r#type) => {
                wgsl_types::tplt::TpltParam::Type(self.to_wgsl_types(r#type)?)
            },
            eval::TemplateParameter::Instance(instance) => {
                wgsl_types::tplt::TpltParam::Instance(instance?)
            },
            eval::TemplateParameter::Enumerant(enumerant) => {
                wgsl_types::tplt::TpltParam::Enumerant(enumerant)
            },
        })
    }

    #[expect(
        clippy::wrong_self_convention,
        reason = "naming things is hard and this is probably changing in the future"
    )]
    fn from_wgsl_types(
        &self,
        r#type: wgsl_types::Type,
    ) -> Type {
        #[expect(
            clippy::todo,
            reason = "See https://github.com/wgsl-analyzer/wgsl-analyzer/issues/442"
        )]
        match r#type {
            wgsl_types::Type::Bool => TypeKind::Scalar(ScalarType::Bool).intern(self.database),
            wgsl_types::Type::AbstractInt => {
                TypeKind::Scalar(ScalarType::AbstractInt).intern(self.database)
            },
            wgsl_types::Type::AbstractFloat => {
                TypeKind::Scalar(ScalarType::AbstractFloat).intern(self.database)
            },
            wgsl_types::Type::I32 => TypeKind::Scalar(ScalarType::I32).intern(self.database),
            wgsl_types::Type::U32 => TypeKind::Scalar(ScalarType::U32).intern(self.database),
            wgsl_types::Type::I64 => todo!("naga extension"),
            wgsl_types::Type::U64 => todo!("naga extension"),
            wgsl_types::Type::F16 => TypeKind::Scalar(ScalarType::F16).intern(self.database),
            wgsl_types::Type::F32 => TypeKind::Scalar(ScalarType::F32).intern(self.database),
            wgsl_types::Type::F64 => todo!("naga extension"),
            wgsl_types::Type::Struct(struct_type) => {
                let struct_id = self
                    .get_interned_struct(&struct_type.name)
                    // I think this doesn't hold true when calling `atomicCompareExchangeWeak`
                    .expect("Only struct types that have been passed in should be returned");
                TypeKind::Struct(struct_id).intern(self.database)
            },
            wgsl_types::Type::Array(r#type, size) => TypeKind::Array(ArrayType {
                inner: self.from_wgsl_types(*r#type),
                binding_array: false,
                size: match size {
                    Some(size) => {
                        debug_assert!(u32::try_from(size).is_ok());
                        #[expect(
                            clippy::cast_possible_truncation,
                            clippy::as_conversions,
                            reason = "externally defined"
                        )]
                        ArraySize::Constant(size as u32)
                    },
                    None => ArraySize::Dynamic,
                },
            })
            .intern(self.database),
            wgsl_types::Type::BindingArray(r#type, size) => TypeKind::Array(ArrayType {
                inner: self.from_wgsl_types(*r#type),
                binding_array: true,
                size: match size {
                    Some(size) => {
                        debug_assert!(u32::try_from(size).is_ok());
                        #[expect(
                            clippy::cast_possible_truncation,
                            clippy::as_conversions,
                            reason = "externally defined"
                        )]
                        ArraySize::Constant(size as u32)
                    },
                    None => ArraySize::Dynamic,
                },
            })
            .intern(self.database),
            wgsl_types::Type::Vec(size, r#type) => TypeKind::Vector(VectorType {
                size: VecSize::try_from(size).unwrap(),
                component_type: self.from_wgsl_types(*r#type),
            })
            .intern(self.database),
            wgsl_types::Type::Mat(columns, rows, r#type) => TypeKind::Matrix(MatrixType {
                columns: VecSize::try_from(columns).unwrap(),
                rows: VecSize::try_from(rows).unwrap(),
                inner: self.from_wgsl_types(*r#type),
            })
            .intern(self.database),
            wgsl_types::Type::Atomic(r#type) => TypeKind::Atomic(AtomicType {
                inner: self.from_wgsl_types(*r#type),
            })
            .intern(self.database),
            wgsl_types::Type::Ptr(address_space, r#type, access_mode) => {
                TypeKind::Pointer(Pointer {
                    address_space,
                    inner: self.from_wgsl_types(*r#type),
                    access_mode,
                })
                .intern(self.database)
            },
            wgsl_types::Type::Ref(address_space, r#type, access_mode) => {
                TypeKind::Reference(Reference {
                    address_space,
                    inner: self.from_wgsl_types(*r#type),
                    access_mode,
                })
                .intern(self.database)
            },
            wgsl_types::Type::Texture(texture_type) => {
                TypeKind::Texture(self.from_wgsl_texture_type(&texture_type)).intern(self.database)
            },
            wgsl_types::Type::Sampler(sampler_type) => {
                TypeKind::Sampler(sampler_type).intern(self.database)
            },
            wgsl_types::Type::RayQuery(_) => todo!("naga extension"),
            wgsl_types::Type::AccelerationStructure(_) => todo!("naga extension"),
        }
    }

    #[expect(clippy::too_many_lines, reason = "long but simple match")]
    #[expect(
        clippy::wrong_self_convention,
        reason = "naming things is hard and this is probably changing in the future"
    )]
    fn from_wgsl_texture_type(
        &self,
        value: &wgsl_types::ty::TextureType,
    ) -> TextureType {
        match *value {
            wgsl_types::ty::TextureType::Sampled1D(sampled_type) => TextureType {
                kind: TextureKind::from_sampled(sampled_type, self.database),
                dimension: TextureDimensionality::D1,
                arrayed: false,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Sampled1DArray(sampled_type) => TextureType {
                kind: TextureKind::from_sampled(sampled_type, self.database),
                dimension: TextureDimensionality::D1,
                arrayed: true,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Sampled2D(sampled_type) => TextureType {
                kind: TextureKind::from_sampled(sampled_type, self.database),
                dimension: TextureDimensionality::D2,
                arrayed: false,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Sampled2DArray(sampled_type) => TextureType {
                kind: TextureKind::from_sampled(sampled_type, self.database),
                dimension: TextureDimensionality::D2,
                arrayed: true,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Sampled3D(sampled_type) => TextureType {
                kind: TextureKind::from_sampled(sampled_type, self.database),
                dimension: TextureDimensionality::D3,
                arrayed: false,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::SampledCube(sampled_type) => TextureType {
                kind: TextureKind::from_sampled(sampled_type, self.database),
                dimension: TextureDimensionality::Cube,
                arrayed: false,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::SampledCubeArray(sampled_type) => TextureType {
                kind: TextureKind::from_sampled(sampled_type, self.database),
                dimension: TextureDimensionality::Cube,
                arrayed: true,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Multisampled2D(sampled_type) => TextureType {
                kind: TextureKind::from_sampled(sampled_type, self.database),
                dimension: TextureDimensionality::D2,
                arrayed: false,
                multisampled: true,
            },
            wgsl_types::ty::TextureType::Multisampled2DArray(sampled_type) => TextureType {
                kind: TextureKind::from_sampled(sampled_type, self.database),
                dimension: TextureDimensionality::D2,
                arrayed: true,
                multisampled: true,
            },
            wgsl_types::ty::TextureType::DepthMultisampled2D => TextureType {
                kind: TextureKind::Depth,
                dimension: TextureDimensionality::D2,
                arrayed: false,
                multisampled: true,
            },
            wgsl_types::ty::TextureType::External => TextureType {
                kind: TextureKind::External,
                dimension: TextureDimensionality::D2,
                arrayed: false,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Storage1D(texel_format, access_mode) => TextureType {
                kind: TextureKind::Storage(from_wgsl_texel_format(texel_format), access_mode),
                dimension: TextureDimensionality::D1,
                arrayed: false,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Storage1DArray(texel_format, access_mode) => TextureType {
                kind: TextureKind::Storage(from_wgsl_texel_format(texel_format), access_mode),
                dimension: TextureDimensionality::D1,
                arrayed: true,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Storage2D(texel_format, access_mode) => TextureType {
                kind: TextureKind::Storage(from_wgsl_texel_format(texel_format), access_mode),
                dimension: TextureDimensionality::D2,
                arrayed: false,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Storage2DArray(texel_format, access_mode) => TextureType {
                kind: TextureKind::Storage(from_wgsl_texel_format(texel_format), access_mode),
                dimension: TextureDimensionality::D2,
                arrayed: true,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Storage3D(texel_format, access_mode) => TextureType {
                kind: TextureKind::Storage(from_wgsl_texel_format(texel_format), access_mode),
                dimension: TextureDimensionality::D3,
                arrayed: false,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Depth2D => TextureType {
                kind: TextureKind::Depth,
                dimension: TextureDimensionality::D2,
                arrayed: false,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Depth2DArray => TextureType {
                kind: TextureKind::Depth,
                dimension: TextureDimensionality::D2,
                arrayed: true,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::DepthCube => TextureType {
                kind: TextureKind::Depth,
                dimension: TextureDimensionality::Cube,
                arrayed: false,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::DepthCubeArray => TextureType {
                kind: TextureKind::Depth,
                dimension: TextureDimensionality::Cube,
                arrayed: true,
                multisampled: false,
            },
        }
    }

    fn to_wgsl_texture_type(
        &self,
        value: TextureType,
    ) -> wgsl_types::ty::TextureType {
        match (value.kind, value.dimension, value.arrayed) {
            (TextureKind::Sampled(sampled), TextureDimensionality::D1, false) => {
                wgsl_types::ty::TextureType::Sampled1D(self.to_wgsl_sampled(sampled))
            },
            (TextureKind::Sampled(sampled), TextureDimensionality::D1, true) => {
                wgsl_types::ty::TextureType::Sampled1DArray(self.to_wgsl_sampled(sampled))
            },
            (TextureKind::Sampled(sampled), TextureDimensionality::D2, false) => {
                wgsl_types::ty::TextureType::Sampled2D(self.to_wgsl_sampled(sampled))
            },
            (TextureKind::Sampled(sampled), TextureDimensionality::D2, true) => {
                wgsl_types::ty::TextureType::Sampled2DArray(self.to_wgsl_sampled(sampled))
            },
            (TextureKind::Sampled(sampled), TextureDimensionality::D3, false) => {
                wgsl_types::ty::TextureType::Sampled3D(self.to_wgsl_sampled(sampled))
            },
            (TextureKind::Sampled(sampled), TextureDimensionality::Cube, false) => {
                wgsl_types::ty::TextureType::SampledCube(self.to_wgsl_sampled(sampled))
            },
            (TextureKind::Sampled(sampled), TextureDimensionality::Cube, true) => {
                wgsl_types::ty::TextureType::SampledCubeArray(self.to_wgsl_sampled(sampled))
            },
            (TextureKind::Storage(texel_format, access_mode), TextureDimensionality::D1, false) => {
                wgsl_types::ty::TextureType::Storage1D(
                    to_wgsl_texel_format(texel_format),
                    access_mode,
                )
            },
            (TextureKind::Storage(texel_format, access_mode), TextureDimensionality::D1, true) => {
                wgsl_types::ty::TextureType::Storage1DArray(
                    to_wgsl_texel_format(texel_format),
                    access_mode,
                )
            },
            (TextureKind::Storage(texel_format, access_mode), TextureDimensionality::D2, false) => {
                wgsl_types::ty::TextureType::Storage2D(
                    to_wgsl_texel_format(texel_format),
                    access_mode,
                )
            },
            (TextureKind::Storage(texel_format, access_mode), TextureDimensionality::D2, true) => {
                wgsl_types::ty::TextureType::Storage2DArray(
                    to_wgsl_texel_format(texel_format),
                    access_mode,
                )
            },
            (TextureKind::Storage(texel_format, access_mode), TextureDimensionality::D3, false) => {
                wgsl_types::ty::TextureType::Storage3D(
                    to_wgsl_texel_format(texel_format),
                    access_mode,
                )
            },
            (TextureKind::Depth, TextureDimensionality::D2, false) => {
                wgsl_types::ty::TextureType::Depth2D
            },
            (TextureKind::Depth, TextureDimensionality::D2, true) => {
                wgsl_types::ty::TextureType::Depth2DArray
            },
            (TextureKind::Depth, TextureDimensionality::Cube, false) => {
                wgsl_types::ty::TextureType::DepthCube
            },
            (TextureKind::Depth, TextureDimensionality::Cube, true) => {
                wgsl_types::ty::TextureType::DepthCubeArray
            },
            (TextureKind::External, _, _) => wgsl_types::ty::TextureType::External,
            (_, _, _) => panic!("invalid texture"),
        }
    }

    fn intern_struct(
        &mut self,
        struct_id: StructId,
    ) -> String {
        let index = self.interned_structs.len();
        self.interned_structs.push(struct_id);
        format!("struct{index}")
    }

    fn get_interned_struct(
        &self,
        name: &str,
    ) -> Option<StructId> {
        let index = name.strip_prefix("struct")?.parse::<usize>().ok()?;
        self.interned_structs.get(index).copied()
    }

    fn to_wgsl_sampled(
        &self,
        sampled: Type,
    ) -> wgsl_types::syntax::SampledType {
        match sampled.kind(self.database) {
            TypeKind::Scalar(ScalarType::I32) => wgsl_types::syntax::SampledType::I32,
            TypeKind::Scalar(ScalarType::U32) => wgsl_types::syntax::SampledType::U32,
            TypeKind::Scalar(ScalarType::F32) => wgsl_types::syntax::SampledType::F32,
            kind @ (TypeKind::Error
            | TypeKind::Scalar(_)
            | TypeKind::Atomic(_)
            | TypeKind::Vector(_)
            | TypeKind::Matrix(_)
            | TypeKind::Struct(_)
            | TypeKind::Array(_)
            | TypeKind::Texture(_)
            | TypeKind::Sampler(_)
            | TypeKind::Reference(_)
            | TypeKind::Pointer(_)
            | TypeKind::BoundVariable(_)
            | TypeKind::StorageTypeOfTexelFormat(_)) => panic!("invalid sampled type {kind:?}"),
        }
    }
}

#[must_use]
pub fn from_wgsl_texel_format(
    texel_format: wgsl_types::syntax::TexelFormat
) -> crate::ty::TexelFormat {
    match texel_format {
        wgsl_types::syntax::TexelFormat::Rgba8Unorm => crate::ty::TexelFormat::Rgba8unorm,
        wgsl_types::syntax::TexelFormat::Rgba8Snorm => crate::ty::TexelFormat::Rgba8snorm,
        wgsl_types::syntax::TexelFormat::Rgba8Uint => crate::ty::TexelFormat::Rgba8uint,
        wgsl_types::syntax::TexelFormat::Rgba8Sint => crate::ty::TexelFormat::Rgba8sint,
        wgsl_types::syntax::TexelFormat::Rgba16Uint => crate::ty::TexelFormat::Rgba16uint,
        wgsl_types::syntax::TexelFormat::Rgba16Sint => crate::ty::TexelFormat::Rgba16sint,
        wgsl_types::syntax::TexelFormat::Rgba16Float => crate::ty::TexelFormat::Rgba16float,
        wgsl_types::syntax::TexelFormat::R32Uint => crate::ty::TexelFormat::R32uint,
        wgsl_types::syntax::TexelFormat::R32Sint => crate::ty::TexelFormat::R32sint,
        wgsl_types::syntax::TexelFormat::R32Float => crate::ty::TexelFormat::R32float,
        wgsl_types::syntax::TexelFormat::Rg32Uint => crate::ty::TexelFormat::Rg32uint,
        wgsl_types::syntax::TexelFormat::Rg32Sint => crate::ty::TexelFormat::Rg32sint,
        wgsl_types::syntax::TexelFormat::Rg32Float => crate::ty::TexelFormat::Rg32float,
        wgsl_types::syntax::TexelFormat::Rgba32Uint => crate::ty::TexelFormat::Rgba32uint,
        wgsl_types::syntax::TexelFormat::Rgba32Sint => crate::ty::TexelFormat::Rgba32sint,
        wgsl_types::syntax::TexelFormat::Rgba32Float => crate::ty::TexelFormat::Rgba32float,
        wgsl_types::syntax::TexelFormat::Bgra8Unorm => crate::ty::TexelFormat::Bgra8unorm,
        wgsl_types::syntax::TexelFormat::R8Unorm
        | wgsl_types::syntax::TexelFormat::R8Snorm
        | wgsl_types::syntax::TexelFormat::R8Uint
        | wgsl_types::syntax::TexelFormat::R8Sint
        | wgsl_types::syntax::TexelFormat::R16Unorm
        | wgsl_types::syntax::TexelFormat::R16Snorm
        | wgsl_types::syntax::TexelFormat::R16Uint
        | wgsl_types::syntax::TexelFormat::R16Sint
        | wgsl_types::syntax::TexelFormat::R16Float
        | wgsl_types::syntax::TexelFormat::Rg8Unorm
        | wgsl_types::syntax::TexelFormat::Rg8Snorm
        | wgsl_types::syntax::TexelFormat::Rg8Uint
        | wgsl_types::syntax::TexelFormat::Rg8Sint
        | wgsl_types::syntax::TexelFormat::Rg16Unorm
        | wgsl_types::syntax::TexelFormat::Rg16Snorm
        | wgsl_types::syntax::TexelFormat::Rg16Uint
        | wgsl_types::syntax::TexelFormat::Rg16Sint
        | wgsl_types::syntax::TexelFormat::Rg16Float
        | wgsl_types::syntax::TexelFormat::Rgb10a2Uint
        | wgsl_types::syntax::TexelFormat::Rgb10a2Unorm
        | wgsl_types::syntax::TexelFormat::Rg11b10Float
        | wgsl_types::syntax::TexelFormat::R64Uint
        | wgsl_types::syntax::TexelFormat::Rgba16Unorm
        | wgsl_types::syntax::TexelFormat::Rgba16Snorm => {
            #[expect(
                clippy::unimplemented,
                reason = "TODO: support naga texture formats, see: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/675"
            )]
            {
                unimplemented!("not yet supported naga extension")
            }
        },
    }
}

/// Convert a [`crate::ty::TexelFormat`] into a [`wgsl_types::syntax::TexelFormat`].
///
/// # Panics
///
/// Panics if `texel_format` is `BoundVariable` or `Any`.
#[expect(
    deprecated,
    reason = "TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/559"
)]
#[must_use]
pub fn to_wgsl_texel_format(
    texel_format: crate::ty::TexelFormat
) -> wgsl_types::syntax::TexelFormat {
    match texel_format {
        crate::ty::TexelFormat::Rgba8unorm => wgsl_types::syntax::TexelFormat::Rgba8Unorm,
        crate::ty::TexelFormat::Rgba8snorm => wgsl_types::syntax::TexelFormat::Rgba8Snorm,
        crate::ty::TexelFormat::Rgba8uint => wgsl_types::syntax::TexelFormat::Rgba8Uint,
        crate::ty::TexelFormat::Rgba8sint => wgsl_types::syntax::TexelFormat::Rgba8Sint,
        crate::ty::TexelFormat::Rgba16uint => wgsl_types::syntax::TexelFormat::Rgba16Uint,
        crate::ty::TexelFormat::Rgba16sint => wgsl_types::syntax::TexelFormat::Rgba16Sint,
        crate::ty::TexelFormat::Rgba16float => wgsl_types::syntax::TexelFormat::Rgba16Float,
        crate::ty::TexelFormat::R32uint => wgsl_types::syntax::TexelFormat::R32Uint,
        crate::ty::TexelFormat::R32sint => wgsl_types::syntax::TexelFormat::R32Sint,
        crate::ty::TexelFormat::R32float => wgsl_types::syntax::TexelFormat::R32Float,
        crate::ty::TexelFormat::Rg32uint => wgsl_types::syntax::TexelFormat::Rg32Uint,
        crate::ty::TexelFormat::Rg32sint => wgsl_types::syntax::TexelFormat::Rg32Sint,
        crate::ty::TexelFormat::Rg32float => wgsl_types::syntax::TexelFormat::Rg32Float,
        crate::ty::TexelFormat::Rgba32uint => wgsl_types::syntax::TexelFormat::Rgba32Uint,
        crate::ty::TexelFormat::Rgba32sint => wgsl_types::syntax::TexelFormat::Rgba32Sint,
        crate::ty::TexelFormat::Rgba32float => wgsl_types::syntax::TexelFormat::Rgba32Float,
        crate::ty::TexelFormat::Bgra8unorm => wgsl_types::syntax::TexelFormat::Bgra8Unorm,
        crate::ty::TexelFormat::BoundVariable(_) => {
            panic!("bound var is not a valid texel format to convert")
        },
        crate::ty::TexelFormat::Any => panic!("any is not a valid texel format to convert"),
    }
}
