use dprint_core_macros::sc;
use itertools::put_back;
use parser::{SyntaxKind, SyntaxNode};
use syntax::{AstNode as _, ast};

use crate::format::{
    ast_parse::{
        parse_end, parse_many_comments_and_blankspace, parse_node, parse_node_by_kind, parse_token,
        parse_token_optional,
    },
    gen_comments::gen_comments,
    print_item_buffer::PrintItemBuffer,
    reporting::FormatDocumentResult,
};

use super::print_item_buffer::request_folder::RequestItem;

pub fn gen_diagnostic_control(
    node: &ast::DiagnosticControl
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = put_back(node.syntax().children_with_tokens());

    parse_token(&mut syntax, SyntaxKind::ParenthesisLeft)?;
    let item_comments_after_open = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_control_name = parse_node::<ast::SeverityControlName>(&mut syntax)?;
    let item_comments_after_name = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::Comma)?;
    let item_comments_after_comma = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_rule_name = parse_node::<ast::DiagnosticRuleName>(&mut syntax)?;
    let item_comments_after_rule = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::ParenthesisRight)?;
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("("));
    formatted.extend(gen_comments(&item_comments_after_open));
    formatted.extend(gen_severity_control_name(&item_control_name)?);
    formatted.extend(gen_comments(&item_comments_after_name));
    formatted.push_sc(sc!(","));
    formatted.expect(RequestItem::Space);
    formatted.extend(gen_comments(&item_comments_after_comma));
    formatted.extend(gen_diagnostic_rule_name(&item_rule_name)?);
    formatted.extend(gen_comments(&item_comments_after_rule));
    formatted.push_sc(sc!(")"));
    Ok(formatted)
}

pub fn gen_severity_control_name(
    node: &ast::SeverityControlName
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = put_back(node.syntax().children_with_tokens());
    let item_identifier = parse_token(&mut syntax, SyntaxKind::Identifier)?;
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::new();
    formatted.push_string(item_identifier.text().to_owned());
    Ok(formatted)
}

pub fn gen_diagnostic_rule_name(
    node: &ast::DiagnosticRuleName
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = put_back(node.syntax().children_with_tokens());

    let item_control_first = parse_token(&mut syntax, SyntaxKind::Identifier)?;
    let item_comments_after_first = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token_optional(&mut syntax, SyntaxKind::Period);

    let item_comments_after_period = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_control_second = parse_token_optional(&mut syntax, SyntaxKind::Identifier);
    let item_comments_after_second = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::new();
    formatted.push_string(item_control_first.text().to_owned());
    formatted.extend(gen_comments(&item_comments_after_first));
    if item_control_second.is_some() {
        formatted.push_sc(sc!("."));
    }
    formatted.extend(gen_comments(&item_comments_after_period));
    if let Some(item_control_second) = item_control_second {
        formatted.push_string(item_control_second.text().to_owned());
    }
    formatted.extend(gen_comments(&item_comments_after_second));
    Ok(formatted)
}
