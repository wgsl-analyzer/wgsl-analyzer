use std::{collections::hash_map::Entry, sync::Arc};

use either::Either;
use hir_def::{
    body::{BindingId, Body},
    data::{FieldId, FunctionData, GlobalConstantData, GlobalVariableData, OverrideData},
    database::{
        DefinitionWithBodyId, FunctionId, GlobalConstantId, GlobalVariableId, OverrideId,
        TypeAliasId,
    },
    expression::{
        ArithmeticOperation, BinaryOperation, Callee, ComparisonOperation, Expression,
        ExpressionId, Statement, StatementId, UnaryOperator,
    },
    module_data::Name,
    resolver::{ResolveType, Resolver},
    type_ref::{self, AccessMode, AddressSpace, TypeReference, VecDimensionality},
};
use la_arena::ArenaMap;
use rustc_hash::FxHashMap;

use crate::{
    builtins::{Builtin, BuiltinId, BuiltinOverload, BuiltinOverloadId},
    database::HirDatabase,
    function::{FunctionDetails, ResolvedFunctionId},
    ty::{
        ArraySize, ArrayType, AtomicType, BoundVar, MatrixType, Pointer, Reference, SamplerType,
        ScalarType, TexelFormat, TextureDimensionality, TextureKind, TextureType, TyKind, Type,
        VecSize, VectorType,
    },
};

pub fn infer_query(
    database: &dyn HirDatabase,
    def: DefinitionWithBodyId,
) -> Arc<InferenceResult> {
    let resolver = def.resolver(database);
    let mut context = InferenceContext::new(database, def, resolver);

    match def {
        DefinitionWithBodyId::Function(function) => {
            context.collect_fn(function, &database.fn_data(function));
        },
        DefinitionWithBodyId::GlobalVariable(var) => {
            context.collect_global_variable(var, &database.global_var_data(var));
        },
        DefinitionWithBodyId::GlobalConstant(constant) => {
            context.collect_global_constant(constant, &database.global_constant_data(constant));
        },
        DefinitionWithBodyId::Override(override_decl) => {
            context.collect_override(override_decl, &database.override_data(override_decl));
        },
    }

    context.infer_body();

    Arc::new(context.resolve_all())
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
}

#[derive(PartialEq, Eq, Debug)]
pub enum TypeContainer {
    Expr(ExpressionId),
    GlobalVar(GlobalVariableId),
    GlobalConstant(GlobalConstantId),
    Override(OverrideId),
    TypeAlias(TypeAliasId),
    FunctionParameter(FunctionId, BindingId),
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
#[derive(Default, PartialEq, Eq, Debug)]
pub struct InferenceResult {
    pub type_of_expression: ArenaMap<ExpressionId, Type>,
    pub type_of_binding: ArenaMap<BindingId, Type>,
    pub diagnostics: Vec<InferenceDiagnostic>,
    pub return_type: Option<Type>,
    call_resolutions: FxHashMap<ExpressionId, ResolvedCall>,
    field_resolutions: FxHashMap<ExpressionId, FieldId>,
}

impl InferenceResult {
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
    owner: DefinitionWithBodyId,
    resolver: Resolver,
    body: Arc<Body>,
    result: InferenceResult,
    return_ty: Option<Type>,
}

impl<'database> InferenceContext<'database> {
    pub fn new(
        database: &'database dyn HirDatabase,
        owner: DefinitionWithBodyId,
        resolver: Resolver,
    ) -> Self {
        Self {
            database,
            owner,
            resolver,
            body: database.body(owner),
            result: InferenceResult::default(),
            return_ty: None,
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
        let r#type = var.r#type.map(|r#type| {
            self.lower_ty(
                TypeContainer::GlobalVar(id),
                &self.database.lookup_intern_type_ref(r#type),
            )
        });

        if let Some(r#type) = r#type {
            if let Some(binding) = self.body.main_binding {
                self.set_binding_ty(binding, r#type);
            }
        }

        self.return_ty = r#type;
    }

    fn collect_global_constant(
        &mut self,
        id: GlobalConstantId,
        constant: &GlobalConstantData,
    ) {
        let r#type = constant.r#type.map(|r#type| {
            self.lower_ty(
                TypeContainer::GlobalConstant(id),
                &self.database.lookup_intern_type_ref(r#type),
            )
        });

        if let Some(r#type) = r#type {
            if let Some(binding) = self.body.main_binding {
                self.set_binding_ty(binding, r#type);
            }
        }

        self.return_ty = r#type;
    }

    fn collect_override(
        &mut self,
        id: OverrideId,
        constant: &OverrideData,
    ) {
        let r#type = constant.r#type.map(|r#type| {
            self.lower_ty(
                TypeContainer::Override(id),
                &self.database.lookup_intern_type_ref(r#type),
            )
        });

        if let Some(r#type) = r#type {
            if let Some(binding) = self.body.main_binding {
                self.set_binding_ty(binding, r#type);
            }
        }

        self.return_ty = r#type;
    }

    fn collect_fn(
        &mut self,
        function_id: FunctionId,
        function_data: &FunctionData,
    ) {
        let body = Arc::clone(&self.body);
        for (&(parameter, _), &id) in function_data.parameters.iter().zip(&body.parameters) {
            let type_ref = self.database.lookup_intern_type_ref(parameter);
            let param_ty =
                self.lower_ty(TypeContainer::FunctionParameter(function_id, id), &type_ref);
            self.set_binding_ty(id, param_ty);
        }
        self.return_ty = function_data.return_type.map(|type_ref| {
            self.lower_ty(
                TypeContainer::FunctionReturn(function_id),
                &self.database.lookup_intern_type_ref(type_ref),
            )
        });
    }

    fn infer_body(&mut self) {
        match self.body.root {
            Some(Either::Left(statement)) => {
                self.infer_statement(statement);
            },
            Some(Either::Right(expression)) => {
                let r#type = self.infer_expression_expect(
                    expression,
                    &TypeExpectation::from_option(self.return_ty),
                );
                if self.return_ty.is_none() {
                    self.return_ty = Some(r#type);
                }

                if let Some(main_binding) = self.body.main_binding {
                    self.set_binding_ty(main_binding, r#type);
                }
            },
            None => (),
        }
    }

    fn resolver_for_expression(
        &self,
        expression: ExpressionId,
    ) -> Resolver {
        let resolver = self.resolver.clone();
        match self.owner {
            DefinitionWithBodyId::Function(function) => {
                let expression_scopes = self.database.expression_scopes(self.owner);
                let scope_id = expression_scopes.scope_for_expression(expression).unwrap();
                resolver.push_expression_scope(function, expression_scopes, scope_id)
            },
            DefinitionWithBodyId::GlobalVariable(_)
            | DefinitionWithBodyId::GlobalConstant(_)
            | DefinitionWithBodyId::Override(_) => resolver,
        }
    }

    #[expect(clippy::too_many_lines, reason = "TODO")]
    fn infer_statement(
        &mut self,
        statement: StatementId,
    ) {
        let body = Arc::clone(&self.body);

        match &body.statements[statement] {
            Statement::Compound { statements } => {
                for statement in statements {
                    self.infer_statement(*statement);
                }
            },
            Statement::VariableStatement {
                binding_id,
                type_ref,
                initializer,
                address_space,
                access_mode,
            } => {
                let r#type = type_ref.map(|r#type| {
                    self.lower_ty(
                        TypeContainer::VariableStatement(statement),
                        &self.database.lookup_intern_type_ref(r#type),
                    )
                });
                let r#type = if let Some(init) = initializer {
                    let expression_ty =
                        self.infer_expression_expect(*init, &TypeExpectation::from_option(r#type));
                    r#type.unwrap_or(expression_ty)
                } else {
                    r#type.unwrap_or_else(|| self.error_ty())
                };

                let ref_ty = self.make_ref(
                    r#type,
                    address_space.unwrap_or(AddressSpace::Function),
                    access_mode.unwrap_or_else(AccessMode::read_write),
                );
                self.set_binding_ty(*binding_id, ref_ty);
            },
            Statement::ConstStatement {
                binding_id,
                type_ref,
                initializer,
                ..
            } => {
                let r#type = type_ref.map(|r#type| {
                    self.lower_ty(
                        TypeContainer::VariableStatement(statement),
                        &self.database.lookup_intern_type_ref(r#type),
                    )
                });
                let r#type = if let Some(init) = initializer {
                    let expression_ty =
                        self.infer_expression_expect(*init, &TypeExpectation::from_option(r#type));
                    r#type.unwrap_or(expression_ty)
                } else {
                    r#type.unwrap_or_else(|| self.error_ty())
                };

                self.set_binding_ty(*binding_id, r#type);
            },
            Statement::LetStatement {
                binding_id,
                type_ref,
                initializer,
                ..
            } => {
                let r#type = type_ref.map(|r#type| {
                    self.lower_ty(
                        TypeContainer::VariableStatement(statement),
                        &self.database.lookup_intern_type_ref(r#type),
                    )
                });
                let r#type = if let Some(init) = initializer {
                    let expression_ty =
                        self.infer_expression_expect(*init, &TypeExpectation::from_option(r#type));
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
                        &TypeExpectation::from_option(self.return_ty),
                    );
                }
            },
            Statement::Assignment {
                left_side,
                right_side,
            } => {
                let left_ty = self.infer_expression(*left_side);

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

                self.infer_expression_expect(*right_side, &TypeExpectation::from_ty(left_inner));
            },
            Statement::CompoundAssignment {
                left_side,
                right_side,
                op,
            } => {
                let left_ty = self.infer_expression(*left_side);

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

                let r#type = self.infer_binary_op(*left_side, *right_side, (*op).into());

                self.expect_same_type(*left_side, r#type, left_inner);
            },
            Statement::IncrDecr { expression, .. } => {
                let left_ty = self.infer_expression(*expression);

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
                self.infer_expression_expect(*condition, &TypeExpectation::from_ty(self.bool_ty()));
            },
            Statement::While { condition, block } => {
                self.infer_statement(*block);
                self.infer_expression_expect(*condition, &TypeExpectation::from_ty(self.bool_ty()));
            },
            Statement::Switch {
                expression,
                case_blocks,
                default_block,
            } => {
                let r#type = self.infer_expression(*expression).unref(self.database);

                for (selectors, case) in case_blocks {
                    for selector in selectors {
                        self.infer_expression_expect(*selector, &TypeExpectation::from_ty(r#type));
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
                self.infer_expression(*expression);
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
    ) -> Type {
        let r#type = self.infer_expression(expression).unref(self.database);

        match &expected {
            TypeExpectation::Type(expected_type) => {
                if self.expect_ty_inner(r#type, expected_type) == Ok(()) {
                    r#type
                } else {
                    self.push_diagnostic(InferenceDiagnostic::TypeMismatch {
                        expression,
                        actual: r#type,
                        expected: expected.clone(),
                    });
                    r#type
                }
            },
            TypeExpectation::TypeOrVecOf(expect) => {
                if self.expect_ty_inner(r#type.this_or_vec_inner(self.database), expect) == Ok(()) {
                    r#type
                } else {
                    self.push_diagnostic(InferenceDiagnostic::TypeMismatch {
                        expression,
                        actual: r#type,
                        expected: expected.clone(),
                    });
                    r#type
                }
            },
            TypeExpectation::None => r#type,
        }
    }

    #[expect(clippy::too_many_lines, reason = "TODO")]
    fn infer_expression(
        &mut self,
        expression: ExpressionId,
    ) -> Type {
        let body = Arc::clone(&self.body);
        let r#type = match &body.exprs[expression] {
            Expression::Missing => self.error_ty(),
            Expression::BinaryOperation {
                left_side,
                right_side,
                operation,
            } => self.infer_binary_op(*left_side, *right_side, *operation),
            Expression::UnaryOperator { expression, op } => self.infer_unary_op(*expression, *op),
            Expression::Field {
                expression: field_expression,
                name,
            } => {
                let expression_ty = self.infer_expression(*field_expression);
                if expression_ty.is_err(self.database) {
                    return self.error_ty();
                }

                match expression_ty
                    .kind(self.database)
                    .unref(self.database)
                    .as_ref()
                {
                    TyKind::Struct(r#struct) => {
                        let struct_data = self.database.struct_data(*r#struct);
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
                    TyKind::Matrix(_) => {
                        self.push_diagnostic(InferenceDiagnostic::NoSuchField {
                            expression: *field_expression,
                            name: name.clone(),
                            r#type: expression_ty,
                        });
                        self.error_ty()
                    },
                    _ => {
                        self.push_diagnostic(InferenceDiagnostic::NoSuchField {
                            expression: *field_expression,
                            name: name.clone(),
                            r#type: expression_ty,
                        });
                        self.error_ty()
                    },
                }
            },
            Expression::Call { callee, arguments } => {
                let arguments: Vec<_> = arguments
                    .iter()
                    .map(|&arg| self.infer_expression(arg).unref(self.database))
                    .collect();
                self.infer_call(expression, callee, arguments)
            },
            Expression::Bitcast { r#type, expression } => {
                self.infer_expression(*expression);

                self.try_lower_ty(&self.database.lookup_intern_type_ref(*r#type))
                    .unwrap_or_else(|_| self.error_ty())
            },
            Expression::Index { left_side, index } => {
                let left_side = self.infer_expression(*left_side);
                let _index_expression = self.infer_expression(*index);
                // TODO check index expression

                let left_kind = left_side.kind(self.database);
                let is_reference = matches!(left_kind, TyKind::Reference(_));

                let left_inner = left_kind.unref(self.database);

                let r#type = match &*left_inner {
                    TyKind::Vector(vec) => {
                        // TODO out of bounds
                        vec.inner
                    },
                    TyKind::Matrix(mat) => {
                        // TODO out of bounds
                        self.database.intern_ty(TyKind::Vector(VectorType {
                            inner: mat.inner,
                            size: mat.rows,
                        }))
                    },
                    TyKind::Array(array) => {
                        // TODO out of bounds
                        array.inner
                    },
                    _ => {
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
            Expression::Path(name) => self
                .resolve_path_expression(expression, name)
                .unwrap_or_else(|| {
                    self.push_diagnostic(InferenceDiagnostic::UnresolvedName {
                        expression,
                        name: name.clone(),
                    });
                    self.error_ty()
                }),
        };

        self.set_expression_ty(expression, r#type);

        r#type
    }

    fn validate_function_call(
        &mut self,
        f: &FunctionDetails,
        arguments: Vec<Type>,
        callee: ExpressionId,
        expression: ExpressionId,
    ) -> Type {
        if f.parameters.len() == arguments.len() {
            for (expected, actual) in f.parameters().zip(arguments.iter().copied()) {
                self.expect_same_type(expression, expected, actual);
            }

            f.return_type.unwrap_or_else(|| self.error_ty())
        } else {
            self.push_diagnostic(InferenceDiagnostic::FunctionCallArgCountMismatch {
                expression: callee,
                n_expected: f.parameters.len(),
                n_actual: arguments.len(),
            });
            self.error_ty()
        }
    }

    fn infer_unary_op(
        &mut self,
        expression: ExpressionId,
        op: UnaryOperator,
    ) -> Type {
        let expression_ty = self.infer_expression(expression);
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
                    return self.ref_to_pointer(reference);
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
                    return self.ptr_to_ref(pointer);
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
    ) -> Type {
        let left_ty = self.infer_expression(left_side).unref(self.database);
        let rhs_ty = self.infer_expression(right_side).unref(self.database);

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
        size: &VecDimensionality,
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
        columns: &VecDimensionality,
        rows: &VecDimensionality,
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

    fn resolve_path_expression(
        &self,
        expression: ExpressionId,
        path: &Name,
    ) -> Option<Type> {
        self.resolve_path_expression_inner(expression, path)
    }

    fn resolve_path_expression_inner(
        &self,
        expression: ExpressionId,
        path: &Name,
    ) -> Option<Type> {
        let resolver = self.resolver_for_expression(expression);
        let resolve = resolver.resolve_value(path)?;
        let r#type = match resolve {
            hir_def::resolver::ResolveValue::Local(local) => {
                *self.result.type_of_binding.get(local)?
            },
            hir_def::resolver::ResolveValue::GlobalVariable(loc) => {
                let id = self.database.intern_global_variable(loc);
                let data = self.database.global_var_data(id);
                let result = self
                    .database
                    .infer(DefinitionWithBodyId::GlobalVariable(id));
                let r#type = result.return_type.unwrap_or_else(|| self.error_ty());
                // TODO use correct defaults
                self.make_ref(
                    r#type,
                    data.address_space.unwrap_or(AddressSpace::Private),
                    AccessMode::read_write(),
                )
            },
            hir_def::resolver::ResolveValue::GlobalConstant(loc) => {
                let id = self.database.intern_global_constant(loc);
                let result = self
                    .database
                    .infer(DefinitionWithBodyId::GlobalConstant(id));
                result.return_type.unwrap_or_else(|| self.error_ty())
            },
            hir_def::resolver::ResolveValue::Override(loc) => {
                let id = self.database.intern_override(loc);
                let result = self.database.infer(DefinitionWithBodyId::Override(id));
                result.return_type.unwrap_or_else(|| self.error_ty())
            },
        };
        Some(r#type)
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
                .map(|size| TyKind::Vector(VectorType { size, inner }))
                .unwrap_or(TyKind::Error);
            self.database.intern_ty(kind)
        }
    }

    fn vec_swizzle(
        &self,
        vec_type: &VectorType,
        name: &Name,
    ) -> Result<Type, ()> {
        const SWIZZLES: [[char; 4]; 2] = [['x', 'y', 'z', 'w'], ['r', 'g', 'b', 'a']];
        let max_size = 4;
        let max_swizzle_index = vec_type.size.as_u8();

        if name.as_str().len() > max_size {
            return Err(());
        }

        for swizzle in &SWIZZLES {
            let allowed_chars = &swizzle[..max_swizzle_index as usize];
            if name
                .as_str()
                .chars()
                .all(|character| allowed_chars.contains(&character))
            {
                let r#type = self.ty_from_vec_size(vec_type.inner, name.as_str().len() as u8);
                let r = self.make_ref(r#type, AddressSpace::Function, AccessMode::read_write()); // TODO is correct?
                return Ok(r);
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
        &mut self,
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
        callee: &Callee,
        arguments: Vec<Type>,
    ) -> Type {
        match callee {
            Callee::InferredComponentMatrix { rows, columns } => {
                let builtin_id = self.builtin_matrix_inferred_constructor(columns, rows);

                self.call_builtin(
                    expression,
                    builtin_id,
                    &arguments,
                    Some("matrix construction"),
                )
            },
            Callee::InferredComponentVec(size) => {
                let builtin_id = self.builtin_vector_inferred_constructor(size);

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
                        hir_def::resolver::ResolveCallable::Struct(loc) => {
                            let r#struct = self.database.intern_struct(loc);
                            let kind = TyKind::Struct(r#struct);
                            let r#type = self.database.intern_ty(kind);
                            self.check_ty_initialiser(expression, r#type, arguments);
                            r#type
                        },
                        hir_def::resolver::ResolveCallable::TypeAlias(alias) => {
                            let alias = self.database.intern_type_alias(alias);
                            let data = self.database.type_alias_data(alias);
                            let type_ref = self.database.lookup_intern_type_ref(data.r#type);

                            let r#type = self.lower_ty(TypeContainer::TypeAlias(alias), &type_ref);
                            self.check_ty_initialiser(expression, r#type, arguments);
                            r#type
                        },
                        hir_def::resolver::ResolveCallable::Function(loc) => {
                            let id = self.database.intern_function(loc);
                            let resolved = self.database.function_type(id);
                            let details = resolved.lookup(self.database);
                            self.result
                                .call_resolutions
                                .insert(expression, ResolvedCall::Function(resolved));
                            self.validate_function_call(&details, arguments, expression, expression)
                        },
                        hir_def::resolver::ResolveCallable::PredeclaredTypeAlias(type_ref) => {
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
            TyKind::Array(_) => {
                if arguments.is_empty() {}
                // TODO: Implement checking that all the arguments have the same type (inner)
            },
            TyKind::Vector(vec) => {
                if arguments.is_empty() {
                    return;
                }
                let construction_builtin_id =
                    self.builtin_vector_inferred_constructor(&size_to_dimension(vec.size));
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
                    &size_to_dimension(matrix.columns),
                    &size_to_dimension(matrix.rows),
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
            TyKind::BoundVar(_) | TyKind::Reference(_) => unreachable!(),
            TyKind::Error => {},
        }
    }
}

#[derive(Default)]
struct UnificationTable {
    type_vars: FxHashMap<BoundVar, Type>,
    vec_size_vars: FxHashMap<BoundVar, VecSize>,
    texel_format_vars: FxHashMap<BoundVar, TexelFormat>,
}

impl UnificationTable {
    fn set_vec_size(
        &mut self,
        var: BoundVar,
        vec_size: VecSize,
    ) -> Result<(), ()> {
        match self.vec_size_vars.entry(var) {
            Entry::Occupied(entry) if *entry.get() == vec_size => Ok(()),
            Entry::Occupied(_) => Err(()),
            Entry::Vacant(entry) => {
                entry.insert(vec_size);
                Ok(())
            },
        }
    }

    fn set_type(
        &mut self,
        var: BoundVar,
        r#type: Type,
    ) -> Result<(), ()> {
        match self.type_vars.entry(var) {
            Entry::Occupied(entry) if *entry.get() == r#type => Ok(()),
            Entry::Occupied(_) => Err(()),
            Entry::Vacant(entry) => {
                entry.insert(r#type);
                Ok(())
            },
        }
    }

    fn set_texel_format(
        &mut self,
        var: BoundVar,
        format: TexelFormat,
    ) -> Result<(), ()> {
        match self.texel_format_vars.entry(var) {
            Entry::Occupied(entry) if *entry.get() == format => Ok(()),
            Entry::Occupied(_) => Err(()),
            Entry::Vacant(entry) => {
                entry.insert(format);
                Ok(())
            },
        }
    }

    fn resolve(
        &self,
        database: &dyn HirDatabase,
        r#type: Type,
    ) -> Type {
        match r#type.kind(database) {
            TyKind::BoundVar(var) => *self.type_vars.get(&var).expect("type var not constrained"),
            TyKind::Vector(VectorType { size, inner }) => {
                let size = match size {
                    VecSize::BoundVar(size_var) => *self
                        .vec_size_vars
                        .get(&size_var)
                        .expect("vec size var not constrained"),
                    size => size,
                };
                let inner = self.resolve(database, inner);
                TyKind::Vector(VectorType { size, inner }).intern(database)
            },
            TyKind::Matrix(mat) => {
                let columns = match mat.columns {
                    VecSize::BoundVar(var) => self.vec_size_vars[&var],
                    other => other,
                };
                let rows = match mat.rows {
                    VecSize::BoundVar(var) => self.vec_size_vars[&var],
                    other => other,
                };

                let inner = self.resolve(database, mat.inner);
                TyKind::Matrix(MatrixType {
                    columns,
                    rows,
                    inner,
                })
                .intern(database)
            },
            TyKind::Texture(TextureType {
                kind: TextureKind::Storage(TexelFormat::BoundVar(var), mode),
                dimension,
                arrayed,
                multisampled,
            }) => {
                let format = self.texel_format_vars[&var];

                TyKind::Texture(TextureType {
                    kind: TextureKind::Storage(format, mode),
                    dimension,
                    arrayed,
                    multisampled,
                })
                .intern(database)
            },
            TyKind::Texture(TextureType {
                kind: TextureKind::Sampled(sampled_ty),
                dimension,
                arrayed,
                multisampled,
            }) => {
                let sampled_ty = self.resolve(database, sampled_ty);
                TyKind::Texture(TextureType {
                    kind: TextureKind::Sampled(sampled_ty),
                    dimension,
                    arrayed,
                    multisampled,
                })
                .intern(database)
            },
            TyKind::StorageTypeOfTexelFormat(var) => {
                let format = self.texel_format_vars[&var];
                storage_type_of_texel_format(database, format)
            },
            _ => r#type,
        }
    }
}

// found type should not contain bound variables
#[expect(clippy::too_many_lines, reason = "TODO")]
fn unify(
    database: &dyn HirDatabase,
    table: &mut UnificationTable,
    expected: Type,
    found: Type,
) -> Result<(), ()> {
    let expected_kind = expected.kind(database);
    let found_kind = found.kind(database);

    match expected_kind {
        TyKind::BoundVar(var) => {
            table.set_type(var, found)?;
            Ok(())
        },
        TyKind::Vector(VectorType { size, inner }) => match found_kind {
            TyKind::Vector(found_vec) => {
                unify(database, table, inner, found_vec.inner)?;
                if let VecSize::BoundVar(vec_size_var) = size {
                    table.set_vec_size(vec_size_var, found_vec.size)?;
                } else if size != found_vec.size {
                    return Err(());
                }
                Ok(())
            },
            _ => Err(()),
        },
        TyKind::Matrix(MatrixType {
            columns,
            rows,
            inner,
        }) => match found_kind {
            TyKind::Matrix(found_mat) => {
                unify(database, table, inner, found_mat.inner)?;

                if let VecSize::BoundVar(var) = columns {
                    table.set_vec_size(var, found_mat.columns)?;
                } else if columns != found_mat.columns {
                    return Err(());
                }

                if let VecSize::BoundVar(var) = rows {
                    table.set_vec_size(var, found_mat.rows)?;
                } else if rows != found_mat.rows {
                    return Err(());
                }

                Ok(())
            },
            _ => Err(()),
        },
        TyKind::Pointer(pointer) => match found_kind {
            TyKind::Pointer(found_pointer) => {
                unify(database, table, pointer.inner, found_pointer.inner)?;

                Ok(())
            },
            _ => Err(()),
        },
        TyKind::Array(array) => match found_kind {
            TyKind::Array(found_array) => {
                unify(database, table, array.inner, found_array.inner)?;

                Ok(())
            },
            _ => Err(()),
        },
        TyKind::Atomic(atomic) => match found_kind {
            TyKind::Atomic(found_atomic) => {
                unify(database, table, atomic.inner, found_atomic.inner)?;

                Ok(())
            },
            _ => Err(()),
        },
        TyKind::Texture(TextureType {
            kind: TextureKind::Storage(format, mode),
            arrayed,
            multisampled,
            dimension,
        }) => match found_kind {
            TyKind::Texture(TextureType {
                kind: TextureKind::Storage(format_2, mode_2),
                arrayed: arrayed_2,
                multisampled: multisampled_2,
                dimension: dimension_2,
            }) => {
                if arrayed != arrayed_2
                    || multisampled != multisampled_2
                    || dimension != dimension_2
                {
                    return Err(());
                }

                match format {
                    TexelFormat::Any => {},
                    TexelFormat::BoundVar(var) => {
                        table.set_texel_format(var, format_2)?;
                    },
                    _ => {
                        if format != format_2 {
                            return Err(());
                        }
                    },
                }
                match (mode, mode_2) {
                    (AccessMode::Any, _) => {},
                    (_, AccessMode::Any) => unreachable!(),

                    (AccessMode::ReadWrite, AccessMode::ReadWrite) => {},
                    (AccessMode::Read, AccessMode::ReadWrite | AccessMode::Read) => {},
                    (AccessMode::Write, AccessMode::ReadWrite | AccessMode::Write) => {},

                    (AccessMode::Write | AccessMode::ReadWrite, AccessMode::Read)
                    | (AccessMode::Read | AccessMode::ReadWrite, AccessMode::Write) => {
                        return Err(());
                    },
                }

                Ok(())
            },
            _ => Err(()),
        },
        TyKind::StorageTypeOfTexelFormat(format) => {
            let format = table.texel_format_vars[&format];
            let storage_type = storage_type_of_texel_format(database, format);

            if storage_type != found {
                return Err(());
            }

            Ok(())
        },

        TyKind::Texture(TextureType {
            kind: TextureKind::Sampled(sampled_ty),
            arrayed,
            multisampled,
            dimension,
        }) => match found_kind {
            TyKind::Texture(TextureType {
                kind: TextureKind::Sampled(found_sampled_ty),
                arrayed: arrayed_2,
                multisampled: multisampled_2,
                dimension: dimension_2,
            }) => {
                if arrayed != arrayed_2
                    || multisampled != multisampled_2
                    || dimension != dimension_2
                {
                    return Err(());
                }

                unify(database, table, sampled_ty, found_sampled_ty)?;

                Ok(())
            },
            _ => Err(()),
        },

        _ if expected == found => Ok(()),
        _ => Err(()),
    }
}

fn storage_type_of_texel_format(
    database: &dyn HirDatabase,
    format: TexelFormat,
) -> Type {
    let channel_type = match format {
        TexelFormat::Rgba8unorm
        | TexelFormat::Rgba8snorm
        | TexelFormat::Rgba16float
        | TexelFormat::Rgba32float
        | TexelFormat::R32float
        | TexelFormat::Rg32float => ScalarType::F32,
        TexelFormat::Rgba8sint
        | TexelFormat::Rgba16sint
        | TexelFormat::Rgba32sint
        | TexelFormat::R32sint
        | TexelFormat::Rg32sint => ScalarType::I32,
        TexelFormat::Rgba8uint
        | TexelFormat::Rgba16uint
        | TexelFormat::Rgba32uint
        | TexelFormat::R32uint
        | TexelFormat::Rg32uint => ScalarType::U32,
        TexelFormat::BoundVar(_) => unreachable!(),
        TexelFormat::Any => unreachable!(),
    };
    TyKind::Vector(VectorType {
        size: VecSize::Four,
        inner: TyKind::Scalar(channel_type).intern(database),
    })
    .intern(database)
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
            inner: r#type,
            address_space,
            access_mode,
        }))
    }

    fn ref_to_pointer(
        &self,
        reference: Reference,
    ) -> Type {
        self.database.intern_ty(TyKind::Pointer(Pointer {
            inner: reference.inner,
            address_space: reference.address_space,
            access_mode: reference.access_mode,
        }))
    }

    fn ptr_to_ref(
        &self,
        pointer: Pointer,
    ) -> Type {
        self.database.intern_ty(TyKind::Reference(Reference {
            inner: pointer.inner,
            address_space: pointer.address_space,
            access_mode: pointer.access_mode,
        }))
    }

    fn error_ty(&self) -> Type {
        self.database.intern_ty(TyKind::Error)
    }

    fn bool_ty(&self) -> Type {
        self.database.intern_ty(TyKind::Scalar(ScalarType::Bool))
    }

    fn try_lower_ty(
        &mut self,
        type_ref: &TypeReference,
    ) -> Result<Type, TypeLoweringError> {
        TyLoweringContext::new(self.database, &self.resolver).try_lower_ty(type_ref)
    }

    fn lower_ty(
        &mut self,
        container: impl Into<TypeContainer>,
        type_ref: &TypeReference,
    ) -> Type {
        match self.try_lower_ty(type_ref) {
            Ok(r#type) => r#type,
            Err(error) => {
                self.push_diagnostic(InferenceDiagnostic::InvalidType {
                    container: container.into(),
                    error,
                });
                self.error_ty()
            },
        }
    }
}

pub struct TyLoweringContext<'database> {
    database: &'database dyn HirDatabase,
    resolver: &'database Resolver,

    pub diagnostics: Vec<TypeLoweringError>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TypeLoweringError {
    UnresolvedName(Name),
    InvalidTexelFormat(String),
}

impl std::fmt::Display for TypeLoweringError {
    fn fmt(
        &self,
        #[expect(clippy::min_ident_chars, reason = "trait impl")] f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::UnresolvedName(name) => {
                write!(f, "type `{}` not found in scope", name.as_str())
            },
            Self::InvalidTexelFormat(format) => {
                let all_formats = "rgba8unorm,\nrgba8snorm,\nrgba8uint,\nrgba8sint,\nrgba16uint,\nrgba16sint,\nrgba16float,\nr32uint,\nr32sint,\nr32float,\nrg32uint,\nrg32sint,\nrg32float,\nrgba32uint,\nrgba32sint,\nrgba32float";
                write!(
                    f,
                    "`{format}` is not a valid texel format, expected one of:\n{all_formats}"
                )
            },
        }
    }
}

impl<'database> TyLoweringContext<'database> {
    pub fn new(
        database: &'database dyn HirDatabase,
        resolver: &'database Resolver,
    ) -> Self {
        Self {
            database,
            resolver,
            diagnostics: Vec::new(),
        }
    }

    pub fn lower_ty(
        &mut self,
        type_ref: &TypeReference,
    ) -> Type {
        self.try_lower_ty(type_ref)
            .unwrap_or_else(|_| TyKind::Error.intern(self.database))
    }

    pub fn try_lower_ty(
        &mut self,
        type_ref: &TypeReference,
    ) -> Result<Type, TypeLoweringError> {
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
                inner: self.lower_ty(&vec.inner),
            }),
            TypeReference::Matrix(matrix) => TyKind::Matrix(MatrixType {
                columns: matrix.columns.into(),
                rows: matrix.rows.into(),
                inner: self.lower_ty(&matrix.inner),
            }),
            TypeReference::Texture(tex) => TyKind::Texture(TextureType {
                dimension: match tex.dimension {
                    type_ref::TextureDimension::D1 => TextureDimensionality::D1,
                    type_ref::TextureDimension::D2 => TextureDimensionality::D2,
                    type_ref::TextureDimension::D3 => TextureDimensionality::D3,
                    type_ref::TextureDimension::Cube => TextureDimensionality::Cube,
                },
                arrayed: tex.arrayed,
                multisampled: tex.multisampled,
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
            }),
            TypeReference::Sampler(sampler) => TyKind::Sampler(SamplerType {
                comparison: sampler.comparison,
            }),
            TypeReference::Atomic(atomic) => TyKind::Atomic(AtomicType {
                inner: self.lower_ty(&atomic.inner),
            }),
            TypeReference::Array(array) => TyKind::Array(ArrayType {
                binding_array: array.binding_array,
                inner: self.lower_ty(&array.inner),
                size: match array.size {
                    type_ref::ArraySize::Int(i) => ArraySize::Constant(i as u64), // TODO error
                    type_ref::ArraySize::Uint(u) => ArraySize::Constant(u),
                    type_ref::ArraySize::Path(_) => ArraySize::Constant(0), // TODO: Path array sizes
                    type_ref::ArraySize::Dynamic => ArraySize::Dynamic,
                },
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
                    let data = self.database.type_alias_data(alias);
                    let type_ref = &self.database.lookup_intern_type_ref(data.r#type);

                    return Ok(self.lower_ty(type_ref));
                },
                Some(ResolveType::PredeclaredTypeAlias(type_ref)) => {
                    return Ok(self.lower_ty(&type_ref));
                },
                None => return Err(TypeLoweringError::UnresolvedName(name.clone())),
            },
        };
        Ok(self.database.intern_ty(ty_kind))
    }
}
