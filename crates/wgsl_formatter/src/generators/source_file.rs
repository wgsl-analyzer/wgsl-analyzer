use itertools::put_back;
use parser::{SyntaxKind, SyntaxNode};
use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::{
    ast_parse::{
        FilterAction, parse_any_node_optional, parse_end, parse_node_with_trivia_filter,
        parse_token_optional, syntax_iter,
    },
    generators::{
        comments::{Comment, gen_comment, parse_comment_optional},
        node::{gen_node, gen_node_with_trivia},
    },
    helpers::{
        LineSpacing, NextGenLineSpacing, gen_line_spacing, parse_line_spacing, read_blankspace,
    },
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem, RequestItemSet},
    },
    reporting::FormatDocumentResult,
    trivia::NodeWithTriviaContent,
};

pub fn gen_source_file(node: &ast::SourceFile) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====

    let mut syntax = syntax_iter(node.syntax());

    let mut items = Vec::new();

    loop {
        let mut item = parse_node_with_trivia_filter(&mut syntax, |_| None);

        if item
            .kind()
            .is_some_and(|item| item == SyntaxKind::Semicolon)
        {
            item.node = NodeWithTriviaContent::NoContent;
        }

        let is_end = item.is_end();
        if !item.is_whitespace() {
            items.push(item);
        }
        if is_end {
            break;
        }
    }

    parse_end(&mut syntax)?;

    // ==== Format ====

    let mut formatted = PrintItemBuffer::default();
    formatted.request(Request::Unconditional {
        expected: RequestItemSet::empty(),
        discouraged: RequestItemSet::empty()
            .extended_by(RequestItem::EmptyLine)
            .extended_by(RequestItem::LineBreak)
            .extended_by(RequestItem::Space),
        forced: RequestItemSet::empty(),
        suggest_linebreak: false,
    });

    for item in items {
        formatted.request(Request::expect(RequestItem::LineBreak));
        formatted.extend(gen_node_with_trivia(&item)?);
    }

    formatted.request(Request::Unconditional {
        forced: RequestItemSet::empty(),
        discouraged: RequestItemSet::from(RequestItem::EmptyLine),
        expected: RequestItemSet::from(RequestItem::LineBreak),
        suggest_linebreak: false,
    });

    Ok(formatted)
}
