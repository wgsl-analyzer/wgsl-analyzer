use either::Either;
use syntax::{AstNode as _, HasGenerics as _, HasName as _, ast, pointer::AstPointer};

use super::{Binding, BindingId, Body, BodySourceMap, SyntheticSyntax};
use crate::{
    HirFileId, InFile,
    database::DefDatabase,
    expression::{
        Expression, ExpressionId, Statement, StatementId, SwitchCaseSelector, parse_literal,
    },
    expression_store::{ExpressionStoreBuilder, lower::ExprCollector},
    hir_file_id::relative_file,
    module_data::Name,
    type_ref::TypeReference,
};

pub(super) fn lower_function_body(
    database: &dyn DefDatabase,
    file_id: HirFileId,
    param_list: Option<ast::FunctionParameters>,
    body: Option<ast::CompoundStatement>,
) -> (Body, BodySourceMap) {
    Collector::new(database, file_id).collect_function(param_list, body)
}

pub(super) fn lower_global_var_declaration(
    database: &dyn DefDatabase,
    file_id: HirFileId,
    declaration: &ast::VariableDeclaration,
) -> (Body, BodySourceMap) {
    Collector::new(database, file_id).collect_global_var_declaration(declaration)
}

pub(super) fn lower_global_constant_declaration(
    database: &dyn DefDatabase,
    file_id: HirFileId,
    declaration: &ast::ConstantDeclaration,
) -> (Body, BodySourceMap) {
    Collector::new(database, file_id).collect_global_constant_declaration(declaration)
}

pub(super) fn lower_override_declaration(
    database: &dyn DefDatabase,
    file_id: HirFileId,
    declaration: &ast::OverrideDeclaration,
) -> (Body, BodySourceMap) {
    Collector::new(database, file_id).collect_override_declaration(declaration)
}

struct Collector<'database> {
    expressions: ExprCollector<'database>,
    database: &'database dyn DefDatabase,
    body: Body,
    source_map: BodySourceMap,
    file_id: HirFileId,
}

impl Collector<'_> {
    fn new<'a>(
        database: &'a dyn DefDatabase,
        file_id: HirFileId,
    ) -> Collector<'a> {
        Collector {
            expressions: ExprCollector::new(database),
            database,
            body: Body::default(),
            source_map: BodySourceMap::default(),
            file_id,
        }
    }
    fn collect_function(
        mut self,
        param_list: Option<ast::FunctionParameters>,
        body: Option<ast::CompoundStatement>,
    ) -> (Body, BodySourceMap) {
        self.collect_function_param_list(param_list);

        self.body.root = body
            .map(|body| self.collect_compound_statement(&body))
            .map(Either::Left);
        (self.body.store, self.source_map.expressions) = self.expressions.store.finish();

        (self.body, self.source_map)
    }

    fn collect_function_param_list(
        &mut self,
        param_list: Option<ast::FunctionParameters>,
    ) {
        if let Some(param_list) = param_list {
            for parameter in param_list.parameters() {
                let binding_id = self.collect_name_opt(parameter.name());
                self.body.parameters.push(binding_id);
            }
        }
    }

    fn collect_global_var_declaration(
        mut self,
        declaration: &ast::VariableDeclaration,
    ) -> (Body, BodySourceMap) {
        self.body.root = declaration
            .init()
            .map(|expression| self.collect_expression(expression))
            .map(Either::Right);

        self.body.main_binding = declaration.name().map(|binding| self.collect_name(binding));
        (self.body.store, self.source_map.expressions) = self.expressions.store.finish();

        (self.body, self.source_map)
    }

    fn collect_global_constant_declaration(
        mut self,
        declaration: &ast::ConstantDeclaration,
    ) -> (Body, BodySourceMap) {
        self.body.root = declaration
            .init()
            .map(|expression| self.collect_expression(expression))
            .map(Either::Right);

        self.body.main_binding = declaration.name().map(|binding| self.collect_name(binding));
        (self.body.store, self.source_map.expressions) = self.expressions.store.finish();

        (self.body, self.source_map)
    }

    fn collect_override_declaration(
        mut self,
        declaration: &ast::OverrideDeclaration,
    ) -> (Body, BodySourceMap) {
        self.body.root = declaration
            .init()
            .map(|expression| self.collect_expression(expression))
            .map(Either::Right);

        self.body.main_binding = declaration.name().map(|binding| self.collect_name(binding));
        (self.body.store, self.source_map.expressions) = self.expressions.store.finish();

        (self.body, self.source_map)
    }
    fn collect_name(
        &mut self,
        binding: ast::Name,
    ) -> BindingId {
        let source = AstPointer::new(&binding);
        let name = Name::from(binding);
        self.alloc_name(Binding { name }, source)
    }

    fn collect_name_opt(
        &mut self,
        binding: Option<ast::Name>,
    ) -> BindingId {
        match binding {
            Some(binding) => self.collect_name(binding),
            None => self.missing_binding(),
        }
    }

    fn collect_compound_statement_opt(
        &mut self,
        compound_statement: Option<ast::CompoundStatement>,
    ) -> StatementId {
        match compound_statement {
            Some(statement) => self.collect_compound_statement(&statement),
            None => self.missing_statement(),
        }
    }

    fn collect_compound_statement(
        &mut self,
        compound_statement: &ast::CompoundStatement,
    ) -> StatementId {
        let statements = compound_statement
            .statements()
            .filter_map(|statement| self.collect_statement(&statement))
            .collect();

        self.body
            .statements
            .alloc(Statement::Compound { statements })
    }

    #[expect(clippy::too_many_lines, reason = "TODO")]
    fn collect_statement(
        &mut self,
        statement: &ast::Statement,
    ) -> Option<StatementId> {
        let hir_statement = match &statement {
            ast::Statement::VariableDeclaration(variable_statement) => {
                let binding_id = self.collect_name_opt(variable_statement.name());
                let initializer = variable_statement
                    .init()
                    .map(|expression| self.collect_expression(expression));
                let type_ref = variable_statement
                    .ty()
                    .map(|typo| self.expressions.collect_type_specifier(typo));

                let (address_space, access_mode) = variable_statement
                    .generic_arg_list()
                    .map(|v| v.generics())
                    .map(|mut v| {
                        (
                            v.next()
                                .map(|expression| self.collect_expression(expression)),
                            v.next()
                                .map(|expression| self.collect_expression(expression)),
                        )
                    })
                    .unwrap_or_default();

                Statement::Variable {
                    binding_id,
                    type_ref,
                    initializer,
                    address_space,
                    access_mode,
                }
            },
            ast::Statement::ConstantDeclaration(variable_statement) => {
                let binding_id = self.collect_name_opt(variable_statement.name());
                let initializer = variable_statement
                    .init()
                    .map(|expression| self.collect_expression(expression));
                let type_ref = variable_statement
                    .ty()
                    .map(|typo| self.expressions.collect_type_specifier(typo));

                Statement::Const {
                    binding_id,
                    type_ref,
                    initializer,
                }
            },

            ast::Statement::LetDeclaration(variable_statement) => {
                let binding_id = self.collect_name_opt(variable_statement.name());
                let initializer = variable_statement
                    .init()
                    .map(|expression| self.collect_expression(expression));
                let type_ref = variable_statement
                    .ty()
                    .map(|typo| self.expressions.collect_type_specifier(typo));

                Statement::Let {
                    binding_id,
                    type_ref,
                    initializer,
                }
            },
            ast::Statement::CompoundStatement(compound_statement) => {
                return Some(self.collect_compound_statement(compound_statement));
            },
            ast::Statement::ReturnStatement(ret_statement) => {
                let expression = ret_statement
                    .expression()
                    .map(|expression| self.collect_expression(expression));
                Statement::Return { expression }
            },
            ast::Statement::AssignmentStatement(assignment) => {
                let left_side = self.collect_expression_opt(assignment.left_side());
                let right_side = self.collect_expression_opt(assignment.right_side());
                Statement::Assignment {
                    left_side,
                    right_side,
                }
            },
            ast::Statement::CompoundAssignmentStatement(assignment) => {
                let left_side = self.collect_expression_opt(assignment.left_side());
                let right_side = self.collect_expression_opt(assignment.right_side());
                let op = assignment.operator()?;
                Statement::CompoundAssignment {
                    left_side,
                    right_side,
                    op,
                }
            },
            ast::Statement::IncrementDecrementStatement(statement) => {
                let expression = self.collect_expression_opt(statement.expression());
                let op = statement.increment_decrement()?;
                Statement::IncrDecr { expression, op }
            },
            ast::Statement::IfStatement(if_statement) => {
                let condition = self
                    .collect_expression_opt(if_statement.if_block().and_then(|v| v.condition()));
                let block = self.collect_compound_statement_opt(
                    if_statement.if_block().and_then(|v| v.block()),
                );
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
            ast::Statement::SwitchStatement(statement) => {
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
                                            .map(|expression| match expression {
                                                ast::SwitchCaseSelector::Expression(expression) => {
                                                    SwitchCaseSelector::Expression(
                                                        self.collect_expression(expression),
                                                    )
                                                },
                                                ast::SwitchCaseSelector::Default(_) => {
                                                    SwitchCaseSelector::Default
                                                },
                                            })
                                            .collect()
                                    });
                                let block = self.collect_compound_statement_opt(case.block());
                                (selectors, block)
                            })
                            .collect();

                        // TODO: What if there are multiple default blocks?
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
            ast::Statement::ForStatement(for_statement) => {
                let initializer = for_statement
                    .initializer()
                    .and_then(|initializer| self.collect_statement(&initializer));
                let condition = for_statement
                    .condition()
                    .map(|init| self.collect_expression(init));
                let continuing_part = for_statement
                    .continuing_part()
                    .and_then(|initializer| self.collect_statement(&initializer));

                let block = self.collect_compound_statement_opt(for_statement.block());

                Statement::For {
                    initializer,
                    condition,
                    continuing_part,
                    block,
                }
            },
            ast::Statement::WhileStatement(while_statement) => {
                let condition = self.collect_expression_opt(while_statement.condition());
                let block = self.collect_compound_statement_opt(while_statement.block());
                Statement::While { condition, block }
            },
            ast::Statement::Discard(_) => Statement::Discard,
            ast::Statement::Break(_) => Statement::Break,
            ast::Statement::Continue(_) => Statement::Continue,
            ast::Statement::ContinuingStatement(continuing) => Statement::Continuing {
                block: self.collect_compound_statement_opt(continuing.block()),
            },
            ast::Statement::FunctionCallStatement(expression) => {
                let expression = self.collect_expression_opt(expression.expression());
                Statement::Expression { expression }
            },
            ast::Statement::LoopStatement(statement) => {
                let body = self.collect_compound_statement_opt(statement.block());
                Statement::Loop { body }
            },
        };

        let id = self.allocate_statement(hir_statement, AstPointer::new(statement));
        Some(id)
    }

    fn collect_expression(
        &mut self,
        expression: ast::Expression,
    ) -> ExpressionId {
        self.expressions.collect_expression(expression)
    }
    fn collect_expression_opt(
        &mut self,
        expression: Option<ast::Expression>,
    ) -> ExpressionId {
        self.expressions.collect_expression_opt(expression)
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

    fn alloc_name(
        &mut self,
        binding: Binding,
        source: AstPointer<ast::Name>,
    ) -> BindingId {
        let id = self.make_binding(binding, Ok(source.clone()));
        self.source_map.binding_map.insert(source, id);
        id
    }

    fn make_binding(
        &mut self,
        binding: Binding,
        source: Result<AstPointer<ast::Name>, SyntheticSyntax>,
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

    fn missing_statement(&mut self) -> StatementId {
        self.make_statement(Statement::Missing, Err(SyntheticSyntax))
    }
}
