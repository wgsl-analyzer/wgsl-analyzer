use dprint_core_macros::sc;
use itertools::{Itertools as _, Position};
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self},
};

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
pub fn gen_enable_extension_name(
    node: &ast::EnableExtensionName
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = syntax_iter(node.syntax());
    let identifier =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_kind(SyntaxKind::Identifier)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_node_with_trivia(&identifier)?);
    Ok(formatted)
}

pub fn gen_enable_directive(node: &ast::EnableDirective) -> FormatDocumentResult<PrintItemBuffer> {
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
    let mut formatted = PrintItemBuffer::default();
    formatted.push_sc(sc!("enable"));
    let mut multiline_group = MultilineGroup::new_before_requests(&mut formatted);
    multiline_group.start_indent_before_requests();

    multiline_group.request(Request::expect(RequestItem::Space));

    for (position, item) in items.into_iter().with_position() {
        multiline_group.grouped_newline_or_space();

        multiline_group.extend(gen_node_with_trivia(&item)?);
        if position != Position::Last && position != Position::Only {
            multiline_group.push_sc(sc!(","));
        }
    }

    multiline_group.request(Request::discourage(RequestItem::Space));
    multiline_group.push_sc(sc!(";"));

    multiline_group.finish_indent_before_requests();
    multiline_group.end_before_requests();

    Ok(formatted)
}

pub fn gen_language_extension_name(
    node: &ast::LanguageExtensionName
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = syntax_iter(node.syntax());
    let identifier =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_kind(SyntaxKind::Identifier)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_node_with_trivia(&identifier)?);
    Ok(formatted)
}

pub fn gen_requires_directive(
    node: &ast::RequiresDirective
) -> FormatDocumentResult<PrintItemBuffer> {
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
    let mut formatted = PrintItemBuffer::default();
    formatted.push_sc(sc!("requires"));

    let mut multiline_group = MultilineGroup::new_before_requests(&mut formatted);
    multiline_group.start_indent_before_requests();

    multiline_group.request(Request::expect(RequestItem::Space));

    for (position, item) in items.into_iter().with_position() {
        multiline_group.grouped_newline_or_space();

        multiline_group.extend(gen_node_with_trivia(&item)?);
        if position != Position::Last && position != Position::Only {
            multiline_group.push_sc(sc!(","));
        }
    }

    multiline_group.request(Request::discourage(RequestItem::Space));
    multiline_group.push_sc(sc!(";"));

    multiline_group.finish_indent_before_requests();
    multiline_group.end_before_requests();

    Ok(formatted)
}

pub fn gen_diagnostic_directive(
    node: &ast::DiagnosticDirective
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = syntax_iter(node.syntax());

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Diagnostic)?;
    let item_control = parse_node_with(&mut syntax, DiscardBlankspace)
        .expect_kind(SyntaxKind::DiagnosticControl)?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Semicolon)?; //Make optional
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::default();
    formatted.push_sc(sc!("diagnostic"));
    formatted.extend(gen_node_with_trivia(&item_control)?);
    formatted.push_sc(sc!(";"));
    Ok(formatted)
}
