use dprint_core_macros::sc;
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::{
    ast_parse::{IgnoreBlankspace, IgnoreBraces, IgnoreComma, Succeeding, UntilEmptyLine},
    generators::node::gen_node_with_trivia,
};
use crate::{
    ast_parse::{NoTrivia, parse_end, parse_node_with, syntax_iter},
    generators::node::{gen_node_content, gen_node_preceding_trivia, gen_node_succeeding_trivia},
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem, RequestItemSet},
    },
    reporting::FormatDocumentResult,
    trivia::NodeWithTriviaContent,
};

pub fn gen_struct_declaration(
    node: &ast::StructDeclaration
) -> FormatDocumentResult<PrintItemBuffer> {
    // === Parse ===
    let mut syntax = syntax_iter(node.syntax());

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Struct)?;
    let item_name = parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(SyntaxKind::Name)?;
    let item_body =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(SyntaxKind::StructBody)?;
    parse_end(&mut syntax)?;

    // === Format ===
    let mut formatted = PrintItemBuffer::default();

    // Struct
    formatted.push_sc(sc!("struct"));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_node_with_trivia(&item_name)?);
    // Body
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_node_with_trivia(&item_body)?);

    Ok(formatted)
}

pub fn gen_struct_body(body: &ast::StructBody) -> FormatDocumentResult<PrintItemBuffer> {
    // === Parse ===
    let mut syntax = syntax_iter(body.syntax());

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::BraceLeft)?;

    let mut item_members = Vec::new();

    loop {
        let item = parse_node_with(
            &mut syntax,
            (Succeeding(UntilEmptyLine), IgnoreComma, IgnoreBraces),
        )
        .expect_kind_optional(SyntaxKind::StructMember)?;

        let is_end = item.is_end();
        if !item.is_whitespace() {
            item_members.push(item);
        }
        if is_end {
            break;
        }
    }

    parse_end(&mut syntax)?;

    // === Format ===
    let is_empty = item_members.is_empty();
    let mut formatted = PrintItemBuffer::default();

    formatted.push_sc(sc!("{"));
    formatted.start_indent_before_requests();
    formatted.request(Request::discourage(RequestItem::EmptyLine));

    if !is_empty {
        for member in item_members {
            if member.has_content() {
                formatted.request(Request::expect(RequestItem::LineBreak));
            }

            formatted.extend(gen_node_preceding_trivia(&member)?);
            formatted.extend(gen_node_content(&member)?);
            if member.has_content() {
                formatted.push_sc(sc!(","));
            }
            formatted.extend(gen_node_succeeding_trivia(&member)?);
        }
    }

    formatted.request(Request::Unconditional {
        discouraged: RequestItemSet::from(RequestItem::EmptyLine),
        expected: RequestItemSet::from(RequestItem::LineBreak),
        forced: RequestItemSet::empty(),
        suggest_linebreak: false,
    });

    formatted.finish_indent_before_requests();
    formatted.push_sc(sc!("}"));

    if !is_empty {
        formatted.request(Request::expect(RequestItem::LineBreak));
    }

    Ok(formatted)
}

pub fn gen_struct_member(member: &ast::StructMember) -> FormatDocumentResult<PrintItemBuffer> {
    // === Parse ===
    let mut syntax = syntax_iter(member.syntax());

    let item_name = parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(SyntaxKind::Name)?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Colon)?;
    let item_type_specifier =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(SyntaxKind::TypeSpecifier)?;
    parse_end(&mut syntax)?;

    // === Format ===
    let mut formatted = PrintItemBuffer::default();

    formatted.extend(gen_node_with_trivia(&item_name)?);
    formatted.push_sc(sc!(":"));
    formatted.request(Request::expect(RequestItem::Space));
    //The colon should immediately follow the name, we intentionally move the comment
    formatted.extend(gen_node_with_trivia(&item_type_specifier)?);

    Ok(formatted)
}
