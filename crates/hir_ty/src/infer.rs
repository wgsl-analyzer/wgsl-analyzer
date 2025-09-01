mod unify;

use std::{collections::hash_map::Entry, fmt, sync::Arc};

use either::Either;
use hir_def::{
    body::{BindingId, Body},
    data::{
        FieldId, FunctionData, GlobalConstantData, GlobalVariableData, OverrideData, ParamId,
        StructData, TypeAliasData,
    },
    database::{
        DefinitionId, DefinitionWithBodyId, FunctionId, GlobalConstantId, GlobalVariableId,
        OverrideId, StructId, TypeAliasId,
    },
    expression::{
        ArithmeticOperation, BinaryOperation, ComparisonOperation, Expression, ExpressionId,
        Statement, StatementId, SwitchCaseSelector, UnaryOperator,
    },
    expression_store::ExpressionStore,
    module_data::Name,
    resolver::{ResolveType, Resolver},
    type_ref::{self, AccessMode, AddressSpace, TypeReference, VecDimensionality},
    type_specifier::TypeSpecifier,
};
use la_arena::ArenaMap;
use rustc_hash::FxHashMap;

use crate::{
    builtins::{Builtin, BuiltinId, BuiltinOverload, BuiltinOverloadId},
    database::HirDatabase,
    function::{FunctionDetails, ResolvedFunctionId},
    infer::unify::{UnificationTable, unify},
    ty::{
        ArraySize, ArrayType, AtomicType, BoundVar, MatrixType, Pointer, Reference, SamplerType,
        ScalarType, TexelFormat, TextureDimensionality, TextureKind, TextureType, TyKind, Type,
        VecSize, VectorType,
    },
};

/// Infers the type of a global item.
/// For `const`s and co, it first uses the specified type,
/// and then uses the body (expression) to infer the return type.
pub fn infer_query(
    database: &dyn HirDatabase,
    definition: DefinitionId,
) -> Arc<InferenceResult> {
    let resolver = definition.resolver(database);
    let mut context = InferenceContext::new(database, definition, resolver);

    match definition {
        DefinitionId::Function(function) => {
            context.collect_fn(function, &database.fn_data(function).0);
        },
        DefinitionId::GlobalVariable(var) => {
            context.collect_global_variable(var, &database.global_var_data(var).0);
        },
        DefinitionId::GlobalConstant(constant) => {
            context.collect_global_constant(constant, &database.global_constant_data(constant).0);
        },
        DefinitionId::Override(override_decl) => {
            context.collect_override(override_decl, &database.override_data(override_decl).0);
        },
        DefinitionId::Override(override_decl) => {
            context.collect_override(override_decl, &database.override_data(override_decl).0);
        },
        DefinitionId::Struct(struct_decl) => {
            context.collect_struct(struct_decl, &database.struct_data(struct_decl).0);
        },
        DefinitionId::TypeAlias(type_alias) => {
            context.collect_type_alias(type_alias, &database.type_alias_data(type_alias).0);
        },
    }

    context.infer_body();

    context.infer_variables();

    Arc::new(context.resolve_all())
}

pub fn infer_cycle_result(
    database: &dyn HirDatabase,
    _cycle: &[String],
    definition: &DefinitionId,
) -> Arc<InferenceResult> {
    let mut inference_result = InferenceResult::new(database);
    let name = match *definition {
        DefinitionId::Function(id) => database.fn_data(id).0.name.clone(),
        DefinitionId::GlobalVariable(id) => database.global_var_data(id).0.name.clone(),
        DefinitionId::GlobalConstant(id) => database.global_constant_data(id).0.name.clone(),
        DefinitionId::Override(id) => database.override_data(id).0.name.clone(),
        DefinitionId::Struct(id) => database.struct_data(id).0.name.clone(),
        DefinitionId::TypeAlias(id) => database.type_alias_data(id).0.name.clone(),
    };

    inference_result
        .diagnostics
        .push(InferenceDiagnostic::CyclicType { name });

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
        builtins: [BuiltinId; 2],
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
        container: TypeContainer,
        error: TypeLoweringError,
    },
    CyclicType {
        name: Name,
    },
    UnexpectedTemplateArgument {
        expression: ExpressionId,
    },
}

#[derive(PartialEq, Eq, Debug)]
pub enum TypeContainer {
    Expr(ExpressionId),
    GlobalVar(GlobalVariableId),
    GlobalConstant(GlobalConstantId),
    Override(OverrideId),
    TypeAlias(TypeAliasId),
    FunctionParameter(ParamId),
    StructField(FieldId),
    FunctionReturn(FunctionId),
    VariableStatement(StatementId),
}

impl From<ExpressionId> for TypeContainer {
    fn from(id: ExpressionId) -> Self {
        Self::Expr(id)
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum ResolvedCall {
    Function(ResolvedFunctionId),
    OtherTypeInitializer(Type),
}

#[expect(clippy::partial_pub_fields, reason = "TODO")]
#[derive(PartialEq, Eq, Debug)]
pub struct InferenceResult {
    pub type_of_expression: ArenaMap<ExpressionId, Type>,
    pub type_of_binding: ArenaMap<BindingId, Type>,
    pub diagnostics: Vec<InferenceDiagnostic>,
    pub return_type: Type,
    call_resolutions: FxHashMap<ExpressionId, ResolvedCall>,
    field_resolutions: FxHashMap<ExpressionId, FieldId>,
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
}

pub struct InferenceContext<'database> {
    database: &'database dyn HirDatabase,
    owner: DefinitionId,
    /// Root resolver for the entire module
    resolver: Resolver,
    body: Option<Arc<Body>>,
    result: InferenceResult, // set in collect_* calls
    return_ty: Type,
}

impl<'database> InferenceContext<'database> {
    pub fn new(
        database: &'database dyn HirDatabase,
        owner: DefinitionId,
        resolver: Resolver,
    ) -> Self {
        Self {
            database,
            owner,
            resolver,
            body: owner.with_body().map(|owner| database.body(owner)),
            result: InferenceResult::new(database),
            return_ty: TyKind::Error.intern(database),
        }
    }

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
            && let Some(binding) = self.body.as_ref().and_then(|body| body.main_binding)
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

    fn resolve_all(mut self) -> InferenceResult {
        self.result.return_type = self.return_ty;
        self.result
    }

    fn collect_global_variable(
        &mut self,
        id: GlobalVariableId,
        var: &GlobalVariableData,
    ) {
        let r#type = var
            .r#type
            .as_ref()
            .map(|r#type| self.lower_ty(TypeContainer::GlobalVar(id), r#type));

        self.bind_return_ty(r#type);
    }

    fn infer_variables(&mut self) {
        if let DefinitionId::GlobalVariable(var) = self.owner {
            let var = self.database.global_var_data(var).0;
            let (address_space, access_mode) =
                self.infer_variable_template(&var.generics, &var.store);

            self.bind_return_ty(Some(self.make_ref(
                self.return_ty,
                address_space,
                access_mode,
            )));
        }
    }

    fn infer_variable_template(
        &mut self,
        template: &[ExpressionId],
        store: &ExpressionStore,
    ) -> (AddressSpace, AccessMode) {
        let mut var_template = template
            .iter()
            .map(|expression| self.infer_expression(*expression, store));
        let address_space = var_template
            .next() // TODO: infer_expression should be able to return those predeclared types
            .map(|r#type| AddressSpace::Function)
            .unwrap_or(AddressSpace::Function);
        let access_mode = var_template
            .next()
            .map(|r#type| AccessMode::ReadWrite)
            .unwrap_or(AccessMode::ReadWrite);

        // Mark extra template arguments as errors
        for expression in template.iter().skip(2) {
            self.push_diagnostic(InferenceDiagnostic::UnexpectedTemplateArgument {
                expression: *expression,
            });
        }
        (address_space, access_mode)
    }

    fn collect_global_constant(
        &mut self,
        id: GlobalConstantId,
        constant: &GlobalConstantData,
    ) {
        let r#type = constant
            .r#type
            .as_ref()
            .map(|r#type| self.lower_ty(TypeContainer::GlobalConstant(id), r#type));

        self.bind_return_ty(r#type);
    }

    fn collect_override(
        &mut self,
        id: OverrideId,
        override_data: &OverrideData,
    ) {
        let r#type = override_data
            .r#type
            .as_ref()
            .map(|r#type| self.lower_ty(TypeContainer::Override(id), r#type));

        self.bind_return_ty(r#type);
    }

    fn collect_struct(
        &mut self,
        id: StructId,
        _struct: &StructData,
    ) {
        let r#type = self.database.intern_ty(TyKind::Struct(id));

        // TODO: Maybe infer the field types so that other methods can do autocompletions
        // in expressions in struct types?
        // struct Foo { a: array<f32, vec3u(1).x> } is valid after all

        self.bind_return_ty(Some(r#type));
    }

    fn collect_type_alias(
        &mut self,
        id: TypeAliasId,
        type_alias: &TypeAliasData,
    ) {
        let r#type = self.lower_ty(TypeContainer::TypeAlias(id), &r#type_alias.r#type);

        self.bind_return_ty(Some(r#type));
    }

    fn collect_fn(
        &mut self,
        function_id: FunctionId,
        function_data: &FunctionData,
    ) {
        let body = &self
            .body
            .clone()
            .expect("function collection always has a body");
        for ((id, parameter), &binding_id) in function_data.parameters.iter().zip(&body.parameters)
        {
            let param_ty = self.lower_ty(
                TypeContainer::FunctionParameter(ParamId {
                    function: function_id,
                    param: id,
                }),
                &parameter.r#type,
            );
            self.set_binding_ty(binding_id, param_ty);
        }
        self.return_ty = function_data
            .return_type
            .as_ref()
            .map(|type_ref| self.lower_ty(TypeContainer::FunctionReturn(function_id), type_ref))
            .unwrap_or_else(|| self.error_ty());
    }

    /// Runs type inference on the body and infer the type for `const`s, `var`s and `override`s
    fn infer_body(&mut self) {
        if let Some(body) = &self.body {
            match body.root {
                Some(Either::Left(statement)) => {
                    self.infer_statement(statement);
                },
                Some(Either::Right(expression)) => {
                    let body = body.clone();
                    let r#type = self.infer_expression_expect(
                        expression,
                        &TypeExpectation::from_ty(self.return_ty),
                        &body.store,
                    );
                    if self.return_ty.is_err(self.database) {
                        self.bind_return_ty(Some(r#type));
                    }
                },
                None => (),
            }
        }
    }

    fn resolver_for_expression(
        &self,
        expression: ExpressionId,
    ) -> Option<Resolver> {
        match self.owner {
            DefinitionId::Function(function) => {
                let expression_scopes = self
                    .database
                    .expression_scopes(DefinitionWithBodyId::Function(function));
                let scope_id = expression_scopes.scope_for_expression(expression).unwrap();
                Some(self.resolver.clone().push_expression_scope(
                    function,
                    expression_scopes,
                    scope_id,
                ))
            },
            DefinitionId::GlobalVariable(_)
            | DefinitionId::GlobalConstant(_)
            | DefinitionId::Override(_)
            | DefinitionId::Struct(_)
            | DefinitionId::TypeAlias(_) => None,
        }
    }

    fn resolver_for_statement(
        &self,
        statement: StatementId,
    ) -> Option<Resolver> {
        match self.owner {
            DefinitionId::Function(function) => {
                let expression_scopes = self
                    .database
                    .expression_scopes(DefinitionWithBodyId::Function(function));
                let scope_id = expression_scopes.scope_for_statement(statement).unwrap();
                Some(self.resolver.clone().push_expression_scope(
                    function,
                    expression_scopes,
                    scope_id,
                ))
            },
            DefinitionId::GlobalVariable(_)
            | DefinitionId::GlobalConstant(_)
            | DefinitionId::Override(_)
            | DefinitionId::Struct(_)
            | DefinitionId::TypeAlias(_) => None,
        }
    }

    #[expect(clippy::too_many_lines, reason = "TODO")]
    fn infer_statement(
        &mut self,
        statement: StatementId,
    ) {
        let body = &self
            .body
            .clone()
            .expect("statement inference always has a body");

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
                let r#type = type_ref.as_ref().map(|r#type| {
                    self.lower_ty(TypeContainer::VariableStatement(statement), &r#type)
                });
                let r#type = if let Some(init) = initializer {
                    let expression_ty = self.infer_expression_expect(
                        *init,
                        &TypeExpectation::from_option(r#type),
                        &body.store,
                    );
                    r#type.unwrap_or(expression_ty)
                } else {
                    r#type.unwrap_or_else(|| self.error_ty())
                };

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
                let r#type = type_ref.as_ref().map(|r#type| {
                    self.lower_ty(TypeContainer::VariableStatement(statement), &r#type)
                });
                let r#type = if let Some(init) = initializer {
                    let expression_ty = self.infer_expression_expect(
                        *init,
                        &TypeExpectation::from_option(r#type),
                        &body.store,
                    );
                    r#type.unwrap_or(expression_ty)
                } else {
                    r#type.unwrap_or_else(|| self.error_ty())
                };

                self.set_binding_ty(*binding_id, r#type);
            },
            Statement::Let {
                binding_id,
                type_ref,
                initializer,
                ..
            } => {
                let r#type = type_ref.as_ref().map(|r#type| {
                    self.lower_ty(TypeContainer::VariableStatement(statement), &r#type)
                });
                let r#type = if let Some(init) = initializer {
                    let expression_ty = self.infer_expression_expect(
                        *init,
                        &TypeExpectation::from_option(r#type),
                        &body.store,
                    );
                    r#type.unwrap_or(expression_ty)
                } else {
                    r#type.unwrap_or_else(|| self.error_ty())
                };

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
                default_block,
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

                if let Some(default_block) = *default_block {
                    self.infer_statement(default_block);
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
            Statement::Discard | Statement::Break | Statement::Continue | Statement::Missing => {},
            Statement::Continuing { block } => self.infer_statement(*block),
            Statement::Expression { expression } => {
                self.infer_expression(*expression, &body.store);
            },
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
                if expected_type.kind(self.database) == TyKind::Error || r#type == expected_type {
                    Ok(())
                } else {
                    Err(())
                }
            },
            TypeExpectationInner::I32OrF32 => {
                if let TyKind::Scalar(ScalarType::I32 | ScalarType::F32) =
                    r#type.kind(self.database).unref(self.database).as_ref()
                {
                    Ok(())
                } else {
                    Err(())
                }
            },
            TypeExpectationInner::NumericScalar => {
                if let TyKind::Scalar(ScalarType::I32 | ScalarType::F32 | ScalarType::U32) =
                    r#type.kind(self.database).unref(self.database).as_ref()
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

        match &expected {
            TypeExpectation::Type(expected_type) => {
                if self.expect_ty_inner(r#type, expected_type) != Ok(()) {
                    self.push_diagnostic(InferenceDiagnostic::TypeMismatch {
                        expression,
                        actual: r#type,
                        expected: expected.clone(),
                    });
                }
            },
            TypeExpectation::TypeOrVecOf(expect) => {
                if self.expect_ty_inner(r#type.this_or_vec_inner(self.database), expect) != Ok(()) {
                    self.push_diagnostic(InferenceDiagnostic::TypeMismatch {
                        expression,
                        actual: r#type,
                        expected: expected.clone(),
                    });
                }
            },
            TypeExpectation::None => {},
        }
        r#type
    }

    #[expect(clippy::too_many_lines, reason = "TODO")]
    fn infer_expression(
        &mut self,
        expression: ExpressionId,
        store: &ExpressionStore,
    ) -> Type {
        let r#type = match &store.exprs[expression] {
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
                        let field_types = self.database.field_types(*r#struct);

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
                            self.make_ref(field_ty, AddressSpace::Private, AccessMode::read_write())
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
                type_specifier,
                arguments,
            } => {
                let arguments: Vec<_> = arguments
                    .iter()
                    .map(|&arg| self.infer_expression(arg, store).unref(self.database))
                    .collect();
                self.infer_call(expression, type_specifier, arguments)
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
                    self.make_ref(r#type, AddressSpace::Private, AccessMode::read_write())
                } else {
                    r#type
                }
            },
            Expression::Literal(literal) => {
                let ty_kind = match literal {
                    hir_def::expression::Literal::Int(_, _) => TyKind::Scalar(ScalarType::I32),
                    hir_def::expression::Literal::Uint(_, _) => TyKind::Scalar(ScalarType::U32),
                    hir_def::expression::Literal::Float(_, _) => TyKind::Scalar(ScalarType::F32),
                    hir_def::expression::Literal::Bool(_) => TyKind::Scalar(ScalarType::Bool),
                };
                self.database.intern_ty(ty_kind)
            },
            Expression::TypeSpecifier(type_specifier) => self
                .try_lower_ty(
                    type_specifier,
                    self.resolver_for_expression(expression)
                        .as_ref()
                        .unwrap_or(&self.resolver.clone()),
                )
                .unwrap_or_else(|_| {
                    self.push_diagnostic(InferenceDiagnostic::UnresolvedName {
                        expression,
                        name: type_specifier.path.clone(),
                    });
                    self.error_ty()
                }),
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
            UnaryOperator::Minus => {
                Builtin::builtin_op_unary_minus(self.database).intern(self.database)
            },
            UnaryOperator::Not => {
                Builtin::builtin_op_unary_not(self.database).intern(self.database)
            },
            UnaryOperator::BitNot => {
                Builtin::builtin_op_unary_bitnot(self.database).intern(self.database)
            },
            UnaryOperator::Reference => {
                if let TyKind::Reference(reference) = expression_ty.kind(self.database) {
                    return self.ref_to_pointer(&reference);
                }
                self.push_diagnostic(InferenceDiagnostic::AddressOfNotReference {
                    expression,
                    actual: expression_ty,
                });
                return self.error_ty();
            },
            UnaryOperator::Dereference => {
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
                ArithmeticOperation::BitOr
                | ArithmeticOperation::BitAnd
                | ArithmeticOperation::BitXor => {
                    Builtin::builtin_op_binary_bitop(self.database).intern(self.database)
                },
                ArithmeticOperation::Multiply => {
                    Builtin::builtin_op_binary_mul(self.database).intern(self.database)
                },
                ArithmeticOperation::Divide => {
                    Builtin::builtin_op_binary_div(self.database).intern(self.database)
                },
                ArithmeticOperation::Add
                | ArithmeticOperation::Subtract
                | ArithmeticOperation::Modulo => {
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
                    self.make_ref(r#type, AddressSpace::Function, AccessMode::read_write()); // TODO is correct?
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
        self.call_builtin_inner(expression, builtin_id, arguments, name, None)
    }

    fn call_builtin_with_return(
        &mut self,
        expression: ExpressionId,
        builtin_id: BuiltinId,
        arguments: &[Type],
        name: Option<&'static str>,
        r#type: Type,
    ) -> Type {
        self.call_builtin_inner(expression, builtin_id, arguments, name, Some(r#type))
    }

    fn call_builtin_inner(
        &mut self,
        expression: ExpressionId,
        builtin_id: BuiltinId,
        arguments: &[Type],
        name: Option<&'static str>,
        return_ty: Option<Type>,
    ) -> Type {
        if let Ok((return_ty, overload_id)) =
            self.try_call_builtin(builtin_id, arguments, return_ty)
        {
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
        return_type: Option<Type>,
    ) -> Result<(Type, BuiltinOverloadId), ()> {
        let builtin = builtin_id.lookup(self.database);
        for (overload_id, overload) in builtin.overloads() {
            if let Ok(r#type) = self.call_builtin_overload(overload, arguments) {
                if let Some(return_type) = return_type {
                    if return_type == r#type {
                        return Ok((r#type, overload_id));
                    }
                } else {
                    return Ok((r#type, overload_id));
                }
            }
        }
        Err(())
    }

    fn call_builtin_overload(
        &self,
        signatre: &BuiltinOverload,
        arguments: &[Type],
    ) -> Result<Type, ()> {
        let fn_ty = signatre.r#type.lookup(self.database);

        if fn_ty.parameters.len() != arguments.len() {
            return Err(());
        }

        let mut unification_table = UnificationTable::default();
        for (expected, &found) in fn_ty.parameters().zip(arguments.iter()) {
            unify(self.database, &mut unification_table, expected, found)?;
        }

        let return_type = fn_ty
            .return_type
            .map(|r#type| unification_table.resolve(self.database, r#type));

        Ok(return_type.unwrap_or_else(|| self.error_ty()))
    }

    fn infer_call(
        &mut self,
        expression: ExpressionId,
        callee: &TypeSpecifier,
        arguments: Vec<Type>,
    ) -> Type {
        match callee {
            Callee::InferredComponentMatrix { rows, columns } => {
                let builtin_id = self.builtin_matrix_inferred_constructor(*columns, *rows);

                self.call_builtin(
                    expression,
                    builtin_id,
                    &arguments,
                    Some("matrix construction"),
                )
            },
            Callee::InferredComponentVec(size) => {
                let builtin_id = self.builtin_vector_inferred_constructor(*size);

                self.call_builtin(expression, builtin_id, &arguments, Some("vec construction"))
            },
            Callee::InferredComponentArray => {
                let builtin_id =
                    Builtin::builtin_op_array_constructor(self.database).intern(self.database);
                // TODO: Special case calling array initialisers to allow n-ary calls

                self.call_builtin(
                    expression,
                    builtin_id,
                    &arguments,
                    Some("array construction"),
                )
            },
            Callee::Name(name) => {
                if let Some(arg) = self.resolver.resolve_callable(name) {
                    match arg {
                        hir_def::resolver::ResolveType::Struct(loc) => {
                            let r#struct = self.database.intern_struct(loc);
                            let kind = TyKind::Struct(r#struct);
                            let r#type = self.database.intern_ty(kind);
                            self.check_ty_initialiser(expression, r#type, arguments);
                            r#type
                        },
                        hir_def::resolver::ResolveType::TypeAlias(alias) => {
                            let alias = self.database.intern_type_alias(alias);
                            let data = self.database.type_alias_data(alias);
                            let type_ref = self.database.lookup_intern_type_ref(data.r#type);

                            let r#type = self.lower_ty(TypeContainer::TypeAlias(alias), &type_ref);
                            self.check_ty_initialiser(expression, r#type, arguments);
                            r#type
                        },
                        hir_def::resolver::ResolveType::Function(loc) => {
                            let id = self.database.intern_function(loc);
                            let resolved = self.database.function_type(id);
                            let details = resolved.lookup(self.database);
                            self.result
                                .call_resolutions
                                .insert(expression, ResolvedCall::Function(resolved));
                            self.validate_function_call(
                                &details, &arguments, expression, expression,
                            )
                        },
                        hir_def::resolver::ResolveType::PredeclaredTypeAlias(type_ref) => {
                            let r#type = self.lower_ty(expression, &type_ref);
                            self.check_ty_initialiser(expression, r#type, arguments);
                            r#type
                        },
                    }
                } else {
                    let builtin = Builtin::for_name(self.database, name);
                    if let Some(builtin) = builtin {
                        let builtin_id = builtin.intern(self.database);
                        self.call_builtin(expression, builtin_id, &arguments, None)
                    } else {
                        self.push_diagnostic(InferenceDiagnostic::UnresolvedName {
                            expression,
                            name: name.clone(),
                        });
                        self.error_ty()
                    }
                }
            },
            Callee::Type(r#type) => {
                let r#type =
                    self.lower_ty(expression, &self.database.lookup_intern_type_ref(*r#type));
                self.check_ty_initialiser(expression, r#type, arguments);
                // A type initialiser always returns just the returned type
                r#type
            },
        }
    }

    fn check_ty_initialiser(
        &mut self,
        expression: ExpressionId,
        r#type: Type,
        arguments: Vec<Type>,
    ) {
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
            TyKind::Scalar(_) => {
                if arguments.is_empty() {
                    // Permit the zero value
                    return;
                }
                let builtin = Builtin::builtin_op_convert(self.database).intern(self.database);
                self.call_builtin_with_return(
                    expression,
                    builtin,
                    &arguments,
                    Some("conversion"),
                    r#type,
                );
            },
            TyKind::Array(array_type) => {
                if arguments.is_empty() {}
                // checking that all the arguments have the same type (inner)
            },
            TyKind::Vector(vec) => {
                if arguments.is_empty() {
                    return;
                }
                let construction_builtin_id =
                    self.builtin_vector_inferred_constructor(size_to_dimension(vec.size));
                let construction_result =
                    self.try_call_builtin(construction_builtin_id, &arguments, Some(r#type));
                if construction_result.is_ok() {
                    return;
                }
                let conversion_id =
                    Builtin::builtin_op_convert(self.database).intern(self.database);
                let conversion_result =
                    self.try_call_builtin(conversion_id, &arguments, Some(r#type));
                if conversion_result.is_ok() {
                    return;
                }
                self.push_diagnostic(InferenceDiagnostic::NoConstructor {
                    expression,
                    builtins: [construction_builtin_id, conversion_id],
                    r#type,
                    parameters: arguments,
                });
            },
            TyKind::Matrix(matrix) => {
                if arguments.is_empty() {
                    return;
                }
                let construction_builtin_id = self.builtin_matrix_inferred_constructor(
                    size_to_dimension(matrix.columns),
                    size_to_dimension(matrix.rows),
                );
                let construction_result =
                    self.try_call_builtin(construction_builtin_id, &arguments, Some(r#type));
                if construction_result.is_ok() {
                    return;
                }
                let conversion_id =
                    Builtin::builtin_op_convert(self.database).intern(self.database);
                let conversion_result =
                    self.try_call_builtin(conversion_id, &arguments, Some(r#type));
                if conversion_result.is_ok() {
                    return;
                }
                self.push_diagnostic(InferenceDiagnostic::NoConstructor {
                    expression,
                    builtins: [construction_builtin_id, conversion_id],
                    r#type,
                    parameters: arguments,
                });
            },
            TyKind::Struct(_) => {
                if arguments.is_empty() {}
                // TODO: Implement checking field types
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
            },
            #[expect(clippy::unreachable, reason = "TODO")]
            TyKind::BoundVar(_) | TyKind::Reference(_) => unreachable!(),
            TyKind::Error => {},
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TypeExpectationInner {
    Exact(Type),
    I32OrF32,
    NumericScalar,
    IntegerScalar,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TypeExpectation {
    Type(TypeExpectationInner),
    TypeOrVecOf(TypeExpectationInner),
    None,
}

impl TypeExpectation {
    const fn from_option(option: Option<Type>) -> Self {
        match option {
            Some(r#type) => Self::Type(TypeExpectationInner::Exact(r#type)),
            None => Self::None,
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
        self.database.intern_ty(TyKind::Error)
    }

    fn bool_ty(&self) -> Type {
        self.database.intern_ty(TyKind::Scalar(ScalarType::Bool))
    }

    pub fn lower_ty(
        &mut self,
        container: impl Into<TypeContainer>,
        type_ref: &TypeSpecifier,
    ) -> Type {
        let container: TypeContainer = container.into();
        let resolver = match container {
            TypeContainer::Expr(expression) => self.resolver_for_expression(expression),
            TypeContainer::VariableStatement(statement) => self.resolver_for_statement(statement),
            TypeContainer::GlobalVar(_)
            | TypeContainer::GlobalConstant(_)
            | TypeContainer::Override(_)
            | TypeContainer::TypeAlias(_)
            | TypeContainer::FunctionParameter(_)
            | TypeContainer::FunctionReturn(_)
            | TypeContainer::StructField(_) => None,
        };
        match self.try_lower_ty(
            type_ref,
            // TODO: Could I get away without cloning the resolver?
            resolver.as_ref().unwrap_or(&self.resolver.clone()),
        ) {
            Ok(r#type) => r#type,
            Err(error) => {
                self.push_diagnostic(InferenceDiagnostic::InvalidType { container, error });
                self.error_ty()
            },
        }
    }

    /// Convert a [`TypeSpecifier`] into a `[Type]`.
    ///
    /// # Panics
    ///
    /// Panics if an extreme, probably impossible type is give, such as an array with a size exceeding 64 bits.
    ///
    /// # Errors
    ///
    /// This function will return an error if type is a path and the path is unknown.
    pub fn try_lower_ty(
        &mut self,
        type_ref: &TypeSpecifier,
        resolver: &Resolver,
    ) -> Result<Type, TypeLoweringError> {
        let name = &type_ref.path;
        match resolver.resolve_type(&name) {
            Some(ResolveType::Function(loc)) => {
                let id = self.database.intern_function(loc);
                let result = self.database.function_type(id);
                Ok(result
                    .lookup(self.database)
                    .return_type
                    // TODO: This should be a "void" type instead? Or maybe we should emit a diagnostic?
                    .unwrap_or_else(|| TyKind::Error.intern(self.database)))
            },
            Some(ResolveType::GlobalConstant(loc)) => {
                let id = self.database.intern_global_constant(loc);
                let result = self.database.infer(DefinitionId::GlobalConstant(id));
                Ok(result.return_type)
            },
            Some(ResolveType::GlobalVariable(loc)) => {
                let id = self.database.intern_global_variable(loc);
                let result = self.database.infer(DefinitionId::GlobalVariable(id));
                Ok(result.return_type)
            },
            Some(ResolveType::Override(loc)) => {
                let id = self.database.intern_override(loc);
                let result = self.database.infer(DefinitionId::Override(id));
                Ok(result.return_type)
            },
            Some(ResolveType::Struct(loc)) => {
                let r#struct = self.database.intern_struct(loc);
                Ok(self.database.intern_ty(TyKind::Struct(r#struct)))
            },
            Some(ResolveType::TypeAlias(loc)) => {
                let id = self.database.intern_type_alias(loc);
                // It needs a new resolver, so we trigger a new infer call
                let result = self.database.infer(DefinitionId::TypeAlias(id));
                Ok(result.return_type)
            },
            Some(ResolveType::Local(local)) => Ok(self
                .result
                .type_of_binding
                .get(local)
                .cloned()
                .unwrap_or_else(|| TyKind::Error.intern(self.database))),
            None => self.lower_predeclared_ty(type_ref),
        }
        /*
        let ty_kind = match type_ref {
            TypeReference::Error => TyKind::Error,
            TypeReference::Scalar(scalar) => {
                let scalar = match scalar {
                    type_ref::ScalarType::Bool => ScalarType::Bool,
                    type_ref::ScalarType::Float32 => ScalarType::F32,
                    type_ref::ScalarType::Int32 => ScalarType::I32,
                    type_ref::ScalarType::Uint32 => ScalarType::U32,
                };
                TyKind::Scalar(scalar)
            },
            TypeReference::Vec(vec) => TyKind::Vector(VectorType {
                size: vec.size.into(),
                component_type: self.lower_ty(&vec.inner),
            }),
            TypeReference::Matrix(matrix) => TyKind::Matrix(MatrixType {
                columns: matrix.columns.into(),
                rows: matrix.rows.into(),
                inner: self.lower_ty(&matrix.inner),
            }),
            TypeReference::Texture(tex) => TyKind::Texture(TextureType {
                kind: match &tex.kind {
                    type_ref::TextureKind::Sampled(r#type) => {
                        TextureKind::Sampled(self.lower_ty(r#type))
                    },
                    type_ref::TextureKind::Storage(format, mode) => TextureKind::Storage(
                        format
                            .parse()
                            .map_err(|()| TypeLoweringError::InvalidTexelFormat(format.clone()))?,
                        *mode,
                    ),
                    type_ref::TextureKind::Depth => TextureKind::Depth,
                    type_ref::TextureKind::External => TextureKind::External,
                },
                dimension: match tex.dimension {
                    type_ref::TextureDimension::D1 => TextureDimensionality::D1,
                    type_ref::TextureDimension::D2 => TextureDimensionality::D2,
                    type_ref::TextureDimension::D3 => TextureDimensionality::D3,
                    type_ref::TextureDimension::Cube => TextureDimensionality::Cube,
                },
                arrayed: tex.arrayed,
                multisampled: tex.multisampled,
            }),
            TypeReference::Sampler(sampler) => TyKind::Sampler(SamplerType {
                comparison: sampler.comparison,
            }),
            TypeReference::Pointer(pointer) => TyKind::Pointer(Pointer {
                address_space: pointer.address_space,
                inner: self.lower_ty(&pointer.inner),
                access_mode: pointer.access_mode,
            }),
            TypeReference::Path(name) => match self.resolver.resolve_type(name) {
                Some(ResolveType::Struct(loc)) => {
                    let r#struct = self.database.intern_struct(loc);
                    TyKind::Struct(r#struct)
                },
                Some(ResolveType::TypeAlias(loc)) => {
                    let alias = self.database.intern_type_alias(loc);
                    let data = self.database.type_alias_data(alias).0;

                    return Ok(self.lower_ty(&data.r#type));
                },
                Some(ResolveType::PredeclaredTypeAlias(type_ref)) => {
                    return Ok(self.lower_ty(&type_ref));
                },
                None => return Err(TypeLoweringError::UnresolvedName(name.clone())),
            },
        }; */
    }

    fn lower_predeclared_ty(
        &self,
        type_ref: &TypeSpecifier,
    ) -> Result<Type, TypeLoweringError> {
        let name = type_ref.path.as_str();
        let ty_kind: TyKind = match name {
            "vec2i" => TyKind::Vector(VectorType {
                size: VecSize::Two,
                component_type: TyKind::Scalar(ScalarType::I32).intern(self.database),
            }),
            "vec3i" => TyKind::Vector(VectorType {
                size: VecSize::Three,
                component_type: TyKind::Scalar(ScalarType::I32).intern(self.database),
            }),
            "vec4i" => TyKind::Vector(VectorType {
                size: VecSize::Four,
                component_type: TyKind::Scalar(ScalarType::I32).intern(self.database),
            }),
            "vec2u" => TyKind::Vector(VectorType {
                size: VecSize::Two,
                component_type: TyKind::Scalar(ScalarType::U32).intern(self.database),
            }),
            "vec3u" => TyKind::Vector(VectorType {
                size: VecSize::Three,
                component_type: TyKind::Scalar(ScalarType::U32).intern(self.database),
            }),
            "vec4u" => TyKind::Vector(VectorType {
                size: VecSize::Four,
                component_type: TyKind::Scalar(ScalarType::U32).intern(self.database),
            }),
            "vec2f" => TyKind::Vector(VectorType {
                size: VecSize::Two,
                component_type: TyKind::Scalar(ScalarType::F32).intern(self.database),
            }),
            "vec3f" => TyKind::Vector(VectorType {
                size: VecSize::Three,
                component_type: TyKind::Scalar(ScalarType::F32).intern(self.database),
            }),
            "vec4f" => TyKind::Vector(VectorType {
                size: VecSize::Four,
                component_type: TyKind::Scalar(ScalarType::F32).intern(self.database),
            }),
            "array" => {
                TyKind::Array(ArrayType {
                    // This needs access to full expression inference, unlike in Rust.
                    inner: self.infer_expression(&array.inner),
                    binding_array: false,
                    size: match array.size {
                        type_ref::ArraySize::Int(integer) => {
                            // TODO give error instead of panicking
                            ArraySize::Constant(u64::try_from(integer).unwrap())
                        },
                        type_ref::ArraySize::Uint(unsigned_integer) => {
                            ArraySize::Constant(unsigned_integer)
                        },
                        type_ref::ArraySize::Path(_) => ArraySize::Constant(0), // TODO: Path array sizes
                        type_ref::ArraySize::Dynamic => ArraySize::Dynamic,
                    },
                })
            },
            "atomic" => TyKind::Atomic(AtomicType {
                inner: self.lower_ty(&atomic.inner),
            }),
            // Naga extension
            "binding_array" => {
                TyKind::Array(ArrayType {
                    inner: self.lower_ty(&array.inner),
                    binding_array: true,
                    size: match array.size {
                        type_ref::ArraySize::Int(integer) => {
                            // TODO give error instead of panicking
                            ArraySize::Constant(u64::try_from(integer).unwrap())
                        },
                        type_ref::ArraySize::Uint(unsigned_integer) => {
                            ArraySize::Constant(unsigned_integer)
                        },
                        type_ref::ArraySize::Path(_) => ArraySize::Constant(0), // TODO: Path array sizes
                        type_ref::ArraySize::Dynamic => ArraySize::Dynamic,
                    },
                })
            },
            // TODO float16
            _ => return Err(TypeLoweringError::UnresolvedName(type_ref.path.clone())),
        };

        Ok(self.database.intern_ty(ty_kind))
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TypeLoweringError {
    UnresolvedName(Name),
    InvalidTexelFormat(String),
}

impl fmt::Display for TypeLoweringError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::UnresolvedName(name) => {
                write!(formatter, "type `{}` not found in scope", name.as_str())
            },
            Self::InvalidTexelFormat(format) => {
                let all_formats = "rgba8unorm,\nrgba8snorm,\nrgba8uint,\nrgba8sint,\nrgba16uint,\nrgba16sint,\nrgba16float,\nr32uint,\nr32sint,\nr32float,\nrg32uint,\nrg32sint,\nrg32float,\nrgba32uint,\nrgba32sint,\nrgba32float";
                write!(
                    formatter,
                    "`{format}` is not a valid texel format, expected one of:\n{all_formats}"
                )
            },
        }
    }
}
