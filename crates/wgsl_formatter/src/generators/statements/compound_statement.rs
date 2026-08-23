use dprint_core_macros::sc;
use itertools::{Itertools as _, Position};
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::{
    ast_parse::{
        Filter, IgnoreBraces, MatchKind, NoTrivia, PolicyAction, Succeeding, parse_end,
        parse_many_nodes_with, parse_node_with, syntax_iter,
    },
    context_policies::collapse_one_liner_compound_statement_policy,
    generators::node::gen_node_with_trivia,
    helpers::{LineSpacing, read_blankspace},
    multiline_group::MultilineGroup,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
    trivia::{NodeTriviaItem, NodeWithTrivia},
};

pub fn gen_compound_statement(
    with_trivia: &NodeWithTrivia,
    node: &ast::CompoundStatement,
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Context ====

    let is_conditional = with_trivia
        .preceding_trivia
        .iter()
        .any(|trivia| match trivia {
            NodeTriviaItem::LineSpacing(_)
            | NodeTriviaItem::Comment(_)
            | NodeTriviaItem::Discarded(_)
            | NodeTriviaItem::NewlinedComment(_) => false,
            NodeTriviaItem::AttributeList(attribute_list) => {
                attribute_list.attributes().any(|attribute| {
                    matches!(
                        attribute.name().as_ref().map(rowan::SyntaxToken::text),
                        Some("if" | "else" | "elif")
                    )
                })
            },
        });

    // ==== Parse ====

    let mut syntax = syntax_iter(node.syntax());
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::BraceLeft)?;

    let items = parse_many_nodes_with(
        &mut syntax,
        (
            IgnoreBraces,
            MatchKind(SyntaxKind::EmptyStatement, PolicyAction::Ignored),
            Filter(|node| match read_blankspace(node) {
                Some(LineSpacing::OnelineBlankspace(_)) => Some(PolicyAction::Ignored),
                _ => None,
            }),
            Succeeding(Filter(|node| match read_blankspace(node) {
                Some(LineSpacing::LineBreak(_) | LineSpacing::EmptyLine(_)) => {
                    Some(PolicyAction::Stop)
                },
                _ => None,
            })),
        ),
    )
    .map(NodeWithTrivia::trim_starting_linebreaks)
    .filter(|item| !item.is_whitespace())
    .collect_vec();

    parse_end(&mut syntax)?;

    let body_empty = items.iter().all(NodeWithTrivia::is_whitespace);

    dbg!(&items);
    // ==== Format ====

    let mut formatted = PrintItemBuffer::default();

    let mut multiline_group = MultilineGroup::new_before_requests(&mut formatted);

    multiline_group.push_sc(sc!("{"));

    if !body_empty {
        if !is_conditional {
            multiline_group.start_indent_before_requests();
        }

        if collapse_one_liner_compound_statement_policy(node.syntax()) {
            multiline_group.grouped_newline_or_space();
        } else {
            multiline_group.request(Request::discourage(RequestItem::EmptyLine));
            multiline_group.request(Request::expect(RequestItem::LineBreak));
        }

        if is_conditional
            && items.len() == 1
            && let Some(item) = items.first()
            && matches!(item.kind(), Some(SyntaxKind::CompoundStatement))
        {
            multiline_group.request(Request::discourage(RequestItem::LineBreak));
            multiline_group.extend(gen_node_with_trivia(item)?);
            multiline_group.request(Request::discourage(RequestItem::LineBreak));
        } else {
            for (pos, item) in items.iter().with_position() {
                if !matches!(pos, Position::Only | Position::First) {
                    multiline_group.request(Request::expect(RequestItem::LineBreak));
                }

                multiline_group.extend(gen_node_with_trivia(item)?);
            }
        }

        if !is_conditional {
            multiline_group.finish_indent();
        }

        if collapse_one_liner_compound_statement_policy(node.syntax()) {
            multiline_group.grouped_newline_or_space();
        } else {
            multiline_group.request(Request::expect(RequestItem::LineBreak));
        }
        multiline_group.request(Request::discourage(RequestItem::EmptyLine));
    }

    multiline_group.push_sc(sc!("}"));

    multiline_group.end_before_requests();

    Ok(formatted)
}
