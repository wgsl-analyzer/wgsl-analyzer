mod builtin;
mod eval;
mod unify;

use std::{
    borrow::Cow, collections::hash_map::Entry, ffi::os_str::Display, fmt, ops::Index, str::FromStr,
};

use either::Either;
use hir_def::{
    HasSource,
    body::{BindingId, Body},
    data::{
        FieldId, FunctionData, GlobalConstantData, GlobalVariableData, OverrideData, ParameterId,
        StructData, TypeAliasData,
    },
    database::{
        DefinitionWithBodyId, FunctionId, GlobalConstantId, GlobalVariableId, Lookup, OverrideId,
        StructId, TypeAliasId,
    },
    expression::{
        ArithmeticOperation, BinaryOperation, ComparisonOperation, Expression, ExpressionId,
        Statement, StatementId, SwitchCaseSelector, UnaryOperator,
    },
    expression_store::{ExpressionStore, ExpressionStoreSource},
    module_data::Name,
    resolver::{ResolveKind, Resolver},
    type_ref::{self, VecDimensionality},
    type_specifier::{IdentExpression, TypeSpecifier, TypeSpecifierId},
};
use la_arena::ArenaMap;
use rustc_hash::FxHashMap;
use triomphe::Arc;
use wgsl_types::{
    inst::Instance,
    syntax::{AccessMode, AddressSpace, Enumerant},
};

use crate::{
    builtins::{Builtin, BuiltinId, BuiltinOverload, BuiltinOverloadId},
    database::HirDatabase,
    function::{FunctionDetails, ResolvedFunctionId},
    infer::{
        eval::{TemplateParameters, TpltParam},
        unify::{UnificationTable, unify},
    },
    ty::{
        ArraySize, ArrayType, AtomicType, BoundVar, MatrixType, Pointer, Reference, ScalarType,
        TexelFormat, TextureDimensionality, TextureKind, TextureType, TyKind, Type, VecSize,
        VectorType,
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
    let mut context = InferenceContext::new(database, body, definition, resolver);

    match definition {
        DefinitionWithBodyId::Function(function) => {
            let data = database.function_data(function).0;
            let return_type = context.collect_fn(&data);
            context.infer_body(return_type, AbstractHandling::Concretize);
        },
        DefinitionWithBodyId::GlobalVariable(var) => {
            let data = database.global_var_data(var).0;
            let return_type = context.collect_global_variable(&data);
            context.infer_body(return_type, AbstractHandling::Concretize);
            context.infer_variables(&data);
        },
        DefinitionWithBodyId::GlobalConstant(constant) => {
            let data = database.global_constant_data(constant).0;
            let return_type = context.collect_global_constant(&data);
            context.infer_body(return_type, AbstractHandling::Abstract);
        },
        DefinitionWithBodyId::Override(override_decl) => {
            let data = database.override_data(override_decl).0;
            let return_type = context.collect_override(&data);
            context.infer_body(return_type, AbstractHandling::Concretize);
        },
    };

    Arc::new(context.resolve_all())
}

pub fn infer_cycle_result(
    database: &dyn HirDatabase,
    _cycle: &[String],
    definition: &DefinitionWithBodyId,
) -> Arc<InferenceResult> {
    let mut inference_result: InferenceResult = InferenceResult::new(database);

    let (name, range) = match *definition {
        DefinitionWithBodyId::Function(id) => (
            database.function_data(id).0.name.clone(),
            id.lookup(database)
                .source(database)
                .original_file_range(database)
                .range,
        ),
        DefinitionWithBodyId::GlobalVariable(id) => (
            database.global_var_data(id).0.name.clone(),
            id.lookup(database)
                .source(database)
                .original_file_range(database)
                .range,
        ),
        DefinitionWithBodyId::GlobalConstant(id) => (
            database.global_constant_data(id).0.name.clone(),
            id.lookup(database)
                .source(database)
                .original_file_range(database)
                .range,
        ),
        DefinitionWithBodyId::Override(id) => (
            database.override_data(id).0.name.clone(),
            id.lookup(database)
                .source(database)
                .original_file_range(database)
                .range,
        ),
    };

    inference_result
        .diagnostics
        .push(InferenceDiagnostic::CyclicType { name, range });

    Arc::new(inference_result)
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

#[derive(PartialEq, Eq, Debug, Clone)]
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
        InternedStandardTypes {
            unknown: TyKind::Error.intern(database),
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
            type_of_expression: Default::default(),
            type_of_binding: Default::default(),
            diagnostics: Default::default(),
            return_type: TyKind::Error.intern(database),
            call_resolutions: Default::default(),
            field_resolutions: Default::default(),
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

    pub fn diagnostics(&self) -> &[InferenceDiagnostic] {
        &self.diagnostics
    }

    pub fn return_type(&self) -> Type {
        self.return_type
    }
}

impl Index<ExpressionId> for InferenceResult {
    type Output = Type;
    fn index(
        &self,
        expr: ExpressionId,
    ) -> &Type {
        self.type_of_expression
            .get(expr)
            .unwrap_or(&self.standard_types.unknown)
    }
}

impl Index<BindingId> for InferenceResult {
    type Output = Type;
    fn index(
        &self,
        binding: BindingId,
    ) -> &Type {
        self.type_of_binding
            .get(binding)
            .unwrap_or(&self.standard_types.unknown)
    }
}

/// Runs inference for items that have a body, such as functions
pub struct InferenceContext<'database> {
    database: &'database dyn HirDatabase,
    owner: DefinitionWithBodyId,
    /// Root resolver for the entire module
    resolver: Resolver,
    body: Arc<Body>,
    result: InferenceResult, // set in collect_* calls
    return_ty: Type,
}

impl<'database> InferenceContext<'database> {
    pub fn new(
        database: &'database dyn HirDatabase,
        body: Arc<Body>,
        owner: DefinitionWithBodyId,
        resolver: Resolver,
    ) -> Self {
        Self {
            database,
            owner,
            resolver,
            body,
            result: InferenceResult::new(database),
            return_ty: TyKind::Error.intern(database),
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

    fn set_expression_ty(
        &mut self,
        expression: ExpressionId,
        r#type: Type,
    ) {
        self.result.type_of_expression.insert(expression, r#type);
    }

    fn set_binding_ty(
        &mut self,
        binding: BindingId,
        r#type: Type,
    ) {
        self.result.type_of_binding.insert(binding, r#type);
    }

    fn bind_return_ty(
        &mut self,
        r#type: Option<Type>,
    ) {
        if let Some(r#type) = r#type
            && let Some(binding) = self.body.main_binding
        {
            self.set_binding_ty(binding, r#type);
        }

        self.return_ty = r#type.unwrap_or_else(|| self.error_ty());
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
        self.result.return_type = self.return_ty;
        self.result
    }

    fn collect_global_variable(
        &mut self,
        var: &GlobalVariableData,
    ) -> Option<Type> {
        let r#type = var
            .r#type
            .map(|r#type| self.lower_ty(r#type, &self.resolver.clone(), &var.store));

        self.bind_return_ty(r#type);
        r#type
    }

    fn infer_variables(
        &mut self,
        var: &GlobalVariableData,
    ) {
        let (address_space, access_mode) = self.infer_variable_template(&var.generics, &var.store);

        self.bind_return_ty(Some(self.make_ref(
            self.return_ty,
            address_space,
            access_mode,
        )));
    }

    fn infer_variable_template(
        &mut self,
        template: &[ExpressionId],
        store: &ExpressionStore,
    ) -> (AddressSpace, AccessMode) {
        let mut ctx = TyLoweringContext::new(self.database, &self.resolver, store);
        let template_args: Vec<_> = template.iter().map(|arg| ctx.eval_tplt_arg(*arg)).collect();
        self.push_lowering_diagnostics(&mut ctx.diagnostics, store);

        let default_address_space = match store.store_source {
            ExpressionStoreSource::Body => AddressSpace::Function,
            // TODO: Is this the correct default
            ExpressionStoreSource::Signature => AddressSpace::Handle,
        };

        let address_space = match template_args.get(0) {
            Some(TpltParam::Enumerant(Enumerant::AddressSpace(address_space))) => *address_space,
            None => default_address_space,
            _ => {
                self.push_diagnostic(InferenceDiagnostic::UnexpectedTemplateArgument {
                    expression: template[0],
                });
                default_address_space
            },
        };
        let access_mode = match template_args.get(1) {
            Some(TpltParam::Enumerant(Enumerant::AccessMode(access_mode))) => *access_mode,
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
            for expression in template[2..].iter() {
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
    ) -> Option<Type> {
        let r#type = constant
            .r#type
            .map(|r#type| self.lower_ty(r#type, &self.resolver.clone(), &constant.store));

        self.bind_return_ty(r#type);
        r#type
    }

    fn collect_override(
        &mut self,
        override_data: &OverrideData,
    ) -> Option<Type> {
        let r#type = override_data
            .r#type
            .map(|r#type| self.lower_ty(r#type, &self.resolver.clone(), &override_data.store));

        self.bind_return_ty(r#type);
        r#type
    }

    fn collect_fn(
        &mut self,
        function_data: &FunctionData,
    ) -> Option<Type> {
        let body = self.body.clone();
        for ((_, parameter), &binding_id) in function_data.parameters.iter().zip(&body.parameters) {
            let param_ty = self.lower_ty(
                parameter.r#type,
                &self.resolver.clone(),
                &function_data.store,
            );
            self.set_binding_ty(binding_id, param_ty);
        }
        let r#type = function_data
            .return_type
            .map(|type_ref| self.lower_ty(type_ref, &self.resolver.clone(), &function_data.store));
        self.return_ty = r#type.unwrap_or_else(|| self.error_ty());
        r#type
    }

    /// Runs type inference on the body and infer the type for `const`s, `var`s and `override`s
    fn infer_body(
        &mut self,
        return_type: Option<Type>,
        abstract_handling: AbstractHandling,
    ) {
        match self.body.root {
            Some(Either::Left(statement)) => {
                self.infer_statement(statement);
            },
            Some(Either::Right(expression)) => {
                let body = self.body.clone();

                let r#type =
                    self.infer_initializer(&body, Some(expression), return_type, abstract_handling);

                if return_type.is_none() {
                    self.bind_return_ty(Some(r#type));
                }
            },
            None => (),
        }
    }

    fn resolver_for_expression(
        &self,
        expression: ExpressionId,
    ) -> Option<Resolver> {
        let DefinitionWithBodyId::Function(function) = self.owner else {
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
        let DefinitionWithBodyId::Function(function) = self.owner else {
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

    #[expect(clippy::too_many_lines, reason = "TODO")]
    fn infer_statement(
        &mut self,
        statement: StatementId,
    ) {
        let body = self.body.clone();
        let resolver = self.resolver_for_statement(statement);

        match &body.statements[statement] {
            Statement::Compound { statements } => {
                for statement in statements {
                    self.infer_statement(*statement);
                }
            },
            Statement::Variable {
                binding_id,
                type_ref,
                initializer,
                generics,
            } => {
                let r#type = type_ref.map(|r#type| self.lower_ty(r#type, &resolver, &body));
                let r#type = self.infer_initializer(
                    &body,
                    *initializer,
                    r#type,
                    AbstractHandling::Concretize,
                );

                let (address_space, access_mode) =
                    self.infer_variable_template(generics, &body.store);
                let ref_ty = self.make_ref(r#type, address_space, access_mode);
                self.set_binding_ty(*binding_id, ref_ty);
            },
            Statement::Const {
                binding_id,
                type_ref,
                initializer,
                ..
            } => {
                let r#type = type_ref.map(|r#type| self.lower_ty(r#type, &resolver, &body));
                let r#type =
                    self.infer_initializer(&body, *initializer, r#type, AbstractHandling::Abstract);
                self.set_binding_ty(*binding_id, r#type);
            },
            Statement::Let {
                binding_id,
                type_ref,
                initializer,
                ..
            } => {
                let r#type = type_ref.map(|r#type| self.lower_ty(r#type, &resolver, &body));
                let r#type = self.infer_initializer(
                    &body,
                    *initializer,
                    r#type,
                    AbstractHandling::Concretize,
                );
                self.set_binding_ty(*binding_id, r#type);
            },

            Statement::Return { expression } => {
                if let Some(expression) = expression {
                    self.infer_expression_expect(
                        *expression,
                        &TypeExpectation::from_ty(self.return_ty),
                        &body.store,
                    );
                }
            },
            Statement::Assignment {
                left_side,
                right_side,
            } => {
                let left_ty = self.infer_expression(*left_side, &body.store);

                let kind = left_ty.kind(self.database);
                let left_inner = if let TyKind::Reference(reference) = kind {
                    reference.inner
                } else {
                    self.push_diagnostic(InferenceDiagnostic::AssignmentNotAReference {
                        left_side: *left_side,
                        actual: left_ty,
                    });
                    self.error_ty()
                };

                self.infer_expression_expect(
                    *right_side,
                    &TypeExpectation::from_ty(left_inner),
                    &body.store,
                );
            },
            Statement::CompoundAssignment {
                left_side,
                right_side,
                op,
            } => {
                let left_ty = self.infer_expression(*left_side, &body.store);

                let left_kind = left_ty.kind(self.database);
                let left_inner = if let TyKind::Reference(reference) = left_kind {
                    reference.inner
                } else {
                    self.push_diagnostic(InferenceDiagnostic::AssignmentNotAReference {
                        left_side: *left_side,
                        actual: left_ty,
                    });
                    self.error_ty()
                };

                let r#type =
                    self.infer_binary_op(*left_side, *right_side, (*op).into(), &body.store);

                self.expect_same_type(*left_side, r#type, left_inner);
            },
            Statement::PhonyAssignment { right_side } => {
                self.infer_expression(*right_side, &body.store);
            },
            Statement::IncrDecr { expression, .. } => {
                let left_ty = self.infer_expression(*expression, &body.store);

                let left_kind = left_ty.kind(self.database);
                let left_inner = if let TyKind::Reference(reference) = left_kind {
                    reference.inner
                } else {
                    self.push_diagnostic(InferenceDiagnostic::AssignmentNotAReference {
                        left_side: *expression,
                        actual: left_ty,
                    });
                    self.error_ty()
                };

                if self
                    .expect_ty_inner(left_inner, &TypeExpectationInner::IntegerScalar)
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
                self.infer_statement(*block);
                for else_if_block in else_if_blocks {
                    self.infer_statement(*else_if_block);
                }
                if let Some(else_block) = else_block {
                    self.infer_statement(*else_block);
                }
                self.infer_expression_expect(
                    *condition,
                    &TypeExpectation::from_ty(self.bool_ty()),
                    &body.store,
                );
            },
            Statement::While { condition, block } => {
                self.infer_statement(*block);
                self.infer_expression_expect(
                    *condition,
                    &TypeExpectation::from_ty(self.bool_ty()),
                    &body.store,
                );
            },
            Statement::Switch {
                expression,
                case_blocks,
            } => {
                let r#type = self
                    .infer_expression(*expression, &body.store)
                    .unref(self.database);

                for (selectors, case) in case_blocks {
                    for selector in selectors {
                        if let SwitchCaseSelector::Expression(selector) = selector {
                            self.infer_expression_expect(
                                *selector,
                                &TypeExpectation::from_ty(r#type),
                                &body.store,
                            );
                        }
                    }
                    self.infer_statement(*case);
                }
            },
            Statement::For {
                initializer,
                condition,
                continuing_part,
                block,
            } => {
                if let Some(init) = initializer {
                    self.infer_statement(*init);
                }
                if let Some(cont) = continuing_part {
                    self.infer_statement(*cont);
                }

                if let Some(condition) = condition {
                    self.infer_expression_expect(
                        *condition,
                        &TypeExpectation::from_ty(self.bool_ty()),
                        &body.store,
                    );
                }

                self.infer_statement(*block);
            },
            Statement::Loop { body } => {
                self.infer_statement(*body);
            },
            Statement::Assert { expression } => {
                self.infer_expression_expect(
                    *expression,
                    &TypeExpectation::from_ty(self.bool_ty()),
                    &body.store,
                );
            },
            Statement::Discard | Statement::Break | Statement::Continue | Statement::Missing => {},
            Statement::Continuing { block } => self.infer_statement(*block),
            Statement::BreakIf { condition } => {
                self.infer_expression_expect(
                    *condition,
                    &TypeExpectation::from_ty(self.bool_ty()),
                    &body.store,
                );
            },
            Statement::Expression { expression } => {
                self.infer_expression(*expression, &body.store);
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
                self.infer_expression_expect(initializer, &TypeExpectation::from_ty(r#type), store);
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
            (None, None) => self.error_ty(),
        }
    }

    fn expect_ty_inner(
        &self,
        r#type: Type,
        expectation: &TypeExpectationInner,
    ) -> Result<(), ()> {
        let ty_kind = r#type.kind(self.database);
        if ty_kind == TyKind::Error {
            return Ok(());
        }

        match *expectation {
            TypeExpectationInner::Exact(expected_type) => {
                if expected_type.kind(self.database) == TyKind::Error
                    || r#type.is_convertible_to(expected_type, self.database)
                {
                    Ok(())
                } else {
                    Err(())
                }
            },
            TypeExpectationInner::IntegerScalar => {
                if let TyKind::Scalar(ScalarType::I32 | ScalarType::U32) =
                    r#type.kind(self.database).unref(self.database).as_ref()
                {
                    Ok(())
                } else {
                    Err(())
                }
            },
        }
    }

    fn expect_same_type(
        &mut self,
        expression: ExpressionId,
        expected: Type,
        actual: Type,
    ) {
        let actual_unref = actual.unref(self.database);
        if expected != actual_unref {
            self.push_diagnostic(InferenceDiagnostic::TypeMismatch {
                expression,
                actual: actual_unref,
                expected: TypeExpectation::Type(TypeExpectationInner::Exact(expected)),
            });
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
                if self.expect_ty_inner(r#type, expected_type) != Ok(()) {
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

    #[expect(clippy::too_many_lines, reason = "TODO")]
    fn infer_expression(
        &mut self,
        expression: ExpressionId,
        store: &ExpressionStore,
    ) -> Type {
        let r#type = match &store[expression] {
            Expression::Missing => self.error_ty(),
            Expression::BinaryOperation {
                left_side,
                right_side,
                operation,
            } => self.infer_binary_op(*left_side, *right_side, *operation, store),
            Expression::UnaryOperator { expression, op } => {
                self.infer_unary_op(*expression, *op, store)
            },
            Expression::Field {
                expression: field_expression,
                name,
            } => {
                let expression_ty = self.infer_expression(*field_expression, store);
                if expression_ty.is_err(self.database) {
                    return self.error_ty();
                }

                match expression_ty
                    .kind(self.database)
                    .unref(self.database)
                    .as_ref()
                {
                    TyKind::Struct(r#struct) => {
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

                            let field_ty = field_types[field];
                            // TODO: correct Address Spaces/access mode
                            // Implement it like this https://github.com/wgsl-tooling-wg/wesl-rs/blob/fea56c869ba2ee8825b7b06e4d9d0d2876b2bc77/crates/wesl/src/eval/ty.rs#L163
                            self.make_ref(field_ty, AddressSpace::Private, AccessMode::ReadWrite)
                        } else {
                            self.push_diagnostic(InferenceDiagnostic::NoSuchField {
                                expression: *field_expression,
                                name: name.clone(),
                                r#type: expression_ty,
                            });
                            self.error_ty()
                        }
                    },
                    TyKind::Vector(vec_type) => {
                        if let Ok(r#type) = self.vec_swizzle(vec_type, name) {
                            r#type
                        } else {
                            self.push_diagnostic(InferenceDiagnostic::NoSuchField {
                                expression: *field_expression,
                                name: name.clone(),
                                r#type: expression_ty,
                            });
                            self.error_ty()
                        }
                    },
                    TyKind::Error
                    | TyKind::Scalar(_)
                    | TyKind::Atomic(_)
                    | TyKind::Matrix(_)
                    | TyKind::Array(_)
                    | TyKind::Texture(_)
                    | TyKind::Sampler(_)
                    | TyKind::Reference(_)
                    | TyKind::Pointer(_)
                    | TyKind::BoundVar(_)
                    | TyKind::StorageTypeOfTexelFormat(_) => {
                        self.push_diagnostic(InferenceDiagnostic::NoSuchField {
                            expression: *field_expression,
                            name: name.clone(),
                            r#type: expression_ty,
                        });
                        self.error_ty()
                    },
                }
            },
            Expression::Call {
                ident_expression,
                arguments,
            } => {
                let arguments: Vec<_> = arguments
                    .iter()
                    .map(|&arg| self.infer_expression(arg, store).unref(self.database))
                    .collect();
                self.infer_call(expression, ident_expression, arguments, store)
            },
            Expression::Index { left_side, index } => {
                let left_side = self.infer_expression(*left_side, store);
                let _index_expression = self.infer_expression(*index, store);
                // TODO check index expression

                let left_kind = left_side.kind(self.database);
                let is_reference = matches!(left_kind, TyKind::Reference(_));

                let left_inner = left_kind.unref(self.database);

                let r#type = match &*left_inner {
                    TyKind::Vector(vec) => {
                        // TODO out of bounds
                        vec.component_type
                    },
                    TyKind::Matrix(matrix_type) => {
                        // TODO out of bounds
                        self.database.intern_ty(TyKind::Vector(VectorType {
                            size: matrix_type.rows,
                            component_type: matrix_type.inner,
                        }))
                    },
                    TyKind::Array(array) => {
                        // TODO out of bounds
                        array.inner
                    },
                    TyKind::Error
                    | TyKind::Scalar(_)
                    | TyKind::Atomic(_)
                    | TyKind::Struct(_)
                    | TyKind::Texture(_)
                    | TyKind::Sampler(_)
                    | TyKind::Reference(_)
                    | TyKind::Pointer(_)
                    | TyKind::BoundVar(_)
                    | TyKind::StorageTypeOfTexelFormat(_) => {
                        self.push_diagnostic(InferenceDiagnostic::ArrayAccessInvalidType {
                            expression,
                            r#type: left_side,
                        });
                        self.error_ty()
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
                let ty_kind = match literal {
                    Literal::Int(_, BuiltinInt::I32) => TyKind::Scalar(ScalarType::I32),
                    Literal::Int(_, BuiltinInt::U32) => TyKind::Scalar(ScalarType::U32),
                    Literal::Int(_, BuiltinInt::Abstract) => {
                        TyKind::Scalar(ScalarType::AbstractInt)
                    },
                    Literal::Float(_, BuiltinFloat::F16) => TyKind::Scalar(ScalarType::F16),
                    Literal::Float(_, BuiltinFloat::F32) => TyKind::Scalar(ScalarType::F32),
                    Literal::Float(_, BuiltinFloat::Abstract) => {
                        TyKind::Scalar(ScalarType::AbstractFloat)
                    },
                    Literal::Bool(_) => TyKind::Scalar(ScalarType::Bool),
                };
                self.database.intern_ty(ty_kind)
            },
            Expression::IdentExpression(ident_expression) => {
                self.infer_ident_expression(expression, ident_expression, store)
            },
        };

        self.set_expression_ty(expression, r#type);

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
                self.expect_same_type(expression, expected, actual);
            }

            function.return_type.unwrap_or_else(|| self.error_ty())
        } else {
            self.push_diagnostic(InferenceDiagnostic::FunctionCallArgCountMismatch {
                expression: callee,
                n_expected: function.parameters.len(),
                n_actual: arguments.len(),
            });
            self.error_ty()
        }
    }

    fn infer_unary_op(
        &mut self,
        expression: ExpressionId,
        op: UnaryOperator,
        store: &ExpressionStore,
    ) -> Type {
        let expression_ty = self.infer_expression(expression, store);
        if expression_ty.is_err(self.database) {
            return self.error_ty();
        }

        let builtin = match op {
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
                if let TyKind::Reference(reference) = expression_ty.kind(self.database) {
                    return self.ref_to_pointer(&reference);
                }
                self.push_diagnostic(InferenceDiagnostic::AddressOfNotReference {
                    expression,
                    actual: expression_ty,
                });
                return self.error_ty();
            },
            UnaryOperator::Indirection => {
                let arg_ty = expression_ty.unref(self.database);
                if let TyKind::Pointer(pointer) = arg_ty.kind(self.database) {
                    return self.ptr_to_ref(&pointer);
                }
                self.push_diagnostic(InferenceDiagnostic::DerefNotAPointer {
                    expression,
                    actual: arg_ty,
                });
                return self.error_ty();
            },
        };

        let arg_ty = expression_ty.unref(self.database);
        self.call_builtin(expression, builtin, &[arg_ty], Some(op.symbol()))
    }

    fn infer_binary_op(
        &mut self,
        left_side: ExpressionId,
        right_side: ExpressionId,
        op: BinaryOperation,
        store: &ExpressionStore,
    ) -> Type {
        let left_ty = self.infer_expression(left_side, store).unref(self.database);
        let rhs_ty = self
            .infer_expression(right_side, store)
            .unref(self.database);

        if left_ty.is_err(self.database) || rhs_ty.is_err(self.database) {
            return self.error_ty();
        }

        let builtin = match op {
            BinaryOperation::Logical(_) => {
                Builtin::builtin_op_binary_bool(self.database).intern(self.database)
            },
            BinaryOperation::Arithmetic(op) => match op {
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
                ComparisonOperation::Equality { .. } => {
                    Builtin::builtin_op_eq(self.database).intern(self.database)
                },
                ComparisonOperation::Ordering { .. } => {
                    Builtin::builtin_op_cmp(self.database).intern(self.database)
                },
            },
        };

        self.call_builtin(left_side, builtin, &[left_ty, rhs_ty], Some(op.symbol()))
    }

    fn infer_ident_expression(
        &mut self,
        expression: ExpressionId,
        ident_expression: &IdentExpression,
        store: &ExpressionStore,
    ) -> Type {
        let resolver = self.resolver_for_expression(expression);
        let mut ctx = TyLoweringContext::new(
            self.database,
            resolver.as_ref().unwrap_or(&self.resolver),
            store,
        );
        let lowered = ctx.lower(
            TypeContainer::Expression(expression),
            &ident_expression.path,
            &ident_expression.generics,
        );
        self.push_lowering_diagnostics(&mut ctx.diagnostics, store);

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
                self.error_ty()
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

    fn ty_from_vec_size(
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
                    TyKind::Vector(VectorType {
                        size,
                        component_type: inner,
                    })
                })
                .unwrap_or(TyKind::Error);
            self.database.intern_ty(kind)
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
                let r#type = self.ty_from_vec_size(
                    vector_type.component_type,
                    u8::try_from(name.as_str().len()).unwrap(),
                );
                let result_type =
                    self.make_ref(r#type, AddressSpace::Function, AccessMode::ReadWrite); // TODO is correct?
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
        if let Ok((return_ty, overload_id)) = self.try_call_builtin(builtin_id, arguments) {
            let builtin = builtin_id.lookup(self.database);
            let resolved = builtin.overload(overload_id).r#type;
            self.result
                .call_resolutions
                .insert(expression, ResolvedCall::Function(resolved));
            return_ty
        } else {
            self.push_diagnostic(InferenceDiagnostic::NoBuiltinOverload {
                expression,
                builtin: builtin_id,
                name,
                parameters: arguments.to_vec(),
            });
            self.error_ty()
        }
    }

    fn try_call_builtin(
        &self,
        builtin_id: BuiltinId,
        arguments: &[Type],
    ) -> Result<(Type, BuiltinOverloadId), ()> {
        let builtin = builtin_id.lookup(self.database);
        for (overload_id, overload) in builtin.overloads() {
            // TODO: Pick overload with lowest rank
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
        let fn_ty = signature.r#type.lookup(self.database);

        if fn_ty.parameters.len() != arguments.len() {
            return Err(());
        }

        // TODO: Do the conversion rank computation
        let conversion_rank = 0;
        let mut unification_table = UnificationTable::default();
        for (expected, &found) in fn_ty.parameters().zip(arguments.iter()) {
            unify(self.database, &mut unification_table, expected, found)?;
        }

        let return_type = fn_ty
            .return_type
            .map(|r#type| unification_table.resolve(self.database, r#type));

        Ok((
            return_type.unwrap_or_else(|| self.error_ty()),
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
        let mut ctx = TyLoweringContext::new(self.database, &resolver, store);
        let lowered = ctx.lower(
            TypeContainer::Expression(expression),
            &callee.path,
            &callee.generics,
        );
        self.push_lowering_diagnostics(&mut ctx.diagnostics, store);

        match lowered {
            Lowered::Type(r#type) => self.call_type_constructor(expression, r#type, arguments),
            Lowered::TypeWithoutTemplate(r#type) => {
                self.call_type_constructor(expression, r#type, arguments)
            },
            Lowered::Function(id) => {
                let details = id.lookup(self.database);
                self.result
                    .call_resolutions
                    .insert(expression, ResolvedCall::Function(id));
                self.validate_function_call(&details, &arguments, expression, expression)
            },
            Lowered::BuiltinFunction => {
                let template_args =
                    ctx.eval_template_args(TypeContainer::Expression(expression), &callee.generics);
                self.push_lowering_diagnostics(&mut ctx.diagnostics, store);
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
                self.error_ty()
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
            match converter.template_parameter_to_wgsl_types(template_parameter) {
                Some(p) => template_args.push(p),
                None => {
                    // TODO: Proper error reporting
                    return self.error_ty();
                },
            }
        }
        let template_args = if template_args.is_empty() {
            None
        } else {
            Some(template_args.as_slice())
        };

        let converted_arguments: Option<Vec<_>> = arguments
            .iter()
            .map(|ty| converter.to_wgsl_types(*ty))
            .collect();

        let Some(converted_arguments) = converted_arguments else {
            // One of the arguments had an error type
            return self.error_ty();
        };

        let return_type = wgsl_types::builtin::type_builtin_fn(
            callee.path.as_str(),
            template_args,
            &converted_arguments,
        );

        match return_type {
            Ok(Some(ty)) => converter.from_wgsl_types(ty),
            Ok(None) => self.error_ty(),
            Err(error) => {
                self.push_diagnostic(InferenceDiagnostic::WgslError {
                    expression,
                    message: error.to_string(),
                });
                self.error_ty()
            },
        }
    }

    fn call_type_constructor(
        &mut self,
        expression: ExpressionId,
        r#type: Type,
        arguments: Vec<Type>,
    ) -> Type {
        fn size_to_dimension(size: VecSize) -> VecDimensionality {
            #[expect(clippy::unreachable, reason = "TODO")]
            match size {
                VecSize::Two => VecDimensionality::Two,
                VecSize::Three => VecDimensionality::Three,
                VecSize::Four => VecDimensionality::Four,
                VecSize::BoundVar(_) => unreachable!("Can never have unbound type at this point"),
            }
        }

        match r#type.kind(self.database) {
            TyKind::Scalar(scalar_type) => {
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

                let construction_result =
                    self.try_call_builtin(construction_builtin_id, &arguments);
                match construction_result {
                    Ok((r#type, _)) => r#type,
                    Err(()) => {
                        self.push_diagnostic(InferenceDiagnostic::NoConstructor {
                            expression,
                            builtins: construction_builtin_id,
                            r#type,
                            parameters: arguments,
                        });
                        self.error_ty()
                    },
                }
            },
            TyKind::Array(array_type) => {
                if arguments.is_empty() {
                    return r#type;
                }
                // TODO: checking that all the arguments have the same type (inner)
                r#type
            },
            TyKind::Vector(vec) => {
                if arguments.is_empty() {
                    return r#type;
                }
                let construction_builtin_id =
                    self.builtin_vector_inferred_constructor(size_to_dimension(vec.size));
                let construction_result =
                    self.try_call_builtin(construction_builtin_id, &arguments);

                match construction_result {
                    Ok((r#type, _)) => r#type,
                    Err(()) => {
                        self.push_diagnostic(InferenceDiagnostic::NoConstructor {
                            expression,
                            builtins: construction_builtin_id,
                            r#type,
                            parameters: arguments,
                        });
                        self.error_ty()
                    },
                }
            },
            TyKind::Matrix(matrix) => {
                if arguments.is_empty() {
                    return r#type;
                }
                let construction_builtin_id = self.builtin_matrix_inferred_constructor(
                    size_to_dimension(matrix.columns),
                    size_to_dimension(matrix.rows),
                );
                let construction_result =
                    self.try_call_builtin(construction_builtin_id, &arguments);
                match construction_result {
                    Ok((r#type, _)) => r#type,
                    Err(()) => {
                        self.push_diagnostic(InferenceDiagnostic::NoConstructor {
                            expression,
                            builtins: construction_builtin_id,
                            r#type,
                            parameters: arguments,
                        });
                        self.error_ty()
                    },
                }
            },
            TyKind::Struct(_) => {
                if arguments.is_empty() {}
                // TODO: Implement checking field types
                r#type
            },

            // Never constructible
            TyKind::Texture(_)
            | TyKind::Sampler(_)
            | TyKind::Pointer(_)
            | TyKind::Atomic(_)
            | TyKind::StorageTypeOfTexelFormat(_) => {
                self.push_diagnostic(InferenceDiagnostic::InvalidConstructionType {
                    expression,
                    r#type,
                });
                self.error_ty()
            },
            #[expect(clippy::unreachable, reason = "TODO")]
            TyKind::BoundVar(_) | TyKind::Reference(_) => unreachable!(),
            TyKind::Error => r#type,
        }
    }

    fn lower_ty(
        &mut self,
        type_ref: TypeSpecifierId,
        resolver: &Resolver,
        store: &ExpressionStore,
    ) -> Type {
        let mut ctx = TyLoweringContext::new(self.database, &resolver, store);
        let r#type = ctx.lower_ty(type_ref);
        self.push_lowering_diagnostics(&mut ctx.diagnostics, store);
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

    const fn from_ty(r#type: Type) -> Self {
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
        self.database.intern_ty(TyKind::Reference(Reference {
            address_space,
            inner: r#type,
            access_mode,
        }))
    }

    fn ref_to_pointer(
        &self,
        reference: &Reference,
    ) -> Type {
        self.database.intern_ty(TyKind::Pointer(Pointer {
            address_space: reference.address_space,
            inner: reference.inner,
            access_mode: reference.access_mode,
        }))
    }

    fn ptr_to_ref(
        &self,
        pointer: &Pointer,
    ) -> Type {
        self.database.intern_ty(TyKind::Reference(Reference {
            address_space: pointer.address_space,
            inner: pointer.inner,
            access_mode: pointer.access_mode,
        }))
    }

    fn error_ty(&self) -> Type {
        self.result.standard_types.unknown
    }

    fn bool_ty(&self) -> Type {
        self.database.intern_ty(TyKind::Scalar(ScalarType::Bool))
    }
}

/// Lowers types and expressions, the two are deeply intertwined.
pub struct TyLoweringContext<'database> {
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
    InvalidTexelFormat(String),
    UnexpectedTemplateArgument(String),
    MissingTemplateArgument(String),
    MissingTemplate,
    WrongNumberOfTemplateArguments {
        expected: std::ops::RangeInclusive<usize>,
        actual: usize,
    },
    ExpectedType(Name),
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
            TypeLoweringErrorKind::UnresolvedName(name) => {
                write!(formatter, "type `{}` not found in scope", name.as_str())
            },
            TypeLoweringErrorKind::InvalidTexelFormat(format) => {
                let all_formats = "rgba8unorm,\nrgba8snorm,\nrgba8uint,\nrgba8sint,\nrgba16uint,\nrgba16sint,\nrgba16float,\nr32uint,\nr32sint,\nr32float,\nrg32uint,\nrg32sint,\nrg32float,\nrgba32uint,\nrgba32sint,\nrgba32float";
                write!(
                    formatter,
                    "`{format}` is not a valid texel format, expected one of:\n{all_formats}"
                )
            },
            TypeLoweringErrorKind::WgslError(error) => {
                write!(formatter, "{error}")
            },
            TypeLoweringErrorKind::UnexpectedTemplateArgument(expected) => {
                write!(
                    formatter,
                    "unexpected template argument, expected {expected}"
                )
            },
            TypeLoweringErrorKind::MissingTemplateArgument(expected) => {
                write!(formatter, "missing template argument, expected {expected}")
            },
            TypeLoweringErrorKind::MissingTemplate => {
                write!(formatter, "missing template arguments")
            },
            TypeLoweringErrorKind::WrongNumberOfTemplateArguments { expected, actual }
                if expected.start() == expected.end() =>
            {
                write!(
                    formatter,
                    "expected {} template arguments, but got {actual}",
                    expected.start()
                )
            },
            TypeLoweringErrorKind::WrongNumberOfTemplateArguments { expected, actual } => {
                write!(
                    formatter,
                    "expected {} to {} template arguments, but got {actual}",
                    expected.start(),
                    expected.end()
                )
            },
            TypeLoweringErrorKind::ExpectedType(name) => {
                write!(formatter, "{} is not a type", name.as_str())
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
    pub fn kind(&self) -> LoweredKind {
        match self {
            Lowered::Type(_) => LoweredKind::Type,
            Lowered::TypeWithoutTemplate(_) => LoweredKind::Type,
            Lowered::Function(_) => LoweredKind::Function,
            Lowered::GlobalConstant(_) => LoweredKind::Constant,
            Lowered::GlobalVariable(_) => LoweredKind::Variable,
            Lowered::Override(_) => LoweredKind::Override,
            Lowered::Local(_) => LoweredKind::Local,
            Lowered::Enumerant(_) => LoweredKind::Enumerant,
            Lowered::BuiltinFunction => LoweredKind::Function,
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
            LoweredKind::Type => write!(f, "type"),
            LoweredKind::Function => write!(f, "function"),
            LoweredKind::Constant => write!(f, "constant"),
            LoweredKind::Variable => write!(f, "variable"),
            LoweredKind::Override => write!(f, "override"),
            LoweredKind::Local => write!(f, "local variable"),
            LoweredKind::Enumerant => write!(f, "enumerant"),
        }
    }
}

impl<'database> TyLoweringContext<'database> {
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
        generics: &[ExpressionId],
    ) -> Lowered {
        match self.try_lower(type_container, path, generics) {
            Ok(lowered) => lowered,
            Err(error) => {
                self.diagnostics.push(error);
                Lowered::Type(self.database.intern_ty(TyKind::Error))
            },
        }
    }

    /// Will lower types, and resolve the definition of other items.
    pub fn try_lower(
        &mut self,
        type_container: TypeContainer,
        path: &Name,
        generics: &[ExpressionId],
    ) -> Result<Lowered, TypeLoweringError> {
        let resolved_ty = self.resolver.resolve(path);

        match resolved_ty {
            // User-defined types currently cannot have generics
            Some(_) => {
                self.expect_no_template(generics);
            },
            _ => {},
        }

        match resolved_ty {
            Some(ResolveKind::TypeAlias(loc)) => {
                let id = self.database.intern_type_alias(loc);
                Ok(Lowered::Type(self.database.type_alias_type(id).0))
            },
            Some(ResolveKind::Struct(loc)) => {
                let id = self.database.intern_struct(loc);
                Ok(Lowered::Type(self.database.intern_ty(TyKind::Struct(id))))
            },
            Some(ResolveKind::Function(loc)) => {
                let id = self.database.intern_function(loc);
                Ok(Lowered::Function(self.database.function_type(id)))
            },
            Some(ResolveKind::GlobalConstant(loc)) => {
                let id = self.database.intern_global_constant(loc);
                Ok(Lowered::GlobalConstant(id))
            },
            Some(ResolveKind::GlobalVariable(loc)) => {
                let id = self.database.intern_global_variable(loc);
                Ok(Lowered::GlobalVariable(id))
            },
            Some(ResolveKind::Override(loc)) => {
                let id = self.database.intern_override(loc);
                Ok(Lowered::Override(id))
            },
            Some(ResolveKind::Local(local)) => Ok(Lowered::Local(local)),
            None => self.lower_predeclared(type_container, path, generics),
        }
    }

    fn expect_no_template(
        &mut self,
        generics: &[ExpressionId],
    ) {
        if generics.is_empty() {
            return;
        }
        for template_expression in generics {
            self.diagnostics.push(TypeLoweringError {
                container: TypeContainer::Expression(*template_expression),
                kind: TypeLoweringErrorKind::UnexpectedTemplateArgument("nothing".to_string()),
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
                container: template_parameters.container.clone(),
                kind: TypeLoweringErrorKind::WrongNumberOfTemplateArguments {
                    expected,
                    actual: template_parameters.len(),
                },
            });

            false
        }
    }

    pub fn lower_ty(
        &mut self,
        type_specifier_id: TypeSpecifierId,
    ) -> Type {
        let type_specifier = &self.store[type_specifier_id];
        let lowered = self.try_lower(
            TypeContainer::TypeSpecifier(type_specifier_id),
            &type_specifier.path,
            &type_specifier.generics,
        );
        match lowered {
            Ok(Lowered::Type(r#type)) => r#type,
            Ok(Lowered::TypeWithoutTemplate(_)) => {
                self.diagnostics.push(TypeLoweringError {
                    container: TypeContainer::TypeSpecifier(type_specifier_id),
                    kind: TypeLoweringErrorKind::MissingTemplate,
                });
                self.database.intern_ty(TyKind::Error)
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
                self.database.intern_ty(TyKind::Error)
            },
            Err(error) => {
                self.diagnostics.push(error);
                self.database.intern_ty(TyKind::Error)
            },
        }
    }
}

#[derive(PartialEq, Eq)]
enum AbstractHandling {
    Concretize,
    Abstract,
}
struct WgslTypeConverter<'a> {
    database: &'a dyn HirDatabase,
    interned_structs: Vec<StructId>,
}

impl<'a> WgslTypeConverter<'a> {
    fn new(database: &'a dyn HirDatabase) -> Self {
        Self {
            database,
            interned_structs: Default::default(),
        }
    }
    fn to_wgsl_types(
        &mut self,
        ty: Type,
    ) -> Option<wgsl_types::Type> {
        Some(match ty.kind(self.database) {
            TyKind::Error => return None,
            TyKind::Scalar(ScalarType::AbstractFloat) => wgsl_types::Type::AbstractFloat,
            TyKind::Scalar(ScalarType::AbstractInt) => wgsl_types::Type::AbstractInt,
            TyKind::Scalar(ScalarType::Bool) => wgsl_types::Type::Bool,
            TyKind::Scalar(ScalarType::F16) => wgsl_types::Type::F16,
            TyKind::Scalar(ScalarType::F32) => wgsl_types::Type::F32,
            TyKind::Scalar(ScalarType::I32) => wgsl_types::Type::I32,
            TyKind::Scalar(ScalarType::U32) => wgsl_types::Type::U32,
            TyKind::Atomic(AtomicType { inner }) => {
                wgsl_types::Type::Atomic(Box::new(self.to_wgsl_types(inner)?))
            },
            TyKind::Vector(VectorType {
                size,
                component_type,
            }) => {
                wgsl_types::Type::Vec(size.as_u8(), Box::new(self.to_wgsl_types(component_type)?))
            },
            TyKind::Matrix(MatrixType {
                columns,
                rows,
                inner,
            }) => wgsl_types::Type::Mat(
                columns.as_u8(),
                rows.as_u8(),
                Box::new(self.to_wgsl_types(inner)?),
            ),
            TyKind::Struct(struct_id) => {
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
                                name: data.name.as_str().to_string(),
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
            TyKind::Array(ArrayType {
                inner,
                binding_array: false,
                size,
            }) => wgsl_types::Type::Array(
                Box::new(self.to_wgsl_types(inner)?),
                match size {
                    ArraySize::Constant(size) => Some(size as usize),
                    ArraySize::Dynamic => None,
                },
            ),
            TyKind::Array(ArrayType {
                inner,
                binding_array: true,
                size,
            }) => wgsl_types::Type::BindingArray(
                Box::new(self.to_wgsl_types(inner)?),
                match size {
                    ArraySize::Constant(size) => Some(size as usize),
                    ArraySize::Dynamic => None,
                },
            ),
            TyKind::Texture(texture_type) => {
                wgsl_types::Type::Texture(self.to_wgsl_texture_type(texture_type))
            },
            TyKind::Sampler(sampler_type) => wgsl_types::Type::Sampler(sampler_type.into()),
            TyKind::Reference(Reference {
                address_space,
                inner,
                access_mode,
            }) => wgsl_types::Type::Ref(
                address_space,
                Box::new(self.to_wgsl_types(inner)?),
                access_mode,
            ),
            TyKind::Pointer(Pointer {
                address_space,
                inner,
                access_mode,
            }) => wgsl_types::Type::Ptr(
                address_space,
                Box::new(self.to_wgsl_types(inner)?),
                access_mode,
            ),
            TyKind::BoundVar(_) => return None,
            TyKind::StorageTypeOfTexelFormat(_) => return None,
        })
    }

    /// Returns none if it is an error type
    fn template_parameter_to_wgsl_types(
        &mut self,
        param: eval::TpltParam,
    ) -> Option<wgsl_types::tplt::TpltParam> {
        Some(match param {
            eval::TpltParam::Type(ty) => wgsl_types::tplt::TpltParam::Type(self.to_wgsl_types(ty)?),
            eval::TpltParam::Instance(instance) => wgsl_types::tplt::TpltParam::Instance(instance?),
            eval::TpltParam::Enumerant(enumerant) => {
                wgsl_types::tplt::TpltParam::Enumerant(enumerant)
            },
        })
    }

    fn from_wgsl_types(
        &self,
        ty: wgsl_types::Type,
    ) -> Type {
        match ty {
            wgsl_types::Type::Bool => TyKind::Scalar(ScalarType::Bool).intern(self.database),
            wgsl_types::Type::AbstractInt => {
                TyKind::Scalar(ScalarType::AbstractInt).intern(self.database)
            },
            wgsl_types::Type::AbstractFloat => {
                TyKind::Scalar(ScalarType::AbstractFloat).intern(self.database)
            },
            wgsl_types::Type::I32 => TyKind::Scalar(ScalarType::I32).intern(self.database),
            wgsl_types::Type::U32 => TyKind::Scalar(ScalarType::U32).intern(self.database),
            wgsl_types::Type::I64 => todo!("naga extension"),
            wgsl_types::Type::U64 => todo!("naga extension"),
            wgsl_types::Type::F16 => TyKind::Scalar(ScalarType::F16).intern(self.database),
            wgsl_types::Type::F32 => TyKind::Scalar(ScalarType::F32).intern(self.database),
            wgsl_types::Type::F64 => todo!("naga extension"),
            wgsl_types::Type::Struct(struct_type) => {
                let struct_id = self
                    .get_interned_struct(&struct_type.name)
                    // I think this doesn't hold true when calling `atomicCompareExchangeWeak`
                    .expect("Only struct types that have been passed in should be returned");
                TyKind::Struct(struct_id).intern(self.database)
            },
            wgsl_types::Type::Array(ty, size) => TyKind::Array(ArrayType {
                inner: self.from_wgsl_types(*ty),
                binding_array: false,
                size: match size {
                    Some(size) => ArraySize::Constant(size as u64),
                    None => ArraySize::Dynamic,
                },
            })
            .intern(self.database),
            wgsl_types::Type::BindingArray(ty, size) => TyKind::Array(ArrayType {
                inner: self.from_wgsl_types(*ty),
                binding_array: true,
                size: match size {
                    Some(size) => ArraySize::Constant(size as u64),
                    None => ArraySize::Dynamic,
                },
            })
            .intern(self.database),
            wgsl_types::Type::Vec(size, ty) => TyKind::Vector(VectorType {
                size: VecSize::try_from(size).unwrap(),
                component_type: self.from_wgsl_types(*ty),
            })
            .intern(self.database),
            wgsl_types::Type::Mat(columns, rows, ty) => TyKind::Matrix(MatrixType {
                columns: VecSize::try_from(columns).unwrap(),
                rows: VecSize::try_from(rows).unwrap(),
                inner: self.from_wgsl_types(*ty),
            })
            .intern(self.database),
            wgsl_types::Type::Atomic(ty) => TyKind::Atomic(AtomicType {
                inner: self.from_wgsl_types(*ty),
            })
            .intern(self.database),
            wgsl_types::Type::Ptr(address_space, ty, access_mode) => TyKind::Pointer(Pointer {
                address_space,
                inner: self.from_wgsl_types(*ty),
                access_mode,
            })
            .intern(self.database),
            wgsl_types::Type::Ref(address_space, ty, access_mode) => TyKind::Reference(Reference {
                address_space,
                inner: self.from_wgsl_types(*ty),
                access_mode,
            })
            .intern(self.database),
            wgsl_types::Type::Texture(texture_type) => {
                TyKind::Texture(self.from_wgsl_texture_type(texture_type)).intern(self.database)
            },
            wgsl_types::Type::Sampler(sampler_type) => {
                TyKind::Sampler(sampler_type).intern(self.database)
            },
            wgsl_types::Type::RayQuery(_) => todo!("naga extension"),
            wgsl_types::Type::AccelerationStructure(_) => todo!("naga extension"),
        }
    }

    fn from_wgsl_texture_type(
        &self,
        value: wgsl_types::ty::TextureType,
    ) -> TextureType {
        match value {
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
        format!("struct{}", index)
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
            TyKind::Scalar(ScalarType::I32) => wgsl_types::syntax::SampledType::I32,
            TyKind::Scalar(ScalarType::U32) => wgsl_types::syntax::SampledType::U32,
            TyKind::Scalar(ScalarType::F32) => wgsl_types::syntax::SampledType::F32,
            kind => panic!("invalid sampled type {kind:?}"),
        }
    }
}

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
        _ => panic!("not yet supported naga extension"),
    }
}

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
        crate::ty::TexelFormat::BoundVar(_) => {
            panic!("bound var is not a valid texel format to convert")
        },
        crate::ty::TexelFormat::Any => panic!("any is not a valid texel format to convert"),
    }
}
