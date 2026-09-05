use itertools::Itertools as _;
use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::{
    ast_parse::{
        DiscardBlankspace, DiscardSemicolon, ParseNodePolicy, StopAtNewline, Succeeding, parse_end,
        parse_many_nodes_with, syntax_iter,
    },
    generators::node::gen_node_with_trivia,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

#[must_use]
pub const fn source_file_item_policy() -> impl ParseNodePolicy {
    Succeeding((StopAtNewline, DiscardBlankspace, DiscardSemicolon))
}

pub fn gen_source_file(node: &ast::SourceFile) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====

    let mut syntax = syntax_iter(node.syntax());

    let items = parse_many_nodes_with(&mut syntax, source_file_item_policy())
        .filter(|item| !item.is_whitespace())
        .collect_vec();

    parse_end(&mut syntax)?;

    // ==== Format ====

    let mut formatted = PrintItemBuffer::default();
    formatted.request(Request::discourage(RequestItem::EmptyLine));
    formatted.request(Request::discourage(RequestItem::LineBreak));
    formatted.request(Request::discourage(RequestItem::Space));

    for item in items {
        formatted.request(Request::expect(RequestItem::LineBreak));
        formatted.extend(gen_node_with_trivia(&item)?);
    }

    formatted.request(Request::expect(RequestItem::LineBreak));
    formatted.request(Request::discourage(RequestItem::EmptyLine));
    formatted.request(Request::discourage(RequestItem::Space));

    Ok(formatted)
}
