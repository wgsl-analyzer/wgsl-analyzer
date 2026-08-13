use parser::SyntaxKind;
use syntax::{AstNode as _, ast};

use crate::{
    ast_parse::{IgnoreBlankspace, parse_end, parse_many_nodes_with, syntax_iter},
    generators::node::gen_node_with_trivia,
    print_item_buffer::{PrintItemBuffer, spacing_request::Request},
    reporting::FormatDocumentResult,
};

pub fn gen_path(path: &ast::Path) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = syntax_iter(path.syntax());

    let items = parse_many_nodes_with(&mut syntax, IgnoreBlankspace)
        .filter(|node| !node.is_whitespace())
        .collect::<Vec<_>>();

    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    for item in items {
        if matches!(item.kind(), Some(SyntaxKind::ColonColon)) {
            formatted.start_indent_before_requests();
            formatted.start_new_line_group_before_requests();
            formatted.request(Request::empty().or_newline());
            formatted.extend(gen_node_with_trivia(&item)?);
            formatted.finish_new_line_group_before_requests();
            formatted.finish_indent_before_requests();
        } else {
            formatted.extend(gen_node_with_trivia(&item)?);
        }
    }
    Ok(formatted)
}
