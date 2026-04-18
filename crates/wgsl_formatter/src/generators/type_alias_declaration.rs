use dprint_core_macros::sc;
use itertools::put_back;
use syntax::{
    AstNode as _,
    ast::{Name, TypeAliasDeclaration, TypeSpecifier},
};

use crate::{
    ast_parse::{parse_end, parse_node, parse_token},
    generators::{
        comments::{gen_comments, parse_many_comments_and_blankspace},
        types::gen_type_specifier,
    },
    print_item_buffer::{PrintItemBuffer, request_folder::RequestItem},
    reporting::FormatDocumentError,
};

pub fn gen_type_alias_declaration(
    statement: &TypeAliasDeclaration,
    include_semicolon: bool,
) -> Result<PrintItemBuffer, FormatDocumentError> {
    // ==== Parse ====
    let mut syntax = put_back(statement.syntax().children_with_tokens());
    parse_token(&mut syntax, parser::SyntaxKind::Alias)?;
    let item_comments_after_alias = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_name = parse_node::<Name>(&mut syntax)?;
    let item_comments_after_ident = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, parser::SyntaxKind::Equal)?;
    let item_comments_after_equal = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_type = parse_node::<TypeSpecifier>(&mut syntax)?;
    let item_comments_after_type = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, parser::SyntaxKind::Semicolon)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("alias"));
    formatted.expect(RequestItem::Space);
    formatted.extend(gen_comments(&item_comments_after_alias));
    formatted.push_string(item_name.text().to_string());
    formatted.extend(gen_comments(&item_comments_after_ident));
    formatted.expect(RequestItem::Space);
    formatted.push_sc(sc!("="));
    formatted.expect(RequestItem::Space);
    formatted.extend(gen_comments(&item_comments_after_equal));
    formatted.extend(gen_type_specifier(&item_type)?);
    formatted.extend(gen_comments(&item_comments_after_type));
    if include_semicolon {
        formatted.discourage(RequestItem::Space);
        formatted.push_sc(sc!(";"));
    }
    Ok(formatted)
}
