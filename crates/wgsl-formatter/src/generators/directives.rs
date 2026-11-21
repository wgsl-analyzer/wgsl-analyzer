use dprint_core::formatting::Arena;
use dprint_core_macros::sc;
use itertools::Itertools as _;
use syntax::{AstNode as _, SyntaxKind, ast};

use crate::{
    ast_parse::{
        DiscardBlankspace, DiscardComma, DiscardSemicolon, NoTrivia, parse_end,
        parse_many_nodes_with, parse_node_with, syntax_iter,
    },
    generators::node::gen_node_with_trivia,
    multiline_group::MultilineGroup,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};
pub(crate) fn gen_enable_extension_name<'buffer>(
    arena: &'buffer Arena,
    node: &ast::EnableExtensionName,
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    let mut syntax = syntax_iter(node.syntax());
    let identifier =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_kind(SyntaxKind::Identifier)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new(arena);
    formatted.extend(arena, gen_node_with_trivia(arena, &identifier)?);
    Ok(formatted)
}

pub(crate) fn gen_enable_directive<'buffer>(
    arena: &'buffer Arena,
    node: &ast::EnableDirective,
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    let mut syntax = syntax_iter(node.syntax());

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Enable)?;

    let items = parse_many_nodes_with(
        &mut syntax,
        (DiscardBlankspace, DiscardComma, DiscardSemicolon),
    )
    .filter(|node| !node.is_whitespace())
    .collect::<Vec<_>>();

    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new(arena);
    formatted.push_sc(arena, sc!("enable"));
    let mut multiline_group = MultilineGroup::new_before_requests(arena, &mut formatted);
    multiline_group.start_indent_before_requests(arena, );

    multiline_group.request(Request::expect(RequestItem::Space));

    for (position, item) in items.into_iter().with_position() {
        multiline_group.grouped_newline_or_space();

        multiline_group.extend(arena, gen_node_with_trivia(arena, &item)?);
        if !position.is_last && !position.is_exactly_one() {
            multiline_group.push_sc(arena, sc!(","));
        }
    }

    multiline_group.request(Request::discourage(RequestItem::Space));
    multiline_group.push_sc(arena, sc!(";"));

    multiline_group.finish_indent_before_requests(arena);
    multiline_group.end_before_requests(arena);

    Ok(formatted)
}

pub(crate) fn gen_language_extension_name<'buffer>(
    arena: &'buffer Arena,
    node: &ast::LanguageExtensionName,
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    let mut syntax = syntax_iter(node.syntax());
    let identifier =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_kind(SyntaxKind::Identifier)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new(arena);
    formatted.extend(arena, gen_node_with_trivia(arena, &identifier)?);
    Ok(formatted)
}

pub(crate) fn gen_requires_directive<'buffer>(
    arena: &'buffer Arena,
    node: &ast::RequiresDirective,
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    let mut syntax = syntax_iter(node.syntax());

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Requires)?;

    let items = parse_many_nodes_with(
        &mut syntax,
        (DiscardBlankspace, DiscardComma, DiscardSemicolon),
    )
    .filter(|node| !node.is_whitespace())
    .collect::<Vec<_>>();

    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new(arena);
    formatted.push_sc(arena, sc!("requires"));

    let mut multiline_group = MultilineGroup::new_before_requests(arena, &mut formatted);
    multiline_group.start_indent_before_requests(arena, );

    multiline_group.request(Request::expect(RequestItem::Space));

    for (position, item) in items.into_iter().with_position() {
        multiline_group.grouped_newline_or_space();

        multiline_group.extend(arena, gen_node_with_trivia(arena, &item)?);
        if !position.is_last && !position.is_exactly_one() {
            multiline_group.push_sc(arena, sc!(","));
        }
    }

    multiline_group.request(Request::discourage(RequestItem::Space));
    multiline_group.push_sc(arena, sc!(";"));

    multiline_group.finish_indent_before_requests(arena);
    multiline_group.end_before_requests(arena);

    Ok(formatted)
}

pub(crate) fn gen_diagnostic_directive<'buffer>(
    arena: &'buffer Arena,
    node: &ast::DiagnosticDirective,
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    let mut syntax = syntax_iter(node.syntax());

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Diagnostic)?;
    let item_control = parse_node_with(&mut syntax, DiscardBlankspace)
        .expect_kind(SyntaxKind::DiagnosticControl)?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Semicolon)?; //Make optional
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::new(arena);
    formatted.push_sc(arena, sc!("diagnostic"));
    formatted.extend(arena, gen_node_with_trivia(arena, &item_control)?);
    formatted.push_sc(arena, sc!(";"));
    Ok(formatted)
}
