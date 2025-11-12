use dprint_core::formatting::Signal;
use dprint_core_macros::sc;
use itertools::{Itertools, Position, put_back};
use parser::{SyntaxKind, SyntaxToken};
use syntax::{
    AstNode as _,
    ast::{
        CompoundStatement, ParenthesisExpression, SwitchBody, SwitchBodyCase, SwitchCaseSelector,
        SwitchCaseSelectors, SwitchDefaultSelector, SwitchStatement,
    },
};

use crate::format::{
    ast_parse::{
        parse_end, parse_many_comments_and_blankspace, parse_node, parse_node_optional,
        parse_token, parse_token_optional,
    },
    gen_comments::gen_comments,
    gen_expression::{gen_expression, gen_parenthesis_expression},
    gen_statement::gen_compound_statement,
    helpers::todo_verbatim,
    print_item_buffer::PrintItemBuffer,
    reporting::FormatDocumentError,
};

pub fn gen_switch_statement(
    statement: &SwitchStatement
) -> Result<PrintItemBuffer, FormatDocumentError> {
    dbg!(statement.syntax());

    // ==== Parse ====
    let mut syntax = put_back(statement.syntax().children_with_tokens());
    parse_token(&mut syntax, SyntaxKind::Switch)?;
    let item_comments_after_switch = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_parens = parse_node::<ParenthesisExpression>(&mut syntax)?;
    let item_comments_after_parens = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_body = parse_node::<SwitchBody>(&mut syntax)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();

    formatted.push_sc(sc!("switch"));
    formatted.extend(gen_comments(item_comments_after_switch));
    formatted.extend(gen_parenthesis_expression(&item_parens)?);
    formatted.expect_single_space();
    formatted.extend(gen_comments(item_comments_after_parens));
    formatted.extend(gen_switch_body(&item_body)?);

    Ok(formatted)
}

pub fn gen_switch_body(statement: &SwitchBody) -> Result<PrintItemBuffer, FormatDocumentError> {
    dbg!(statement.syntax());

    // ==== Parse ====
    let mut syntax = put_back(statement.syntax().children_with_tokens());
    parse_token(&mut syntax, SyntaxKind::BraceLeft)?;
    let item_comments_after_brace_left = parse_many_comments_and_blankspace(&mut syntax)?;

    let mut item_cases = Vec::new();

    while let Some(item_case) = parse_node_optional::<SwitchBodyCase>(&mut syntax) {
        let item_comments_after_case = parse_many_comments_and_blankspace(&mut syntax)?;
        item_cases.push((item_case, item_comments_after_case));
    }

    let item_comments_after_cases = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::BraceRight)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("{"));
    formatted.push_signal(Signal::StartIndent);
    formatted.extend(gen_comments(item_comments_after_brace_left));
    for (item_case, item_comments_after_case) in item_cases {
        formatted.expect_line_break();
        formatted.extend(gen_switch_body_case(&item_case)?);
        formatted.extend(gen_comments(item_comments_after_case));
    }
    formatted.push_signal(Signal::FinishIndent);
    formatted.expect_line_break();
    formatted.push_sc(sc!("}"));
    Ok(formatted)
}

pub enum SwitchBodyCaseKind {
    Default,
    Case {
        item_comments_after_case: Vec<SyntaxToken>,
        item_selectors: SwitchCaseSelectors,
    },
}

pub fn gen_switch_body_case(
    statement: &SwitchBodyCase
) -> Result<PrintItemBuffer, FormatDocumentError> {
    dbg!(statement.syntax());

    // ==== Parse ====
    let mut syntax = put_back(statement.syntax().children_with_tokens());

    // Either default or case
    let kind = {
        let item_default = parse_token_optional(&mut syntax, SyntaxKind::Default);
        if item_default.is_some() {
            SwitchBodyCaseKind::Default
        } else {
            parse_token(&mut syntax, SyntaxKind::Case);
            let item_comments_after_case = parse_many_comments_and_blankspace(&mut syntax)?;
            let item_selectors = parse_node::<SwitchCaseSelectors>(&mut syntax)?;

            SwitchBodyCaseKind::Case {
                item_comments_after_case,
                item_selectors,
            }
        }
    };

    let item_comments_after_selectors = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_colon = parse_token_optional(&mut syntax, SyntaxKind::Colon);
    let item_comments_after_colon = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_body = parse_node::<CompoundStatement>(&mut syntax)?;
    parse_end(&mut syntax)?;

    let item_comments_after_cases = parse_many_comments_and_blankspace(&mut syntax)?;
    //parse_token(&mut syntax, SyntaxKind::BraceRight)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();

    match kind {
        SwitchBodyCaseKind::Default => {
            formatted.push_sc(sc!("default"));
        },
        SwitchBodyCaseKind::Case {
            item_comments_after_case,
            item_selectors,
        } => {
            if is_case_default(&item_selectors) {
                formatted.push_sc(sc!("default"));
                formatted.extend(gen_comments(item_comments_after_case));
            } else {
                formatted.push_sc(sc!("case"));
                formatted.expect_single_space();
                formatted.extend(gen_comments(item_comments_after_case));
                formatted.extend(gen_switch_case_selectors(&item_selectors)?);
            }
        },
    }
    formatted.extend(gen_comments(item_comments_after_selectors));

    // For now we opted for option a) because we like it more. Its easy to add support for a wgslfmt.toml later
    // Option a) Always trim colon
    drop(item_colon);
    // Option b) Use colon whenever the user has it
    // if let Some(item_colon) = item_colon {
    //     formatted.push_sc(sc!(":"));
    // }
    // Option b) Force colon
    // formatted.push_sc(sc!(":"));
    formatted.extend(gen_comments(item_comments_after_colon));
    formatted.expect_single_space();
    formatted.extend(gen_compound_statement(&item_body)?);
    Ok(formatted)
}

/// Check if the SwitchCaseSelectors only contains one "default" expr, and nothing else
fn is_case_default(item_selectors: &SwitchCaseSelectors) -> bool {
    let mut exprs = item_selectors.exprs();
    let maybe_default = exprs.next();

    return matches!(
        maybe_default,
        Some(SwitchCaseSelector::SwitchDefaultSelector(_))
    ) && exprs.next().is_none();
}

pub fn gen_switch_case_selectors(
    statement: &SwitchCaseSelectors
) -> Result<PrintItemBuffer, FormatDocumentError> {
    dbg!(statement.syntax());

    // ==== Parse ====
    let mut syntax = put_back(statement.syntax().children_with_tokens());

    let mut selectors = Vec::new();
    while let Some(selector) = parse_node_optional::<SwitchCaseSelector>(&mut syntax) {
        let item_comments_after_selector = parse_many_comments_and_blankspace(&mut syntax)?;
        parse_token_optional(&mut syntax, SyntaxKind::Comma);
        let item_comments_after_comma = parse_many_comments_and_blankspace(&mut syntax)?;

        selectors.push((
            selector,
            item_comments_after_selector,
            item_comments_after_comma,
        ));
    }
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    for (position, (selector, item_comments_after_selector, item_comments_after_comma)) in
        selectors.into_iter().with_position()
    {
        formatted.extend(gen_switch_case_selector(&selector)?);
        formatted.extend(gen_comments(item_comments_after_selector));
        if !matches!(position, Position::Last | Position::Only) {
            formatted.push_sc(sc!(","));
            formatted.expect_single_space();
        }
        formatted.extend(gen_comments(item_comments_after_comma));
    }
    Ok(formatted)
}

pub fn gen_switch_case_selector(
    statement: &SwitchCaseSelector
) -> Result<PrintItemBuffer, FormatDocumentError> {
    // ==== Parse ====

    // ==== Format ====
    match statement {
        SwitchCaseSelector::Expression(expression) => gen_expression(expression),
        SwitchCaseSelector::SwitchDefaultSelector(switch_default_selector) => {
            gen_switch_case_default_selector(switch_default_selector)
        },
    }
}

pub fn gen_switch_case_default_selector(
    statement: &SwitchDefaultSelector
) -> Result<PrintItemBuffer, FormatDocumentError> {
    // ==== Parse ====
    let mut syntax = put_back(statement.syntax().children_with_tokens());
    parse_token(&mut syntax, SyntaxKind::Default);
    parse_end(&mut syntax);

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("default"));
    Ok(formatted)
}
