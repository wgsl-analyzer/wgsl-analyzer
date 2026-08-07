use dprint_core_macros::sc;
use itertools::put_back;
use parser::{SyntaxKind, SyntaxNode};
use syntax::{
    AstNode as _,
    ast::{self, CompoundAssignmentOperator, Expression},
};

use crate::{
    ast_parse::{IgnoreBlankspace, NoTrivia, parse_end, parse_node_with},
    context_policies::statement_needs_semicolon_policy,
    generators::node::gen_node_with_trivia,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentError,
};

pub fn gen_assignment_statement(
    assignment_statement: &ast::AssignmentStatement
) -> Result<PrintItemBuffer, FormatDocumentError> {
    // NOTE!! - When updating this function, keep in mind to
    // update gen_assignment_statement, gen_compound_assignment_statement, gen_phony_assignment_statement together
    // This is non-dry code, but when inevitably at some point there will be some differences between
    // them, this should clearly communicate that they should be split up and not
    // continue to be one function with a whole lot of parameters and ifs.

    // ==== Parse ====
    let mut syntax = put_back(assignment_statement.syntax().children_with_tokens());
    let item_target =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_castable_kind::<Expression>()?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Equal)?;
    let item_value =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_castable_kind::<Expression>()?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind_optional(SyntaxKind::Semicolon)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_node_with_trivia(&item_target)?);
    formatted.request(Request::expect(RequestItem::Space));
    formatted.push_sc(sc!("="));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.start_indent();
    formatted.extend(gen_node_with_trivia(&item_value)?);
    if statement_needs_semicolon_policy(assignment_statement.syntax()) {
        formatted.request(Request::discourage(RequestItem::Space));
        formatted.push_sc(sc!(";"));
    }
    formatted.finish_indent();
    Ok(formatted)
}

pub fn gen_phony_assignment_statement(
    phony_assignment_statement: &ast::PhonyAssignmentStatement
) -> Result<PrintItemBuffer, FormatDocumentError> {
    // NOTE!! - When updating this function, keep in mind to
    // update gen_assignment_statement, gen_compound_assignment_statement, gen_phony_assignment_statement together
    // This is non-dry code, but when inevitably at some point there will be some differences between
    // them, this should clearly communicate that they should be split up and not
    // continue to be one function with a whole lot of parameters and ifs.

    // ==== Parse ====
    let mut syntax = put_back(phony_assignment_statement.syntax().children_with_tokens());
    let item_phony =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(SyntaxKind::Underscore)?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Equal)?;
    let item_value =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_castable_kind::<Expression>()?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind_optional(SyntaxKind::Semicolon)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_node_with_trivia(&item_phony)?);
    formatted.request(Request::expect(RequestItem::Space));
    formatted.push_sc(sc!("="));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.start_indent();
    formatted.extend(gen_node_with_trivia(&item_value)?);
    if statement_needs_semicolon_policy(phony_assignment_statement.syntax()) {
        formatted.request(Request::discourage(RequestItem::Space));
        formatted.push_sc(sc!(";"));
    }
    formatted.finish_indent();
    Ok(formatted)
}

pub fn gen_compound_assignment_statement(
    compound_assignment_statement: &ast::CompoundAssignmentStatement
) -> Result<PrintItemBuffer, FormatDocumentError> {
    // NOTE!! - When updating this function, keep in mind to
    // update gen_assignment_statement, gen_compound_assignment_statement, gen_phony_assignment_statement together
    // This is non-dry code, but when inevitably at some point there will be some differences between
    // them, this should clearly communicate that they should be split up and not
    // continue to be one function with a whole lot of parameters and ifs.

    // ==== Parse ====
    let mut syntax = put_back(
        compound_assignment_statement
            .syntax()
            .children_with_tokens(),
    );
    let item_target =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_castable_kind::<Expression>()?;
    let item_operator =
        parse_node_with(&mut syntax, NoTrivia).expect_ast_token::<CompoundAssignmentOperator>()?;
    let item_value =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_castable_kind::<Expression>()?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind_optional(SyntaxKind::Semicolon)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_node_with_trivia(&item_target)?);
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_node_with_trivia(&item_operator)?);
    formatted.request(Request::expect(RequestItem::Space));
    formatted.start_indent();
    formatted.extend(gen_node_with_trivia(&item_value)?);
    if statement_needs_semicolon_policy(compound_assignment_statement.syntax()) {
        formatted.request(Request::discourage(RequestItem::Space));
        formatted.push_sc(sc!(";"));
    }
    formatted.finish_indent();
    Ok(formatted)
}

pub fn remove_assignment_statement_parens_rule(node: &SyntaxNode) -> bool {
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
