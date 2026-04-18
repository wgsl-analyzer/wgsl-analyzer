use itertools::put_back;
use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::generators::{
    ast_parse::{parse_end, parse_node, parse_token_any},
    expressions::gen_expression,
    gen_comments::{gen_comments, parse_many_comments_and_blankspace},
    print_item_buffer::{
        PrintItemBuffer,
        request_folder::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

pub fn gen_infix_expression(
    infix_expression: &ast::InfixExpression
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
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
    formatted.extend(gen_expression(&item_left, false)?);
    formatted.extend(gen_comments(&item_comment_after_left));
    formatted.request(Request::expect(RequestItem::Space).or_newline());
    formatted.push_string(item_operator.to_string()); //TODO I don't like to-stringing the operator here, would be better to special case on it... we would need a parse_token(any_of(...)) kind of thing.
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_comments(&item_comment_after_operator));
    formatted.extend(gen_expression(&item_right, false)?);
    formatted.extend(gen_comments(&item_comment_after_right));
    Ok(formatted)
}
