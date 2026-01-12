use dprint_core::formatting::Signal;
use dprint_core_macros::sc;
use itertools::put_back;
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::format::{
    ast_parse::{
        parse_end, parse_many_comments_and_blankspace, parse_node, parse_node_optional,
        parse_token, parse_token_optional,
    },
    gen_attributes::gen_attributes,
    gen_comments::gen_comments,
    gen_types::gen_type_specifier,
    print_item_buffer::PrintItemBuffer,
    reporting::FormatDocumentResult,
};

pub fn gen_struct_declaration(
    node: &ast::StructDeclaration
) -> FormatDocumentResult<PrintItemBuffer> {
    // === Parse ===
    let mut syntax = put_back(node.syntax().children_with_tokens());

    parse_token(&mut syntax, SyntaxKind::Struct)?;
    let item_comments_after_struct = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_name = parse_node::<ast::Name>(&mut syntax)?;
    let item_comments_after_name = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_body = parse_node::<ast::StructBody>(&mut syntax)?;
    parse_end(&mut syntax)?;

    // === Format ===
    let mut formatted = PrintItemBuffer::new();

    // Struct
    formatted.push_sc(sc!("struct"));
    formatted.expect_single_space();
    formatted.extend(gen_comments(item_comments_after_struct));

    // Name
    formatted.expect_single_space();
    formatted.push_string(item_name.text().to_string());
    formatted.extend(gen_comments(item_comments_after_name));

    // Body
    formatted.expect_single_space();
    formatted.extend(gen_struct_body(&item_body)?);

    Ok(formatted)
}

fn gen_struct_body(body: &ast::StructBody) -> FormatDocumentResult<PrintItemBuffer> {
    // === Parse ===
    let mut syntax = put_back(body.syntax().children_with_tokens());

    parse_token(&mut syntax, SyntaxKind::BraceLeft)?;
    let item_comments_after_open_paren = parse_many_comments_and_blankspace(&mut syntax)?;

    let mut item_members = Vec::new();

    loop {
        let Some(item_member) = parse_node_optional::<ast::StructMember>(&mut syntax) else {
            break;
        };
        let item_comments_after_member = parse_many_comments_and_blankspace(&mut syntax)?;

        parse_token_optional(&mut syntax, SyntaxKind::Comma); //Optional
        let item_comments_after_comma = parse_many_comments_and_blankspace(&mut syntax)?;

        item_members.push((
            item_member,
            item_comments_after_member,
            item_comments_after_comma,
        ));
    }

    parse_token(&mut syntax, SyntaxKind::BraceRight)?;
    parse_end(&mut syntax)?;

    // === Format ===
    let mut formatted = PrintItemBuffer::new();

    formatted.push_sc(sc!("{"));
    formatted.push_signal(Signal::StartIndent);

    //TODO This should be handled by gen_comments, and probably
    // take into account whether the comment was on the same line as the opening brace
    if !item_comments_after_open_paren.is_empty() {
        formatted.expect_line_break();
        formatted.extend(gen_comments(item_comments_after_open_paren));
    }

    if !item_members.is_empty() {
        formatted.expect_line_break();
        for (member, comments_after_member, comments_after_comma) in item_members {
            formatted.extend(gen_struct_member(&member)?);
            formatted.push_sc(sc!(","));

            // Intentionally reorder comments to move them after the comma
            formatted.extend(gen_comments(comments_after_member));
            formatted.extend(gen_comments(comments_after_comma));

            formatted.expect_line_break();
        }
    }

    formatted.push_signal(Signal::FinishIndent);
    formatted.push_sc(sc!("}"));

    Ok(formatted)
}

fn gen_struct_member(member: &ast::StructMember) -> FormatDocumentResult<PrintItemBuffer> {
    // === Parse ===
    let mut syntax = put_back(member.syntax().children_with_tokens());

    // TODO Think about a clean way to abstract this, to deduplicate code from functions
    // maybe even a "many with commments" combinator, to also deduplicate code from fn parameters/struct members
    let mut attributes = Vec::new();
    loop {
        let Some(item_attribute) = parse_node_optional::<ast::Attribute>(&mut syntax) else {
            break;
        };
        let item_comments_after_attribute = parse_many_comments_and_blankspace(&mut syntax)?;

        attributes.push((item_attribute, item_comments_after_attribute));
    }
    let item_comments_after_attributes = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_name = parse_node::<ast::Name>(&mut syntax)?;
    let item_comments_after_name = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::Colon)?;
    let item_comments_after_colon = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_type_specifier = parse_node::<ast::TypeSpecifier>(&mut syntax)?;
    parse_end(&mut syntax)?;

    // === Format ===
    let mut formatted = PrintItemBuffer::new();

    formatted.extend(gen_attributes(attributes)?);
    formatted.extend(gen_comments(item_comments_after_attributes));
    formatted.push_string(item_name.text().to_string());
    formatted.push_sc(sc!(":"));
    formatted.expect_single_space();
    //The colon should immediately follow the name, we intentionally move the comment
    formatted.extend(gen_comments(item_comments_after_name));
    formatted.extend(gen_comments(item_comments_after_colon));
    formatted.extend(gen_type_specifier(&item_type_specifier)?);

    Ok(formatted)
}
