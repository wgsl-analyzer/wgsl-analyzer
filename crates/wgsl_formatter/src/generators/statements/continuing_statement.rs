use dprint_core_macros::sc;
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::{
    ast_parse::{IgnoreBlankspace, NoTrivia, parse_end, parse_node_with, syntax_iter},
    generators::node::gen_node_with_trivia,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

pub fn gen_continuing_statement(
    statement: &ast::ContinuingStatement
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = syntax_iter(statement.syntax());
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Continuing)?;
    let item_body = parse_node_with(&mut syntax, IgnoreBlankspace)
        .expect_kind(SyntaxKind::CompoundStatement)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.push_sc(sc!("continuing"));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_node_with_trivia(&item_body)?);
    formatted.request(Request::expect(RequestItem::LineBreak));

    Ok(formatted)
}
