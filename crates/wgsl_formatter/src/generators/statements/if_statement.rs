use dprint_core_macros::sc;
use itertools::Itertools as _;
use parser::{SyntaxKind, SyntaxNode};
use syntax::{
    AstNode as _,
    ast::{self, CompoundStatement, ElseClause, ElseIfClause, Expression, IfClause},
};

use crate::{
    ast_parse::{
        DiscardBlankspace, Filter, NoTrivia, PolicyAction, parse_end, parse_many_nodes_with,
        parse_node_with, syntax_iter,
    },
    generators::node::gen_node_with_trivia,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem, RequestItemSet},
    },
    reporting::FormatDocumentResult,
};

pub fn gen_if_statement(statement: &ast::IfStatement) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = syntax_iter(statement.syntax());

    let item_if_clause =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_kind(SyntaxKind::IfClause)?;

    let else_if_clauses = parse_many_nodes_with(
        &mut syntax,
        (
            DiscardBlankspace,
            Filter(|node| {
                (!matches!(node.kind(), SyntaxKind::ElseIfClause)).then_some(PolicyAction::MarkEnd)
            }),
        ),
    )
    .filter(|node| !node.is_whitespace())
    .collect_vec();

    let item_else_clause = parse_node_with(&mut syntax, DiscardBlankspace)
        .expect_kind_optional(SyntaxKind::ElseClause)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_node_with_trivia(&item_if_clause)?);
    for else_if_clause in else_if_clauses {
        formatted.request(Request::Unconditional {
            expected: RequestItemSet::from(RequestItem::Space),
            discouraged: RequestItemSet::from(RequestItem::LineBreak),
            forced: RequestItemSet::empty(),
            suggest_linebreak: false,
        });
        formatted.extend(gen_node_with_trivia(&else_if_clause)?);
    }
    if !item_else_clause.is_whitespace() {
        formatted.request(Request::Unconditional {
            expected: RequestItemSet::from(RequestItem::Space),
            discouraged: RequestItemSet::from(RequestItem::LineBreak),
            forced: RequestItemSet::empty(),
            suggest_linebreak: false,
        });
        formatted.extend(gen_node_with_trivia(&item_else_clause)?);
    }

    Ok(formatted)
}

pub fn gen_if_statement_if_clause(statement: &IfClause) -> FormatDocumentResult<PrintItemBuffer> {
    // NOTE: When editing this function, ensure that gen_if_statement_else_clause and gen_if_statement_else_if_clause
    // reflect the changes as well.
    // This is not very DRY, but abstraction here would introduce more complexity and probably be a leaky abstraction.

    // ==== Parse ====
    let mut syntax = syntax_iter(statement.syntax());
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::If)?;
    let item_condition =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_ast_node::<Expression>()?;
    let item_body =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_ast_node::<CompoundStatement>()?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.push_sc(sc!("if"));
    formatted.start_indent_before_requests();
    formatted.request(Request::expect(RequestItem::Space));
    formatted.request(Request::discourage(RequestItem::LineBreak));
    formatted.extend(gen_node_with_trivia(&item_condition)?);
    formatted.finish_indent_before_requests();
    formatted.request(Request::discourage(RequestItem::LineBreak));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_node_with_trivia(&item_body)?);
    Ok(formatted)
}

pub fn gen_if_statement_else_clause(
    statement: &ElseClause
) -> FormatDocumentResult<PrintItemBuffer> {
    // NOTE: When editing this function, ensure that gen_if_statement_if_clause and gen_if_statement_else_if_clause
    // reflect the changes as well.
    // This is not very DRY, but abstraction here would introduce more complexity and probably be a leaky abstraction.

    // ==== Parse ====
    let mut syntax = syntax_iter(statement.syntax());
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Else)?;
    let item_body =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_ast_node::<CompoundStatement>()?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.push_sc(sc!("else"));
    formatted.request(Request::discourage(RequestItem::LineBreak));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_node_with_trivia(&item_body)?);
    Ok(formatted)
}

pub fn gen_if_statement_else_if_clause(
    statement: &ElseIfClause
) -> FormatDocumentResult<PrintItemBuffer> {
    // NOTE: When editing this function, ensure that gen_if_statement_if_clause and gen_if_statement_else_clause
    // reflect the changes as well.
    // This is not very DRY, but abstraction here would introduce more complexity and probably be a leaky abstraction.

    // ==== Parse ====
    let mut syntax = syntax_iter(statement.syntax());
    let item_else =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_kind(SyntaxKind::Else)?;
    let item_if = parse_node_with(&mut syntax, DiscardBlankspace).expect_kind(SyntaxKind::If)?;
    let item_condition =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_ast_node::<Expression>()?;
    let item_body =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_ast_node::<CompoundStatement>()?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_node_with_trivia(&item_else)?);
    formatted.request(Request::discourage(RequestItem::LineBreak));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_node_with_trivia(&item_if)?);
    formatted.start_indent_before_requests();
    formatted.request(Request::discourage(RequestItem::LineBreak));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_node_with_trivia(&item_condition)?);
    formatted.finish_indent_before_requests();
    formatted.request(Request::discourage(RequestItem::LineBreak));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_node_with_trivia(&item_body)?);
    Ok(formatted)
}

#[must_use]
pub fn remove_if_condition_parens_rule(node: &SyntaxNode) -> bool {
    let Some(parent) = node.parent() else {
        return false;
    };
    matches!(
        parent.kind(),
        SyntaxKind::ElseIfClause | SyntaxKind::IfClause
    )
}
