use dprint_core::formatting::Arena;
use dprint_core_macros::sc;
use syntax::{
    AstNode as _, SyntaxKind, SyntaxNode,
    ast::{self, Expression},
};

use crate::{
    ast_parse::{DiscardBlankspace, NoTrivia, parse_end, parse_node_with, syntax_iter},
    context_policies::statement_needs_semicolon_policy,
    generators::node::gen_node_with_trivia,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

pub(crate) fn gen_break_if_statement<'buffer>(
    arena: &'buffer Arena,
    statement: &ast::BreakIfStatement,
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    // ==== Parse ====
    let mut syntax = syntax_iter(statement.syntax());
    let item_break =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_kind(SyntaxKind::Break)?;
    let item_if = parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::If)?;
    let item_condition =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_ast_node::<Expression>()?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind_optional(SyntaxKind::Semicolon)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new(arena);
    formatted.extend(arena, gen_node_with_trivia(arena, &item_break)?);
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(arena, gen_node_with_trivia(arena, &item_if)?);
    formatted.start_indent_before_requests(arena, );
    formatted.request(Request::expect(RequestItem::Space).or_newline());
    formatted.extend(arena, gen_node_with_trivia(arena, &item_condition)?);
    formatted.request(Request::discourage(RequestItem::Space));
    if statement_needs_semicolon_policy(statement.syntax()) {
        formatted.push_sc(arena, sc!(";"));
    }
    formatted.finish_indent_before_requests(arena);

    Ok(formatted)
}

#[must_use]
pub(crate) fn remove_break_if_condition_parens_rule(node: &SyntaxNode) -> bool {
    let Some(parent) = node.parent() else {
        return false;
    };
    matches!(parent.kind(), SyntaxKind::BreakIfStatement)
}
