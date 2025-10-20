use dprint_core_macros::sc;
use itertools::put_back;
use parser::{SyntaxKind, SyntaxNode};
use syntax::{
    AstNode as _,
    ast::{self, CompoundStatement, ElseClause, ElseIfClause, Expression, IfClause},
};

use crate::format::{
    ast_parse::{
        parse_end, parse_many_comments_and_blankspace, parse_node, parse_node_optional, parse_token,
    },
    gen_comments::gen_comments,
    gen_expression::gen_expression,
    gen_statement::gen_compound_statement,
    print_item_buffer::PrintItemBuffer,
    reporting::FormatDocumentResult,
};

pub fn gen_if_statement(statement: &ast::IfStatement) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(statement.syntax().children_with_tokens());
    let item_if_clause = parse_node::<IfClause>(&mut syntax)?;
    let comments_after_if_clause = parse_many_comments_and_blankspace(&mut syntax)?;

    let mut else_if_clauses = Vec::new();
    while let Some(else_if_clause) = parse_node_optional::<ElseIfClause>(&mut syntax) {
        let comments_after_else_if_clause = parse_many_comments_and_blankspace(&mut syntax)?;
        else_if_clauses.push((else_if_clause, comments_after_else_if_clause));
    }

    let item_else_clause = parse_node_optional::<ElseClause>(&mut syntax);
    parse_end(&mut syntax);

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.extend(gen_if_statement_if_clause(&item_if_clause)?);
    formatted.extend(gen_comments(comments_after_if_clause));
    for (else_if_clause, comments_after_else_if_clause) in else_if_clauses {
        formatted.expect_single_space();
        formatted.extend(gen_if_statement_else_if_clause(&else_if_clause)?);
        formatted.extend(gen_comments(comments_after_else_if_clause));
    }
    if let Some(item_else_clause) = item_else_clause {
        formatted.expect_single_space();
        formatted.extend(gen_if_statement_else_clause(&item_else_clause)?);
    }

    Ok(formatted)
}

fn gen_if_statement_if_clause(statement: &IfClause) -> FormatDocumentResult<PrintItemBuffer> {
    // NOTE: When editing this function, ensure that gen_if_statement_else_clause and gen_if_statement_else_if_clause
    // reflect the changes as well.
    // This is not very DRY, but abstraction here would introduce more complexity and probably be a leaky abstraction.

    // ==== Parse ====
    let mut syntax = put_back(statement.syntax().children_with_tokens());
    parse_token(&mut syntax, SyntaxKind::If);
    let comments_after_if = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_condition = parse_node::<Expression>(&mut syntax)?;
    let comments_after_condition = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_body = parse_node::<CompoundStatement>(&mut syntax)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("if"));
    formatted.expect_single_space();
    formatted.extend(gen_comments(comments_after_if));
    formatted.extend(gen_expression(&item_condition)?);
    formatted.extend(gen_comments(comments_after_condition));
    formatted.expect_single_space();
    formatted.extend(gen_compound_statement(&item_body)?);
    Ok(formatted)
}

fn gen_if_statement_else_clause(statement: &ElseClause) -> FormatDocumentResult<PrintItemBuffer> {
    // NOTE: When editing this function, ensure that gen_if_statement_if_clause and gen_if_statement_else_if_clause
    // reflect the changes as well.
    // This is not very DRY, but abstraction here would introduce more complexity and probably be a leaky abstraction.

    // ==== Parse ====
    let mut syntax = put_back(statement.syntax().children_with_tokens());
    parse_token(&mut syntax, SyntaxKind::Else);
    let comments_after_clause_token = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_body = parse_node::<CompoundStatement>(&mut syntax)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("else"));
    formatted.expect_single_space();
    formatted.extend(gen_comments(comments_after_clause_token));
    formatted.extend(gen_compound_statement(&item_body)?);
    Ok(formatted)
}

fn gen_if_statement_else_if_clause(
    statement: &ElseIfClause
) -> FormatDocumentResult<PrintItemBuffer> {
    // NOTE: When editing this function, ensure that gen_if_statement_if_clause and gen_if_statement_else_clause
    // reflect the changes as well.
    // This is not very DRY, but abstraction here would introduce more complexity and probably be a leaky abstraction.

    // ==== Parse ====
    let mut syntax = put_back(statement.syntax().children_with_tokens());
    parse_token(&mut syntax, SyntaxKind::Else);
    let comments_after_else = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::If);
    let comments_after_if = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_condition = parse_node::<Expression>(&mut syntax)?;
    let comments_after_condition = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_body = parse_node::<CompoundStatement>(&mut syntax)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("else"));
    formatted.expect_single_space();
    formatted.extend(gen_comments(comments_after_else));
    formatted.push_sc(sc!("if"));
    formatted.expect_single_space();
    formatted.extend(gen_comments(comments_after_if));
    formatted.extend(gen_expression(&item_condition)?);
    formatted.extend(gen_comments(comments_after_condition));
    formatted.expect_single_space();
    formatted.extend(gen_compound_statement(&item_body)?);
    Ok(formatted)
}
