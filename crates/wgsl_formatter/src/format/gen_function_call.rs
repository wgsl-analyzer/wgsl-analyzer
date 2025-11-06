use std::rc::Rc;

use dprint_core::formatting::{LineNumber, LineNumberAnchor, PrintItems, Signal, conditions};
use dprint_core_macros::sc;
use itertools::{Itertools as _, Position, put_back};
use parser::{SyntaxKind, SyntaxToken};
use syntax::{AstNode as _, ast};

use crate::format::{
    ast_parse::{
        parse_end, parse_many_comments_and_blankspace, parse_node, parse_node_optional,
        parse_token, parse_token_optional,
    },
    gen_comments::gen_comments,
    gen_expression::{gen_expression, gen_ident_expression},
    gen_function::gen_fn_parameter,
    helpers::create_is_multiple_lines_resolver,
    print_item_buffer::{PrintItemBuffer, SeparationPolicy, SeparationRequest},
    reporting::FormatDocumentResult,
};

pub fn gen_function_call_expression(
    function_call: &ast::FunctionCall
) -> FormatDocumentResult<PrintItemBuffer> {
    dbg!(&function_call.syntax());

    // ==== Parse ====
    let mut syntax = put_back(function_call.syntax().children_with_tokens());
    let item_identifier = parse_node::<ast::IdentExpression>(&mut syntax)?;
    let item_comments_after_identifier = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_arguments = parse_node::<ast::Arguments>(&mut syntax)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.extend(gen_ident_expression(&item_identifier)?);
    formatted.extend(gen_comments(item_comments_after_identifier));
    formatted.extend(gen_function_call_expression_arguments(&item_arguments)?);
    Ok(formatted)
}

pub fn gen_function_call_expression_arguments(
    arguments: &ast::Arguments
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(arguments.syntax().children_with_tokens());
    parse_token(&mut syntax, SyntaxKind::ParenthesisLeft)?;
    let item_comments_after_open_paren = parse_many_comments_and_blankspace(&mut syntax)?;

    let mut item_parameters = Vec::new();
    loop {
        let Some(item_param) = parse_node_optional::<ast::Expression>(&mut syntax) else {
            break;
        };
        let item_comments_after_param = parse_many_comments_and_blankspace(&mut syntax)?;

        parse_token_optional(&mut syntax, SyntaxKind::Comma);
        let item_comments_after_comma = parse_many_comments_and_blankspace(&mut syntax)?;

        item_parameters.push((
            item_param,
            item_comments_after_param,
            item_comments_after_comma,
        ));
    }

    parse_token(&mut syntax, SyntaxKind::ParenthesisRight)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    // TODO Abstract this "fully multiline if at all multiline" functionality from here, index exprs, fn declarations, for-loops and wherever it also exists
    let start_ln = LineNumber::new("start");
    let end_ln = LineNumber::new("end");
    let is_multiple_lines = create_is_multiple_lines_resolver(start_ln, end_ln);

    let mut formatted = PrintItemBuffer::new();
    formatted.push_info(start_ln);
    formatted.push_anchor(LineNumberAnchor::new(end_ln));
    formatted.push_sc(sc!("("));

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

    formatted.extend(gen_comments(item_comments_after_open_paren));

    for (pos, (item_expression, item_comments_after_param, item_comments_after_comma)) in
        item_parameters.into_iter().with_position()
    {
        formatted.extend(gen_expression(&item_expression)?);
        if pos == Position::Last || pos == Position::Only {
            formatted.push_condition(conditions::if_true(
                "paramTrailingComma",
                Rc::clone(&is_multiple_lines),
                {
                    let mut pi = PrintItems::default();
                    pi.push_sc(sc!(","));
                    pi
                },
            ));
        } else {
            formatted.push_sc(sc!(","));
        }

        //The comma should be immediately after the parameter, we move the comment back
        formatted.extend(gen_comments(item_comments_after_param));
        formatted.extend(gen_comments(item_comments_after_comma));

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
    }

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

    formatted.push_sc(sc!(")"));

    formatted.push_signal(Signal::FinishNewLineGroup);
    formatted.push_info(end_ln);
    formatted.push_reevaluation(start_reeval);

    Ok(formatted)
}
