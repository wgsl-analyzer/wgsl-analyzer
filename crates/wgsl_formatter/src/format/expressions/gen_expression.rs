use syntax::ast;

use crate::format::{
    expressions::{
        gen_field::gen_field_expression, gen_ident::gen_ident_expression,
        gen_index::gen_index_expression, gen_infix::gen_infix_expression,
        gen_literal::gen_literal_expression, gen_parenthesis::gen_parenthesis_expression,
        gen_prefix::gen_prefix_expression,
    },
    print_item_buffer::PrintItemBuffer,
    reporting::FormatDocumentResult,
    statements::gen_function_call::gen_function_call,
};

pub fn gen_expression(
    expression: &ast::Expression,
    remove_parentheses: bool,
) -> FormatDocumentResult<PrintItemBuffer> {
    match expression {
        ast::Expression::IndexExpression(index_expression) => {
            gen_index_expression(index_expression)
        },
        ast::Expression::FieldExpression(field_expression) => {
            gen_field_expression(field_expression)
        },
        ast::Expression::PrefixExpression(prefix_expression) => {
            gen_prefix_expression(prefix_expression)
        },
        ast::Expression::InfixExpression(infix_expression) => {
            gen_infix_expression(infix_expression)
        },
        ast::Expression::IdentExpression(ident_expression) => {
            gen_ident_expression(ident_expression)
        },
        ast::Expression::FunctionCall(function_call) => gen_function_call(function_call),
        ast::Expression::ParenthesisExpression(parenthesis_expression) => {
            gen_parenthesis_expression(parenthesis_expression, remove_parentheses)
        },
        ast::Expression::Literal(literal) => gen_literal_expression(literal),
    }
}
