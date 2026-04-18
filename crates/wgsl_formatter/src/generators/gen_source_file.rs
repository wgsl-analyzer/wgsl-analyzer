use std::collections::BTreeSet;

use itertools::put_back;
use parser::{SyntaxKind, SyntaxNode};
use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::{
    ast_parse::{parse_any_node_optional, parse_end, parse_token_optional},
    generators::{
        gen_comments::{Comment, gen_comment, parse_comment_optional},
        gen_node::gen_node,
    },
    helpers::{LineSpacing, gen_line_spacing, parse_line_spacing},
    print_item_buffer::{
        PrintItemBuffer,
        request_folder::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

pub fn gen_source_file(node: &ast::SourceFile) -> FormatDocumentResult<PrintItemBuffer> {
    enum SourceFileItem {
        Other(SyntaxNode),
        Comment(Comment),
        LineSpacing(LineSpacing),
    }

    // ==== Parse ====

    let mut syntax = put_back(node.syntax().children_with_tokens());

    let mut items = Vec::new();
    // TODO(MonaMayrhofer) This is basically duplicated code from compound statement, and the user would
    // expect them to behave similarly so they should be combined.
    loop {
        if let Some(spacing) = parse_line_spacing(&mut syntax) {
            items.push(SourceFileItem::LineSpacing(spacing));
        } else if let Some(_statement) = parse_token_optional(&mut syntax, SyntaxKind::Blankspace) {
            // If its not a line_spacing blankspace, then we simply discard it
        } else if let Some(_statement) = parse_token_optional(&mut syntax, SyntaxKind::Semicolon) {
            // Top level semicolons, like after struct defs
        } else if let Some(comment) = parse_comment_optional(&mut syntax) {
            items.push(SourceFileItem::Comment(comment));
        } else if let Some(item) = parse_any_node_optional(&mut syntax) {
            // Any other node. We do not care what exact node it is, because that will be handled by gen_node later
            // The formatter should format items that are in wrong places, its the job of the parser to check correctness
            items.push(SourceFileItem::Other(item));
        } else {
            break;
        }
    }

    parse_end(&mut syntax)?;

    // ==== Format ====

    let mut formatted = PrintItemBuffer::new();
    formatted.request(Request::Unconditional {
        expected: BTreeSet::new(),
        discouraged: BTreeSet::from([
            RequestItem::EmptyLine,
            RequestItem::LineBreak,
            RequestItem::Space,
        ]),
        forced: BTreeSet::new(),
        suggest_linebreak: false,
    });

    for item in items {
        match item {
            SourceFileItem::Other(item) => {
                // Every item should start on a new line.
                formatted.expect(RequestItem::LineBreak);
                formatted.extend(gen_node(&item)?);
            },
            SourceFileItem::Comment(comment) => {
                formatted.extend(gen_comment(&comment));
            },
            SourceFileItem::LineSpacing(line_spacing) => {
                formatted.extend(gen_line_spacing(&line_spacing)?);
            },
        }
    }

    formatted.request(Request::Unconditional {
        forced: BTreeSet::new(),
        discouraged: BTreeSet::from([RequestItem::EmptyLine]),
        expected: BTreeSet::from([RequestItem::LineBreak]),
        suggest_linebreak: false,
    });

    Ok(formatted)
}
