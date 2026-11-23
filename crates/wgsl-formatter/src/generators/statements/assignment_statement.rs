use dprint_core::formatting::Arena;
use dprint_core_macros::sc;
use syntax::{
    AstNode as _, SyntaxKind, SyntaxNode,
    ast::{self, CompoundAssignmentOperator, Expression},
};

use crate::{
    ast_parse::{DiscardBlankspace, NoTrivia, parse_end, parse_node_with, syntax_iter},
    context_policies::statement_needs_semicolon_policy,
    generators::node::gen_node_with_trivia,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentError,
};

pub(crate) fn gen_assignment_statement<'buffer>(
    arena: &'buffer Arena,
    assignment_statement: &ast::AssignmentStatement,
) -> Result<PrintItemBuffer<'buffer>, FormatDocumentError> {
    // NOTE!! - When updating this function, keep in mind to
    // update gen_assignment_statement, gen_compound_assignment_statement, gen_phony_assignment_statement together
    // This is non-dry code, but when inevitably at some point there will be some differences between
    // them, this should clearly communicate that they should be split up and not
    // continue to be one function with a whole lot of parameters and ifs.

    // ==== Parse ====
    let mut syntax = syntax_iter(assignment_statement.syntax());
    let item_target =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_ast_node::<Expression>()?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Equal)?;
    let item_value =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_ast_node::<Expression>()?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind_optional(SyntaxKind::Semicolon)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new(arena);
    formatted.extend(arena, gen_node_with_trivia(arena, &item_target)?);
    formatted.request(Request::expect(RequestItem::Space));
    formatted.start_new_line_group_before_requests(arena);
    formatted.push_sc(arena, sc!("="));
    formatted.request(Request::expect(RequestItem::Space).or_newline());
    formatted.finish_new_line_group_after_requests(arena);
    formatted.start_indent_before_requests(arena);
    formatted.extend(arena, gen_node_with_trivia(arena, &item_value)?);
    if statement_needs_semicolon_policy(assignment_statement.syntax()) {
        formatted.request(Request::discourage(RequestItem::Space));
        formatted.push_sc(arena, sc!(";"));
    }
    formatted.finish_indent_before_requests(arena);
    Ok(formatted)
}

pub(crate) fn gen_phony_assignment_statement<'buffer>(
    arena: &'buffer Arena,
    phony_assignment_statement: &ast::PhonyAssignmentStatement,
) -> Result<PrintItemBuffer<'buffer>, FormatDocumentError> {
    // NOTE!! - When updating this function, keep in mind to
    // update gen_assignment_statement, gen_compound_assignment_statement, gen_phony_assignment_statement together
    // This is non-dry code, but when inevitably at some point there will be some differences between
    // them, this should clearly communicate that they should be split up and not
    // continue to be one function with a whole lot of parameters and ifs.

    // ==== Parse ====
    let mut syntax = syntax_iter(phony_assignment_statement.syntax());
    let item_phony =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_kind(SyntaxKind::Underscore)?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Equal)?;
    let item_value =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_ast_node::<Expression>()?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind_optional(SyntaxKind::Semicolon)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new(arena);
    formatted.extend(arena, gen_node_with_trivia(arena, &item_phony)?);
    formatted.request(Request::expect(RequestItem::Space));
    formatted.push_sc(arena, sc!("="));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.start_indent_before_requests(arena);
    formatted.extend(arena, gen_node_with_trivia(arena, &item_value)?);
    if statement_needs_semicolon_policy(phony_assignment_statement.syntax()) {
        formatted.request(Request::discourage(RequestItem::Space));
        formatted.push_sc(arena, sc!(";"));
    }
    formatted.finish_indent_before_requests(arena);
    Ok(formatted)
}

pub(crate) fn gen_compound_assignment_statement<'buffer>(
    arena: &'buffer Arena,
    compound_assignment_statement: &ast::CompoundAssignmentStatement,
) -> Result<PrintItemBuffer<'buffer>, FormatDocumentError> {
    // NOTE!! - When updating this function, keep in mind to
    // update gen_assignment_statement, gen_compound_assignment_statement, gen_phony_assignment_statement together
    // This is non-dry code, but when inevitably at some point there will be some differences between
    // them, this should clearly communicate that they should be split up and not
    // continue to be one function with a whole lot of parameters and ifs.

    // ==== Parse ====
    let mut syntax = syntax_iter(compound_assignment_statement.syntax());
    let item_target =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_ast_node::<Expression>()?;
    let item_operator =
        parse_node_with(&mut syntax, NoTrivia).expect_ast_token::<CompoundAssignmentOperator>()?;
    let item_value =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_ast_node::<Expression>()?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind_optional(SyntaxKind::Semicolon)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new(arena);
    formatted.extend(arena, gen_node_with_trivia(arena, &item_target)?);
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(arena, gen_node_with_trivia(arena, &item_operator)?);
    formatted.request(Request::expect(RequestItem::Space));
    formatted.start_indent_before_requests(arena);
    formatted.extend(arena, gen_node_with_trivia(arena, &item_value)?);
    if statement_needs_semicolon_policy(compound_assignment_statement.syntax()) {
        formatted.request(Request::discourage(RequestItem::Space));
        formatted.push_sc(arena, sc!(";"));
    }
    formatted.finish_indent_before_requests(arena);
    Ok(formatted)
}

#[must_use]
pub(crate) fn remove_assignment_statement_parens_rule(node: &SyntaxNode) -> bool {
    let Some(parent) = node.parent() else {
        return false;
    };
    matches!(
        parent.kind(),
        SyntaxKind::CompoundAssignmentStatement
            | SyntaxKind::PhonyAssignmentStatement
            | SyntaxKind::AssignmentStatement
    )
}
