use dprint_core_macros::sc;
use itertools::{Itertools as _, Position};
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::{
    ast_parse::{Filter, FilterAction, NoTrivia, parse_end, parse_node_with, syntax_iter},
    context_policies::collapse_one_liner_compound_statement_policy,
    generators::node::gen_node_with_trivia,
    helpers::{LineSpacing, read_blankspace},
    multiline_group::MultilineGroup,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
    trivia::{NodeTriviaItem, NodeWithTrivia, NodeWithTriviaContent},
};

pub fn gen_compound_statement(
    node: &ast::CompoundStatement
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Context ====

    // ==== Parse ====

    let mut syntax = syntax_iter(node.syntax());
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::BraceLeft)?;

    let mut items = Vec::new();

    loop {
        let mut item = parse_node_with(
            &mut syntax,
            Filter(|node| match read_blankspace(node) {
                Some(LineSpacing::OnelineBlankspace(_)) => Some(FilterAction::Ignored),
                _ => None,
            }),
        );

        // We only care about newlines if they are somewhere within the trivia, not at the start or end
        let first_interesting_item = item.preceding_trivia.iter().position(|node| {
            !matches!(node, NodeTriviaItem::LineSpacing(LineSpacing::LineBreak(_)))
        });
        if let Some(first_interesting_item) = first_interesting_item {
            item.preceding_trivia = item.preceding_trivia.split_off(first_interesting_item);
        } else {
            item.preceding_trivia = Vec::new();
        }
        let last_interesting_item = item.succeeding_trivia.iter().rev().position(|node| {
            !matches!(node, NodeTriviaItem::LineSpacing(LineSpacing::LineBreak(_)))
        });
        if let Some(last_interesting_item) = last_interesting_item {
            item.succeeding_trivia
                .shrink_to(item.succeeding_trivia.len() - last_interesting_item);
        } else {
            item.succeeding_trivia = Vec::new();
        }

        // TODO This needs to be absorbed into parse_node..
        if matches!(item.kind(), Some(SyntaxKind::BraceRight)) {
            let old_node = std::mem::replace(&mut item.node, NodeWithTriviaContent::End);
            syntax.put_back(old_node.into_option().unwrap()); //TODO
        }

        let is_end = item.is_end();
        if !item.is_whitespace() {
            items.push(item);
        }
        if is_end {
            break;
        }
    }
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::BraceRight)?;
    parse_end(&mut syntax)?;

    let body_empty = items.iter().all(NodeWithTrivia::is_whitespace);

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    let mut multiline_group = MultilineGroup::new(&mut formatted);

    multiline_group.push_sc(sc!("{"));

    if !body_empty {
        multiline_group.start_indent();

        if collapse_one_liner_compound_statement_policy(node.syntax()) {
            //TODO This is a dirty hack to get rid of the discouragement of spaces in start_indent
            multiline_group.push_sc(dprint_core_macros::sc!(""));
            //TODO and now we need to get the discouragement of newlines back...
            multiline_group.request(Request::discourage(RequestItem::LineBreak));

            multiline_group.grouped_newline_or_space();
        } else {
            multiline_group.request(Request::discourage(RequestItem::EmptyLine));
            multiline_group.request(Request::expect(RequestItem::LineBreak));
        }

        for (pos, item) in items.iter().with_position() {
            if !matches!(pos, Position::Only | Position::First) {
                multiline_group.request(Request::expect(RequestItem::LineBreak));
            }
            multiline_group.extend(gen_node_with_trivia(item)?);
        }

        multiline_group.finish_indent();

        if collapse_one_liner_compound_statement_policy(node.syntax()) {
            //TODO This is a dirty hack to get rid of the discouragement of spaces in start_indent
            multiline_group.push_sc(dprint_core_macros::sc!(""));

            multiline_group.grouped_newline_or_space();
        } else {
            multiline_group.request(Request::discourage(RequestItem::EmptyLine));
            multiline_group.request(Request::expect(RequestItem::LineBreak));
        }
    }

    multiline_group.push_sc(sc!("}"));

    multiline_group.end();

    if !body_empty {
        // This exists mainly for things like
        // fn a { let a = 1; } // Thing
        // ==>
        // fn a {
        //   let a = 1;
        // }
        // // Thing
        // So the comment is not on the same line as the closing brace.
        formatted.request(Request::expect(RequestItem::LineBreak));
    }

    Ok(formatted)
}
