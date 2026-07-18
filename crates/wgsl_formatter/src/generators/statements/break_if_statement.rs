use dprint_core_macros::sc;
use itertools::put_back;
use parser::{SyntaxKind, SyntaxNode};
use syntax::{
    AstNode as _,
    ast::{self, Expression},
};

use crate::{
    ast_parse::{parse_end, parse_node, parse_token, parse_token_optional},
    context_policies::statement_needs_semicolon_policy,
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

pub fn gen_break_if_statement(
    statement: &ast::BreakIfStatement
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(statement.syntax().children_with_tokens());
    parse_token(&mut syntax, SyntaxKind::Break)?;
    let comments_after_break = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::If)?;
    let comments_after_if = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_condition = parse_node::<Expression>(&mut syntax)?;
    let comments_after_condition = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token_optional(&mut syntax, SyntaxKind::Semicolon);
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.push_sc(sc!("break"));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_comments(&comments_after_break));
    formatted.push_sc(sc!("if"));
    formatted.start_indent();
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_comments(&comments_after_if));
    formatted.extend(gen_expression(&item_condition)?);
    formatted.extend(gen_comments(&comments_after_condition));
    formatted.request(Request::discourage(RequestItem::Space));
    if statement_needs_semicolon_policy(statement.syntax()) {
        formatted.push_sc(sc!(";"));
    }
    formatted.finish_indent();

    Ok(formatted)
}

pub fn remove_break_if_condition_parens(node: &SyntaxNode) -> bool {
    let Some(parent) = node.parent() else {
        return false;
    };
    match parent.kind() {
        SyntaxKind::BreakIfStatement => true,
        _ => false,
    }
}
