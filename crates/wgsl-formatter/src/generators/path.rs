use syntax::{AstNode as _, ast};

use crate::{
    ast_parse::{DiscardBlankspace, parse_end, parse_many_nodes_with, syntax_iter},
    generators::node::gen_node_with_trivia,
    print_item_buffer::PrintItemBuffer,
    reporting::FormatDocumentResult,
};

pub fn gen_path(path: &ast::Path) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = syntax_iter(path.syntax());

    let items = parse_many_nodes_with(&mut syntax, DiscardBlankspace)
        .filter(|node| !node.is_whitespace())
        .collect::<Vec<_>>();

    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    for item in items {
        formatted.extend(gen_node_with_trivia(&item)?);
    }
    Ok(formatted)
}
