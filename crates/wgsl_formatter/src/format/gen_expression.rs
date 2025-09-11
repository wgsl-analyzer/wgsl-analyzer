use dprint_core::formatting::{PrintItems, Signal};
use itertools::put_back;
use syntax::{AstNode, ast};

use crate::format::{
    ast_parse::{
        parse_end, parse_many_comments_and_blankspace, parse_node, parse_token, parse_token_any,
    },
    gen_comments::gen_comments,
    helpers::todo_verbatim,
    print_item_buffer::PrintItemBuffer,
    reporting::FormatDocumentResult,
};

pub fn gen_expression(expression: &ast::Expression) -> FormatDocumentResult<PrintItemBuffer> {
    match expression {
        ast::Expression::IndexExpression(index_expression) => {
            todo_verbatim(index_expression.syntax())
        },
        ast::Expression::FieldExpression(field_expression) => {
            todo_verbatim(field_expression.syntax())
        },
        ast::Expression::PrefixExpression(prefix_expression) => {
            todo_verbatim(prefix_expression.syntax())
        },
        ast::Expression::InfixExpression(infix_expression) => {
            gen_infix_expression(infix_expression)
        },
        ast::Expression::IdentExpression(ident_expression) => {
            todo_verbatim(ident_expression.syntax())
        },
        ast::Expression::FunctionCall(function_call) => todo_verbatim(function_call.syntax()),
        ast::Expression::ParenthesisExpression(parenthesis_expression) => {
            todo_verbatim(parenthesis_expression.syntax())
        },
        ast::Expression::Literal(literal) => gen_literal_expression(literal),
    }
}

pub fn gen_literal_expression(
    literal_expression: &ast::Literal
) -> FormatDocumentResult<PrintItemBuffer> {
    todo_verbatim(literal_expression.syntax())
}

pub fn gen_infix_expression(
    infix_expression: &ast::InfixExpression
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    dbg!(&infix_expression);
    let mut syntax = put_back(infix_expression.syntax().children_with_tokens());

    let item_left = parse_node::<ast::Expression>(&mut syntax)?;
    let item_comment_after_left = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_operator = parse_token_any(&mut syntax)?;
    let item_comment_after_operator = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_right = parse_node::<ast::Expression>(&mut syntax)?;
    let item_comment_after_right = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.extend(gen_expression(&item_left)?);
    formatted.extend(gen_comments(item_comment_after_left));
    formatted.request_single_space();
    formatted.push_string(item_operator.to_string()); //TODO I don't like to-stringing the operator here, would be better to special case on it... we would need a parse_token(any_of(...)) kind of thing.
    formatted.request_single_space();
    formatted.extend(gen_comments(item_comment_after_operator));
    formatted.extend(gen_expression(&item_right)?);
    formatted.extend(gen_comments(item_comment_after_right));
    Ok(formatted)
}
