use dprint_core_macros::sc;
use itertools::put_back;
use parser::SyntaxKind;
use syntax::{AstNode as _, ast};

use crate::{
    ast_parse::{IgnoreBlankspace, NoTrivia, parse_end, parse_node_with},
    context_policies::statement_needs_semicolon_policy,
    generators::node::gen_node_with_trivia,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

pub fn gen_break_statement(node: &ast::BreakStatement) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    // We still parse through the break syntax even tho there is no information for
    // the formatter to get out of it. This exists to ensure we don't accidentally delete
    // user's code should future changes to wgsl allow more complex break statements.
    let mut syntax = put_back(node.syntax().children_with_tokens());
    let item_break =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(SyntaxKind::Break)?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind_optional(SyntaxKind::Semicolon)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_node_with_trivia(&item_break)?);
    if statement_needs_semicolon_policy(node.syntax()) {
        formatted.push_sc(sc!(";"));
    }
    formatted.request(Request::expect(RequestItem::LineBreak));
    Ok(formatted)
}
