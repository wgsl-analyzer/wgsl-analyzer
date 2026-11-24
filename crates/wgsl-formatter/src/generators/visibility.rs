use syntax::{
    AstNode as _,
    ast::{self, VisibilityToken},
};

use crate::{
    FormattingOptions,
    ast_parse::{NoTrivia, parse_end, parse_node_with, syntax_iter},
    generators::node::gen_node_with_trivia,
    print_item_buffer::PrintItemBuffer,
    reporting::FormatDocumentResult,
};

pub(crate) fn gen_visibility(
    options: &FormattingOptions,
    node: &ast::Visibility,
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = syntax_iter(node.syntax());
    let visibility =
        parse_node_with(&mut syntax, NoTrivia).expect_ast_token::<VisibilityToken>()?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    gen_node_with_trivia(options, &visibility)
}
