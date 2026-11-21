use dprint_core::formatting::Arena;
use dprint_core_macros::sc;
use syntax::{
    AstNode as _, SyntaxKind,
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
    reporting::FormatDocumentError,
};

pub(crate) fn gen_increment_decrement_statement<'buffer>(
    arena: &'buffer Arena,
    increment_decrement_statement: &ast::IncrementDecrementStatement,
) -> Result<PrintItemBuffer<'buffer>, FormatDocumentError> {
    // ==== Parse ====
    let mut syntax = syntax_iter(increment_decrement_statement.syntax());

    let item_ident =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_ast_node::<Expression>()?;
    let item_operator = parse_node_with(&mut syntax, DiscardBlankspace);
    parse_node_with(&mut syntax, NoTrivia).expect_kind_optional(SyntaxKind::Semicolon)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new(arena);
    formatted.extend(arena, gen_node_with_trivia(arena, &item_ident)?);
    formatted.extend(arena, gen_node_with_trivia(arena, &item_operator)?);

    if statement_needs_semicolon_policy(increment_decrement_statement.syntax()) {
        formatted.request(Request::discourage(RequestItem::Space));
        formatted.push_sc(arena, sc!(";"));
    }
    Ok(formatted)
}
