use dprint_core_macros::sc;
use itertools::put_back;
use parser::{SyntaxKind, SyntaxNode};
use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::{
    ast_parse::{parse_end, parse_node, parse_token},
    context_policies::expression_parens_are_irrelevant_policy,
    generators::{
        comments::{gen_comments, parse_many_comments_and_blankspace},
        expressions::gen_expression,
    },
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
    let mut syntax = put_back(parenthesis_expression.syntax().children_with_tokens());
    parse_token(&mut syntax, parser::SyntaxKind::ParenthesisLeft)?;
    let item_comment_after_left_paren = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_content = parse_node::<ast::Expression>(&mut syntax)?;
    let item_comment_after_content = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, parser::SyntaxKind::ParenthesisRight)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    if expression_parens_are_irrelevant_policy(parenthesis_expression.syntax()) {
        formatted.request(Request::expect(RequestItem::Space));
    } else {
        formatted.push_sc(sc!("("));
        formatted.start_new_line_group();
        formatted.start_indent();

        formatted.request(Request::discourage(RequestItem::Space));
    }
    formatted.extend(gen_comments(&item_comment_after_left_paren));
    formatted.extend(gen_expression(&item_content)?);
    formatted.extend(gen_comments(&item_comment_after_content));

    if expression_parens_are_irrelevant_policy(parenthesis_expression.syntax()) {
        formatted.request(Request::expect(RequestItem::Space));
    } else {
        formatted.request(Request::discourage(RequestItem::Space));
        formatted.finish_indent();
        formatted.finish_new_line_group();
        formatted.push_sc(sc!(")"));
    }
    Ok(formatted)
}

pub fn remove_nested_parenthesis(node: &SyntaxNode) -> bool {
    let Some(parent) = node.parent() else {
        return false;
    };
    match parent.kind() {
        SyntaxKind::ParenthesisExpression => true,
        _ => false,
    }
}
