use itertools::put_back;
use syntax::{
    AstNode as _,
    ast::{self, SyntaxKind},
};

use crate::{
    ast_parse::{IgnoreBlankspace, parse_end, parse_node_with, syntax_iter},
    generators::node::gen_node_with_trivia,
    print_item_buffer::PrintItemBuffer,
    reporting::FormatDocumentResult,
};

pub fn gen_ident_expression(
    ident_expression: &ast::IdentExpression
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = syntax_iter(ident_expression.syntax());
    let item_path = parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(SyntaxKind::Path)?;
    let item_template = {
        let item = parse_node_with(&mut syntax, IgnoreBlankspace);
        (item.kind() == Some(SyntaxKind::TemplateList)).then_some(item)
    };
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_node_with_trivia(&item_path)?);
    if let Some(item_template) = item_template {
        formatted.extend(gen_node_with_trivia(&item_template)?);
    }
    Ok(formatted)
}
