use itertools::put_back;
use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::{
    ast_parse::{parse_end, parse_node, parse_token_any},
    generators::{
        comments::{gen_comments, parse_many_comments_and_blankspace},
        expressions::gen_expression,
    },
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
    let mut formatted = PrintItemBuffer::default();
    //I don't like to-stringing the operator here, would be better to special case on it,
    //In a benchmark on my machine however, removing this push_string() did not make any measurable impact - so it's fine.
    formatted.push_string(item_operator.to_string());
    formatted.extend(gen_comments(&item_comment_after_operator));
    formatted.extend(gen_expression(&item_expr)?);
    formatted.extend(gen_comments(&item_comment_after_expr));
    Ok(formatted)
}
