use itertools::put_back;
use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::{
    ast_parse::{IgnoreBlankspace, NoTrivia, parse_end, parse_node_with},
    generators::node::gen_node_with_trivia,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

pub fn gen_infix_expression(
    infix_expression: &ast::InfixExpression
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(infix_expression.syntax().children_with_tokens());

    let item_left = parse_node_with(&mut syntax, IgnoreBlankspace);
    let item_operator = parse_node_with(&mut syntax, NoTrivia);
    let item_right = parse_node_with(&mut syntax, IgnoreBlankspace);
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_node_with_trivia(&item_left)?);
    formatted.request(Request::expect(RequestItem::Space).or_newline());
    formatted.extend(gen_node_with_trivia(&item_operator)?);
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_node_with_trivia(&item_right)?);
    Ok(formatted)
}
