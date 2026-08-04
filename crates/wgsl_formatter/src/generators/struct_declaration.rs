use dprint_core_macros::sc;
use itertools::{Itertools, Position, put_back};
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::{
    ast_parse::{
        parse_end, parse_node, parse_node_optional, parse_node_with_trivia, parse_token,
        parse_token_optional,
    },
    generators::node::gen_node_with_trivia,
    helpers::{LineSpacing, gen_line_spacing},
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem, RequestItemSet},
    },
    reporting::FormatDocumentResult,
    trivia::NodeWithTriviaContent,
};
use crate::{
    generators::{
        attributes::{AttributeLayout, gen_attributes, parse_many_attributes},
        comments::{
            Comment, gen_comment, gen_comments, parse_comment_optional,
            parse_many_comments_and_blankspace,
        },
        types::gen_type_specifier,
    },
    helpers::parse_line_spacing,
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
    let mut formatted = PrintItemBuffer::default();

    // Struct
    formatted.push_sc(sc!("struct"));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_comments(&item_comments_after_struct));

    // Name
    formatted.request(Request::expect(RequestItem::Space));
    formatted.push_string(item_name.text().to_string());
    formatted.extend(gen_comments(&item_comments_after_name));

    // Body
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_struct_body(&item_body)?);

    Ok(formatted)
}

pub fn gen_struct_body(body: &ast::StructBody) -> FormatDocumentResult<PrintItemBuffer> {
    // === Parse ===
    let mut syntax = put_back(body.syntax().children_with_tokens());

    parse_token(&mut syntax, SyntaxKind::BraceLeft)?;
    let item_comments_after_open_paren = parse_many_comments_and_blankspace(&mut syntax)?;

    let mut item_members = Vec::new();

    loop {
        let mut item = parse_node_with_trivia(&mut syntax);

        if item
            .kind()
            .is_some_and(|item| item == SyntaxKind::BraceRight)
        {
            let old_node = std::mem::replace(&mut item.node, NodeWithTriviaContent::End);
            syntax.put_back(old_node.into_option().unwrap()); //TODO
        }

        if item
            .kind()
            .is_some_and(|item| item != SyntaxKind::StructMember)
        {
            item.node = NodeWithTriviaContent::NoContent;
        }

        let is_end = item.is_end();
        if !item.is_whitespace() {
            item_members.push(item);
        }
        if is_end {
            break;
        }
    }

    parse_token(&mut syntax, SyntaxKind::BraceRight)?;
    parse_end(&mut syntax)?;

    // === Format ===
    let is_empty = item_members.is_empty();
    let mut formatted = PrintItemBuffer::default();

    formatted.push_sc(sc!("{"));
    formatted.start_indent();

    if !item_comments_after_open_paren.is_empty() {
        formatted.request(Request::expect(RequestItem::LineBreak));
        formatted.extend(gen_comments(&item_comments_after_open_paren));
    }

    dbg!(&item_members);
    if !is_empty {
        for (pos, member) in item_members.iter().with_position() {
            if member.has_content() {
                formatted.request(Request::expect(RequestItem::LineBreak));
            }

            formatted.extend(gen_node_with_trivia(&member)?);
            if member.has_content() {
                formatted.push_sc(sc!(","));
            }
        }
    }

    formatted.request(Request::Unconditional {
        discouraged: RequestItemSet::from(RequestItem::EmptyLine),
        expected: RequestItemSet::from(RequestItem::LineBreak),
        forced: RequestItemSet::empty(),
        suggest_linebreak: false,
    });

    formatted.finish_indent();
    formatted.push_sc(sc!("}"));

    if !is_empty {
        formatted.request(Request::expect(RequestItem::LineBreak));
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
    let mut formatted = PrintItemBuffer::default();

    formatted.extend(gen_attributes(&attributes, AttributeLayout::Multiline)?);
    formatted.extend(gen_comments(&item_comments_after_attributes));
    formatted.push_string(item_name.text().to_string());
    formatted.push_sc(sc!(":"));
    formatted.request(Request::expect(RequestItem::Space));
    //The colon should immediately follow the name, we intentionally move the comment
    formatted.extend(gen_comments(&item_comments_after_name));
    formatted.extend(gen_comments(&item_comments_after_colon));
    formatted.extend(gen_type_specifier(&item_type_specifier)?);

    Ok(formatted)
}
