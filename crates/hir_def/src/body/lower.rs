use super::{Binding, BindingId, Body, BodySourceMap, SyntheticSyntax};
use crate::{
    db::DefDatabase,
    expr::{parse_literal, Expr, ExprId, Statement, StatementId},
    module_data::Name,
    type_ref::TypeRef,
    HirFileId, InFile,
};
use either::Either;
use syntax::{ast, ptr::AstPtr, AstNode, HasName};

pub fn lower_function_body(
    db: &dyn DefDatabase,
    file_id: HirFileId,
    param_list: Option<ast::ParamList>,
    body: Option<ast::CompoundStatement>,
) -> (Body, BodySourceMap) {
    Collector {
        db,
        body: Body::default(),
        source_map: BodySourceMap::default(),
        file_id,
    }
    .collect_function(param_list, body)
}

pub fn lower_global_var_decl(
    db: &dyn DefDatabase,
    file_id: HirFileId,
    decl: ast::GlobalVariableDecl,
) -> (Body, BodySourceMap) {
    Collector {
        db,
        body: Body::default(),
        source_map: BodySourceMap::default(),
        file_id: file_id,
    }
    .collect_global_var_decl(decl)
}
pub fn lower_global_constant_decl(
    db: &dyn DefDatabase,
    file_id: HirFileId,
    decl: ast::GlobalConstantDecl,
) -> (Body, BodySourceMap) {
    Collector {
        db,
        body: Body::default(),
        source_map: BodySourceMap::default(),
        file_id,
    }
    .collect_global_constant_decl(decl)
}

struct Collector<'a> {
    db: &'a dyn DefDatabase,
    body: Body,
    source_map: BodySourceMap,
    file_id: HirFileId,
}

impl<'a> Collector<'a> {
    fn collect_function(
        mut self,
        param_list: Option<ast::ParamList>,
        body: Option<ast::CompoundStatement>,
    ) -> (Body, BodySourceMap) {
        self.collect_function_param_list(param_list);

        self.body.root = body
            .map(|body| self.collect_compound_stmt(body))
            .map(Either::Left);

        (self.body, self.source_map)
    }

    fn collect_function_param_list(&mut self, param_list: Option<ast::ParamList>) {
        if let Some(param_list) = param_list {
            for p in param_list.params() {
                if let Some(binding) = p
                    .variable_ident_declaration()
                    .and_then(|decl| decl.binding())
                {
                    let binding_id = self.collect_binding(binding);
                    self.body.params.push(binding_id);
                } else if let Some(import) = p.import() {
                    let import_param_list =
                        crate::module_data::find_import(self.db, self.file_id, &import)
                            .map(|import| self.db.intern_import(InFile::new(self.file_id, import)))
                            .and_then(|import_id| {
                                let import_loc = self.db.lookup_intern_import(import_id);
                                let module_info = self.db.module_info(import_loc.file_id);
                                let import = module_info.get(import_loc.value);

                                match &import.value {
                                    crate::module_data::ImportValue::Path(_) => None, // TODO: path imports
                                    crate::module_data::ImportValue::Custom(key) => self
                                        .db
                                        .parse_import(
                                            key.clone(),
                                            syntax::ParseEntryPoint::FnParamList,
                                        )
                                        .ok(),
                                }
                            })
                            .and_then(|parse| ast::ParamList::cast(parse.syntax()));
                    self.collect_function_param_list(import_param_list);
                }
            }
        }
    }

    fn collect_global_var_decl(mut self, decl: ast::GlobalVariableDecl) -> (Body, BodySourceMap) {
        self.body.root = decl
            .init()
            .map(|expr| self.collect_expr(expr))
            .map(Either::Right);

        self.body.main_binding = decl.binding().map(|binding| self.collect_binding(binding));

        (self.body, self.source_map)
    }
    fn collect_global_constant_decl(
        mut self,
        decl: ast::GlobalConstantDecl,
    ) -> (Body, BodySourceMap) {
        self.body.root = decl
            .init()
            .map(|expr| self.collect_expr(expr))
            .map(Either::Right);

        self.body.main_binding = decl.binding().map(|binding| self.collect_binding(binding));

        (self.body, self.source_map)
    }

    fn collect_binding(&mut self, binding: ast::Binding) -> BindingId {
        let src = AstPtr::new(&binding);
        let name = binding.name().map(Name::from).unwrap_or_else(Name::missing);
        self.alloc_binding(Binding { name }, src)
    }
    fn collect_binding_opt(&mut self, binding: Option<ast::Binding>) -> BindingId {
        match binding {
            Some(binding) => self.collect_binding(binding),
            None => self.missing_binding(),
        }
    }

    fn collect_compound_stmt_opt(
        &mut self,
        compound_stmt: Option<ast::CompoundStatement>,
    ) -> StatementId {
        compound_stmt
            .map(|stmt| self.collect_compound_stmt(stmt))
            .unwrap_or_else(|| self.missing_stmt())
    }
    fn collect_compound_stmt(&mut self, compound_stmt: ast::CompoundStatement) -> StatementId {
        let statements = compound_stmt
            .statements()
            .filter_map(|stmt| self.collect_stmt(stmt))
            .collect();

        self.body
            .statements
            .alloc(Statement::Compound { statements })
    }

    fn collect_stmt(&mut self, stmt: ast::Statement) -> Option<StatementId> {
        let hir_stmt = match stmt {
            ast::Statement::VariableStatement(ref variable_statement) => {
                let binding_id = self.collect_binding_opt(variable_statement.binding());
                let initializer = variable_statement
                    .initializer()
                    .map(|expr| self.collect_expr(expr));
                let type_ref = variable_statement
                    .ty()
                    .and_then(|ty| TypeRef::try_from(ty).ok())
                    .map(|type_ref| self.db.intern_type_ref(type_ref));

                match variable_statement.kind()? {
                    ast::VariableStatementKind::Let => Statement::LetStatement {
                        binding_id,
                        type_ref,
                        initializer,
                    },
                    ast::VariableStatementKind::Var => {
                        let storage_class = variable_statement
                            .variable_qualifier()
                            .and_then(|qualifier| qualifier.storage_class())
                            .map(Into::into);
                        let access_mode = variable_statement
                            .variable_qualifier()
                            .and_then(|qualifier| qualifier.access_mode())
                            .map(Into::into);

                        Statement::VariableStatement {
                            binding_id,
                            type_ref,
                            initializer,
                            storage_class,
                            access_mode,
                        }
                    }
                }
            }
            ast::Statement::CompoundStatement(compound_stmt) => {
                return Some(self.collect_compound_stmt(compound_stmt));
            }
            ast::Statement::ReturnStmt(ref ret_stmt) => {
                let expr = ret_stmt.expr().map(|expr| self.collect_expr(expr));
                Statement::Return { expr }
            }
            ast::Statement::AssignmentStmt(ref assignment) => {
                let lhs = self.collect_expr_opt(assignment.lhs());
                let rhs = self.collect_expr_opt(assignment.rhs());
                Statement::Assignment { lhs, rhs }
            }
            ast::Statement::CompoundAssignmentStmt(ref assignment) => {
                let lhs = self.collect_expr_opt(assignment.lhs());
                let rhs = self.collect_expr_opt(assignment.rhs());
                let op = assignment.op()?;
                Statement::CompoundAssignment { lhs, rhs, op }
            }
            ast::Statement::IncrDecrStatement(ref stmt) => {
                let expr = self.collect_expr_opt(stmt.expr());
                let op = stmt.incr_decr()?;
                Statement::IncrDecr { expr, op }
            }
            ast::Statement::IfStatement(ref if_stmt) => {
                let condition = self.collect_expr_opt(if_stmt.condition());
                let block = self.collect_compound_stmt_opt(if_stmt.block());
                let else_if_blocks = if_stmt
                    .else_if_blocks()
                    .map(|block| self.collect_compound_stmt_opt(block.block()))
                    .collect();
                let else_block = if_stmt
                    .else_block()
                    .map(|block| self.collect_compound_stmt_opt(block.block()));
                Statement::If {
                    condition,
                    block,
                    else_if_blocks,
                    else_block,
                }
            }
            ast::Statement::SwitchStatement(ref stmt) => {
                let expr = self.collect_expr_opt(stmt.expr());

                let (case_blocks, default_block) = match stmt.block() {
                    Some(block) => {
                        let case_blocks = block
                            .cases()
                            .map(|case| {
                                let selectors =
                                    case.selectors().map_or_else(Vec::new, |selectors| {
                                        selectors
                                            .exprs()
                                            .map(|expr| self.collect_expr(expr))
                                            .collect()
                                    });
                                let block = self.collect_compound_stmt_opt(case.block());
                                (selectors, block)
                            })
                            .collect();

                        let default_block = block
                            .default()
                            .last()
                            .map(|default| self.collect_compound_stmt_opt(default.block()));

                        (case_blocks, default_block)
                    }
                    None => (Vec::default(), None),
                };

                Statement::Switch {
                    expr,
                    case_blocks,
                    default_block,
                }
            }
            ast::Statement::ForStatement(ref for_stmt) => {
                let initializer = for_stmt
                    .initializer()
                    .and_then(|init| self.collect_stmt(init));
                let condition = for_stmt.condition().map(|init| self.collect_expr(init));
                let continuing_part = for_stmt
                    .continuing_part()
                    .and_then(|init| self.collect_stmt(init));

                let block = self.collect_compound_stmt_opt(for_stmt.block());

                Statement::For {
                    initializer,
                    condition,
                    continuing_part,
                    block,
                }
            }
            ast::Statement::Discard(_) => Statement::Discard,
            ast::Statement::Break(_) => Statement::Break,
            ast::Statement::Continue(_) => Statement::Continue,
            ast::Statement::ContinuingStatement(ref continuing) => Statement::Continuing {
                block: self.collect_compound_stmt_opt(continuing.block()),
            },
            ast::Statement::ExprStatement(ref expr) => {
                let expr = self.collect_expr_opt(expr.expr());
                Statement::Expr { expr }
            }
            ast::Statement::LoopStatement(ref stmt) => {
                let body = self.collect_compound_stmt_opt(stmt.block());
                Statement::Loop { body }
            }
        };

        let id = self.alloc_stmt(hir_stmt, AstPtr::new(&stmt));
        Some(id)
    }

    fn collect_expr(&mut self, expr: ast::Expr) -> ExprId {
        let syntax_ptr = AstPtr::new(&expr);
        let expr = match expr {
            ast::Expr::InfixExpr(expr) => {
                let lhs = self.collect_expr_opt(expr.lhs());
                let rhs = self.collect_expr_opt(expr.rhs());

                expr.op_kind()
                    .map(|op| Expr::BinaryOp { lhs, rhs, op })
                    .unwrap_or(Expr::Missing)
            }
            ast::Expr::PrefixExpr(prefix_expr) => {
                let expr = self.collect_expr_opt(prefix_expr.expr());
                prefix_expr
                    .op_kind()
                    .map(|op| Expr::UnaryOp { expr, op })
                    .unwrap_or(Expr::Missing)
            }
            ast::Expr::Literal(literal) => {
                let literal = literal.kind();
                Expr::Literal(parse_literal(literal))
            }
            ast::Expr::ParenExpr(expr) => {
                let inner = self.collect_expr_opt(expr.inner());
                // make the paren expr point to the inner expression as well
                self.source_map.expr_map.insert(syntax_ptr, inner);
                return inner;
            }
            ast::Expr::FieldExpr(field) => {
                let expr = self.collect_expr_opt(field.expr());
                let name = field
                    .name_ref()
                    .map(Name::from)
                    .unwrap_or_else(Name::missing);

                Expr::Field { expr, name }
            }
            ast::Expr::FunctionCall(call) => {
                let args = call
                    .params()
                    .into_iter()
                    .flat_map(|params| params.args())
                    .map(|expr| self.collect_expr(expr))
                    .collect();

                match (call.type_initializer(), call.expr()) {
                    (None, Some(expr)) => {
                        let expr = self.collect_expr(expr);
                        Expr::Call { callee: expr, args }
                    }
                    (Some(ty), None) => {
                        let ty = ty
                            .ty()
                            .and_then(|ty| TypeRef::try_from(ty).ok())
                            .unwrap_or(TypeRef::Error);
                        let ty = self.db.intern_type_ref(ty);
                        Expr::TypeInitializer { ty, args }
                    }
                    (Some(_), Some(_)) => unreachable!(),
                    (None, None) => {
                        let expr = self.missing_expr();
                        Expr::Call { callee: expr, args }
                    }
                }
            }
            ast::Expr::PathExpr(path) => {
                let name = path
                    .name_ref()
                    .map(Name::from)
                    .unwrap_or_else(Name::missing);

                Expr::Path(name)
            }
            ast::Expr::IndexExpr(index) => {
                let lhs = self.collect_expr_opt(index.expr());
                let index = self.collect_expr_opt(index.index());
                Expr::Index { lhs, index }
            }
        };

        self.alloc_expr(expr, syntax_ptr)
    }

    fn alloc_expr(&mut self, expr: Expr, src: AstPtr<ast::Expr>) -> ExprId {
        let id = self.make_expr(expr, Ok(src.clone()));
        self.source_map.expr_map.insert(src, id);
        id
    }
    fn make_expr(&mut self, expr: Expr, src: Result<AstPtr<ast::Expr>, SyntheticSyntax>) -> ExprId {
        let id = self.body.exprs.alloc(expr);
        self.source_map.expr_map_back.insert(id, src);
        id
    }
    fn alloc_stmt(&mut self, stmt: Statement, src: AstPtr<ast::Statement>) -> StatementId {
        let id = self.make_stmt(stmt, Ok(src.clone()));
        self.source_map.stmt_map.insert(src, id);
        id
    }
    fn make_stmt(
        &mut self,
        stmt: Statement,
        src: Result<AstPtr<ast::Statement>, SyntheticSyntax>,
    ) -> StatementId {
        let id = self.body.statements.alloc(stmt);
        self.source_map.stmt_map_back.insert(id, src);
        id
    }

    fn alloc_binding(&mut self, binding: Binding, src: AstPtr<ast::Binding>) -> BindingId {
        let id = self.make_binding(binding, Ok(src.clone()));
        self.source_map.binding_map.insert(src, id);
        id
    }
    fn make_binding(
        &mut self,
        binding: Binding,
        src: Result<AstPtr<ast::Binding>, SyntheticSyntax>,
    ) -> BindingId {
        let id = self.body.bindings.alloc(binding);
        self.source_map.binding_map_back.insert(id, src);
        id
    }

    fn missing_binding(&mut self) -> la_arena::Idx<Binding> {
        self.make_binding(
            Binding {
                name: Name::missing(),
            },
            Err(SyntheticSyntax),
        )
    }
    fn missing_expr(&mut self) -> ExprId {
        self.make_expr(Expr::Missing, Err(SyntheticSyntax))
    }
    fn missing_stmt(&mut self) -> StatementId {
        self.make_stmt(Statement::Missing, Err(SyntheticSyntax))
    }

    fn collect_expr_opt(&mut self, expr: Option<ast::Expr>) -> ExprId {
        match expr {
            Some(expr) => self.collect_expr(expr),
            None => self.missing_expr(),
        }
    }
}
