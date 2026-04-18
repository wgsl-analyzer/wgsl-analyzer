use itertools::put_back;
use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::format::{
    ast_parse::{parse_end, parse_node, parse_token_any},
    expressions::gen_expression::gen_expression,
    gen_comments::{gen_comments, parse_many_comments_and_blankspace},
    print_item_buffer::PrintItemBuffer,
    reporting::FormatDocumentResult,
};
pub fn gen_prefix_expression(
    infix_expression: &ast::PrefixExpression
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(infix_expression.syntax().children_with_tokens());

    let item_operator = parse_token_any(&mut syntax)?;
    let item_comment_after_operator = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_expr = parse_node::<ast::Expression>(&mut syntax)?;
    let item_comment_after_expr = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.push_string(item_operator.to_string()); //TODO I don't like to-stringing the operator here, would be better to match on it... we would need a parse_token(any_of(...)) kind of thing.
    formatted.extend(gen_comments(&item_comment_after_operator));
    formatted.extend(gen_expression(&item_expr, false)?);
    formatted.extend(gen_comments(&item_comment_after_expr));
    Ok(formatted)
}
