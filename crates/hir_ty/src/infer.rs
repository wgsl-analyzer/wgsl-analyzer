mod unify;

use std::{fmt, ops::Index};

use base_db::{Lookup as _, TextRange, TextSize};
use either::Either;
use hir_def::{
    HasSource as _,
    body::{BindingId, Body},
    database::{
        DefinitionWithBodyId, GlobalConstantId, GlobalVariableId, ModuleDefinitionId, OverrideId,
        StructId,
    },
    expression::{
        ArithmeticOperation, BinaryOperation, ComparisonOperation, Expression, ExpressionId,
        Statement, StatementId, SwitchCaseSelector, UnaryOperator,
    },
    expression_store::{ExpressionStore, ExpressionStoreSource, path::Path},
    item_tree::Name,
    mod_path::PathKind,
    name_resolution::ModuleData,
    resolver::{ResolveKind, Resolver},
    signature::{
        ConstantSignature, FieldId, FunctionSignature, OverrideSignature, VariableSignature,
    },
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
    diagnostics::{InferenceDiagnostic, InferenceDiagnosticKind},
    function::{FunctionDetails, ResolvedFunctionId},
    infer::unify::{UnificationTable, unify},
    lower::{
        Lowered, LoweredKind, ResolvedCall, TemplateParameter, TemplateParameters, TypeContainer,
        TypeLoweringContext, TypeLoweringError, WgslTypeConverter,
    },
    ty::{
        ArraySize, ArrayType, AtomicType, MatrixType, Pointer, Reference, ScalarType,
        TextureDimensionality, TextureKind, TextureType, Type, TypeKind, VecSize, VectorType,
    },
};

#[salsa::tracked]
impl InferenceResult {
    /// Infers the type of a global item.
    /// For `const`s and co, it first uses the specified type,
    /// and then uses the body (expression) to infer the return type.
    #[salsa::tracked(returns(ref), cycle_result = infer_cycle_result)]
    pub fn of(
        db: &dyn HirDatabase,
        definition: DefinitionWithBodyId,
    ) -> Self {
        infer_query(db, definition)
    }
}

fn infer_query(
    database: &dyn HirDatabase,
    definition: DefinitionWithBodyId,
) -> InferenceResult {
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

    context.resolve_all()
}

fn infer_cycle_result(
    database: &dyn HirDatabase,
    _: salsa::Id,
    definition: DefinitionWithBodyId,
) -> InferenceResult {
    let mut inference_result = InferenceResult::new(database);
    let (name, range) = get_name_and_range(database, ModuleDefinitionId::from(definition));

    inference_result.diagnostics.push(InferenceDiagnostic {
        source: ExpressionStoreSource::Body,
        kind: InferenceDiagnosticKind::CyclicType { name, range },
    });

    inference_result
}

fn get_name_and_range(
    database: &dyn HirDatabase,
    definition: ModuleDefinitionId,
) -> (Name, base_db::TextRange) {
    match definition {
        ModuleDefinitionId::Module(file_id) => {
            let module_data = ModuleData::of(database, file_id);
            let full_range = TextRange::empty(TextSize::new(0));

            let name = module_data.as_ref().map_or_else(Name::missing, |module| {
                module.name.clone().unwrap_or_else(|| Name::from("package"))
            });
            (name, full_range)
        },
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

/// Runs inference for items that have a body, such as functions.
pub struct InferenceContext<'database> {
    database: &'database dyn HirDatabase,
    owner: ModuleDefinitionId,
    /// Root resolver for the entire module.
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
        source: ExpressionStoreSource,
        diagnostic: InferenceDiagnosticKind,
    ) {
        self.result.diagnostics.push(InferenceDiagnostic {
            source,
            kind: diagnostic,
        });
    }

    fn push_lowering_diagnostics(
        &mut self,
        diagnostics: &mut Vec<TypeLoweringError>,
        store: &ExpressionStore,
    ) {
        for diagnostic in diagnostics.drain(..) {
            self.push_diagnostic(
                store.store_source,
                InferenceDiagnosticKind::InvalidType { error: diagnostic },
            );
        }
    }

    fn resolve_all(mut self) -> InferenceResult {
        self.result.return_type = self.return_type;
        self.result
    }

    fn collect_global_variable(
        &mut self,
        variable: &VariableSignature,
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
        variable: &VariableSignature,
        body: &Body,
    ) {
        let (address_space, access_mode) =
            self.infer_variable_template(&variable.template_parameters, &variable.store);
        if address_space == AddressSpace::Function {
            // Function address space is not allowed at the module level
            self.push_diagnostic(
                variable.store.store_source,
                InferenceDiagnosticKind::UnexpectedTemplateArgument {
                    expression: variable.template_parameters[0],
                },
            );
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
                self.push_diagnostic(
                    store.store_source,
                    InferenceDiagnosticKind::UnexpectedTemplateArgument {
                        expression: template[0],
                    },
                );
                default_address_space
            },
        };
        let access_mode = match template_args.get(1) {
            Some(TemplateParameter::Enumerant(Enumerant::AccessMode(access_mode))) => {
                if address_space == AddressSpace::Storage {
                    *access_mode
                } else {
                    // Only the storage address space allows for an access mode
                    self.push_diagnostic(
                        store.store_source,
                        InferenceDiagnosticKind::UnexpectedTemplateArgument {
                            expression: template[0],
                        },
                    );
                    address_space.default_access_mode()
                }
            },
            None => address_space.default_access_mode(),
            _ => {
                self.push_diagnostic(
                    store.store_source,
                    InferenceDiagnosticKind::UnexpectedTemplateArgument {
                        expression: template[0],
                    },
                );
                address_space.default_access_mode()
            },
        };

        // Mark extra template arguments as errors
        if template.len() > 2 {
            for expression in &template[2..] {
                self.push_diagnostic(
                    store.store_source,
                    InferenceDiagnosticKind::UnexpectedTemplateArgument {
                        expression: *expression,
                    },
                );
            }
        }
        (address_space, access_mode)
    }

    fn collect_global_constant(
        &mut self,
        constant: &ConstantSignature,
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
        override_data: &OverrideSignature,
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
        function_data: &FunctionSignature,
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

    /// Runs type inference on the body and infer the type for `const`s, `var`s and `override`s.
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
                    self.push_diagnostic(
                        body.store_source,
                        InferenceDiagnosticKind::UnexpectedTemplateArgument {
                            expression: template_parameters[0],
                        },
                    );
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
                    self.push_diagnostic(
                        body.store_source,
                        InferenceDiagnosticKind::AssignmentNotAReference {
                            left_side: *left_side,
                            actual: left_type,
                        },
                    );
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
                    self.push_diagnostic(
                        body.store_source,
                        InferenceDiagnosticKind::AssignmentNotAReference {
                            left_side: *left_side,
                            actual: left_type,
                        },
                    );
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
                    self.push_diagnostic(
                        body.store_source,
                        InferenceDiagnosticKind::TypeMismatch {
                            expression: *right_side,
                            actual: r#type,
                            expected: TypeExpectation::Type(TypeExpectationInner::Exact(
                                left_inner,
                            )),
                        },
                    );
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
                    self.push_diagnostic(
                        body.store_source,
                        InferenceDiagnosticKind::AssignmentNotAReference {
                            left_side: *expression,
                            actual: left_type,
                        },
                    );
                    self.error_type()
                };

                if self
                    .expect_type_inner(left_inner, &TypeExpectationInner::IntegerScalar)
                    .is_err()
                {
                    self.push_diagnostic(
                        body.store_source,
                        InferenceDiagnosticKind::TypeMismatch {
                            expression: *expression,
                            actual: left_inner,
                            expected: TypeExpectation::Type(TypeExpectationInner::IntegerScalar),
                        },
                    );
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
                if let TypeKind::Scalar(
                    ScalarType::I32 | ScalarType::U32 | ScalarType::I64 | ScalarType::U64,
                ) = r#type.kind(self.database).unref(self.database).as_ref()
                {
                    Ok(())
                } else {
                    Err(())
                }
            },
            TypeExpectationInner::IntegerIndex => {
                if let TypeKind::Scalar(
                    ScalarType::I32 | ScalarType::U32 | ScalarType::AbstractInt,
                ) = r#type.kind(self.database).unref(self.database).as_ref()
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
                    self.push_diagnostic(
                        store.store_source,
                        InferenceDiagnosticKind::TypeMismatch {
                            expression,
                            actual: r#type,
                            expected: expected.clone(),
                        },
                    );
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
                            self.push_diagnostic(
                                store.store_source,
                                InferenceDiagnosticKind::NoSuchField {
                                    expression: *field_expression,
                                    name: name.clone(),
                                    r#type: expression_type,
                                },
                            );
                            self.error_type()
                        }
                    },
                    TypeKind::Vector(vec_type) => {
                        if let Ok(r#type) = self.vec_swizzle(vec_type, name) {
                            r#type
                        } else {
                            self.push_diagnostic(
                                store.store_source,
                                InferenceDiagnosticKind::NoSuchField {
                                    expression: *field_expression,
                                    name: name.clone(),
                                    r#type: expression_type,
                                },
                            );
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
                        self.push_diagnostic(
                            store.store_source,
                            InferenceDiagnosticKind::NoSuchField {
                                expression: *field_expression,
                                name: name.clone(),
                                r#type: expression_type,
                            },
                        );
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
                    .map(|&argument| {
                        (
                            argument,
                            self.infer_expression(argument, store).unref(self.database),
                        )
                    })
                    .collect();
                self.infer_call(expression, ident_expression, arguments, store)
            },
            Expression::Index { left_side, index } => {
                let left_side = self.infer_expression(*left_side, store);
                let left_kind = left_side.kind(self.database);
                let is_reference = matches!(left_kind, TypeKind::Reference(_));
                let left_inner = left_kind.unref(self.database);

                let index_type = self.infer_expression(*index, store);
                let index_kind = index_type.kind(self.database);
                let index_inner = index_kind.unref(self.database);
                if !index_inner.is_index() {
                    self.push_diagnostic(
                        store.store_source,
                        InferenceDiagnosticKind::TypeMismatch {
                            expression: *index,
                            expected: TypeExpectation::Type(TypeExpectationInner::IntegerIndex),
                            actual: index_type.unref(self.database),
                        },
                    );
                }

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
                        self.push_diagnostic(
                            store.store_source,
                            InferenceDiagnosticKind::ArrayAccessInvalidType {
                                expression,
                                r#type: left_side,
                            },
                        );
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
                    Literal::Int(_, BuiltinInt::I64) => TypeKind::Scalar(ScalarType::I64),
                    Literal::Int(_, BuiltinInt::U64) => TypeKind::Scalar(ScalarType::U64),
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
        arguments: &[(ExpressionId, Type)],
        store: &ExpressionStore,
        callee: ExpressionId,
        expression: ExpressionId,
    ) -> Type {
        if function.parameters.len() == arguments.len() {
            for (expected, (actual_expression, actual_type)) in
                function.parameters().zip(arguments.iter().copied())
            {
                if !actual_type.is_convertible_to(expected, self.database) {
                    self.push_diagnostic(
                        store.store_source,
                        InferenceDiagnosticKind::TypeMismatch {
                            expression: actual_expression,
                            actual: actual_type,
                            expected: TypeExpectation::Type(TypeExpectationInner::Exact(expected)),
                        },
                    );
                }
            }

            function.return_type.unwrap_or_else(|| self.error_type())
        } else {
            self.push_diagnostic(
                store.store_source,
                InferenceDiagnosticKind::FunctionCallArgCountMismatch {
                    expression: callee,
                    n_expected: function.parameters.len(),
                    n_actual: arguments.len(),
                },
            );
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
                self.push_diagnostic(
                    store.store_source,
                    InferenceDiagnosticKind::AddressOfNotReference {
                        expression,
                        actual: expression_type,
                    },
                );
                return self.error_type();
            },
            UnaryOperator::Indirection => {
                let argument_type = expression_type.unref(self.database);
                if let TypeKind::Pointer(pointer) = argument_type.kind(self.database) {
                    return self.ptr_to_ref(&pointer);
                }
                self.push_diagnostic(
                    store.store_source,
                    InferenceDiagnosticKind::DerefNotAPointer {
                        expression,
                        actual: argument_type,
                    },
                );
                return self.error_type();
            },
        };

        let argument_type = expression_type.unref(self.database);
        self.call_builtin(
            store,
            expression,
            builtin,
            &[(expression, argument_type)],
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
            store,
            expression,
            builtin,
            &[(left_side, left_type), (right_side, rhs_type)],
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
            expression,
            &ident_expression.path,
            &ident_expression.template_parameters,
        );
        self.push_lowering_diagnostics(&mut context.diagnostics, store);

        match lowered {
            Lowered::GlobalConstant(id) => {
                InferenceResult::of(self.database, DefinitionWithBodyId::GlobalConstant(id))
                    .return_type
            },
            Lowered::GlobalVariable(id) => {
                InferenceResult::of(self.database, DefinitionWithBodyId::GlobalVariable(id))
                    .return_type
            },
            Lowered::Override(id) => {
                InferenceResult::of(self.database, DefinitionWithBodyId::Override(id)).return_type
            },
            Lowered::Local(id) => self.result.type_of_binding[id],
            Lowered::Type(_)
            | Lowered::TypeWithoutTemplate(_)
            | Lowered::Function(_)
            | Lowered::BuiltinFunction
            | Lowered::Enumerant(_) => {
                self.push_diagnostic(
                    store.store_source,
                    InferenceDiagnosticKind::ExpectedLoweredKind {
                        expression,
                        expected: LoweredKind::Variable,
                        actual: lowered.kind(),
                        path: ident_expression.path.clone(),
                    },
                );
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
            let kind = vec_size.try_into().map_or(TypeKind::Error, |size| {
                TypeKind::Vector(VectorType {
                    size,
                    component_type: inner,
                })
            });
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
        store: &ExpressionStore,
        expression: ExpressionId,
        builtin_id: BuiltinId,
        arguments: &[(ExpressionId, Type)],
        name: Option<&'static str>,
    ) -> Type {
        self.call_builtin_inner(store, expression, builtin_id, arguments, name)
    }

    fn call_builtin_inner(
        &mut self,
        store: &ExpressionStore,
        expression: ExpressionId,
        builtin_id: BuiltinId,
        arguments: &[(ExpressionId, Type)],
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
            self.push_diagnostic(
                store.store_source,
                InferenceDiagnosticKind::NoBuiltinOverload {
                    expression,
                    builtin: builtin_id,
                    name,
                    parameters: arguments
                        .iter()
                        .copied()
                        .map(|(_, r#type)| r#type)
                        .collect(),
                },
            );
            self.error_type()
        }
    }

    fn try_call_builtin(
        &self,
        builtin_id: BuiltinId,
        arguments: &[(ExpressionId, Type)],
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
        arguments: &[(ExpressionId, Type)],
    ) -> Result<(Type, u32), ()> {
        let function_type = signature.r#type.lookup(self.database);

        if function_type.parameters.len() != arguments.len() {
            return Err(());
        }

        let conversion_rank = 0;
        let mut unification_table = UnificationTable::default();
        for (expected, &found) in function_type.parameters().zip(arguments.iter()) {
            unify(self.database, &mut unification_table, expected, found.1)?;
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
        arguments: Vec<(ExpressionId, Type)>,
        store: &ExpressionStore,
    ) -> Type {
        let resolver = self
            .resolver_for_expression(expression)
            .unwrap_or_else(|| self.resolver.clone());
        let mut context = TypeLoweringContext::new(self.database, &resolver, store);
        let lowered = context.lower(expression, &callee.path, &callee.template_parameters);
        self.push_lowering_diagnostics(&mut context.diagnostics, store);

        match lowered {
            Lowered::Type(r#type) => {
                self.call_templated_type_constructor(store, expression, r#type, arguments)
            },
            Lowered::TypeWithoutTemplate(r#type) => {
                self.call_type_without_template_constructor(store, expression, r#type, arguments)
            },
            Lowered::Function(id) => {
                let details = id.lookup(self.database);
                self.result
                    .call_resolutions
                    .insert(expression, ResolvedCall::Function(id));
                self.validate_function_call(&details, &arguments, store, expression, expression)
            },
            Lowered::BuiltinFunction => {
                let template_args = context.eval_template_args(
                    TypeContainer::Expression(expression),
                    &callee.template_parameters,
                );
                self.push_lowering_diagnostics(&mut context.diagnostics, store);
                self.call_builtin_function(store, expression, callee, template_args, &arguments)
            },
            Lowered::Enumerant(_)
            | Lowered::GlobalConstant(_)
            | Lowered::GlobalVariable(_)
            | Lowered::Override(_)
            | Lowered::Local(_) => {
                self.push_diagnostic(
                    store.store_source,
                    InferenceDiagnosticKind::ExpectedLoweredKind {
                        expression,
                        expected: LoweredKind::Function,
                        actual: lowered.kind(),
                        path: callee.path.clone(),
                    },
                );
                self.error_type()
            },
        }
    }

    fn call_builtin_function(
        &mut self,
        store: &ExpressionStore,
        expression: ExpressionId,
        callee: &IdentExpression,
        mut template_parameters: TemplateParameters,
        arguments: &[(ExpressionId, Type)],
    ) -> Type {
        let Some(name) = callee.path.mod_path().as_ident() else {
            self.push_diagnostic(
                store.store_source,
                InferenceDiagnosticKind::WgslError {
                    expression,
                    message: format!("invalid builtin {}", callee.path.mod_path()),
                },
            );
            return self.error_type();
        };

        let mut converter = WgslTypeConverter::new(self.database);
        let mut template_args = vec![];
        while let Some((template_parameter, _)) = template_parameters.take_next() {
            if let Some(template_parameter) =
                converter.template_parameter_to_wgsl_types(template_parameter)
            {
                template_args.push(template_parameter);
            } else {
                self.push_diagnostic(
                    store.store_source,
                    InferenceDiagnosticKind::WgslError {
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
            .map(|(_, r#type)| converter.to_wgsl_types(*r#type))
            .collect();

        let Some(converted_arguments) = converted_arguments else {
            // One of the arguments had an error type
            return self.error_type();
        };

        let return_type = wgsl_types::builtin::type_builtin_fn(
            name.as_str(),
            template_args,
            &converted_arguments,
        );

        match return_type {
            Ok(Some(r#type)) => converter.from_wgsl_types(r#type),
            Ok(None) => self.error_type(),
            Err(error) => {
                self.push_diagnostic(
                    store.store_source,
                    InferenceDiagnosticKind::WgslError {
                        expression,
                        message: error.to_string(),
                    },
                );
                self.error_type()
            },
        }
    }

    #[expect(
        clippy::too_many_lines,
        reason = "large bug not complex match expression"
    )]
    /// Constructor for a type with a fully specified template.
    fn call_templated_type_constructor(
        &mut self,
        store: &ExpressionStore,
        expression: ExpressionId,
        r#type: Type,
        arguments: Vec<(ExpressionId, Type)>,
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
                self.call_scalar_constructor(store, scalar_type, expression, r#type, arguments)
            },
            TypeKind::Array(array_type) => {
                for (argument_expression, argument_type) in &arguments {
                    if !argument_type.is_convertible_to(array_type.inner, self.database) {
                        self.push_diagnostic(
                            store.store_source,
                            InferenceDiagnosticKind::TypeMismatch {
                                expression: *argument_expression,
                                expected: TypeExpectation::Type(TypeExpectationInner::Exact(
                                    array_type.inner,
                                )),
                                actual: *argument_type,
                            },
                        );
                    }
                }
                #[expect(
                    clippy::as_conversions,
                    reason = "constructing an array with too many parameters is an error anyway"
                )]
                if let ArraySize::Constant(size) = array_type.size
                    && arguments.len() != size as usize
                {
                    self.push_diagnostic(
                        store.store_source,
                        InferenceDiagnosticKind::FunctionCallArgCountMismatch {
                            expression,
                            n_expected: size as usize,
                            n_actual: arguments.len(),
                        },
                    );
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
                    self.push_diagnostic(
                        store.store_source,
                        InferenceDiagnosticKind::NoConstructor {
                            expression,
                            builtins: construction_builtin_id,
                            r#type,
                            parameters: arguments.into_iter().map(|(_, r#type)| r#type).collect(),
                        },
                    );
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
                    self.push_diagnostic(
                        store.store_source,
                        InferenceDiagnosticKind::NoConstructor {
                            expression,
                            builtins: construction_builtin_id,
                            r#type,
                            parameters: arguments.into_iter().map(|(_, r#type)| r#type).collect(),
                        },
                    );
                    self.error_type()
                }
            },
            TypeKind::Struct(struct_id) => {
                self.validate_struct_constructor(store, struct_id, expression, r#type, &arguments)
            },

            // Never constructible
            TypeKind::Texture(_)
            | TypeKind::Sampler(_)
            | TypeKind::Pointer(_)
            | TypeKind::Atomic(_)
            | TypeKind::StorageTypeOfTexelFormat(_)
            | TypeKind::BoundVariable(_)
            | TypeKind::Reference(_) => {
                self.push_diagnostic(
                    store.store_source,
                    InferenceDiagnosticKind::InvalidConstructionType { expression, r#type },
                );
                self.error_type()
            },
            TypeKind::Error => r#type,
        }
    }

    #[expect(
        clippy::too_many_lines,
        reason = "large bug not complex match expression"
    )]
    /// Constructor for just a type name.
    fn call_type_without_template_constructor(
        &mut self,
        store: &ExpressionStore,
        expression: ExpressionId,
        r#type: Type,
        arguments: Vec<(ExpressionId, Type)>,
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
                self.call_scalar_constructor(store, scalar_type, expression, r#type, arguments)
            },
            TypeKind::Array(array_type) => {
                let Some((_, mut first_argument_type)) = arguments.first().copied() else {
                    self.push_diagnostic(
                        store.store_source,
                        InferenceDiagnosticKind::FunctionCallArgCountMismatch {
                            expression,
                            n_expected: 1,
                            n_actual: arguments.len(),
                        },
                    );
                    return self.error_type();
                };

                // all of the following arguments must be the same type as the first argument
                for (argument_expression, argument_type) in &arguments[1..] {
                    if argument_type.is_convertible_to(first_argument_type, self.database) {
                        // Everything is as intended
                    } else if first_argument_type.is_convertible_to(*argument_type, self.database) {
                        // Narrowing the expected type
                        first_argument_type = *argument_type;
                    } else {
                        self.push_diagnostic(
                            store.store_source,
                            InferenceDiagnosticKind::TypeMismatch {
                                expression: *argument_expression,
                                expected: TypeExpectation::Type(TypeExpectationInner::Exact(
                                    first_argument_type,
                                )),
                                actual: *argument_type,
                            },
                        );
                    }
                }
                if let Ok(validated_length) = u32::try_from(arguments.len()) {
                    TypeKind::Array(ArrayType {
                        inner: first_argument_type,
                        binding_array: array_type.binding_array,
                        size: ArraySize::Constant(validated_length),
                    })
                    .intern(self.database)
                } else {
                    self.push_diagnostic(
                        store.store_source,
                        InferenceDiagnosticKind::FunctionCallArgCountMismatch {
                            expression,
                            #[expect(clippy::as_conversions, reason = "usize always holds a u32")]
                            n_expected: ArraySize::MAX as usize,
                            n_actual: arguments.len(),
                        },
                    );
                    TypeKind::Array(ArrayType {
                        inner: first_argument_type,
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
                    self.push_diagnostic(
                        store.store_source,
                        InferenceDiagnosticKind::NoConstructor {
                            expression,
                            builtins: construction_builtin_id,
                            r#type,
                            parameters: arguments.into_iter().map(|(_, r#type)| r#type).collect(),
                        },
                    );
                    self.error_type()
                }
            },
            TypeKind::Matrix(matrix) => {
                if arguments.is_empty() {
                    self.push_diagnostic(
                        store.store_source,
                        InferenceDiagnosticKind::FunctionCallArgCountMismatch {
                            expression,
                            n_expected: 1,
                            n_actual: arguments.len(),
                        },
                    );
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
                    self.push_diagnostic(
                        store.store_source,
                        InferenceDiagnosticKind::NoConstructor {
                            expression,
                            builtins: construction_builtin_id,
                            r#type,
                            parameters: arguments.into_iter().map(|(_, r#type)| r#type).collect(),
                        },
                    );
                    self.error_type()
                }
            },
            TypeKind::Struct(struct_id) => {
                self.validate_struct_constructor(store, struct_id, expression, r#type, &arguments)
            },
            // Never constructible
            TypeKind::Texture(_)
            | TypeKind::Sampler(_)
            | TypeKind::Pointer(_)
            | TypeKind::Atomic(_)
            | TypeKind::StorageTypeOfTexelFormat(_)
            | TypeKind::BoundVariable(_)
            | TypeKind::Reference(_) => {
                self.push_diagnostic(
                    store.store_source,
                    InferenceDiagnosticKind::InvalidConstructionType { expression, r#type },
                );
                self.error_type()
            },
            TypeKind::Error => r#type,
        }
    }

    fn call_scalar_constructor(
        &mut self,
        store: &ExpressionStore,
        scalar_type: ScalarType,
        expression: ExpressionId,
        r#type: Type,
        arguments: Vec<(ExpressionId, Type)>,
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
                #[expect(
                    clippy::unreachable,
                    reason = "TODO: Refactor to make this not representable"
                )]
                {
                    unreachable!("cannot construct abstract types")
                }
            },
            ScalarType::I64 => {
                Builtin::builtin_op_i64_constructor(self.database).intern(self.database)
            },
            ScalarType::U64 => {
                Builtin::builtin_op_u64_constructor(self.database).intern(self.database)
            },
        };

        let construction_result = self.try_call_builtin(construction_builtin_id, &arguments);
        if let Ok((r#type, _)) = construction_result {
            r#type
        } else {
            self.push_diagnostic(
                store.store_source,
                InferenceDiagnosticKind::NoConstructor {
                    expression,
                    builtins: construction_builtin_id,
                    r#type,
                    parameters: arguments.into_iter().map(|(_, r#type)| r#type).collect(),
                },
            );
            self.error_type()
        }
    }

    fn validate_struct_constructor(
        &mut self,
        store: &ExpressionStore,
        struct_id: StructId,
        expression: ExpressionId,
        r#type: Type,
        arguments: &[(ExpressionId, Type)],
    ) -> Type {
        // https://www.w3.org/TR/WGSL/#zero-value-builtin-function
        if arguments.is_empty() {
            return r#type;
        }

        let signature = self.database.struct_data(struct_id).0;
        if arguments.len() != signature.fields.len() {
            self.push_diagnostic(
                store.store_source,
                InferenceDiagnosticKind::FunctionCallArgCountMismatch {
                    expression,
                    n_expected: signature.fields.len(),
                    n_actual: arguments.len(),
                },
            );
            return self.error_type();
        }

        let field_types = &self.database.field_types(struct_id).0;
        let mut has_errors = false;
        for ((field_data, field_type), (argument_expression, argument_type)) in
            field_types.iter().zip(arguments.iter())
        {
            if !argument_type.is_convertible_to(*field_type, self.database) {
                self.push_diagnostic(
                    store.store_source,
                    InferenceDiagnosticKind::TypeMismatch {
                        expression: *argument_expression,
                        expected: TypeExpectation::from_type(*field_type),
                        actual: *argument_type,
                    },
                );
                has_errors = true;
            }
        }

        if has_errors {
            self.error_type()
        } else {
            r#type
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

#[derive(PartialEq, Eq, Copy, Clone)]
enum AbstractHandling {
    Concretize,
    Abstract,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TypeExpectationInner {
    Exact(Type),
    IntegerScalar,
    IntegerIndex,
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
