use syntax::{HasGenerics, ast, pointer::AstPointer};

use crate::{
    HirFileId,
    database::DefDatabase,
    expression::{Callee, Expression, ExpressionId, parse_literal},
    expression_store::{ExpressionStoreBuilder, SyntheticSyntax},
    module_data::Name,
    type_ref::{TypeReference, matrix_dimensions, vector_dimensions},
};

pub struct ExprCollector<'database> {
    database: &'database dyn DefDatabase,
    pub store: ExpressionStoreBuilder,
}

impl ExprCollector<'_> {
    pub fn new<'a>(database: &'a dyn DefDatabase) -> ExprCollector<'a> {
        ExprCollector {
            database,
            store: ExpressionStoreBuilder::default(),
        }
    }

    #[expect(clippy::too_many_lines, reason = "TODO")]
    pub fn collect_expression(
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
                    .map_or(Expression::Missing, |op| Expression::BinaryOperation {
                        left_side,
                        right_side,
                        operation: op,
                    })
            },
            ast::Expression::PrefixExpression(prefix_expression) => {
                let expression = self.collect_expression_opt(prefix_expression.expression());
                prefix_expression
                    .op_kind()
                    .map_or(Expression::Missing, |op| Expression::UnaryOperator {
                        expression,
                        op,
                    })
            },
            ast::Expression::Literal(literal) => {
                let literal = literal.kind();
                Expression::Literal(parse_literal(literal))
            },
            ast::Expression::ParenthesisExpression(expression) => {
                let inner = self.collect_expression_opt(expression.inner());
                // make the paren expression point to the inner expression as well
                self.store.expression_map.insert(syntax_pointer, inner);
                self.store.parenthesis_expressions.insert(inner);
                return inner;
            },
            ast::Expression::BitcastExpression(expression) => {
                let inner = self.collect_expression_opt(
                    expression
                        .inner()
                        .map(ast::Expression::ParenthesisExpression),
                );

                let r#type = expression
                    .ty()
                    .and_then(|r#type| TypeReference::try_from(r#type).ok())
                    .unwrap_or(TypeReference::Error);
                let r#type = self.database.intern_type_ref(r#type);

                Expression::Bitcast {
                    expression: inner,
                    r#type,
                }
            },
            ast::Expression::FieldExpression(field) => {
                let expression = self.collect_expression_opt(field.expression());
                let name = field.name_ref().map_or_else(Name::missing, Name::from);

                Expression::Field { expression, name }
            },
            ast::Expression::FunctionCall(call) => {
                let arguments = call
                    .parameters()
                    .into_iter()
                    .flat_map(|parameters| parameters.arguments())
                    .map(|expression| self.collect_expression(expression))
                    .collect();

                let name = call.name_ref().map_or_else(Name::missing, Name::from);

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
                let name = path.name_ref().map_or_else(Name::missing, Name::from);

                Expression::Path(name)
            },
            ast::Expression::IndexExpression(index) => {
                let left_side = self.collect_expression_opt(index.expression());
                let index = self.collect_expression_opt(index.index());
                Expression::Index { left_side, index }
            },
            ast::Expression::TypeInitializer(r#type) => {
                let arguments = r#type
                    .arguments()
                    .into_iter()
                    .flat_map(|parameters| parameters.arguments())
                    .map(|expression| self.collect_expression(expression))
                    .collect();

                let r#type = r#type.ty();
                if let Some(r#type) = r#type {
                    let has_generic = r#type.generic_arg_list().is_some();
                    #[expect(
                        clippy::wildcard_enum_match_arm,
                        reason = "To many to list, but could be improved."
                    )]
                    let callee = match r#type {
                        ast::Type::VecType(vec) if !has_generic => {
                            let dimensions = vector_dimensions(&vec);
                            Callee::InferredComponentVec(dimensions)
                        },
                        ast::Type::MatrixType(matrix) if !has_generic => {
                            let (columns, rows) = matrix_dimensions(&matrix);
                            Callee::InferredComponentMatrix { rows, columns }
                        },
                        ast::Type::ArrayType(_) if !has_generic => Callee::InferredComponentArray,
                        other => {
                            let r#type =
                                TypeReference::try_from(other).unwrap_or(TypeReference::Error);
                            let r#type = self.database.intern_type_ref(r#type);
                            Callee::Type(r#type)
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
        self.store.expression_map.insert(source, id);
        id
    }

    fn make_expression(
        &mut self,
        expression: Expression,
        source: Result<AstPointer<ast::Expression>, SyntheticSyntax>,
    ) -> ExpressionId {
        let id = self.store.exprs.alloc(expression);
        self.store.expression_map_back.insert(id, source);
        id
    }

    fn missing_expression(&mut self) -> ExpressionId {
        self.make_expression(Expression::Missing, Err(SyntheticSyntax))
    }

    pub fn collect_expression_opt(
        &mut self,
        expression: Option<ast::Expression>,
    ) -> ExpressionId {
        match expression {
            Some(expression) => self.collect_expression(expression),
            None => self.missing_expression(),
        }
    }
}
