use dprint_core_macros::sc;
use syntax::{AstNode as _, SyntaxKind, ast::TypeAliasDeclaration};

use crate::{
    FormattingOptions,
    ast_parse::{DiscardBlankspace, NoTrivia, parse_end, parse_node_with, syntax_iter},
    context_policies::statement_needs_semicolon_policy,
    generators::node::gen_node_with_trivia,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentError,
};

pub(crate) fn gen_type_alias_declaration(
    options: &FormattingOptions,
    statement: &TypeAliasDeclaration,
) -> Result<PrintItemBuffer, FormatDocumentError> {
    // ==== Parse ====
    let mut syntax = syntax_iter(statement.syntax());
    let item_visibility =
        parse_node_with(&mut syntax, NoTrivia).only_if_kind(SyntaxKind::Visibility, &mut syntax);
    let item_alias =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_kind(SyntaxKind::Alias)?;
    let item_name =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_kind(SyntaxKind::Name)?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Equal)?;
    let item_type =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_kind(SyntaxKind::TypeSpecifier)?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Semicolon)?; //Optional?
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    if let Some(item_visibility) = item_visibility {
        formatted.extend(gen_node_with_trivia(options, &item_visibility)?);
        formatted.request(Request::expect(RequestItem::Space));
    }
    formatted.extend(gen_node_with_trivia(options, &item_alias)?);
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_node_with_trivia(options, &item_name)?);
    formatted.request(Request::expect(RequestItem::Space));
    formatted.push_sc(sc!("="));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_node_with_trivia(options, &item_type)?);
    if statement_needs_semicolon_policy(statement.syntax()) {
        formatted.request(Request::discourage(RequestItem::Space));
        formatted.push_sc(sc!(";"));
    }
    Ok(formatted)
}
