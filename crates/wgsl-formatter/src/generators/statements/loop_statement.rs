use dprint_core::formatting::Arena;
use dprint_core_macros::sc;
use syntax::{AstNode as _, SyntaxKind, ast};

use crate::{
    ast_parse::{DiscardBlankspace, NoTrivia, parse_end, parse_node_with, syntax_iter},
    generators::node::gen_node_with_trivia,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

pub(crate) fn gen_loop_statement<'buffer>(
    arena: &'buffer Arena,
    statement: &ast::LoopStatement,
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    // ==== Parse ====
    let mut syntax = syntax_iter(statement.syntax());
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Loop)?;
    let item_body = parse_node_with(&mut syntax, DiscardBlankspace)
        .expect_kind(SyntaxKind::CompoundStatement)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new(arena);
    formatted.push_sc(arena, sc!("loop"));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(arena, gen_node_with_trivia(arena, &item_body)?);
    formatted.request(Request::expect(RequestItem::LineBreak));

    Ok(formatted)
}
