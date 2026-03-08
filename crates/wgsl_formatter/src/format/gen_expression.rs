use dprint_core::formatting::Signal;
use dprint_core_macros::sc;
use itertools::put_back;
use syntax::{
    AstNode as _,
    ast::{self, TemplateList},
};

use crate::format::{
    ast_parse::{
        parse_end, parse_many_comments_and_blankspace, parse_node, parse_node_optional,
        parse_token, parse_token_any,
    },
    gen_comments::gen_comments,
    gen_function_call::gen_function_call,
    gen_path::gen_path,
    gen_types::gen_template_list,
    multiline_group::gen_surrounded_group,
    print_item_buffer::PrintItemBuffer,
    reporting::FormatDocumentResult,
};

use super::print_item_buffer::request_folder::RequestItem;

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
    let item_path = parse_node::<ast::Path>(&mut syntax)?;
    let item_comments_after_name_reference = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_template = parse_node_optional::<TemplateList>(&mut syntax);
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.extend(gen_path(&item_path)?);
    formatted.extend(gen_comments(&item_comments_after_name_reference));
    if let Some(item_template) = item_template {
        formatted.extend(gen_template_list(&item_template)?);
    }
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
        formatted.expect(RequestItem::Space);
    } else {
        formatted.push_sc(sc!("("));
        formatted.push_signal(Signal::StartNewLineGroup);
        formatted.push_signal(Signal::StartIndent);

        formatted.discourage(RequestItem::Space);
    }
    formatted.extend(gen_comments(&item_comment_after_left_paren));
    formatted.extend(gen_expression(&item_content, true)?);
    formatted.extend(gen_comments(&item_comment_after_content));

    if remove_parentheses {
        formatted.expect(RequestItem::Space);
    } else {
        formatted.discourage(RequestItem::Space);
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
    formatted.extend(gen_comments(&item_comment_after_left));
    formatted.expect(RequestItem::SpaceOrNewline);
    formatted.push_string(item_operator.to_string()); //TODO I don't like to-stringing the operator here, would be better to special case on it... we would need a parse_token(any_of(...)) kind of thing.
    formatted.expect(RequestItem::Space);
    formatted.extend(gen_comments(&item_comment_after_operator));
    formatted.extend(gen_expression(&item_right, false)?);
    formatted.extend(gen_comments(&item_comment_after_right));
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
    formatted.extend(gen_comments(&item_comment_after_operator));
    formatted.extend(gen_expression(&item_expr, false)?);
    formatted.extend(gen_comments(&item_comment_after_expr));
    Ok(formatted)
}

pub fn gen_field_expression(
    field_expression: &ast::FieldExpression
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(field_expression.syntax().children_with_tokens());
    let item_struct_expr = parse_node::<ast::Expression>(&mut syntax)?;
    let comments_after_ident_expr = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, parser::SyntaxKind::Period)?;
    let comments_after_period = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_target_ident = parse_token(&mut syntax, parser::SyntaxKind::Identifier)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.extend(gen_expression(&item_struct_expr, false)?);
    formatted.extend(gen_comments(&comments_after_ident_expr));
    formatted.push_sc(sc!("."));
    formatted.extend(gen_comments(&comments_after_period));
    formatted.push_string(item_target_ident.text().to_owned());
    Ok(formatted)
}

pub fn gen_index_expression(
    index_expression: &ast::IndexExpression
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(index_expression.syntax().children_with_tokens());
    let item_array_expr = parse_node::<ast::Expression>(&mut syntax)?;
    let comments_after_ident_expr = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, parser::SyntaxKind::BracketLeft)?;
    let comments_after_open_bracket = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_actual_index = parse_node::<ast::Expression>(&mut syntax)?;
    let comments_after_index_expr = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, parser::SyntaxKind::BracketRight)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();

    formatted.extend(gen_expression(&item_array_expr, false)?);
    formatted.extend(gen_comments(&comments_after_ident_expr));

    formatted.extend(gen_surrounded_group(
        Some({
            let mut pib = PrintItemBuffer::new();
            pib.push_sc(sc!("["));
            pib
        }),
        [{
            let mut pib = PrintItemBuffer::new();
            pib.extend(gen_comments(&comments_after_open_bracket));
            pib.extend(gen_expression(&item_actual_index, true)?);
            pib.extend(gen_comments(&comments_after_index_expr));
            pib
        }],
        Some({
            let mut pib = PrintItemBuffer::new();
            pib.push_sc(sc!("]"));
            pib
        }),
    ));

    Ok(formatted)
}
