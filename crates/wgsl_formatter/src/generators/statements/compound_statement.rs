use dprint_core_macros::sc;
use itertools::{Itertools, Position, put_back};
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::{
    ast_parse::{parse_end, parse_node_with_trivia, parse_token},
    context_policies::collapse_one_liner_compound_statement_policy,
    generators::node::gen_node_with_trivia,
    multiline_group::MultilineGroup,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
    trivia::{NodeWithTrivia, NodeWithTriviaContent},
};

pub fn gen_compound_statement(
    node: &ast::CompoundStatement
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Context ====

    // ==== Parse ====

    let mut syntax = put_back(node.syntax().children_with_tokens());
    parse_token(&mut syntax, SyntaxKind::BraceLeft)?;

    let mut items = Vec::new();

    loop {
        let mut item = parse_node_with_trivia(&mut syntax);

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
    parse_token(&mut syntax, SyntaxKind::BraceRight)?;
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
