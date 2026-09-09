use syntax::{AstNode as _, ast};

use crate::{
    ast_parse::{DiscardBlankspace, parse_end, parse_node_with, syntax_iter},
    generators::node::gen_node_with_trivia,
    print_item_buffer::PrintItemBuffer,
    reporting::FormatDocumentResult,
};

pub fn gen_name(name: &ast::Name) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = syntax_iter(name.syntax());
    let identifier =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_kind(ast::SyntaxKind::Identifier)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    gen_node_with_trivia(&identifier)
}
