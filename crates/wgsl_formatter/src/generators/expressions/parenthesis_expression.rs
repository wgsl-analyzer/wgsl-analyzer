use dprint_core_macros::sc;
use parser::{SyntaxKind, SyntaxNode};
use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::{
    ast_parse::{IgnoreBlankspace, NoTrivia, parse_end, parse_node_with, syntax_iter},
    context_policies::expression_parens_are_irrelevant_policy,
    generators::node::gen_node_with_trivia,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

pub fn gen_parenthesis_expression(
    parenthesis_expression: &ast::ParenthesisExpression
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = syntax_iter(parenthesis_expression.syntax());

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::ParenthesisLeft)?;
    let item_content =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_castable_kind::<ast::Expression>()?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::ParenthesisRight)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    if expression_parens_are_irrelevant_policy(parenthesis_expression.syntax()) {
        formatted.request(Request::expect(RequestItem::Space));
    } else {
        formatted.push_sc(sc!("("));
        formatted.start_new_line_group();
        formatted.start_indent_before_requests();

        formatted.request(Request::discourage(RequestItem::Space));
    }
    formatted.extend(gen_node_with_trivia(&item_content)?);

    if expression_parens_are_irrelevant_policy(parenthesis_expression.syntax()) {
        formatted.request(Request::expect(RequestItem::Space));
    } else {
        formatted.request(Request::discourage(RequestItem::Space));
        formatted.finish_indent_before_requests();
        formatted.finish_new_line_group();
        formatted.push_sc(sc!(")"));
    }
    Ok(formatted)
}

pub fn remove_nested_parens_rule(node: &SyntaxNode) -> bool {
    let Some(parent) = node.parent() else {
        return false;
    };
    matches!(parent.kind(), SyntaxKind::ParenthesisExpression)
}
