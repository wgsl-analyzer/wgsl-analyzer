use std::rc::Rc;

use dprint_core::formatting::{LineNumber, LineNumberAnchor, PrintItems, Signal, conditions};
use dprint_core_macros::sc;
use itertools::put_back;
use syntax::{AstNode as _, ast};

use crate::format::{
    ast_parse::{
        parse_end, parse_many_comments_and_blankspace, parse_node, parse_token, parse_token_any,
    },
    gen_comments::gen_comments,
    gen_function_call::gen_function_call,
    helpers::create_is_multiple_lines_resolver,
    print_item_buffer::{PrintItemBuffer, SeparationPolicy, SeparationRequest},
    reporting::FormatDocumentResult,
};

pub fn gen_expression(
    expression: &ast::Expression,
    remove_parentheses: bool,
) -> FormatDocumentResult<PrintItemBuffer> {
    match expression {
        ast::Expression::IndexExpression(index_expression) => {
            gen_index_expression(index_expression)
        },
        ast::Expression::FieldExpression(field_expression) => {
            gen_field_expression(field_expression)
        },
        ast::Expression::PrefixExpression(prefix_expression) => {
            gen_prefix_expression(prefix_expression)
        },
        ast::Expression::InfixExpression(infix_expression) => {
            gen_infix_expression(infix_expression)
        },
        ast::Expression::IdentExpression(ident_expression) => {
            gen_ident_expression(ident_expression)
        },
        ast::Expression::FunctionCall(function_call) => gen_function_call(function_call),
        ast::Expression::ParenthesisExpression(parenthesis_expression) => {
            gen_parenthesis_expression(parenthesis_expression, remove_parentheses)
        },
        ast::Expression::Literal(literal) => gen_literal_expression(literal),
    }
}

#[expect(
    clippy::unnecessary_wraps,
    reason = "Keep API uniform with other gen functions"
)]
pub fn gen_literal_expression(
    literal_expression: &ast::Literal
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.push_string(literal_expression.syntax().to_string());
    Ok(formatted)
}

pub fn gen_ident_expression(
    ident_expression: &ast::IdentExpression
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(ident_expression.syntax().children_with_tokens());
    let item_name_reference = parse_node::<ast::NameReference>(&mut syntax)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.push_string(item_name_reference.text().to_string());
    Ok(formatted)
}

pub fn gen_parenthesis_expression(
    parenthesis_expression: &ast::ParenthesisExpression,
    remove_parentheses: bool,
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
    let mut formatted = PrintItemBuffer::new();
    if remove_parentheses {
        formatted.request(SeparationRequest {
            space: SeparationPolicy::Expected,
            ..Default::default()
        });
    } else {
        formatted.push_sc(sc!("("));
        formatted.push_signal(Signal::StartNewLineGroup);
        formatted.push_signal(Signal::StartIndent);

        formatted.request(SeparationRequest {
            space: SeparationPolicy::Discouraged,
            ..Default::default()
        });
    }
    formatted.extend(gen_comments(item_comment_after_left_paren));
    formatted.extend(gen_expression(&item_content, true)?);
    formatted.extend(gen_comments(item_comment_after_content));

    if remove_parentheses {
        formatted.request(SeparationRequest {
            space: SeparationPolicy::Expected,
            ..Default::default()
        });
    } else {
        formatted.request(SeparationRequest {
            space: SeparationPolicy::Discouraged,
            ..Default::default()
        });
        formatted.push_signal(Signal::FinishIndent);
        formatted.push_signal(Signal::FinishNewLineGroup);
        formatted.push_sc(sc!(")"));
    }
    Ok(formatted)
}

pub fn gen_infix_expression(
    infix_expression: &ast::InfixExpression
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(infix_expression.syntax().children_with_tokens());

    let item_left = parse_node::<ast::Expression>(&mut syntax)?;
    let item_comment_after_left = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_operator = parse_token_any(&mut syntax)?;
    let item_comment_after_operator = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_right = parse_node::<ast::Expression>(&mut syntax)?;
    let item_comment_after_right = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.extend(gen_expression(&item_left, false)?);
    formatted.extend(gen_comments(item_comment_after_left));
    formatted.expect_single_space();
    formatted.request_line_break(SeparationPolicy::Allowed);
    formatted.push_string(item_operator.to_string()); //TODO I don't like to-stringing the operator here, would be better to special case on it... we would need a parse_token(any_of(...)) kind of thing.
    formatted.expect_single_space();
    formatted.extend(gen_comments(item_comment_after_operator));
    formatted.extend(gen_expression(&item_right, false)?);
    formatted.extend(gen_comments(item_comment_after_right));
    Ok(formatted)
}

pub fn gen_prefix_expression(
    infix_expression: &ast::PrefixExpression
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(infix_expression.syntax().children_with_tokens());

    let item_operator = parse_token_any(&mut syntax)?;
    let item_comment_after_operator = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_expr = parse_node::<ast::Expression>(&mut syntax)?;
    let item_comment_after_expr = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.push_string(item_operator.to_string()); //TODO I don't like to-stringing the operator here, would be better to match on it... we would need a parse_token(any_of(...)) kind of thing.
    formatted.extend(gen_comments(item_comment_after_operator));
    formatted.extend(gen_expression(&item_expr, false)?);
    formatted.extend(gen_comments(item_comment_after_expr));
    Ok(formatted)
}

pub fn gen_field_expression(
    field_expression: &ast::FieldExpression
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(field_expression.syntax().children_with_tokens());
    let item_ident_expr = parse_node::<ast::IdentExpression>(&mut syntax)?;
    let comments_after_ident_expr = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, parser::SyntaxKind::Period)?;
    let comments_after_period = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_target_ident = parse_token(&mut syntax, parser::SyntaxKind::Identifier)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.extend(gen_ident_expression(&item_ident_expr)?);
    formatted.extend(gen_comments(comments_after_ident_expr));
    formatted.push_sc(sc!("."));
    formatted.extend(gen_comments(comments_after_period));
    formatted.push_string(item_target_ident.text().to_owned());
    Ok(formatted)
}

pub fn gen_index_expression(
    index_expression: &ast::IndexExpression
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(index_expression.syntax().children_with_tokens());
    let item_ident_expr = parse_node::<ast::IdentExpression>(&mut syntax)?;
    let comments_after_ident_expr = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, parser::SyntaxKind::BracketLeft)?;
    let comments_after_open_bracket = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_index_literal = parse_node::<ast::Literal>(&mut syntax)?;
    let comments_after_index_expr = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, parser::SyntaxKind::BracketRight)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();

    formatted.extend(gen_ident_expression(&item_ident_expr)?);
    formatted.extend(gen_comments(comments_after_ident_expr));
    // formatted.push_sc(sc!("["));
    // formatted.extend(gen_comments(comments_after_open_bracket));
    // formatted.extend(gen_literal_expression(&item_index_literal)?);
    // formatted.extend(gen_comments(comments_after_index_expr));
    // formatted.push_sc(sc!("]"));

    // TODO Abstract this "fully multiline if at all multiline" functionality from here, index exprs, fn declarations and wherever it also exists
    let start_ln = LineNumber::new("start");
    let end_ln = LineNumber::new("end");
    let is_multiple_lines = create_is_multiple_lines_resolver(start_ln, end_ln);

    formatted.push_info(start_ln);
    formatted.push_anchor(LineNumberAnchor::new(end_ln));
    formatted.push_sc(sc!("["));

    let mut start_nl_condition = conditions::if_true_or(
        "paramMultilineStartIndent",
        Rc::clone(&is_multiple_lines),
        {
            let mut pi = PrintItems::default();
            pi.push_signal(Signal::NewLine);
            pi.push_signal(Signal::StartIndent);
            pi
        },
        {
            let mut pi = PrintItems::default();
            pi.push_signal(Signal::PossibleNewLine);
            pi
        },
    );
    let start_reeval = start_nl_condition.create_reevaluation();
    formatted.push_condition(start_nl_condition);
    formatted.push_signal(Signal::StartNewLineGroup);

    // TODO This is a bit of a shortcoming of the PBI api, we would want to write this after the "(", but can't because of the conditions between
    formatted.request(SeparationRequest::discouraged());

    formatted.extend(gen_comments(comments_after_open_bracket));

    formatted.extend(gen_literal_expression(&item_index_literal)?);

    formatted.extend(gen_comments(comments_after_index_expr));

    formatted.request(SeparationRequest {
        line_break: SeparationPolicy::ExpectedIf {
            on_branch: true,
            of_resolver: Rc::clone(&is_multiple_lines),
        },
        space: SeparationPolicy::ExpectedIf {
            on_branch: false,
            of_resolver: Rc::clone(&is_multiple_lines),
        },
        ..Default::default()
    });

    // No trailing spaces
    formatted.request(SeparationRequest {
        space: SeparationPolicy::Discouraged,
        ..Default::default()
    });

    formatted.push_condition(conditions::if_true(
        "paramMultilineEndIndent",
        is_multiple_lines,
        {
            let mut pi = PrintItems::default();
            pi.push_signal(Signal::FinishIndent);
            pi
        },
    ));

    formatted.push_sc(sc!("]"));
    formatted.push_signal(Signal::FinishNewLineGroup);
    formatted.push_info(end_ln);
    formatted.push_reevaluation(start_reeval);

    Ok(formatted)
}
