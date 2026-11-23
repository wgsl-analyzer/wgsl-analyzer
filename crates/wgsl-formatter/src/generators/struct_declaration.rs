use dprint_core::formatting::Arena;
use dprint_core_macros::sc;
use syntax::{AstNode as _, SyntaxKind, ast};

use crate::{
    ast_parse::{
        DiscardBlankspace, DiscardBraces, DiscardComma, NoTrivia, StopAtNewline, Succeeding,
        parse_end, parse_many_nodes_with, parse_node_with, syntax_iter,
    },
    generators::node::{
        gen_node_content, gen_node_preceding_trivia, gen_node_succeeding_trivia,
        gen_node_with_trivia,
    },
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem, RequestItemSet},
    },
    reporting::FormatDocumentResult,
};

pub(crate) fn gen_struct_declaration<'buffer>(
    arena: &'buffer Arena,
    node: &ast::StructDeclaration,
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    // === Parse ===
    let mut syntax = syntax_iter(node.syntax());

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Struct)?;
    let item_name =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_kind(SyntaxKind::Name)?;
    let item_body =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_kind(SyntaxKind::StructBody)?;
    parse_end(&mut syntax)?;

    // === Format ===
    let mut formatted = PrintItemBuffer::new(arena);

    // Struct
    formatted.push_sc(arena, sc!("struct"));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(arena, gen_node_with_trivia(arena, &item_name)?);
    // Body
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(arena, gen_node_with_trivia(arena, &item_body)?);

    Ok(formatted)
}

pub(crate) fn gen_struct_body<'buffer>(
    arena: &'buffer Arena,
    body: &ast::StructBody,
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    // === Parse ===
    let mut syntax = syntax_iter(body.syntax());

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::BraceLeft)?;

    let item_members = parse_many_nodes_with(
        &mut syntax,
        (Succeeding(StopAtNewline), DiscardComma, DiscardBraces),
    )
    .filter(|node| !node.is_whitespace())
    .map(|node| node.expect_kind_optional(SyntaxKind::StructMember))
    .collect::<Result<Vec<_>, _>>()?;

    parse_end(&mut syntax)?;

    // === Format ===
    let is_empty = item_members.is_empty();
    let mut formatted = PrintItemBuffer::new(arena);

    formatted.push_sc(arena, sc!("{"));
    formatted.start_indent_before_requests(arena);
    formatted.request(Request::discourage(RequestItem::EmptyLine));

    if !is_empty {
        for member in item_members {
            if member.has_content() {
                formatted.request(Request::expect(RequestItem::LineBreak));
            }

            formatted.extend(arena, gen_node_preceding_trivia(arena, &member)?);
            formatted.extend(arena, gen_node_content(arena, &member)?);
            if member.has_content() {
                formatted.push_sc(arena, sc!(","));
            }
            formatted.extend(arena, gen_node_succeeding_trivia(arena, &member)?);
        }
    }

    formatted.request(Request::Unconditional {
        discouraged: RequestItemSet::from(RequestItem::EmptyLine),
        expected: RequestItemSet::from(RequestItem::LineBreak),
        forced: RequestItemSet::empty(),
        suggest_linebreak: false,
    });

    formatted.finish_indent_before_requests(arena);
    formatted.push_sc(arena, sc!("}"));

    if !is_empty {
        formatted.request(Request::expect(RequestItem::LineBreak));
    }

    Ok(formatted)
}

pub(crate) fn gen_struct_member<'buffer>(
    arena: &'buffer Arena,
    member: &ast::StructMember,
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    // === Parse ===
    let mut syntax = syntax_iter(member.syntax());

    let item_name =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_kind(SyntaxKind::Name)?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Colon)?;
    let item_type_specifier =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_kind(SyntaxKind::TypeSpecifier)?;
    parse_end(&mut syntax)?;

    // === Format ===
    let mut formatted = PrintItemBuffer::new(arena);

    formatted.extend(arena, gen_node_with_trivia(arena, &item_name)?);
    formatted.push_sc(arena, sc!(":"));
    formatted.request(Request::expect(RequestItem::Space));
    //The colon should immediately follow the name, we intentionally move the comment
    formatted.extend(arena, gen_node_with_trivia(arena, &item_type_specifier)?);

    Ok(formatted)
}
