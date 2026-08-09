use dprint_core_macros::sc;
use itertools::{Itertools as _, Position};
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::{
    ast_parse::{
        Filter, FilterAction, IgnoreBlankspace, NoTrivia, parse_end, parse_node_with, syntax_iter,
    },
    generators::node::gen_node_with_trivia,
    multiline_group::MultilineGroup,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
    trivia::NodeWithTriviaContent,
};
pub fn gen_enable_extension_name(
    node: &ast::EnableExtensionName
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = syntax_iter(node.syntax());
    let identifier =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(SyntaxKind::Identifier)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_node_with_trivia(&identifier)?);
    Ok(formatted)
}

pub fn gen_enable_directive(node: &ast::EnableDirective) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = syntax_iter(node.syntax());

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Enable)?;

    let mut items = Vec::new();

    loop {
        let mut item = parse_node_with(
            &mut syntax,
            Filter(|node| match node.kind() {
                SyntaxKind::Blankspace | SyntaxKind::Comma => Some(FilterAction::Ignored),
                _ => None,
            }),
        );
        //.expect_kind_optional(SyntaxKind::EnableExtensionName)?;

        // TODO This needs to be absorbed into parse_node..
        if matches!(item.kind(), Some(SyntaxKind::Semicolon)) {
            let old_node = std::mem::replace(&mut item.node, NodeWithTriviaContent::End);
            syntax.put_back(old_node.into_option().unwrap()); //TODO
        }

        let is_end = item.is_end();
        if !item.is_whitespace() {
            items.push(item);
        }
        if is_end {
            break;
        }
    }

    parse_node_with(&mut syntax, NoTrivia).expect_kind_optional(SyntaxKind::Semicolon)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.push_sc(sc!("enable"));
    formatted.request(Request::expect(RequestItem::Space));

    let mut multiline_group = MultilineGroup::new(&mut formatted);
    multiline_group.start_indent();

    for (position, item) in items.into_iter().with_position() {
        multiline_group.grouped_newline_or_space();

        multiline_group.extend(gen_node_with_trivia(&item)?);
        if position != Position::Last && position != Position::Only {
            multiline_group.push_sc(sc!(","));
        }
    }

    multiline_group.request(Request::discourage(RequestItem::Space));
    multiline_group.push_sc(sc!(";"));

    multiline_group.finish_indent();
    multiline_group.end();

    Ok(formatted)
}

pub fn gen_language_extension_name(
    node: &ast::LanguageExtensionName
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = syntax_iter(node.syntax());
    let identifier =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(SyntaxKind::Identifier)?;
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

    let mut items = Vec::new();

    loop {
        let mut item = parse_node_with(
            &mut syntax,
            Filter(|node| match node.kind() {
                SyntaxKind::Blankspace | SyntaxKind::Comma => Some(FilterAction::Ignored),
                _ => None,
            }),
        );
        //.expect_kind_optional(SyntaxKind::LanguageExtensionName)?;

        // TODO This needs to be absorbed into parse_node..
        if matches!(item.kind(), Some(SyntaxKind::Semicolon)) {
            let old_node = std::mem::replace(&mut item.node, NodeWithTriviaContent::End);
            syntax.put_back(old_node.into_option().unwrap()); //TODO
        }

        let is_end = item.is_end();
        if !item.is_whitespace() {
            items.push(item);
        }
        if is_end {
            break;
        }
    }

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Semicolon)?; //Optionalize
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.push_sc(sc!("requires"));
    formatted.request(Request::expect(RequestItem::Space));

    let mut multiline_group = MultilineGroup::new(&mut formatted);
    multiline_group.start_indent();

    for (position, item) in items.into_iter().with_position() {
        multiline_group.grouped_newline_or_space();

        multiline_group.extend(gen_node_with_trivia(&item)?);
        if position != Position::Last && position != Position::Only {
            multiline_group.push_sc(sc!(","));
        }
    }

    multiline_group.request(Request::discourage(RequestItem::Space));
    multiline_group.push_sc(sc!(";"));

    multiline_group.finish_indent();
    multiline_group.end();

    Ok(formatted)
}

pub fn gen_diagnostic_directive(
    node: &ast::DiagnosticDirective
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = syntax_iter(node.syntax());

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Diagnostic)?;
    let item_control = parse_node_with(&mut syntax, IgnoreBlankspace)
        .expect_kind(SyntaxKind::DiagnosticControl)?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Semicolon)?; //Make optional
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::default();
    formatted.push_sc(sc!("diagnostic"));
    formatted.extend(gen_node_with_trivia(&item_control)?);
    formatted.push_sc(sc!(";"));
    Ok(formatted)
}
