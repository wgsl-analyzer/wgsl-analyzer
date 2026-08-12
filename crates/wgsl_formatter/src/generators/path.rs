use parser::SyntaxKind;
use syntax::{AstNode as _, ast};

use crate::{
    ast_parse::{IgnoreBlankspace, NoTrivia, parse_end, parse_node_with, syntax_iter},
    generators::node::gen_node_with_trivia,
    print_item_buffer::{PrintItemBuffer, spacing_request::Request},
    reporting::FormatDocumentResult,
    trivia::NodeWithTrivia,
};

pub fn gen_path(path: &ast::Path) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = syntax_iter(path.syntax());

    enum PathItem {
        Identifier(NodeWithTrivia),
        ColonColon(NodeWithTrivia),
    }

    let mut items = Vec::new();

    loop {
        let item = parse_node_with(&mut syntax, IgnoreBlankspace)
            .expect_kind_optional(SyntaxKind::Identifier)?;

        let is_end = item.is_end();
        if !item.is_whitespace() {
            items.push(PathItem::Identifier(item));
        }
        if is_end {
            break;
        }

        let item =
            parse_node_with(&mut syntax, NoTrivia).expect_kind_optional(SyntaxKind::ColonColon)?;

        let is_end = item.is_end();
        if !item.is_whitespace() {
            items.push(PathItem::ColonColon(item));
        }
        if is_end {
            break;
        }
    }

    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    for item in items {
        match item {
            PathItem::Identifier(item) => {
                formatted.extend(gen_node_with_trivia(&item)?);
            },
            PathItem::ColonColon(item) => {
                formatted.start_indent_before_requests();
                formatted.start_new_line_group_before_requests();
                formatted.request(Request::empty().or_newline());
                formatted.extend(gen_node_with_trivia(&item)?);
                formatted.finish_new_line_group_before_requests();
                formatted.finish_indent_before_requests();
            },
        }
    }
    Ok(formatted)
}
