use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::{
    ast_parse::{DiscardBlankspace, NoTrivia, parse_end, parse_node_with, syntax_iter},
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
    let mut syntax = syntax_iter(infix_expression.syntax());

    let item_left = parse_node_with(&mut syntax, DiscardBlankspace);
    let item_operator = parse_node_with(&mut syntax, NoTrivia);
    let item_right = parse_node_with(&mut syntax, DiscardBlankspace);
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.start_new_line_group_after_requests();
    formatted.extend(gen_node_with_trivia(&item_left)?);
    formatted.finish_new_line_group_before_requests();
    formatted.request(Request::expect(RequestItem::Space).or_newline());

    formatted.extend(gen_node_with_trivia(&item_operator)?);

    formatted.start_new_line_group_before_requests();
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_node_with_trivia(&item_right)?);
    formatted.finish_new_line_group_before_requests();
    Ok(formatted)
}
