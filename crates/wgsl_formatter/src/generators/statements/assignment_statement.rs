use dprint_core_macros::sc;
use itertools::put_back;
use parser::{SyntaxKind, SyntaxNode};
use syntax::{
    AstNode as _,
    ast::{self, CompoundAssignmentOperator, Expression},
};

use crate::{
    ast_parse::{parse_ast_token, parse_end, parse_node, parse_token, parse_token_optional},
    context_policies::statement_needs_semicolon_policy,
    generators::{
        comments::{gen_comments, parse_many_comments_and_blankspace},
        expressions::gen_expression,
    },
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
    let item_target = parse_node::<Expression>(&mut syntax)?;
    let item_comments_after_target = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::Equal)?;
    let item_comments_after_equal = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_value = parse_node::<Expression>(&mut syntax)?;
    let item_comments_after_value = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token_optional(&mut syntax, SyntaxKind::Semicolon);
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_expression(&item_target)?);
    formatted.extend(gen_comments(&item_comments_after_target));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.push_sc(sc!("="));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.start_indent();
    formatted.extend(gen_comments(&item_comments_after_equal));
    formatted.extend(gen_expression(&item_value)?);
    formatted.extend(gen_comments(&item_comments_after_value));
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
    parse_token(&mut syntax, SyntaxKind::Underscore)?;
    let item_comments_after_target = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::Equal)?;
    let item_comments_after_equal = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_value = parse_node::<Expression>(&mut syntax)?;
    let item_comments_after_value = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token_optional(&mut syntax, SyntaxKind::Semicolon);
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.push_sc(sc!("_"));
    formatted.extend(gen_comments(&item_comments_after_target));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.push_sc(sc!("="));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.start_indent();
    formatted.extend(gen_comments(&item_comments_after_equal));
    formatted.extend(gen_expression(&item_value)?);
    formatted.extend(gen_comments(&item_comments_after_value));
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
    let item_target = parse_node::<Expression>(&mut syntax)?;
    let item_comments_after_target = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_operator = parse_ast_token::<CompoundAssignmentOperator>(&mut syntax)?;
    let item_comments_after_equal = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_value = parse_node::<Expression>(&mut syntax)?;
    let item_comments_after_value = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token_optional(&mut syntax, SyntaxKind::Semicolon);
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_expression(&item_target)?);
    formatted.extend(gen_comments(&item_comments_after_target));

    let operator_sc = match item_operator {
        CompoundAssignmentOperator::PlusEqual(_) => sc!("+="),
        CompoundAssignmentOperator::MinusEqual(_) => sc!("-="),
        CompoundAssignmentOperator::TimesEqual(_) => sc!("*="),
        CompoundAssignmentOperator::DivisionEqual(_) => sc!("/="),
        CompoundAssignmentOperator::ModuloEqual(_) => sc!("%="),
        CompoundAssignmentOperator::AndEqual(_) => sc!("&="),
        CompoundAssignmentOperator::OrEqual(_) => sc!("|="),
        CompoundAssignmentOperator::XorEqual(_) => sc!("^="),
        CompoundAssignmentOperator::ShiftRightEqual(_) => sc!(">>="),
        CompoundAssignmentOperator::ShiftLeftEqual(_) => sc!("<<="),
    };
    formatted.request(Request::expect(RequestItem::Space));
    formatted.push_sc(operator_sc);
    formatted.request(Request::expect(RequestItem::Space));
    formatted.start_indent();
    formatted.extend(gen_comments(&item_comments_after_equal));
    formatted.extend(gen_expression(&item_value)?);
    formatted.extend(gen_comments(&item_comments_after_value));
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
