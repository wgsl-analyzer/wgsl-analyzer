use dprint_core_macros::sc;
use parser::SyntaxKind;
use syntax::{AstNode as _, ast};

use crate::{
    ast_parse::{IgnoreBlankspace, NoTrivia, parse_end, parse_node_with, syntax_iter},
    generators::node::gen_node_with_trivia,
    multiline_group::MultilineGroup,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

pub fn gen_diagnostic_control(
    node: &ast::DiagnosticControl
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = syntax_iter(node.syntax());

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::ParenthesisLeft)?;
    let item_control_name = parse_node_with(&mut syntax, IgnoreBlankspace)
        .expect_kind(SyntaxKind::SeverityControlName)?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Comma)?;
    let item_rule_name = parse_node_with(&mut syntax, IgnoreBlankspace)
        .expect_kind(SyntaxKind::DiagnosticRuleName)?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::ParenthesisRight)?;
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::default();
    let mut multiline_group = MultilineGroup::new_before_requests(&mut formatted);
    multiline_group.push_sc(sc!("("));
    multiline_group.start_indent_before_requests();
    multiline_group.grouped_possible_newline();
    multiline_group.extend(gen_node_with_trivia(&item_control_name)?);
    multiline_group.push_sc(sc!(","));
    multiline_group.grouped_newline_or_space();
    multiline_group.extend(gen_node_with_trivia(&item_rule_name)?);
    multiline_group.finish_indent();
    multiline_group.grouped_possible_newline();
    multiline_group.push_sc(sc!(")"));
    multiline_group.end_before_requests();
    Ok(formatted)
}

pub fn gen_severity_control_name(
    node: &ast::SeverityControlName
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = syntax_iter(node.syntax());
    let item_identifier =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(SyntaxKind::Identifier)?;
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_node_with_trivia(&item_identifier)?);
    Ok(formatted)
}

pub fn gen_diagnostic_rule_name(
    node: &ast::DiagnosticRuleName
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = syntax_iter(node.syntax());

    let item_control_first =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(SyntaxKind::Identifier)?;

    let item_period =
        parse_node_with(&mut syntax, NoTrivia).only_if_kind(SyntaxKind::Period, &mut syntax);

    let item_control_second = if item_period.is_some() {
        let item_control_second =
            parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(SyntaxKind::Identifier)?;
        Some(item_control_second)
    } else {
        None
    };
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_node_with_trivia(&item_control_first)?);
    if let Some(item_control_second) = item_control_second {
        formatted.push_sc(sc!("."));
        formatted.extend(gen_node_with_trivia(&item_control_second)?);
    }
    Ok(formatted)
}
