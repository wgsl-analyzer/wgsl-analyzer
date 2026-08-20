use dprint_core_macros::sc;
use syntax::{AstNode as _, ast::TypeAliasDeclaration};

use crate::{
    ast_parse::{IgnoreBlankspace, NoTrivia, parse_end, parse_node_with, syntax_iter},
    context_policies::statement_needs_semicolon_policy,
    generators::node::gen_node_with_trivia,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentError,
};

pub fn gen_type_alias_declaration(
    statement: &TypeAliasDeclaration
) -> Result<PrintItemBuffer, FormatDocumentError> {
    // ==== Parse ====
    let mut syntax = syntax_iter(statement.syntax());
    parse_node_with(&mut syntax, NoTrivia).expect_kind(parser::SyntaxKind::Alias)?;
    let item_name =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(parser::SyntaxKind::Name)?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind(parser::SyntaxKind::Equal)?;
    let item_type = parse_node_with(&mut syntax, IgnoreBlankspace)
        .expect_kind(parser::SyntaxKind::TypeSpecifier)?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind(parser::SyntaxKind::Semicolon)?; //Optional?
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.push_sc(sc!("alias"));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_node_with_trivia(&item_name)?);
    formatted.request(Request::expect(RequestItem::Space));
    formatted.push_sc(sc!("="));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_node_with_trivia(&item_type)?);
    if statement_needs_semicolon_policy(statement.syntax()) {
        formatted.request(Request::discourage(RequestItem::Space));
        formatted.push_sc(sc!(";"));
    }
    Ok(formatted)
}
