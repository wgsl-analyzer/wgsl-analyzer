use dprint_core_macros::sc;
use itertools::put_back;
use syntax::{
    AstNode as _,
    ast::{Name, TypeAliasDeclaration, TypeSpecifier},
};

use crate::{
    ast_parse::{NoTrivia, parse_end, parse_node, parse_node_with, parse_token, syntax_iter},
    context_policies::statement_needs_semicolon_policy,
    generators::{
        comments::{gen_comments, parse_many_comments_and_blankspace},
        types::gen_type_specifier,
    },
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
    let item_comments_after_alias = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_name = parse_node::<Name>(&mut syntax)?;
    let item_comments_after_ident = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind(parser::SyntaxKind::Equal)?;
    let item_comments_after_equal = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_type = parse_node::<TypeSpecifier>(&mut syntax)?;
    let item_comments_after_type = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind(parser::SyntaxKind::Semicolon)?; //Optional?
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.push_sc(sc!("alias"));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_comments(&item_comments_after_alias));
    formatted.push_string(item_name.text().to_string());
    formatted.extend(gen_comments(&item_comments_after_ident));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.push_sc(sc!("="));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_comments(&item_comments_after_equal));
    formatted.extend(gen_type_specifier(&item_type)?);
    formatted.extend(gen_comments(&item_comments_after_type));
    if statement_needs_semicolon_policy(statement.syntax()) {
        formatted.request(Request::discourage(RequestItem::Space));
        formatted.push_sc(sc!(";"));
    }
    Ok(formatted)
}
