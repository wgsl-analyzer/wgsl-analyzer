use dprint_core::formatting::{ColumnNumber, LineNumber, LineNumberAnchor, PrintItems, Signal};
use dprint_core_macros::sc;
use itertools::put_back;
use parser::{SyntaxKind, SyntaxToken};
use rowan::NodeOrToken;
use syntax::{
    AstNode,
    ast::{
        self, CompoundAssignmentOperator, CompoundStatement, ElseClause, ElseIfClause, Expression,
        FunctionCall, IdentExpression, IfClause, IncrementDecrement, Literal,
        ParenthesisExpression, Statement,
    },
};

use crate::format::{
    self,
    ast_parse::{
        parse_ast_token, parse_end, parse_many_comments_and_blankspace, parse_node,
        parse_node_by_kind, parse_node_by_kind_optional, parse_node_optional, parse_token,
        parse_token_optional,
    },
    gen_comments::{gen_comment, gen_comments},
    gen_expression::{gen_expression, gen_parenthesis_expression},
    gen_function_call::gen_function_call,
    gen_if_statement::gen_if_statement,
    gen_switch_statement::gen_switch_statement,
    gen_types::gen_type_specifier,
    gen_var_let_const_statement::{
        gen_const_declaration_statement, gen_let_declaration_statement,
        gen_var_declaration_statement,
    },
    helpers::{create_is_multiple_lines_resolver, gen_spaced_lines, todo_verbatim},
    multiline_group::gen_multiline_group,
    print_item_buffer::{PrintItemBuffer, SeparationPolicy, SeparationRequest},
    reporting::{FormatDocumentError, FormatDocumentErrorKind, FormatDocumentResult},
};

pub fn gen_assignment_statement(
    assignment_statement: &ast::AssignmentStatement,
    include_semicolon: bool,
) -> Result<PrintItemBuffer, FormatDocumentError> {
    // NOTE!! - When updating this function, keep in mind to
    // update gen_assignment_statement, gen_compound_assignment_statement, gen_phony_assignment_statement together
    // This is non-dry code, but when inevitably at some point there will be some differences between
    // them, this should clearly communicate that they should be split up and not
    // continue to be one function with a whole lot of parameters and ifs.

    dbg!(assignment_statement.syntax());
    // ==== Parse ====
    let mut syntax = put_back(assignment_statement.syntax().children_with_tokens());
    let item_target = parse_node::<Expression>(&mut syntax)?;
    let item_comments_after_target = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::Equal)?;
    let item_comments_after_equal = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_value = parse_node::<Expression>(&mut syntax)?;
    let item_comments_after_value = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token_optional(&mut syntax, SyntaxKind::Semicolon);
    parse_end(&mut syntax);

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.extend(gen_expression(&item_target, true)?);
    formatted.extend(gen_comments(item_comments_after_target));
    formatted.expect_single_space();
    formatted.push_sc(sc!("="));
    formatted.expect_single_space();
    formatted.extend(gen_comments(item_comments_after_equal));
    formatted.extend(gen_expression(&item_value, true)?);
    formatted.extend(gen_comments(item_comments_after_value));
    if include_semicolon {
        formatted.request_space(SeparationPolicy::Discouraged);
        formatted.push_sc(sc!(";"));
    }
    Ok(formatted)
}

pub fn gen_phony_assignment_statement(
    phony_assignment_statement: &ast::PhonyAssignmentStatement,
    include_semicolon: bool,
) -> Result<PrintItemBuffer, FormatDocumentError> {
    // NOTE!! - When updating this function, keep in mind to
    // update gen_assignment_statement, gen_compound_assignment_statement, gen_phony_assignment_statement together
    // This is non-dry code, but when inevitably at some point there will be some differences between
    // them, this should clearly communicate that they should be split up and not
    // continue to be one function with a whole lot of parameters and ifs.

    dbg!(phony_assignment_statement.syntax());
    // ==== Parse ====
    let mut syntax = put_back(phony_assignment_statement.syntax().children_with_tokens());
    parse_token(&mut syntax, SyntaxKind::Underscore)?;
    let item_comments_after_target = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::Equal)?;
    let item_comments_after_equal = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_value = parse_node::<Expression>(&mut syntax)?;
    let item_comments_after_value = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token_optional(&mut syntax, SyntaxKind::Semicolon);
    parse_end(&mut syntax);

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("_"));
    formatted.extend(gen_comments(item_comments_after_target));
    formatted.expect_single_space();
    formatted.push_sc(sc!("="));
    formatted.expect_single_space();
    formatted.extend(gen_comments(item_comments_after_equal));
    formatted.extend(gen_expression(&item_value, true)?);
    formatted.extend(gen_comments(item_comments_after_value));
    if include_semicolon {
        formatted.request_space(SeparationPolicy::Discouraged);
        formatted.push_sc(sc!(";"));
    }
    Ok(formatted)
}

pub fn gen_compound_assignment_statement(
    compound_assignment_statement: &ast::CompoundAssignmentStatement,
    include_semicolon: bool,
) -> Result<PrintItemBuffer, FormatDocumentError> {
    // NOTE!! - When updating this function, keep in mind to
    // update gen_assignment_statement, gen_compound_assignment_statement, gen_phony_assignment_statement together
    // This is non-dry code, but when inevitably at some point there will be some differences between
    // them, this should clearly communicate that they should be split up and not
    // continue to be one function with a whole lot of parameters and ifs.

    dbg!(compound_assignment_statement.syntax());
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
    parse_end(&mut syntax);

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.extend(gen_expression(&item_target, true)?);
    formatted.extend(gen_comments(item_comments_after_target));

    let operator_sc = match item_operator {
        CompoundAssignmentOperator::PlusEqual(syntax_token) => sc!("+="),
        CompoundAssignmentOperator::MinusEqual(syntax_token) => sc!("-="),
        CompoundAssignmentOperator::TimesEqual(syntax_token) => sc!("*="),
        CompoundAssignmentOperator::DivisionEqual(syntax_token) => sc!("/="),
        CompoundAssignmentOperator::ModuloEqual(syntax_token) => sc!("%="),
        CompoundAssignmentOperator::AndEqual(syntax_token) => sc!("&="),
        CompoundAssignmentOperator::OrEqual(syntax_token) => sc!("|="),
        CompoundAssignmentOperator::XorEqual(syntax_token) => sc!("^="),
        CompoundAssignmentOperator::ShiftRightEqual(syntax_token) => sc!(">>="),
        CompoundAssignmentOperator::ShiftLeftEqual(syntax_token) => sc!("<<="),
    };
    formatted.expect_single_space();
    formatted.push_sc(operator_sc);
    formatted.expect_single_space();

    formatted.extend(gen_comments(item_comments_after_equal));
    formatted.extend(gen_expression(&item_value, true)?);
    formatted.extend(gen_comments(item_comments_after_value));
    if include_semicolon {
        formatted.request_space(SeparationPolicy::Discouraged);
        formatted.push_sc(sc!(";"));
    }
    Ok(formatted)
}
