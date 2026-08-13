use itertools::Itertools as _;
use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::{
    ast_parse::{
        IgnoreBlankspace, IgnoreSemicolon, Succeeding, UntilEmptyLine, UntilNewline, parse_end,
        parse_many_nodes_with, syntax_iter,
    },
    generators::node::gen_node_with_trivia,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem, RequestItemSet},
    },
    reporting::FormatDocumentResult,
};

pub fn gen_source_file(node: &ast::SourceFile) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====

    let mut syntax = syntax_iter(node.syntax());

    let items = parse_many_nodes_with(
        &mut syntax,
        Succeeding((
            UntilEmptyLine,
            UntilNewline,
            IgnoreBlankspace,
            IgnoreSemicolon,
        )),
    )
    .filter(|item| !item.is_whitespace())
    .collect_vec();

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
