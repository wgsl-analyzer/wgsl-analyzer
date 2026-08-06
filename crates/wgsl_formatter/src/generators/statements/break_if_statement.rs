use dprint_core_macros::sc;
use itertools::put_back;
use parser::{SyntaxKind, SyntaxNode};
use syntax::{
    AstNode as _,
    ast::{self, Expression},
};

use crate::{
    ast_parse::{
        IgnoreBlankspace, NoTrivia, parse_end, parse_node, parse_node_with, parse_token,
        parse_token_optional,
    },
    context_policies::statement_needs_semicolon_policy,
    generators::{
        comments::{gen_comments, parse_many_comments_and_blankspace},
        expressions::gen_expression,
        node::gen_node_with_trivia,
    },
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

pub fn gen_break_if_statement(
    statement: &ast::BreakIfStatement
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(statement.syntax().children_with_tokens());
    let item_break =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(SyntaxKind::Break)?;
    let item_if = parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(SyntaxKind::If)?;
    let item_condition = parse_node::<Expression>(&mut syntax)?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind_optional(SyntaxKind::Semicolon)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_node_with_trivia(&item_break)?);
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_node_with_trivia(&item_if)?);
    formatted.start_indent();
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_expression(&item_condition)?);
    formatted.request(Request::discourage(RequestItem::Space));
    if statement_needs_semicolon_policy(statement.syntax()) {
        formatted.push_sc(sc!(";"));
    }
    formatted.finish_indent();

    Ok(formatted)
}

pub fn remove_break_if_condition_parens_rule(node: &SyntaxNode) -> bool {
    let Some(parent) = node.parent() else {
        return false;
    };
    matches!(parent.kind(), SyntaxKind::BreakIfStatement)
}
