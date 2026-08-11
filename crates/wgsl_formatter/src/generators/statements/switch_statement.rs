use dprint_core_macros::sc;
use itertools::{Itertools as _, Position};
use parser::{SyntaxKind, SyntaxNode};
use syntax::{
    AstNode as _,
    ast::{
        CompoundStatement, Expression, SwitchBody, SwitchBodyCase, SwitchCaseSelector,
        SwitchCaseSelectors, SwitchDefaultSelector, SwitchStatement,
    },
};

use crate::{
    ast_parse::{
        Chain, Filter, FilterAction, IgnoreBlankspace, NoTrivia, UntilSucceedingNewline, parse_end,
        parse_node_with, syntax_iter,
    },
    generators::node::gen_node_with_trivia,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentError,
    trivia::{NodeWithTrivia, NodeWithTriviaContent},
};

pub fn gen_switch_statement(
    statement: &SwitchStatement
) -> Result<PrintItemBuffer, FormatDocumentError> {
    // ==== Parse ====
    let mut syntax = syntax_iter(statement.syntax());
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Switch)?;
    let item_expression =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_castable_kind::<Expression>()?;
    let item_body =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_castable_kind::<SwitchBody>()?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    formatted.push_sc(sc!("switch"));
    formatted.request(Request::expect(RequestItem::Space)); // We trim out the parens, so we expect a space
    formatted.request(Request::discourage(RequestItem::LineBreak));
    formatted.extend(gen_node_with_trivia(&item_expression)?);
    formatted.request(Request::expect(RequestItem::Space)); // We trim out the parens, so we expect a space
    formatted.request(Request::discourage(RequestItem::LineBreak));
    formatted.extend(gen_node_with_trivia(&item_body)?);

    Ok(formatted)
}

pub fn gen_switch_body(statement: &SwitchBody) -> Result<PrintItemBuffer, FormatDocumentError> {
    // ==== Parse ====
    let mut syntax = syntax_iter(statement.syntax());
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::BraceLeft)?;

    let mut item_cases = Vec::new();

    loop {
        let mut item =
            parse_node_with(&mut syntax, Chain(UntilSucceedingNewline, IgnoreBlankspace));

        // TODO Absorb this into the parse_node_filter...
        if item
            .kind()
            .is_some_and(|item| item == SyntaxKind::BraceRight)
        {
            let old_node = std::mem::replace(&mut item.node, NodeWithTriviaContent::End);
            syntax.put_back(old_node.into_option().unwrap()); //TODO
        }

        let is_end = item.is_end();
        if !item.is_whitespace() {
            item_cases.push(item);
        }
        if is_end {
            break;
        }
    }

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::BraceRight)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.push_sc(sc!("{"));
    formatted.start_indent();

    let is_empty = item_cases.is_empty();
    if !is_empty {
        for item_case in &item_cases {
            formatted.request(Request::expect(RequestItem::LineBreak));
            formatted.extend(gen_node_with_trivia(item_case)?);
        }
        formatted.request(Request::expect(RequestItem::LineBreak));
    }
    formatted.finish_indent();
    formatted.push_sc(sc!("}"));

    if !is_empty {
        formatted.request(Request::expect(RequestItem::LineBreak));
    }

    Ok(formatted)
}

#[derive(Debug)]
pub enum SwitchBodyCaseKind {
    Default {
        item_default: NodeWithTrivia,
    },
    Case {
        item_case: NodeWithTrivia,
        item_selectors: NodeWithTrivia,
    },
}

pub fn gen_switch_body_case(
    statement: &SwitchBodyCase
) -> Result<PrintItemBuffer, FormatDocumentError> {
    // ==== Parse ====
    let mut syntax = syntax_iter(statement.syntax());

    // Either default or case
    let item_case_keyword = parse_node_with(&mut syntax, IgnoreBlankspace);
    let kind = {
        if item_case_keyword
            .kind()
            .is_some_and(|keyword| keyword == SyntaxKind::Default)
        {
            SwitchBodyCaseKind::Default {
                item_default: item_case_keyword,
            }
        } else {
            let selectors = parse_node_with(&mut syntax, IgnoreBlankspace);

            let mut item_case_keyword = item_case_keyword;

            if selectors
                .node
                .as_ref()
                .and_then(|selectors| match selectors {
                    rowan::NodeOrToken::Node(node) => Some(node),
                    rowan::NodeOrToken::Token(_) => None,
                })
                .is_some_and(is_case_default)
            {
                item_case_keyword.node = NodeWithTriviaContent::NoContent;
            }

            SwitchBodyCaseKind::Case {
                item_case: item_case_keyword,
                item_selectors: selectors,
            }
        }
    };

    //let item_comments_after_selectors = parse_many_comments_and_blankspace(&mut syntax)?; TODO Verify this is unneeded

    let item_colon =
        parse_node_with(&mut syntax, NoTrivia).only_if_kind(SyntaxKind::Colon, &mut syntax);
    let item_body = parse_node_with(&mut syntax, IgnoreBlankspace)
        .expect_castable_kind::<CompoundStatement>()?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    match kind {
        SwitchBodyCaseKind::Default { item_default } => {
            formatted.extend(gen_node_with_trivia(&item_default)?);
        },
        SwitchBodyCaseKind::Case {
            item_case,
            item_selectors,
        } => {
            formatted.extend(gen_node_with_trivia(&item_case)?);
            if !item_case.is_whitespace() {
                formatted.request(Request::discourage(RequestItem::LineBreak));
            }
            formatted.request(Request::expect(RequestItem::Space));
            formatted.extend(gen_node_with_trivia(&item_selectors)?);
        },
    }

    //formatted.extend(gen_comments(&item_comments_after_selectors));

    // For now we opted for option a) because we like it more. Its easy to add support for a wgslfmt.toml later
    // Option a) Always trim colon
    drop(item_colon);
    // Option b) Use colon whenever the user has it
    // if let Some(item_colon) = item_colon {
    //     formatted.push_sc(sc!(":"));
    // }
    // Option b) Force colon
    // formatted.push_sc(sc!(":"));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.request(Request::discourage(RequestItem::LineBreak));
    formatted.extend(gen_node_with_trivia(&item_body)?);
    Ok(formatted)
}

/// Check if the [`SwitchCaseSelectors`] only contains one "default" expr, and nothing else.
fn is_case_default(item_selectors: &SyntaxNode) -> bool {
    let Some(item_selectors) = SwitchCaseSelectors::cast(item_selectors.clone()) else {
        return false;
    };
    let mut exprs = item_selectors.exprs();
    let maybe_default = exprs.next();

    (matches!(
        maybe_default,
        Some(SwitchCaseSelector::SwitchDefaultSelector(_))
    ) && exprs.next().is_none())
}

pub fn gen_switch_case_selectors(
    statement: &SwitchCaseSelectors
) -> Result<PrintItemBuffer, FormatDocumentError> {
    // ==== Parse ====
    let mut syntax = syntax_iter(statement.syntax());

    let mut selectors = Vec::new();

    loop {
        let item = parse_node_with(
            &mut syntax,
            Filter(|node| match node.kind() {
                SyntaxKind::Comma => Some(FilterAction::IgnoreAndStop),
                _ => None,
            }),
        )
        .expect_ast_node_optional::<SwitchCaseSelector>()?;

        let is_end = item.is_end();
        if !item.is_whitespace() {
            selectors.push(item);
        }
        if is_end {
            break;
        }
    }

    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    for (position, selector) in selectors.into_iter().with_position() {
        formatted.extend(gen_node_with_trivia(&selector)?);
        if !matches!(position, Position::Last | Position::Only) {
            formatted.push_sc(sc!(","));
            formatted.request(Request::expect(RequestItem::Space));
        }
    }
    Ok(formatted)
}

pub fn gen_switch_case_default_selector(
    statement: &SwitchDefaultSelector
) -> Result<PrintItemBuffer, FormatDocumentError> {
    // ==== Parse ====
    let mut syntax = syntax_iter(statement.syntax());
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Default)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.push_sc(sc!("default"));
    Ok(formatted)
}

pub fn collapse_one_liner_case_body_rule(node: &SyntaxNode) -> bool {
    let Some(parent) = node.parent() else {
        return false;
    };
    matches!(parent.kind(), SyntaxKind::SwitchBodyCase)
}

pub fn remove_switch_subject_parens_rule(node: &SyntaxNode) -> bool {
    let Some(parent) = node.parent() else {
        return false;
    };
    matches!(parent.kind(), SyntaxKind::SwitchStatement)
}
