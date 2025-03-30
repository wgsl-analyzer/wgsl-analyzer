use either::Either;
use syntax::{AstNode, HasGenerics, HasName, ast, pointer::AstPointer};

use super::{Binding, BindingId, Body, BodySourceMap, SyntheticSyntax};
use crate::{
    HirFileId, InFile,
    db::DefDatabase,
    expression::{Callee, Expression, ExpressionId, Statement, StatementId, parse_literal},
    hir_file_id::relative_file,
    module_data::Name,
    type_ref::{TypeReference, matrix_dimensions, vector_dimensions},
};

pub(super) fn lower_function_body(
    db: &dyn DefDatabase,
    file_id: HirFileId,
    param_list: Option<ast::ParameterList>,
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

pub(super) fn lower_global_var_declaration(
    db: &dyn DefDatabase,
    file_id: HirFileId,
    declaration: ast::GlobalVariableDeclaration,
) -> (Body, BodySourceMap) {
    Collector {
        db,
        body: Body::default(),
        source_map: BodySourceMap::default(),
        file_id,
    }
    .collect_global_var_declaration(declaration)
}

pub(super) fn lower_global_constant_declaration(
    db: &dyn DefDatabase,
    file_id: HirFileId,
    declaration: ast::GlobalConstantDeclaration,
) -> (Body, BodySourceMap) {
    Collector {
        db,
        body: Body::default(),
        source_map: BodySourceMap::default(),
        file_id,
    }
    .collect_global_constant_declaration(declaration)
}

pub(super) fn lower_override_declaration(
    db: &dyn DefDatabase,
    file_id: HirFileId,
    declaration: ast::OverrideDeclaration,
) -> (Body, BodySourceMap) {
    Collector {
        db,
        body: Body::default(),
        source_map: BodySourceMap::default(),
        file_id,
    }
    .collect_override_declaration(declaration)
}

struct Collector<'a> {
    db: &'a dyn DefDatabase,
    body: Body,
    source_map: BodySourceMap,
    file_id: HirFileId,
}

impl Collector<'_> {
    fn collect_function(
        mut self,
        param_list: Option<ast::ParameterList>,
        body: Option<ast::CompoundStatement>,
    ) -> (Body, BodySourceMap) {
        self.collect_function_param_list(param_list);

        self.body.root = body
            .map(|body| self.collect_compound_statement(body))
            .map(Either::Left);

        (self.body, self.source_map)
    }

    fn collect_function_param_list(
        &mut self,
        param_list: Option<ast::ParameterList>,
    ) {
        if let Some(param_list) = param_list {
            for p in param_list.parameters() {
                if let Some(binding) = p
                    .variable_ident_declaration()
                    .and_then(|declaration| declaration.binding())
                {
                    let binding_id = self.collect_binding(binding);
                    self.body.parameters.push(binding_id);
                } else if let Some(import) = p.import() {
                    let import_param_list =
                        crate::module_data::find_import(self.db, self.file_id, &import)
                            .map(|import| self.db.intern_import(InFile::new(self.file_id, import)))
                            .and_then(|import_id| {
                                let import_loc = self.db.lookup_intern_import(import_id);
                                let module_info = self.db.module_info(import_loc.file_id);
                                let import = module_info.get(import_loc.value);

                                match &import.value {
                                    crate::module_data::ImportValue::Path(path) => {
                                        let file_id =
                                            relative_file(self.db, import_loc.file_id, path)?;
                                        Some(self.db.parse(file_id))
                                    },
                                    crate::module_data::ImportValue::Custom(key) => self
                                        .db
                                        .parse_import(
                                            key.clone(),
                                            syntax::ParseEntryPoint::FunctionParameterList,
                                        )
                                        .ok(),
                                }
                            })
                            .and_then(|parse| ast::ParameterList::cast(parse.syntax()));
                    self.collect_function_param_list(import_param_list);
                }
            }
        }
    }

    fn collect_global_var_declaration(
        mut self,
        declaration: ast::GlobalVariableDeclaration,
    ) -> (Body, BodySourceMap) {
        self.body.root = declaration
            .init()
            .map(|expression| self.collect_expression(expression))
            .map(Either::Right);

        self.body.main_binding = declaration
            .binding()
            .map(|binding| self.collect_binding(binding));

        (self.body, self.source_map)
    }

    fn collect_global_constant_declaration(
        mut self,
        declaration: ast::GlobalConstantDeclaration,
    ) -> (Body, BodySourceMap) {
        self.body.root = declaration
            .init()
            .map(|expression| self.collect_expression(expression))
            .map(Either::Right);

        self.body.main_binding = declaration
            .binding()
            .map(|binding| self.collect_binding(binding));

        (self.body, self.source_map)
    }

    fn collect_override_declaration(
        mut self,
        declaration: ast::OverrideDeclaration,
    ) -> (Body, BodySourceMap) {
        self.body.root = declaration
            .init()
            .map(|expression| self.collect_expression(expression))
            .map(Either::Right);

        self.body.main_binding = declaration
            .binding()
            .map(|binding| self.collect_binding(binding));

        (self.body, self.source_map)
    }

    fn collect_binding(
        &mut self,
        binding: ast::Binding,
    ) -> BindingId {
        let source = AstPointer::new(&binding);
        let name = binding.name().map(Name::from).unwrap_or_else(Name::missing);
        self.alloc_binding(Binding { name }, source)
    }

    fn collect_binding_opt(
        &mut self,
        binding: Option<ast::Binding>,
    ) -> BindingId {
        match binding {
            Some(binding) => self.collect_binding(binding),
            None => self.missing_binding(),
        }
    }

    fn collect_compound_statement_opt(
        &mut self,
        compound_statement: Option<ast::CompoundStatement>,
    ) -> StatementId {
        compound_statement
            .map(|statement| self.collect_compound_statement(statement))
            .unwrap_or_else(|| self.missing_statement())
    }

    fn collect_compound_statement(
        &mut self,
        compound_statement: ast::CompoundStatement,
    ) -> StatementId {
        let statements = compound_statement
            .statements()
            .filter_map(|statement| self.collect_statement(statement))
            .collect();

        self.body
            .statements
            .alloc(Statement::Compound { statements })
    }

    fn collect_statement(
        &mut self,
        statement: ast::Statement,
    ) -> Option<StatementId> {
        let hir_statement = match statement {
            ast::Statement::VariableStatement(ref variable_statement) => {
                let binding_id = self.collect_binding_opt(variable_statement.binding());
                let initializer = variable_statement
                    .initializer()
                    .map(|expression| self.collect_expression(expression));
                let type_ref = variable_statement
                    .ty()
                    .and_then(|ty| TypeReference::try_from(ty).ok())
                    .map(|type_ref| self.db.intern_type_ref(type_ref));

                match variable_statement.kind()? {
                    ast::VariableStatementKind::Let => Statement::LetStatement {
                        binding_id,
                        type_ref,
                        initializer,
                    },
                    ast::VariableStatementKind::Constant => Statement::ConstStatement {
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
                    },
                }
            },
            ast::Statement::CompoundStatement(compound_statement) => {
                return Some(self.collect_compound_statement(compound_statement));
            },
            ast::Statement::ReturnStatement(ref ret_statement) => {
                let expression = ret_statement
                    .expression()
                    .map(|expression| self.collect_expression(expression));
                Statement::Return { expression }
            },
            ast::Statement::AssignmentStatement(ref assignment) => {
                let left_side = self.collect_expression_opt(assignment.left_side());
                let right_side = self.collect_expression_opt(assignment.right_side());
                Statement::Assignment {
                    left_side,
                    right_side,
                }
            },
            ast::Statement::CompoundAssignmentStatement(ref assignment) => {
                let left_side = self.collect_expression_opt(assignment.left_side());
                let right_side = self.collect_expression_opt(assignment.right_side());
                let op = assignment.operator()?;
                Statement::CompoundAssignment {
                    left_side,
                    right_side,
                    op,
                }
            },
            ast::Statement::IncrementDecrementStatement(ref statement) => {
                let expression = self.collect_expression_opt(statement.expression());
                let op = statement.increment_decrement()?;
                Statement::IncrDecr { expression, op }
            },
            ast::Statement::IfStatement(ref if_statement) => {
                let condition = self.collect_expression_opt(if_statement.condition());
                let block = self.collect_compound_statement_opt(if_statement.block());
                let else_if_blocks = if_statement
                    .else_if_blocks()
                    .map(|block| self.collect_compound_statement_opt(block.block()))
                    .collect();
                let else_block = if_statement
                    .else_block()
                    .map(|block| self.collect_compound_statement_opt(block.block()));
                Statement::If {
                    condition,
                    block,
                    else_if_blocks,
                    else_block,
                }
            },
            ast::Statement::SwitchStatement(ref statement) => {
                let expression = self.collect_expression_opt(statement.expression());

                let (case_blocks, default_block) = match statement.block() {
                    Some(block) => {
                        let case_blocks = block
                            .cases()
                            .map(|case| {
                                let selectors =
                                    case.selectors().map_or_else(Vec::new, |selectors| {
                                        selectors
                                            .exprs()
                                            .map(|expression| self.collect_expression(expression))
                                            .collect()
                                    });
                                let block = self.collect_compound_statement_opt(case.block());
                                (selectors, block)
                            })
                            .collect();

                        let default_block = block
                            .default()
                            .last()
                            .map(|default| self.collect_compound_statement_opt(default.block()));

                        (case_blocks, default_block)
                    },
                    None => (Vec::default(), None),
                };

                Statement::Switch {
                    expression,
                    case_blocks,
                    default_block,
                }
            },
            ast::Statement::ForStatement(ref for_statement) => {
                let initializer = for_statement
                    .initializer()
                    .and_then(|init| self.collect_statement(init));
                let condition = for_statement
                    .condition()
                    .map(|init| self.collect_expression(init));
                let continuing_part = for_statement
                    .continuing_part()
                    .and_then(|init| self.collect_statement(init));

                let block = self.collect_compound_statement_opt(for_statement.block());

                Statement::For {
                    initializer,
                    condition,
                    continuing_part,
                    block,
                }
            },
            ast::Statement::WhileStatement(ref while_statement) => {
                let condition = self.collect_expression_opt(while_statement.condition());
                let block = self.collect_compound_statement_opt(while_statement.block());
                Statement::While { condition, block }
            },
            ast::Statement::Discard(_) => Statement::Discard,
            ast::Statement::Break(_) => Statement::Break,
            ast::Statement::Continue(_) => Statement::Continue,
            ast::Statement::ContinuingStatement(ref continuing) => Statement::Continuing {
                block: self.collect_compound_statement_opt(continuing.block()),
            },
            ast::Statement::ExpressionStatement(ref expression) => {
                let expression = self.collect_expression_opt(expression.expression());
                Statement::Expression { expression }
            },
            ast::Statement::LoopStatement(ref statement) => {
                let body = self.collect_compound_statement_opt(statement.block());
                Statement::Loop { body }
            },
        };

        let id = self.allocate_statement(hir_statement, AstPointer::new(&statement));
        Some(id)
    }

    fn collect_expression(
        &mut self,
        expression: ast::Expression,
    ) -> ExpressionId {
        let syntax_pointer = AstPointer::new(&expression);
        let expression = match expression {
            ast::Expression::InfixExpression(expression) => {
                let left_side = self.collect_expression_opt(expression.left_side());
                let right_side = self.collect_expression_opt(expression.right_side());

                expression
                    .op_kind()
                    .map(|op| Expression::BinaryOperation {
                        left_side,
                        right_side,
                        operation: op,
                    })
                    .unwrap_or(Expression::Missing)
            },
            ast::Expression::PrefixExpression(prefix_expression) => {
                let expression = self.collect_expression_opt(prefix_expression.expression());
                prefix_expression
                    .op_kind()
                    .map(|op| Expression::UnaryOperator { expression, op })
                    .unwrap_or(Expression::Missing)
            },
            ast::Expression::Literal(literal) => {
                let literal = literal.kind();
                Expression::Literal(parse_literal(literal))
            },
            ast::Expression::ParenthesisExpression(expression) => {
                let inner = self.collect_expression_opt(expression.inner());
                // make the paren expression point to the inner expression as well
                self.source_map.expression_map.insert(syntax_pointer, inner);
                self.body.parenthesis_expressions.insert(inner);
                return inner;
            },
            ast::Expression::BitcastExpression(expression) => {
                let inner = self.collect_expression_opt(
                    expression
                        .inner()
                        .map(ast::Expression::ParenthesisExpression),
                );

                let ty = expression
                    .ty()
                    .and_then(|ty| TypeReference::try_from(ty).ok())
                    .unwrap_or(TypeReference::Error);
                let ty = self.db.intern_type_ref(ty);

                Expression::Bitcast {
                    expression: inner,
                    ty,
                }
            },
            ast::Expression::FieldExpression(field) => {
                let expression = self.collect_expression_opt(field.expression());
                let name = field
                    .name_ref()
                    .map(Name::from)
                    .unwrap_or_else(Name::missing);

                Expression::Field { expression, name }
            },
            ast::Expression::FunctionCall(call) => {
                let arguments = call
                    .parameters()
                    .into_iter()
                    .flat_map(|parameters| parameters.arguments())
                    .map(|expression| self.collect_expression(expression))
                    .collect();

                let name = call
                    .name_ref()
                    .map(Name::from)
                    .unwrap_or_else(Name::missing);

                Expression::Call {
                    callee: Callee::Name(name),
                    arguments,
                }
            },
            ast::Expression::InvalidFunctionCall(call) => {
                if let Some(expression) = call.expression() {
                    self.collect_expression(expression);
                }
                call.parameters()
                    .into_iter()
                    .flat_map(|parameters| parameters.arguments())
                    .for_each(|expression| {
                        self.collect_expression(expression);
                    });

                Expression::Missing
            },
            ast::Expression::PathExpression(path) => {
                let name = path
                    .name_ref()
                    .map(Name::from)
                    .unwrap_or_else(Name::missing);

                Expression::Path(name)
            },
            ast::Expression::IndexExpression(index) => {
                let left_side = self.collect_expression_opt(index.expression());
                let index = self.collect_expression_opt(index.index());
                Expression::Index { left_side, index }
            },
            ast::Expression::TypeInitializer(ty) => {
                let arguments = ty
                    .arguments()
                    .into_iter()
                    .flat_map(|parameters| parameters.arguments())
                    .map(|expression| self.collect_expression(expression))
                    .collect();

                let ty = ty.ty();
                if let Some(ty) = ty {
                    let has_generic = ty.generic_arg_list().is_some();
                    let callee = match ty {
                        ast::Type::VecType(vec) if !has_generic => {
                            let dimensions = vector_dimensions(&vec);
                            Callee::InferredComponentVec(dimensions)
                        },
                        ast::Type::MatrixType(matrix) if !has_generic => {
                            let (columns, rows) = matrix_dimensions(&matrix);
                            Callee::InferredComponentMatrix { rows, columns }
                        },
                        ast::Type::ArrayType(_) if !has_generic => Callee::InferredComponentArray,
                        ty => {
                            let ty = TypeReference::try_from(ty).unwrap_or(TypeReference::Error);
                            let ty = self.db.intern_type_ref(ty);
                            Callee::Type(ty)
                        },
                    };
                    Expression::Call { callee, arguments }
                } else {
                    Expression::Missing
                }
            },
        };

        self.alloc_expression(expression, syntax_pointer)
    }

    fn alloc_expression(
        &mut self,
        expression: Expression,
        source: AstPointer<ast::Expression>,
    ) -> ExpressionId {
        let id = self.make_expression(expression, Ok(source.clone()));
        self.source_map.expression_map.insert(source, id);
        id
    }

    fn make_expression(
        &mut self,
        expression: Expression,
        source: Result<AstPointer<ast::Expression>, SyntheticSyntax>,
    ) -> ExpressionId {
        let id = self.body.exprs.alloc(expression);
        self.source_map.expression_map_back.insert(id, source);
        id
    }

    fn allocate_statement(
        &mut self,
        statement: Statement,
        source: AstPointer<ast::Statement>,
    ) -> StatementId {
        let id = self.make_statement(statement, Ok(source.clone()));
        self.source_map.statement_map.insert(source, id);
        id
    }

    fn make_statement(
        &mut self,
        statement: Statement,
        source: Result<AstPointer<ast::Statement>, SyntheticSyntax>,
    ) -> StatementId {
        let id = self.body.statements.alloc(statement);
        self.source_map.statement_map_back.insert(id, source);
        id
    }

    fn alloc_binding(
        &mut self,
        binding: Binding,
        source: AstPointer<ast::Binding>,
    ) -> BindingId {
        let id = self.make_binding(binding, Ok(source.clone()));
        self.source_map.binding_map.insert(source, id);
        id
    }

    fn make_binding(
        &mut self,
        binding: Binding,
        source: Result<AstPointer<ast::Binding>, SyntheticSyntax>,
    ) -> BindingId {
        let id = self.body.bindings.alloc(binding);
        self.source_map.binding_map_back.insert(id, source);
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

    fn missing_expression(&mut self) -> ExpressionId {
        self.make_expression(Expression::Missing, Err(SyntheticSyntax))
    }

    fn missing_statement(&mut self) -> StatementId {
        self.make_statement(Statement::Missing, Err(SyntheticSyntax))
    }

    fn collect_expression_opt(
        &mut self,
        expression: Option<ast::Expression>,
    ) -> ExpressionId {
        match expression {
            Some(expression) => self.collect_expression(expression),
            None => self.missing_expression(),
        }
    }
}
