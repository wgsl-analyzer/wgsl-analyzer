use dprint_core::formatting::Arena;
use dprint_core_macros::sc;
use syntax::{AstNode as _, ast::TypeAliasDeclaration};

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

pub(crate) fn gen_type_alias_declaration<'buffer>(
    arena: &'buffer Arena,
    statement: &TypeAliasDeclaration,
) -> Result<PrintItemBuffer<'buffer>, FormatDocumentError> {
    // ==== Parse ====
    let mut syntax = syntax_iter(statement.syntax());
    parse_node_with(&mut syntax, NoTrivia).expect_kind(syntax::SyntaxKind::Alias)?;
    let item_name =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_kind(syntax::SyntaxKind::Name)?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind(syntax::SyntaxKind::Equal)?;
    let item_type = parse_node_with(&mut syntax, DiscardBlankspace)
        .expect_kind(syntax::SyntaxKind::TypeSpecifier)?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind(syntax::SyntaxKind::Semicolon)?; //Optional?
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new(arena);
    formatted.push_sc(arena, sc!("alias"));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(arena, gen_node_with_trivia(arena, &item_name)?);
    formatted.request(Request::expect(RequestItem::Space));
    formatted.push_sc(arena, sc!("="));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(arena, gen_node_with_trivia(arena, &item_type)?);
    if statement_needs_semicolon_policy(statement.syntax()) {
        formatted.request(Request::discourage(RequestItem::Space));
        formatted.push_sc(arena, sc!(";"));
    }
    Ok(formatted)
}
