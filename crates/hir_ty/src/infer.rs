use either::Either;
use hir_def::{
    body::{BindingId, Body},
    data::{FieldId, FunctionData, GlobalConstantData, GlobalVariableData, OverrideData},
    db::{DefWithBodyId, FunctionId, GlobalConstantId, GlobalVariableId, OverrideId, TypeAliasId},
    expr::{ArithOp, BinaryOp, Callee, CmpOp, Expr, ExprId, Statement, StatementId, UnaryOp},
    module_data::Name,
    resolver::{ResolveType, Resolver},
    type_ref::{self, AccessMode, StorageClass, TypeRef, VecDimensionality},
};
use la_arena::ArenaMap;
use rustc_hash::FxHashMap;
use std::{collections::hash_map::Entry, sync::Arc};

use crate::{
    builtins::{Builtin, BuiltinId, BuiltinOverload, BuiltinOverloadId},
    function::{FunctionDetails, ResolvedFunctionId},
    ty::{
        ArraySize, ArrayType, AtomicType, BoundVar, MatrixType, Ptr, Ref, SamplerType, ScalarType,
        TexelFormat, TextureDimensionality, TextureKind, TextureType, Ty, TyKind, VecSize,
        VectorType,
    },
    HirDatabase,
};

pub fn infer_query(db: &dyn HirDatabase, def: DefWithBodyId) -> Arc<InferenceResult> {
    let resolver = def.resolver(db.upcast());
    let mut ctx = InferenceContext::new(db, def, resolver);

    match def {
        DefWithBodyId::Function(f) => ctx.collect_fn(f, &db.fn_data(f)),
        DefWithBodyId::GlobalVariable(var) => {
            ctx.collect_global_variable(var, &db.global_var_data(var))
        }
        DefWithBodyId::GlobalConstant(constant) => {
            ctx.collect_global_constant(constant, &db.global_constant_data(constant))
        }
        DefWithBodyId::Override(override_decl) => {
            ctx.collect_override(override_decl, &db.override_data(override_decl))
        }
    }

    ctx.infer_body();

    Arc::new(ctx.resolve_all())
}

#[derive(PartialEq, Eq, Debug)]
pub enum InferenceDiagnostic {
    AssignmentNotAReference {
        lhs: ExprId,
        actual: Ty,
    },
    TypeMismatch {
        expr: ExprId,
        expected: TypeExpectation,
        actual: Ty,
    },
    NoSuchField {
        expr: ExprId,
        name: Name,
        ty: Ty,
    },
    ArrayAccessInvalidType {
        expr: ExprId,
        ty: Ty,
    },
    UnresolvedName {
        expr: ExprId,
        name: Name,
    },
    InvalidConstructionType {
        expr: ExprId,
        ty: Ty,
    },
    FunctionCallArgCountMismatch {
        expr: ExprId,
        n_expected: usize,
        n_actual: usize,
    },
    NoBuiltinOverload {
        expr: ExprId,
        builtin: BuiltinId,
        name: Option<&'static str>,
        parameters: Vec<Ty>,
    },
    NoConstructor {
        expr: ExprId,
        builtins: [BuiltinId; 2],
        ty: Ty,
        parameters: Vec<Ty>,
    },

    AddrOfNotRef {
        expr: ExprId,
        actual: Ty,
    },
    DerefNotAPtr {
        expr: ExprId,
        actual: Ty,
    },

    InvalidType {
        container: TypeContainer,
        error: TypeLoweringError,
    },
}

#[derive(PartialEq, Eq, Debug)]
pub enum TypeContainer {
    Expr(ExprId),
    GlobalVar(GlobalVariableId),
    GlobalConstant(GlobalConstantId),
    Override(OverrideId),
    TypeAlias(TypeAliasId),
    FunctionParameter(FunctionId, BindingId),
    FunctionReturn(FunctionId),
    VariableStatement(StatementId),
}

impl From<ExprId> for TypeContainer {
    fn from(id: ExprId) -> Self {
        TypeContainer::Expr(id)
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum ResolvedCall {
    Function(ResolvedFunctionId),
    OtherTypeInitializer(Ty),
}

#[derive(Default, PartialEq, Eq, Debug)]
pub struct InferenceResult {
    pub type_of_expr: ArenaMap<ExprId, Ty>,
    pub type_of_binding: ArenaMap<BindingId, Ty>,
    pub diagnostics: Vec<InferenceDiagnostic>,
    pub return_type: Option<Ty>,
    call_resolutions: FxHashMap<ExprId, ResolvedCall>,
    field_resolutions: FxHashMap<ExprId, FieldId>,
}

impl InferenceResult {
    pub fn field_resolution(&self, expr: ExprId) -> Option<FieldId> {
        self.field_resolutions.get(&expr).copied()
    }
    pub fn call_resolution(&self, expr: ExprId) -> Option<ResolvedCall> {
        self.call_resolutions.get(&expr).copied()
    }
}

pub struct InferenceContext<'db> {
    db: &'db dyn HirDatabase,
    owner: DefWithBodyId,
    resolver: Resolver,
    body: Arc<Body>,
    result: InferenceResult,
    return_ty: Option<Ty>,
}

impl<'db> InferenceContext<'db> {
    pub fn new(db: &'db dyn HirDatabase, owner: DefWithBodyId, resolver: Resolver) -> Self {
        Self {
            db,
            owner,
            resolver,
            body: db.body(owner),
            result: InferenceResult::default(),
            return_ty: None,
        }
    }

    fn set_expr_ty(&mut self, expr: ExprId, ty: Ty) {
        self.result.type_of_expr.insert(expr, ty);
    }
    fn set_binding_ty(&mut self, binding: BindingId, ty: Ty) {
        self.result.type_of_binding.insert(binding, ty);
    }
    fn set_field_resolution(&mut self, expr: ExprId, field: FieldId) {
        self.result.field_resolutions.insert(expr, field);
    }
    fn push_diagnostic(&mut self, diagnostic: InferenceDiagnostic) {
        self.result.diagnostics.push(diagnostic);
    }

    fn resolve_all(mut self) -> InferenceResult {
        self.result.return_type = self.return_ty;
        self.result
    }

    fn collect_global_variable(&mut self, id: GlobalVariableId, var: &GlobalVariableData) {
        let ty = var.ty.map(|ty| {
            self.lower_ty(
                TypeContainer::GlobalVar(id),
                &self.db.lookup_intern_type_ref(ty),
            )
        });

        if let Some(ty) = ty {
            if let Some(binding) = self.body.main_binding {
                self.set_binding_ty(binding, ty);
            }
        }

        self.return_ty = ty;
    }
    fn collect_global_constant(&mut self, id: GlobalConstantId, constant: &GlobalConstantData) {
        let ty = constant.ty.map(|ty| {
            self.lower_ty(
                TypeContainer::GlobalConstant(id),
                &self.db.lookup_intern_type_ref(ty),
            )
        });

        if let Some(ty) = ty {
            if let Some(binding) = self.body.main_binding {
                self.set_binding_ty(binding, ty);
            }
        }

        self.return_ty = ty;
    }
    fn collect_override(&mut self, id: OverrideId, constant: &OverrideData) {
        let ty = constant.ty.map(|ty| {
            self.lower_ty(
                TypeContainer::Override(id),
                &self.db.lookup_intern_type_ref(ty),
            )
        });

        if let Some(ty) = ty {
            if let Some(binding) = self.body.main_binding {
                self.set_binding_ty(binding, ty);
            }
        }

        self.return_ty = ty;
    }

    fn collect_fn(&mut self, function_id: FunctionId, f: &FunctionData) {
        let body = Arc::clone(&self.body);
        for (&(param, _), &id) in f.params.iter().zip(&body.params) {
            let type_ref = self.db.lookup_intern_type_ref(param);
            let param_ty =
                self.lower_ty(TypeContainer::FunctionParameter(function_id, id), &type_ref);
            self.set_binding_ty(id, param_ty);
        }
        self.return_ty = f.return_type.map(|type_ref| {
            self.lower_ty(
                TypeContainer::FunctionReturn(function_id),
                &self.db.lookup_intern_type_ref(type_ref),
            )
        });
    }

    fn infer_body(&mut self) {
        match self.body.root {
            Some(Either::Left(stmt)) => {
                self.infer_stmt(stmt);
            }
            Some(Either::Right(expr)) => {
                let ty = self.infer_expr_expect(expr, TypeExpectation::from_option(self.return_ty));
                if self.return_ty.is_none() {
                    self.return_ty = Some(ty);
                }

                if let Some(main_binding) = self.body.main_binding {
                    self.set_binding_ty(main_binding, ty);
                }
            }
            None => (),
        }
    }

    fn resolver_for_expr(&self, expr: ExprId) -> Resolver {
        let resolver = self.resolver.clone();
        match self.owner {
            DefWithBodyId::Function(function) => {
                let expr_scopes = self.db.expr_scopes(self.owner);
                let scope_id = expr_scopes.scope_for_expr(expr).unwrap();
                resolver.push_expr_scope(function, expr_scopes, scope_id)
            }
            DefWithBodyId::GlobalVariable(_) => resolver,
            DefWithBodyId::GlobalConstant(_) => resolver,
            DefWithBodyId::Override(_) => resolver,
        }
    }

    fn infer_stmt(&mut self, stmt: StatementId) {
        let body = Arc::clone(&self.body);

        match body.statements[stmt] {
            Statement::Missing => {}
            Statement::Compound { ref statements } => {
                for stmt in statements {
                    self.infer_stmt(*stmt);
                }
            }
            Statement::VariableStatement {
                binding_id,
                type_ref,
                initializer,
                storage_class,
                access_mode,
            } => {
                let ty = type_ref.map(|ty| {
                    self.lower_ty(
                        TypeContainer::VariableStatement(stmt),
                        &self.db.lookup_intern_type_ref(ty),
                    )
                });
                let ty = if let Some(init) = initializer {
                    let expr_ty = self.infer_expr_expect(init, TypeExpectation::from_option(ty));
                    ty.unwrap_or(expr_ty)
                } else {
                    ty.unwrap_or_else(|| self.err_ty())
                };

                let ref_ty = self.make_ref(
                    ty,
                    storage_class.unwrap_or(StorageClass::Function),
                    access_mode.unwrap_or_else(AccessMode::read_write),
                );
                self.set_binding_ty(binding_id, ref_ty)
            }
            Statement::ConstStatement {
                binding_id,
                type_ref,
                initializer,
                ..
            } => {
                let ty = type_ref.map(|ty| {
                    self.lower_ty(
                        TypeContainer::VariableStatement(stmt),
                        &self.db.lookup_intern_type_ref(ty),
                    )
                });
                let ty = if let Some(init) = initializer {
                    let expr_ty = self.infer_expr_expect(init, TypeExpectation::from_option(ty));
                    ty.unwrap_or(expr_ty)
                } else {
                    ty.unwrap_or_else(|| self.err_ty())
                };

                self.set_binding_ty(binding_id, ty)
            }
            Statement::LetStatement {
                binding_id,
                type_ref,
                initializer,
                ..
            } => {
                let ty = type_ref.map(|ty| {
                    self.lower_ty(
                        TypeContainer::VariableStatement(stmt),
                        &self.db.lookup_intern_type_ref(ty),
                    )
                });
                let ty = if let Some(init) = initializer {
                    let expr_ty = self.infer_expr_expect(init, TypeExpectation::from_option(ty));
                    ty.unwrap_or(expr_ty)
                } else {
                    ty.unwrap_or_else(|| self.err_ty())
                };

                self.set_binding_ty(binding_id, ty)
            }

            Statement::Return { expr } => {
                if let Some(expr) = expr {
                    self.infer_expr_expect(expr, TypeExpectation::from_option(self.return_ty));
                }
            }
            Statement::Assignment { lhs, rhs } => {
                let lhs_ty = self.infer_expr(lhs);

                let kind = lhs_ty.kind(self.db);
                let lhs_inner = match kind {
                    TyKind::Ref(r) => r.inner,
                    _ => {
                        self.push_diagnostic(InferenceDiagnostic::AssignmentNotAReference {
                            lhs,
                            actual: lhs_ty,
                        });
                        self.err_ty()
                    }
                };

                self.infer_expr_expect(rhs, TypeExpectation::from_ty(lhs_inner));
            }
            Statement::CompoundAssignment { lhs, rhs, op } => {
                let lhs_ty = self.infer_expr(lhs);

                let lhs_kind = lhs_ty.kind(self.db);
                let lhs_inner = match lhs_kind {
                    TyKind::Ref(r) => r.inner,
                    _ => {
                        self.push_diagnostic(InferenceDiagnostic::AssignmentNotAReference {
                            lhs,
                            actual: lhs_ty,
                        });
                        self.err_ty()
                    }
                };

                let ty = self.infer_binary_op(lhs, rhs, op.into());

                self.expect_same_type(lhs, ty, lhs_inner);
            }
            Statement::IncrDecr { expr, .. } => {
                let lhs_ty = self.infer_expr(expr);

                let lhs_kind = lhs_ty.kind(self.db);
                let lhs_inner = match lhs_kind {
                    TyKind::Ref(r) => r.inner,
                    _ => {
                        self.push_diagnostic(InferenceDiagnostic::AssignmentNotAReference {
                            lhs: expr,
                            actual: lhs_ty,
                        });
                        self.err_ty()
                    }
                };

                if self
                    .expect_ty_inner(lhs_inner, &TypeExpectationInner::IntegerScalar)
                    .is_err()
                {
                    self.push_diagnostic(InferenceDiagnostic::TypeMismatch {
                        expr,
                        actual: lhs_inner,
                        expected: TypeExpectation::Type(TypeExpectationInner::IntegerScalar),
                    });
                }
            }
            Statement::If {
                condition,
                block,
                ref else_if_blocks,
                else_block,
            } => {
                self.infer_stmt(block);
                for else_if_block in else_if_blocks {
                    self.infer_stmt(*else_if_block);
                }
                if let Some(else_block) = else_block {
                    self.infer_stmt(else_block);
                }
                self.infer_expr_expect(condition, TypeExpectation::from_ty(self.bool_ty()));
            }
            Statement::While { condition, block } => {
                self.infer_stmt(block);
                self.infer_expr_expect(condition, TypeExpectation::from_ty(self.bool_ty()));
            }
            Statement::Switch {
                expr,
                ref case_blocks,
                ref default_block,
            } => {
                let ty = self.infer_expr(expr).unref(self.db);

                for (selectors, case) in case_blocks {
                    for selector in selectors {
                        self.infer_expr_expect(*selector, TypeExpectation::from_ty(ty));
                    }
                    self.infer_stmt(*case);
                }

                if let Some(default_block) = *default_block {
                    self.infer_stmt(default_block);
                }
            }
            Statement::For {
                initializer,
                condition,
                continuing_part,
                block,
            } => {
                if let Some(init) = initializer {
                    self.infer_stmt(init);
                }
                if let Some(cont) = continuing_part {
                    self.infer_stmt(cont);
                }

                if let Some(condition) = condition {
                    self.infer_expr_expect(condition, TypeExpectation::from_ty(self.bool_ty()));
                }

                self.infer_stmt(block);
            }
            Statement::Loop { body } => {
                self.infer_stmt(body);
            }
            Statement::Discard => {}
            Statement::Break => {}
            Statement::Continue => {}
            Statement::Continuing { block } => self.infer_stmt(block),
            Statement::Expr { expr } => {
                self.infer_expr(expr);
            }
        }
    }

    fn expect_ty_inner(&mut self, ty: Ty, expectation: &TypeExpectationInner) -> Result<(), ()> {
        let ty_kind = ty.kind(self.db);
        if let TyKind::Error = ty_kind {
            return Ok(());
        };

        match *expectation {
            TypeExpectationInner::Exact(expected_type) => match expected_type.kind(self.db) {
                TyKind::Error => Ok(()),
                _ => {
                    if ty == expected_type {
                        Ok(())
                    } else {
                        Err(())
                    }
                }
            },
            TypeExpectationInner::I32OrF32 => match ty.kind(self.db).unref(self.db).as_ref() {
                TyKind::Scalar(ScalarType::I32 | ScalarType::F32) => Ok(()),
                _ => Err(()),
            },
            TypeExpectationInner::NumericScalar => match ty.kind(self.db).unref(self.db).as_ref() {
                TyKind::Scalar(ScalarType::I32 | ScalarType::F32 | ScalarType::U32) => Ok(()),
                _ => Err(()),
            },
            TypeExpectationInner::IntegerScalar => match ty.kind(self.db).unref(self.db).as_ref() {
                TyKind::Scalar(ScalarType::I32 | ScalarType::U32) => Ok(()),
                _ => Err(()),
            },
        }
    }

    fn expect_same_type(&mut self, expr: ExprId, expected: Ty, actual: Ty) {
        let actual_unref = actual.unref(self.db);
        if expected != actual_unref {
            self.push_diagnostic(InferenceDiagnostic::TypeMismatch {
                expr,
                actual: actual_unref,
                expected: TypeExpectation::Type(TypeExpectationInner::Exact(expected)),
            });
        }
    }

    fn infer_expr_expect(&mut self, expr: ExprId, expected: TypeExpectation) -> Ty {
        let ty = self.infer_expr(expr).unref(self.db);

        match &expected {
            TypeExpectation::Type(expected_type) => match self.expect_ty_inner(ty, expected_type) {
                Ok(_) => ty,
                Err(_) => {
                    self.push_diagnostic(InferenceDiagnostic::TypeMismatch {
                        expr,
                        actual: ty,
                        expected: expected.clone(),
                    });
                    ty
                }
            },
            TypeExpectation::TypeOrVecOf(ref expect) => {
                match self.expect_ty_inner(ty.this_or_vec_inner(self.db), expect) {
                    Ok(_) => ty,
                    Err(_) => {
                        self.push_diagnostic(InferenceDiagnostic::TypeMismatch {
                            expr,
                            actual: ty,
                            expected: expected.clone(),
                        });
                        ty
                    }
                }
            }
            TypeExpectation::None => ty,
        }
    }

    fn infer_expr(&mut self, expr: ExprId) -> Ty {
        let body = Arc::clone(&self.body);
        let ty = match body.exprs[expr] {
            Expr::Missing => self.err_ty(),
            Expr::BinaryOp { lhs, rhs, op } => self.infer_binary_op(lhs, rhs, op),
            Expr::UnaryOp { expr, op } => self.infer_unary_op(expr, op),
            Expr::Field {
                expr: field_expr,
                ref name,
            } => {
                let expr_ty = self.infer_expr(field_expr);
                if expr_ty.is_err(self.db) {
                    return self.err_ty();
                }

                match *expr_ty.kind(self.db).unref(self.db).as_ref() {
                    TyKind::Struct(strukt) => {
                        let struct_data = self.db.struct_data(strukt);
                        let field_types = self.db.field_types(strukt);

                        match struct_data.field(name) {
                            Some(field) => {
                                self.set_field_resolution(expr, FieldId { strukt, field });

                                let field_ty = field_types[field];
                                // TODO: correct storage class/access mode
                                self.make_ref(
                                    field_ty,
                                    StorageClass::Private,
                                    AccessMode::read_write(),
                                )
                            }
                            None => {
                                self.push_diagnostic(InferenceDiagnostic::NoSuchField {
                                    expr: field_expr,
                                    name: name.clone(),
                                    ty: expr_ty,
                                });
                                self.err_ty()
                            }
                        }
                    }
                    TyKind::Vector(ref vec_type) => match self.vec_swizzle(vec_type, name) {
                        Ok(ty) => ty,
                        Err(_) => {
                            self.push_diagnostic(InferenceDiagnostic::NoSuchField {
                                expr: field_expr,
                                name: name.clone(),
                                ty: expr_ty,
                            });
                            self.err_ty()
                        }
                    },
                    TyKind::Matrix(_) => {
                        self.push_diagnostic(InferenceDiagnostic::NoSuchField {
                            expr: field_expr,
                            name: name.clone(),
                            ty: expr_ty,
                        });
                        self.err_ty()
                    }
                    _ => {
                        self.push_diagnostic(InferenceDiagnostic::NoSuchField {
                            expr: field_expr,
                            name: name.clone(),
                            ty: expr_ty,
                        });
                        self.err_ty()
                    }
                }
            }
            Expr::Call {
                ref callee,
                ref args,
            } => {
                let args: Vec<_> = args
                    .iter()
                    .map(|&arg| self.infer_expr(arg).unref(self.db))
                    .collect();
                self.infer_call(expr, callee, args)
            }
            Expr::Bitcast { ty, expr } => {
                self.infer_expr(expr);
                
                self
                    .try_lower_ty(&self.db.lookup_intern_type_ref(ty))
                    .unwrap_or_else(|_| self.err_ty())
            }
            Expr::Index { lhs, index } => {
                let lhs = self.infer_expr(lhs);
                let _index_expr = self.infer_expr(index);
                // TODO check index expr

                let lhs_kind = lhs.kind(self.db);
                let is_ref = matches!(lhs_kind, TyKind::Ref(_));

                let lhs_inner = lhs_kind.unref(self.db);

                let ty = match &*lhs_inner {
                    TyKind::Vector(vec) => {
                        // TODO out of bounds
                        vec.inner
                    }
                    TyKind::Matrix(mat) => {
                        // TODO out of bounds
                        self.db.intern_ty(TyKind::Vector(VectorType {
                            inner: mat.inner,
                            size: mat.rows,
                        }))
                    }
                    TyKind::Array(array) => {
                        // TODO out of bounds
                        array.inner
                    }
                    _ => {
                        self.push_diagnostic(InferenceDiagnostic::ArrayAccessInvalidType {
                            expr,
                            ty: lhs,
                        });
                        self.err_ty()
                    }
                };

                match is_ref {
                    true => self.make_ref(ty, StorageClass::Private, AccessMode::read_write()), // TODO use correct
                    false => ty,
                }
            }
            Expr::Literal(ref lit) => {
                let ty_kind = match lit {
                    hir_def::expr::Literal::Int(_, _) => TyKind::Scalar(ScalarType::I32),
                    hir_def::expr::Literal::Uint(_, _) => TyKind::Scalar(ScalarType::U32),
                    hir_def::expr::Literal::Float(_, _) => TyKind::Scalar(ScalarType::F32),
                    hir_def::expr::Literal::Bool(_) => TyKind::Scalar(ScalarType::Bool),
                };
                self.db.intern_ty(ty_kind)
            }
            Expr::Path(ref name) => self.resolve_path_expr(expr, name).unwrap_or_else(|| {
                self.push_diagnostic(InferenceDiagnostic::UnresolvedName {
                    expr,
                    name: name.clone(),
                });
                self.err_ty()
            }),
        };

        self.set_expr_ty(expr, ty);

        ty
    }

    fn validate_function_call(
        &mut self,
        f: &FunctionDetails,
        args: Vec<Ty>,
        callee: ExprId,
        expr: ExprId,
    ) -> Ty {
        if f.parameters.len() != args.len() {
            self.push_diagnostic(InferenceDiagnostic::FunctionCallArgCountMismatch {
                expr: callee,
                n_expected: f.parameters.len(),
                n_actual: args.len(),
            });
            self.err_ty()
        } else {
            for (expected, actual) in f.parameters().zip(args.iter().copied()) {
                self.expect_same_type(expr, expected, actual);
            }

            f.return_type.unwrap_or_else(|| self.err_ty())
        }
    }

    fn infer_unary_op(&mut self, expr: ExprId, op: UnaryOp) -> Ty {
        let expr_ty = self.infer_expr(expr);
        if expr_ty.is_err(self.db) {
            return self.err_ty();
        }

        let builtin = match op {
            UnaryOp::Minus => Builtin::builtin_op_unary_minus(self.db).intern(self.db),
            UnaryOp::Not => Builtin::builtin_op_unary_not(self.db).intern(self.db),
            UnaryOp::BitNot => Builtin::builtin_op_unary_bitnot(self.db).intern(self.db),
            UnaryOp::Ref => {
                match expr_ty.kind(self.db) {
                    TyKind::Ref(reference) => return self.ref_to_ptr(reference),
                    _ => {
                        self.push_diagnostic(InferenceDiagnostic::AddrOfNotRef {
                            expr,
                            actual: expr_ty,
                        });
                        return self.err_ty();
                    }
                };
            }
            UnaryOp::Deref => {
                let arg_ty = expr_ty.unref(self.db);
                match arg_ty.kind(self.db) {
                    TyKind::Ptr(ptr) => return self.ptr_to_ref(ptr),
                    _ => {
                        self.push_diagnostic(InferenceDiagnostic::DerefNotAPtr {
                            expr,
                            actual: arg_ty,
                        });
                        return self.err_ty();
                    }
                }
            }
        };

        let arg_ty = expr_ty.unref(self.db);
        self.call_builtin(expr, builtin, &[arg_ty], Some(op.symbol()))
    }

    fn infer_binary_op(&mut self, lhs: ExprId, rhs: ExprId, op: BinaryOp) -> Ty {
        let lhs_ty = self.infer_expr(lhs).unref(self.db);
        let rhs_ty = self.infer_expr(rhs).unref(self.db);

        if lhs_ty.is_err(self.db) || rhs_ty.is_err(self.db) {
            return self.err_ty();
        }

        let builtin = match op {
            BinaryOp::LogicOp(_) => Builtin::builtin_op_binary_bool(self.db).intern(self.db),
            BinaryOp::ArithOp(op) => match op {
                ArithOp::BitOr | ArithOp::BitAnd | ArithOp::BitXor => {
                    Builtin::builtin_op_binary_bitop(self.db).intern(self.db)
                }
                ArithOp::Mul => Builtin::builtin_op_binary_mul(self.db).intern(self.db),
                ArithOp::Div => Builtin::builtin_op_binary_div(self.db).intern(self.db),
                ArithOp::Add | ArithOp::Sub | ArithOp::Modulo => {
                    Builtin::builtin_op_binary_number(self.db).intern(self.db)
                }
                ArithOp::Shl | ArithOp::Shr => {
                    Builtin::builtin_op_binary_shift(self.db).intern(self.db)
                }
            },
            BinaryOp::CmpOp(cmp) => match cmp {
                CmpOp::Eq { .. } => Builtin::builtin_op_eq(self.db).intern(self.db),
                CmpOp::Ord { .. } => Builtin::builtin_op_cmp(self.db).intern(self.db),
            },
        };

        self.call_builtin(lhs, builtin, &[lhs_ty, rhs_ty], Some(op.symbol()))
    }

    fn builtin_vector_inferred_constructor(&self, size: &VecDimensionality) -> BuiltinId {
        match size {
            VecDimensionality::Two => Builtin::builtin_op_vec2_constructor(self.db),
            VecDimensionality::Three => Builtin::builtin_op_vec3_constructor(self.db),
            VecDimensionality::Four => Builtin::builtin_op_vec4_constructor(self.db),
        }
        .intern(self.db)
    }
    fn builtin_matrix_inferred_constructor(
        &self,
        columns: &VecDimensionality,
        rows: &VecDimensionality,
    ) -> BuiltinId {
        use type_ref::VecDimensionality::*;
        match (columns, rows) {
            (Two, Two) => Builtin::builtin_op_mat2x2_constructor(self.db),
            (Two, Three) => Builtin::builtin_op_mat2x3_constructor(self.db),
            (Two, Four) => Builtin::builtin_op_mat2x4_constructor(self.db),
            (Three, Two) => Builtin::builtin_op_mat3x2_constructor(self.db),
            (Three, Three) => Builtin::builtin_op_mat3x3_constructor(self.db),
            (Three, Four) => Builtin::builtin_op_mat3x4_constructor(self.db),
            (Four, Two) => Builtin::builtin_op_mat4x2_constructor(self.db),
            (Four, Three) => Builtin::builtin_op_mat4x3_constructor(self.db),
            (Four, Four) => Builtin::builtin_op_mat4x4_constructor(self.db),
        }
        .intern(self.db)
    }

    fn resolve_path_expr(&self, expr: ExprId, path: &Name) -> Option<Ty> {
        self.resolve_path_expr_inner(expr, path)
    }
    fn resolve_path_expr_inner(&self, expr: ExprId, path: &Name) -> Option<Ty> {
        let resolver = self.resolver_for_expr(expr);
        let resolve = resolver.resolve_value(path)?;
        let ty = match resolve {
            hir_def::resolver::ResolveValue::Local(local) => {
                
                *self.result.type_of_binding.get(local)?
            }
            hir_def::resolver::ResolveValue::GlobalVariable(loc) => {
                let id = self.db.intern_global_variable(loc);
                let data = self.db.global_var_data(id);
                let result = self.db.infer(DefWithBodyId::GlobalVariable(id));
                let ty = result.return_type.unwrap_or_else(|| self.err_ty());
                // TODO use correct defaults
                self.make_ref(
                    ty,
                    data.storage_class.unwrap_or(StorageClass::Private),
                    AccessMode::read_write(),
                )
            }
            hir_def::resolver::ResolveValue::GlobalConstant(loc) => {
                let id = self.db.intern_global_constant(loc);
                let result = self.db.infer(DefWithBodyId::GlobalConstant(id));
                result.return_type.unwrap_or_else(|| self.err_ty())
            }
            hir_def::resolver::ResolveValue::Override(loc) => {
                let id = self.db.intern_override(loc);
                let result = self.db.infer(DefWithBodyId::Override(id));
                result.return_type.unwrap_or_else(|| self.err_ty())
            }
        };
        Some(ty)
    }

    fn ty_from_vec_size(&self, inner: Ty, vec_size: u8) -> Ty {
        if vec_size == 1 {
            inner
        } else {
            let kind = vec_size
                .try_into()
                .map(|size| TyKind::Vector(VectorType { inner, size }))
                .unwrap_or(TyKind::Error);
            self.db.intern_ty(kind)
        }
    }
    fn vec_swizzle(&self, vec_type: &VectorType, name: &Name) -> Result<Ty, ()> {
        const SWIZZLES: [[char; 4]; 2] = [['x', 'y', 'z', 'w'], ['r', 'g', 'b', 'a']];
        let max_size = 4;
        let max_swizzle_idx = vec_type.size.as_u8();

        if name.as_str().len() > max_size {
            return Err(());
        }

        for swizzle in &SWIZZLES {
            let allowed_chars = &swizzle[..max_swizzle_idx as usize];
            if name
                .as_str()
                .chars()
                .all(|char| allowed_chars.contains(&char))
            {
                let ty = self.ty_from_vec_size(vec_type.inner, name.as_str().len() as u8);
                let r = self.make_ref(ty, StorageClass::Function, AccessMode::read_write()); // TOOD is correct?
                return Ok(r);
            }
        }

        Err(())
    }

    fn call_builtin(
        &mut self,
        expr: ExprId,
        builtin_id: BuiltinId,
        args: &[Ty],
        name: Option<&'static str>,
    ) -> Ty {
        self.call_builtin_inner(expr, builtin_id, args, name, None)
    }

    fn call_builtin_with_return(
        &mut self,
        expr: ExprId,
        builtin_id: BuiltinId,
        args: &[Ty],
        name: Option<&'static str>,
        ty: Ty,
    ) -> Ty {
        self.call_builtin_inner(expr, builtin_id, args, name, Some(ty))
    }

    fn call_builtin_inner(
        &mut self,
        expr: ExprId,
        builtin_id: BuiltinId,
        args: &[Ty],
        name: Option<&'static str>,
        return_ty: Option<Ty>,
    ) -> Ty {
        if let Ok((return_ty, overload_id)) = self.try_call_builtin(builtin_id, args, return_ty) {
            let builtin = builtin_id.lookup(self.db);
            let resolved = builtin.overload(overload_id).ty;
            self.result
                .call_resolutions
                .insert(expr, ResolvedCall::Function(resolved));
            return_ty
        } else {
            self.push_diagnostic(InferenceDiagnostic::NoBuiltinOverload {
                expr,
                builtin: builtin_id,
                name,
                parameters: args.to_vec(),
            });
            self.err_ty()
        }
    }

    fn try_call_builtin(
        &mut self,
        builtin_id: BuiltinId,
        args: &[Ty],
        return_ty: Option<Ty>,
    ) -> Result<(Ty, BuiltinOverloadId), ()> {
        let builtin = builtin_id.lookup(self.db);
        for (overload_id, overload) in builtin.overloads() {
            if let Ok(ty) = self.call_builtin_overload(overload, args) {
                if let Some(ret) = return_ty {
                    if ret == ty {
                        return Ok((ty, overload_id));
                    }
                } else {
                    return Ok((ty, overload_id));
                }
            }
        }
        Err(())
    }

    fn call_builtin_overload(&self, sig: &BuiltinOverload, args: &[Ty]) -> Result<Ty, ()> {
        let fn_ty = sig.ty.lookup(self.db);

        if fn_ty.parameters.len() != args.len() {
            return Err(());
        }

        let mut unification_table = UnificationTable::default();
        for (expected, &found) in fn_ty.parameters().zip(args.iter()) {
            unify(self.db, &mut unification_table, expected, found)?;
        }

        let return_type = fn_ty
            .return_type
            .map(|ty| unification_table.resolve(self.db, ty));

        Ok(return_type.unwrap_or_else(|| self.err_ty()))
    }

    fn infer_call(&mut self, expr: ExprId, callee: &Callee, args: Vec<Ty>) -> Ty {
        match callee {
            Callee::InferredComponentMatrix { rows, columns } => {
                let builtin_id = self.builtin_matrix_inferred_constructor(columns, rows);
                
                self.call_builtin(expr, builtin_id, &args, Some("matrix construction"))
            }
            Callee::InferredComponentVec(size) => {
                let builtin_id = self.builtin_vector_inferred_constructor(size);
                
                self.call_builtin(expr, builtin_id, &args, Some("vec construction"))
            }
            Callee::InferredComponentArray => {
                let builtin_id = Builtin::builtin_op_array_constructor(self.db).intern(self.db);
                // TODO: Special case calling array initialisers to allow n-ary calls
                
                self.call_builtin(expr, builtin_id, &args, Some("array construction"))
            }
            Callee::Name(name) => match self.resolver.resolve_callable(name) {
                Some(arg) => match arg {
                    hir_def::resolver::ResolveCallable::Struct(loc) => {
                        let strukt = self.db.intern_struct(loc);
                        let kind = TyKind::Struct(strukt);
                        let ty = self.db.intern_ty(kind);
                        self.check_ty_initialiser(expr, ty, args);
                        ty
                    }
                    hir_def::resolver::ResolveCallable::TypeAlias(alias) => {
                        let alias = self.db.intern_type_alias(alias);
                        let data = self.db.type_alias_data(alias);
                        let type_ref = self.db.lookup_intern_type_ref(data.ty);

                        let ty = self.lower_ty(TypeContainer::TypeAlias(alias), &type_ref);
                        self.check_ty_initialiser(expr, ty, args);
                        ty
                    }
                    hir_def::resolver::ResolveCallable::Function(loc) => {
                        let id = self.db.intern_function(loc);
                        let resolved = self.db.function_type(id);
                        let details = resolved.lookup(self.db);
                        self.result
                            .call_resolutions
                            .insert(expr, ResolvedCall::Function(resolved));
                        self.validate_function_call(&details, args, expr, expr)
                    }
                    hir_def::resolver::ResolveCallable::PredeclaredTypeAlias(type_ref) => {
                        let ty = self.lower_ty(expr, &type_ref);
                        self.check_ty_initialiser(expr, ty, args);
                        ty
                    }
                },
                None => {
                    let builtin = Builtin::for_name(self.db, name);
                    if let Some(builtin) = builtin {
                        let builtin_id = builtin.intern(self.db);
                        self.call_builtin(expr, builtin_id, &args, None)
                    } else {
                        self.push_diagnostic(InferenceDiagnostic::UnresolvedName {
                            expr,
                            name: name.clone(),
                        });
                        self.err_ty()
                    }
                }
            },
            Callee::Type(ty) => {
                let ty = self.lower_ty(expr, &self.db.lookup_intern_type_ref(*ty));
                self.check_ty_initialiser(expr, ty, args);
                // A type initialiser always returns just the returned type
                ty
            }
        }
    }
    fn check_ty_initialiser(&mut self, expr: ExprId, ty: Ty, args: Vec<Ty>) {
        fn size_to_dimension(size: VecSize) -> VecDimensionality {
            match size {
                VecSize::Two => VecDimensionality::Two,
                VecSize::Three => VecDimensionality::Three,
                VecSize::Four => VecDimensionality::Four,
                VecSize::BoundVar(_) => unreachable!("Can never have unbound type at this point"),
            }
        }

        match ty.kind(self.db) {
            TyKind::Scalar(_) => {
                if args.is_empty() {
                    // Permit the zero value
                    return;
                }
                let builtin = Builtin::builtin_op_convert(self.db).intern(self.db);
                self.call_builtin_with_return(expr, builtin, &args, Some("conversion"), ty);
            }
            TyKind::Array(_) => {
                if args.is_empty() {
                }
                // TODO: Implement checking that all the arguments have the same type (inner)
            }
            TyKind::Vector(vec) => {
                if args.is_empty() {
                    return;
                }
                let construction_builtin_id =
                    self.builtin_vector_inferred_constructor(&size_to_dimension(vec.size));
                let construction_result =
                    self.try_call_builtin(construction_builtin_id, &args, Some(ty));
                if construction_result.is_ok() {
                    return;
                }
                let conversion_id = Builtin::builtin_op_convert(self.db).intern(self.db);
                let conversion_result = self.try_call_builtin(conversion_id, &args, Some(ty));
                if conversion_result.is_ok() {
                    return;
                }
                self.push_diagnostic(InferenceDiagnostic::NoConstructor {
                    expr,
                    builtins: [construction_builtin_id, conversion_id],
                    ty,
                    parameters: args,
                })
            }
            TyKind::Matrix(matrix) => {
                if args.is_empty() {
                    return;
                }
                let construction_builtin_id = self.builtin_matrix_inferred_constructor(
                    &size_to_dimension(matrix.columns),
                    &size_to_dimension(matrix.rows),
                );
                let construction_result =
                    self.try_call_builtin(construction_builtin_id, &args, Some(ty));
                if construction_result.is_ok() {
                    return;
                }
                let conversion_id = Builtin::builtin_op_convert(self.db).intern(self.db);
                let conversion_result = self.try_call_builtin(conversion_id, &args, Some(ty));
                if conversion_result.is_ok() {
                    return;
                }
                self.push_diagnostic(InferenceDiagnostic::NoConstructor {
                    expr,
                    builtins: [construction_builtin_id, conversion_id],
                    ty,
                    parameters: args,
                })
            }
            TyKind::Struct(_) => {
                if args.is_empty() {
                }
                // TODO: Implement checking field types
            }

            // Never constructible
            TyKind::Texture(_)
            | TyKind::Sampler(_)
            | TyKind::Ptr(_)
            | TyKind::Atomic(_)
            | TyKind::StorageTypeOfTexelFormat(_) => {
                self.push_diagnostic(InferenceDiagnostic::InvalidConstructionType { expr, ty })
            }
            TyKind::BoundVar(_) | TyKind::Ref(_) => unreachable!(),
            TyKind::Error => {}
        }
    }
}

#[derive(Default)]
struct UnificationTable {
    type_vars: FxHashMap<BoundVar, Ty>,
    vec_size_vars: FxHashMap<BoundVar, VecSize>,
    texel_format_vars: FxHashMap<BoundVar, TexelFormat>,
}
impl UnificationTable {
    fn set_vec_size(&mut self, var: BoundVar, vec_size: VecSize) -> Result<(), ()> {
        match self.vec_size_vars.entry(var) {
            Entry::Occupied(entry) if *entry.get() == vec_size => Ok(()),
            Entry::Occupied(_) => Err(()),
            Entry::Vacant(entry) => {
                entry.insert(vec_size);
                Ok(())
            }
        }
    }
    fn set_type(&mut self, var: BoundVar, ty: Ty) -> Result<(), ()> {
        match self.type_vars.entry(var) {
            Entry::Occupied(entry) if *entry.get() == ty => Ok(()),
            Entry::Occupied(_) => Err(()),
            Entry::Vacant(entry) => {
                entry.insert(ty);
                Ok(())
            }
        }
    }
    fn set_texel_format(&mut self, var: BoundVar, format: TexelFormat) -> Result<(), ()> {
        match self.texel_format_vars.entry(var) {
            Entry::Occupied(entry) if *entry.get() == format => Ok(()),
            Entry::Occupied(_) => Err(()),
            Entry::Vacant(entry) => {
                entry.insert(format);
                Ok(())
            }
        }
    }

    fn resolve(&self, db: &dyn HirDatabase, ty: Ty) -> Ty {
        match ty.kind(db) {
            TyKind::BoundVar(var) => *self.type_vars.get(&var).expect("type var not contrained"),
            TyKind::Vector(VectorType { size, inner }) => {
                let size = match size {
                    VecSize::BoundVar(size_var) => *self
                        .vec_size_vars
                        .get(&size_var)
                        .expect("vec size var not constrained"),
                    size => size,
                };
                let inner = self.resolve(db, inner);
                TyKind::Vector(VectorType { size, inner }).intern(db)
            }
            TyKind::Matrix(mat) => {
                let columns = match mat.columns {
                    VecSize::BoundVar(var) => *self.vec_size_vars.get(&var).unwrap(),
                    other => other,
                };
                let rows = match mat.rows {
                    VecSize::BoundVar(var) => *self.vec_size_vars.get(&var).unwrap(),
                    other => other,
                };

                let inner = self.resolve(db, mat.inner);
                TyKind::Matrix(MatrixType {
                    columns,
                    rows,
                    inner,
                })
                .intern(db)
            }
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
                .intern(db)
            }
            TyKind::Texture(TextureType {
                kind: TextureKind::Sampled(sampled_ty),
                dimension,
                arrayed,
                multisampled,
            }) => {
                let sampled_ty = self.resolve(db, sampled_ty);
                TyKind::Texture(TextureType {
                    kind: TextureKind::Sampled(sampled_ty),
                    dimension,
                    arrayed,
                    multisampled,
                })
                .intern(db)
            }
            TyKind::StorageTypeOfTexelFormat(var) => {
                let format = self.texel_format_vars[&var];
                storage_type_of_texel_format(db, format)
            }
            _ => ty,
        }
    }
}

// found type should not contain bound variables
fn unify(
    db: &dyn HirDatabase,
    table: &mut UnificationTable,
    expected: Ty,
    found: Ty,
) -> Result<(), ()> {
    let expected_kind = expected.kind(db);
    let found_kind = found.kind(db);

    match expected_kind {
        TyKind::BoundVar(var) => {
            table.set_type(var, found)?;
            Ok(())
        }
        TyKind::Vector(VectorType { size, inner }) => match found_kind {
            TyKind::Vector(found_vec) => {
                unify(db, table, inner, found_vec.inner)?;
                if let VecSize::BoundVar(vec_size_var) = size {
                    table.set_vec_size(vec_size_var, found_vec.size)?;
                } else if size != found_vec.size {
                    return Err(());
                }
                Ok(())
            }
            _ => Err(()),
        },
        TyKind::Matrix(MatrixType {
            columns,
            rows,
            inner,
        }) => match found_kind {
            TyKind::Matrix(found_mat) => {
                unify(db, table, inner, found_mat.inner)?;

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
            }
            _ => Err(()),
        },
        TyKind::Ptr(ptr) => match found_kind {
            TyKind::Ptr(found_ptr) => {
                unify(db, table, ptr.inner, found_ptr.inner)?;

                Ok(())
            }
            _ => Err(()),
        },
        TyKind::Array(array) => match found_kind {
            TyKind::Array(found_array) => {
                unify(db, table, array.inner, found_array.inner)?;

                Ok(())
            }
            _ => Err(()),
        },
        TyKind::Atomic(atomic) => match found_kind {
            TyKind::Atomic(found_atomic) => {
                unify(db, table, atomic.inner, found_atomic.inner)?;

                Ok(())
            }
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
                    TexelFormat::Any => {}
                    TexelFormat::BoundVar(var) => {
                        table.set_texel_format(var, format_2)?;
                    }
                    _ => {
                        if format != format_2 {
                            return Err(());
                        }
                    }
                }
                match (mode, mode_2) {
                    (AccessMode::Any, _) => {}
                    (_, AccessMode::Any) => unreachable!(),

                    (AccessMode::ReadWrite, AccessMode::ReadWrite) => {}
                    (AccessMode::Read, AccessMode::ReadWrite | AccessMode::Read) => {}
                    (AccessMode::Write, AccessMode::ReadWrite | AccessMode::Write) => {}

                    (AccessMode::Write, AccessMode::Read)
                    | (AccessMode::Read, AccessMode::Write)
                    | (AccessMode::ReadWrite, AccessMode::Read)
                    | (AccessMode::ReadWrite, AccessMode::Write) => return Err(()),
                }

                Ok(())
            }
            _ => Err(()),
        },
        TyKind::StorageTypeOfTexelFormat(format) => {
            let format = *table.texel_format_vars.get(&format).unwrap();
            let storage_type = storage_type_of_texel_format(db, format);

            if storage_type != found {
                return Err(());
            }

            Ok(())
        }

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

                unify(db, table, sampled_ty, found_sampled_ty)?;

                Ok(())
            }
            _ => Err(()),
        },

        _ if expected == found => Ok(()),
        _ => Err(()),
    }
}

fn storage_type_of_texel_format(db: &dyn HirDatabase, format: TexelFormat) -> Ty {
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
        inner: TyKind::Scalar(channel_type).intern(db),
    })
    .intern(db)
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TypeExpectationInner {
    Exact(Ty),
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
    fn from_option(option: Option<Ty>) -> Self {
        match option {
            Some(ty) => TypeExpectation::Type(TypeExpectationInner::Exact(ty)),
            None => TypeExpectation::None,
        }
    }
    fn from_ty(ty: Ty) -> Self {
        TypeExpectation::Type(TypeExpectationInner::Exact(ty))
    }
}

impl InferenceContext<'_> {
    fn make_ref(&self, ty: Ty, storage_class: StorageClass, access_mode: AccessMode) -> Ty {
        self.db.intern_ty(TyKind::Ref(Ref {
            inner: ty,
            storage_class,
            access_mode,
        }))
    }
    fn ref_to_ptr(&self, reference: Ref) -> Ty {
        self.db.intern_ty(TyKind::Ptr(Ptr {
            inner: reference.inner,
            storage_class: reference.storage_class,
            access_mode: reference.access_mode,
        }))
    }
    fn ptr_to_ref(&self, ptr: Ptr) -> Ty {
        self.db.intern_ty(TyKind::Ref(Ref {
            inner: ptr.inner,
            storage_class: ptr.storage_class,
            access_mode: ptr.access_mode,
        }))
    }

    fn err_ty(&self) -> Ty {
        self.db.intern_ty(TyKind::Error)
    }
    fn bool_ty(&self) -> Ty {
        self.db.intern_ty(TyKind::Scalar(ScalarType::Bool))
    }

    fn try_lower_ty(&mut self, type_ref: &TypeRef) -> Result<Ty, TypeLoweringError> {
        TyLoweringContext::new(self.db, &self.resolver).try_lower_ty(type_ref)
    }

    fn lower_ty(&mut self, container: impl Into<TypeContainer>, type_ref: &TypeRef) -> Ty {
        match self.try_lower_ty(type_ref) {
            Ok(ty) => ty,
            Err(error) => {
                self.push_diagnostic(InferenceDiagnostic::InvalidType {
                    container: container.into(),
                    error,
                });
                self.err_ty()
            }
        }
    }
}

pub struct TyLoweringContext<'db> {
    db: &'db dyn HirDatabase,
    resolver: &'db Resolver,

    pub diagnostics: Vec<TypeLoweringError>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TypeLoweringError {
    UnresolvedName(Name),
    InvalidTexelFormat(String),
}

impl std::fmt::Display for TypeLoweringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeLoweringError::UnresolvedName(name) => {
                write!(f, "type `{}` not found in scope", name.as_str())
            }
            TypeLoweringError::InvalidTexelFormat(format) => {
                let all_formats = "rgba8unorm,\nrgba8snorm,\nrgba8uint,\nrgba8sint,\nrgba16uint,\nrgba16sint,\nrgba16float,\nr32uint,\nr32sint,\nr32float,\nrg32uint,\nrg32sint,\nrg32float,\nrgba32uint,\nrgba32sint,\nrgba32float";
                write!(
                    f,
                    "`{}` is not a valid texel format, expected one of:\n{}",
                    format, all_formats
                )
            }
        }
    }
}

impl<'db> TyLoweringContext<'db> {
    pub fn new(db: &'db dyn HirDatabase, resolver: &'db Resolver) -> Self {
        Self {
            db,
            resolver,
            diagnostics: Vec::new(),
        }
    }

    pub fn lower_ty(&mut self, type_ref: &TypeRef) -> Ty {
        self.try_lower_ty(type_ref)
            .unwrap_or_else(|_| TyKind::Error.intern(self.db))
    }
    pub fn try_lower_ty(&mut self, type_ref: &TypeRef) -> Result<Ty, TypeLoweringError> {
        let ty_kind = match type_ref {
            TypeRef::Error => TyKind::Error,
            TypeRef::Scalar(scalar) => {
                let scalar = match scalar {
                    type_ref::ScalarType::Bool => ScalarType::Bool,
                    type_ref::ScalarType::Float32 => ScalarType::F32,
                    type_ref::ScalarType::Int32 => ScalarType::I32,
                    type_ref::ScalarType::Uint32 => ScalarType::U32,
                };
                TyKind::Scalar(scalar)
            }
            TypeRef::Vec(vec) => TyKind::Vector(VectorType {
                size: vec.size.into(),
                inner: self.lower_ty(&vec.inner),
            }),
            TypeRef::Matrix(matrix) => TyKind::Matrix(MatrixType {
                columns: matrix.columns.into(),
                rows: matrix.rows.into(),
                inner: self.lower_ty(&matrix.inner),
            }),
            TypeRef::Texture(tex) => TyKind::Texture(TextureType {
                dimension: match tex.dimension {
                    type_ref::TextureDimension::D1 => TextureDimensionality::D1,
                    type_ref::TextureDimension::D2 => TextureDimensionality::D2,
                    type_ref::TextureDimension::D3 => TextureDimensionality::D3,
                    type_ref::TextureDimension::Cube => TextureDimensionality::Cube,
                },
                arrayed: tex.arrayed,
                multisampled: tex.multisampled,
                kind: match &tex.kind {
                    type_ref::TextureKind::Sampled(ty) => TextureKind::Sampled(self.lower_ty(ty)),
                    type_ref::TextureKind::Storage(format, mode) => TextureKind::Storage(
                        format
                            .parse()
                            .map_err(|_| TypeLoweringError::InvalidTexelFormat(format.clone()))?,
                        *mode,
                    ),
                    type_ref::TextureKind::Depth => TextureKind::Depth,
                    type_ref::TextureKind::External => TextureKind::External,
                },
            }),
            TypeRef::Sampler(sampler) => TyKind::Sampler(SamplerType {
                comparison: sampler.comparison,
            }),
            TypeRef::Atomic(atomic) => TyKind::Atomic(AtomicType {
                inner: self.lower_ty(&atomic.inner),
            }),
            TypeRef::Array(array) => TyKind::Array(ArrayType {
                binding_array: array.binding_array,
                inner: self.lower_ty(&array.inner),
                size: match array.size {
                    type_ref::ArraySize::Int(i) => ArraySize::Const(i as u64), // TODO error
                    type_ref::ArraySize::Uint(u) => ArraySize::Const(u),
                    type_ref::ArraySize::Path(_) => ArraySize::Const(0), // TODO: Path array sizes
                    type_ref::ArraySize::Dynamic => ArraySize::Dynamic,
                },
            }),
            TypeRef::Ptr(ptr) => TyKind::Ptr(Ptr {
                storage_class: ptr.storage_class,
                inner: self.lower_ty(&ptr.inner),
                access_mode: ptr.access_mode,
            }),
            TypeRef::Path(name) => match self.resolver.resolve_type(name) {
                Some(ResolveType::Struct(loc)) => {
                    let strukt = self.db.intern_struct(loc);
                    TyKind::Struct(strukt)
                }
                Some(ResolveType::TypeAlias(loc)) => {
                    let alias = self.db.intern_type_alias(loc);
                    let data = self.db.type_alias_data(alias);
                    let type_ref = &self.db.lookup_intern_type_ref(data.ty);

                    return Ok(self.lower_ty(type_ref));
                }
                Some(ResolveType::PredeclaredTypeAlias(type_ref)) => {
                    return Ok(self.lower_ty(&type_ref))
                }
                None => return Err(TypeLoweringError::UnresolvedName(name.clone())),
            },
        };
        Ok(self.db.intern_ty(ty_kind))
    }
}
