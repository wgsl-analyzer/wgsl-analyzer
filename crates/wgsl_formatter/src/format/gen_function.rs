use std::{alloc::alloc, iter::repeat_with, rc::Rc};

use dprint_core::formatting::{
    ConditionResolver, ConditionResolverContext, LineNumber, LineNumberAnchor, PrintItem,
    PrintItems, PrintOptions, Signal, actions, condition_helpers, condition_resolvers, conditions,
    ir_helpers,
};
use dprint_core_macros::sc;
use itertools::{Itertools as _, Position, PutBack, put_back};
use parser::{SyntaxKind, SyntaxNode, SyntaxToken, WeslLanguage};
use rowan::{NodeOrToken, SyntaxElementChildren};
use syntax::{
    AstNode as _, HasName as _,
    ast::{self},
    match_ast,
};

use crate::{
    FormattingOptions,
    format::{
        self,
        ast_parse::{
            parse_end, parse_end_optional, parse_many_comments_and_blankspace, parse_node,
            parse_node_optional, parse_token, parse_token_optional,
        },
        gen_comments::gen_comments,
        gen_statement::gen_compound_statement,
        gen_types::gen_type_specifier,
        helpers::{create_is_multiple_lines_resolver, gen_spaced_lines, into_items, todo_verbatim},
        print_item_buffer::{PrintItemBuffer, SeparationPolicy, SeparationRequest},
        reporting::{FormatDocumentError, FormatDocumentErrorKind, FormatDocumentResult, err_src},
    },
};

pub fn gen_function_declaration(
    node: &ast::FunctionDeclaration
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = put_back(node.syntax().children_with_tokens());

    let item_fn = parse_token(&mut syntax, SyntaxKind::Fn)?;
    let item_comments_after_fn = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_name = parse_node::<ast::Name>(&mut syntax)?;
    let item_comments_after_name = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_params = parse_node::<ast::FunctionParameters>(&mut syntax)?;
    let item_comments_after_params = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_return = parse_node_optional::<ast::ReturnType>(&mut syntax);
    let item_comments_after_return = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_body = parse_node::<ast::CompoundStatement>(&mut syntax)?;
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::new();

    // Fn
    formatted.push_sc(sc!("fn"));
    formatted.expect_single_space();
    formatted.extend(gen_comments(item_comments_after_fn));

    // Name
    formatted.expect_single_space();
    formatted.push_string(item_name.text().to_string());
    formatted.extend(gen_comments(item_comments_after_name));

    // Params
    formatted.extend(gen_fn_parameters(&item_params)?);
    formatted.extend(gen_comments(item_comments_after_params));

    // Return
    if let Some(item_return) = item_return {
        formatted.extend(gen_fn_return_type(&item_return)?);
    }
    formatted.extend(gen_comments(item_comments_after_return));

    // Body
    formatted.expect_single_space();
    formatted.extend(gen_fn_body(&item_body)?);

    Ok(formatted)
}

#[expect(clippy::too_many_lines, reason = "TODO")]
pub fn gen_fn_parameters(node: &ast::FunctionParameters) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====

    let mut syntax = put_back(node.syntax().children_with_tokens());

    parse_token(&mut syntax, SyntaxKind::ParenthesisLeft)?;
    let item_comments_start = parse_many_comments_and_blankspace(&mut syntax)?;

    let mut item_parameters = Vec::new();

    loop {
        let Some(item_param) = parse_node_optional::<ast::Parameter>(&mut syntax) else {
            break;
        };
        let item_comments_after_param = parse_many_comments_and_blankspace(&mut syntax)?;

        parse_token_optional(&mut syntax, SyntaxKind::Comma); //Optional
        let item_comments_after_comma = parse_many_comments_and_blankspace(&mut syntax)?;

        item_parameters.push((
            item_param,
            item_comments_after_param,
            item_comments_after_comma,
        ));
    }

    let item_comments_after_params = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::ParenthesisRight)?;
    parse_end(&mut syntax)?;

    // ==== Format ====

    //TODO Once the formatter is in a state where definitive statement can be made,
    // look at if this formatting of comma seperated items could be sensibly abstracted out
    // and combined with e.g struct members, fn call arguments, etc.
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

    formatted.extend(gen_comments(item_comments_start));

    for (pos, (item_parameter, item_comments_after_param, item_comments_after_comma)) in
        item_parameters.into_iter().with_position()
    {
        formatted.extend(gen_fn_parameter(&item_parameter)?);
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
    formatted.extend(gen_comments(item_comments_after_params));

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

pub fn gen_fn_parameter(syntax: &ast::Parameter) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(syntax.syntax().children_with_tokens());

    let item_name = parse_node::<ast::Name>(&mut syntax)?;
    let item_comments_after_name = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::Colon);
    let item_comments_after_colon = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_type_specifier = parse_node::<ast::TypeSpecifier>(&mut syntax)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    formatted.push_string(item_name.text().to_string());
    formatted.push_sc(sc!(":"));
    formatted.expect_single_space();
    //The colon should immediately follow the name, we intentionally move the comment
    formatted.extend(gen_comments(item_comments_after_name));
    formatted.extend(gen_comments(item_comments_after_colon));
    formatted.extend(gen_type_specifier(&item_type_specifier)?);
    Ok(formatted)
}

pub fn gen_fn_return_type(syntax: &ast::ReturnType) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(syntax.syntax().children_with_tokens());

    parse_token(&mut syntax, SyntaxKind::Arrow);
    let item_comments_after_arrow = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_type_specifier = parse_node::<ast::TypeSpecifier>(&mut syntax)?;
    let item_comments_after_type = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    formatted.expect_single_space();
    formatted.push_sc(sc!("->"));
    formatted.expect_single_space();
    formatted.extend(gen_comments(item_comments_after_arrow));
    formatted.extend(gen_type_specifier(&item_type_specifier)?);
    Ok(formatted)
}

fn gen_fn_body(syntax: &ast::CompoundStatement) -> FormatDocumentResult<PrintItemBuffer> {
    gen_compound_statement(syntax)
}
