use dprint_core_macros::sc;
use itertools::put_back;
use parser::SyntaxKind;
use syntax::{AstNode as _, ast};

use crate::format::{
    ast_parse::{
        parse_end, parse_many_comments_and_blankspace, parse_node, parse_node_by_kind, parse_token,
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
    let item_control_name = parse_node_by_kind(&mut syntax, SyntaxKind::SeverityControlName)?;
    let item_comments_after_name = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::Comma)?;
    let item_comments_after_comma = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_rule_name = parse_node_by_kind(&mut syntax, SyntaxKind::DiagnosticRuleName)?;
    let item_comments_after_rule = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::ParenthesisRight)?;
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("("));
    formatted.extend(gen_comments(&item_comments_after_open));
    formatted.push_string(item_control_name.to_string());
    formatted.extend(gen_comments(&item_comments_after_name));
    formatted.push_sc(sc!(","));
    formatted.expect(RequestItem::Space);
    formatted.extend(gen_comments(&item_comments_after_comma));
    formatted.push_string(item_rule_name.to_string());
    formatted.extend(gen_comments(&item_comments_after_rule));
    formatted.push_sc(sc!(")"));
    Ok(formatted)
}
