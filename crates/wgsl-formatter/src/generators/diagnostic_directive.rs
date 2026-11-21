use dprint_core::formatting::Arena;
use dprint_core_macros::sc;
use syntax::{AstNode as _, SyntaxKind, ast};

use crate::{
    ast_parse::{DiscardBlankspace, NoTrivia, parse_end, parse_node_with, syntax_iter},
    generators::node::gen_node_with_trivia,
    multiline_group::MultilineGroup,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

pub(crate) fn gen_diagnostic_control<'buffer>(
    arena: &'buffer Arena,
    node: &ast::DiagnosticControl,
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    let mut syntax = syntax_iter(node.syntax());

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::ParenthesisLeft)?;
    let item_control_name = parse_node_with(&mut syntax, DiscardBlankspace)
        .expect_kind(SyntaxKind::SeverityControlName)?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Comma)?;
    let item_rule_name = parse_node_with(&mut syntax, DiscardBlankspace)
        .expect_kind(SyntaxKind::DiagnosticRuleName)?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::ParenthesisRight)?;
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::new(arena);
    let mut multiline_group = MultilineGroup::new_before_requests(arena, &mut formatted);
    multiline_group.push_sc(arena, sc!("("));
    multiline_group.start_indent_before_requests(arena);
    multiline_group.grouped_possible_newline();
    multiline_group.request(Request::discourage(RequestItem::EmptyLine));
    multiline_group.request(Request::discourage(RequestItem::Space));
    multiline_group.extend(arena, gen_node_with_trivia(arena, &item_control_name)?);
    multiline_group.push_sc(arena, sc!(","));
    multiline_group.grouped_newline_or_space();
    multiline_group.extend(arena, gen_node_with_trivia(arena, &item_rule_name)?);
    multiline_group.finish_indent_before_requests(arena);
    multiline_group.grouped_possible_newline();
    multiline_group.push_sc(arena, sc!(")"));
    multiline_group.end_before_requests(arena);
    Ok(formatted)
}

pub(crate) fn gen_severity_control_name<'buffer>(
    arena: &'buffer Arena,
    node: &ast::SeverityControlName,
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    let mut syntax = syntax_iter(node.syntax());
    let item_identifier =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_kind(SyntaxKind::Identifier)?;
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::new(arena);
    formatted.extend(arena, gen_node_with_trivia(arena, &item_identifier)?);
    Ok(formatted)
}

pub(crate) fn gen_diagnostic_rule_name<'buffer>(
    arena: &'buffer Arena,
    node: &ast::DiagnosticRuleName,
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    let mut syntax = syntax_iter(node.syntax());

    let item_control_first =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_kind(SyntaxKind::Identifier)?;

    let item_period =
        parse_node_with(&mut syntax, NoTrivia).only_if_kind(SyntaxKind::Period, &mut syntax);

    let item_control_second = if item_period.is_some() {
        let item_control_second =
            parse_node_with(&mut syntax, DiscardBlankspace).expect_kind(SyntaxKind::Identifier)?;
        Some(item_control_second)
    } else {
        None
    };
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::new(arena);
    formatted.extend(arena, gen_node_with_trivia(arena, &item_control_first)?);
    if let Some(item_control_second) = item_control_second {
        formatted.push_sc(arena, sc!("."));
        formatted.extend(arena, gen_node_with_trivia(arena, &item_control_second)?);
    }
    Ok(formatted)
}
