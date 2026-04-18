use std::collections::BTreeSet;

use dprint_core_macros::sc;
use itertools::put_back;
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::generators::{
    ast_parse::{parse_end, parse_node, parse_node_optional, parse_token, parse_token_optional},
    gen_attributes::{AttributeLayout, gen_attributes, parse_many_attributes},
    gen_comments::{
        Comment, gen_comment, gen_comments, parse_comment_optional,
        parse_many_comments_and_blankspace,
    },
    gen_types::gen_type_specifier,
    helpers::{LineSpacing, gen_line_spacing, parse_line_spacing},
    print_item_buffer::{
        PrintItemBuffer,
        request_folder::{Request, RequestItem},
    },
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
    formatted.expect(RequestItem::Space);
    formatted.extend(gen_comments(&item_comments_after_struct));

    // Name
    formatted.expect(RequestItem::Space);
    formatted.push_string(item_name.text().to_string());
    formatted.extend(gen_comments(&item_comments_after_name));

    // Body
    formatted.expect(RequestItem::Space);
    formatted.extend(gen_struct_body(&item_body)?);

    Ok(formatted)
}

pub fn gen_struct_body(body: &ast::StructBody) -> FormatDocumentResult<PrintItemBuffer> {
    enum StructBodyItem {
        StructMember(ast::StructMember),
        LineSpacing(LineSpacing),
        Comment(Comment),
    }

    // === Parse ===
    let mut syntax = put_back(body.syntax().children_with_tokens());

    parse_token(&mut syntax, SyntaxKind::BraceLeft)?;
    let item_comments_after_open_paren = parse_many_comments_and_blankspace(&mut syntax)?;

    let mut item_members = Vec::new();

    loop {
        if let Some(item_member) = parse_node_optional::<ast::StructMember>(&mut syntax) {
            item_members.push(StructBodyItem::StructMember(item_member));
        } else if let Some(line_spacing) = parse_line_spacing(&mut syntax) {
            item_members.push(StructBodyItem::LineSpacing(line_spacing));
        } else if let Some(_blank) = parse_token_optional(&mut syntax, SyntaxKind::Blankspace) {
            // We throw away any non-linespacing information about blanksapces
        } else if let Some(comment) = parse_comment_optional(&mut syntax) {
            item_members.push(StructBodyItem::Comment(comment));
        } else {
            break;
        }
        // We throw away any information about commas
        parse_token_optional(&mut syntax, SyntaxKind::Comma);
    }

    parse_token(&mut syntax, SyntaxKind::BraceRight)?;
    parse_end(&mut syntax)?;

    // === Format ===
    let is_empty = item_members.is_empty();
    let mut formatted = PrintItemBuffer::new();

    formatted.push_sc(sc!("{"));
    formatted.start_indent();

    //TODO This should be handled by gen_comments, and probably
    // take into account whether the comment was on the same line as the opening brace
    if !item_comments_after_open_paren.is_empty() {
        formatted.expect(RequestItem::LineBreak);
        formatted.extend(gen_comments(&item_comments_after_open_paren));
    }

    if !is_empty {
        formatted.expect(RequestItem::LineBreak);
        for member in item_members {
            match member {
                StructBodyItem::StructMember(struct_member) => {
                    formatted.expect(RequestItem::LineBreak); // Any struct member should be on a new line
                    formatted.extend(gen_struct_member(&struct_member)?);
                    formatted.push_sc(sc!(","));
                },
                StructBodyItem::LineSpacing(line_spacing) => {
                    formatted.extend(gen_line_spacing(&line_spacing)?);
                },
                StructBodyItem::Comment(comment) => {
                    formatted.extend(gen_comment(&comment));
                },
            }
        }
    }

    formatted.request(Request::Unconditional {
        discouraged: BTreeSet::from([RequestItem::EmptyLine]),
        expected: BTreeSet::from([RequestItem::LineBreak]),
        forced: BTreeSet::new(),
        suggest_linebreak: false,
    });

    formatted.finish_indent();
    formatted.push_sc(sc!("}"));

    if !is_empty {
        formatted.expect(RequestItem::LineBreak);
    }

    Ok(formatted)
}

pub fn gen_struct_member(member: &ast::StructMember) -> FormatDocumentResult<PrintItemBuffer> {
    // === Parse ===
    let mut syntax = put_back(member.syntax().children_with_tokens());

    let attributes = parse_many_attributes(&mut syntax)?;
    let item_comments_after_attributes = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_name = parse_node::<ast::Name>(&mut syntax)?;
    let item_comments_after_name = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::Colon)?;
    let item_comments_after_colon = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_type_specifier = parse_node::<ast::TypeSpecifier>(&mut syntax)?;
    parse_end(&mut syntax)?;

    // === Format ===
    let mut formatted = PrintItemBuffer::new();

    formatted.extend(gen_attributes(&attributes, AttributeLayout::Multiline)?);
    formatted.extend(gen_comments(&item_comments_after_attributes));
    formatted.push_string(item_name.text().to_string());
    formatted.push_sc(sc!(":"));
    formatted.expect(RequestItem::Space);
    //The colon should immediately follow the name, we intentionally move the comment
    formatted.extend(gen_comments(&item_comments_after_name));
    formatted.extend(gen_comments(&item_comments_after_colon));
    formatted.extend(gen_type_specifier(&item_type_specifier)?);

    Ok(formatted)
}
