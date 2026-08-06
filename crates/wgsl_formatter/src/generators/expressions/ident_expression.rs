use itertools::put_back;
use syntax::{
    AstNode as _,
    ast::{self, SyntaxKind, TemplateList},
};

use crate::{
    ast_parse::{
        FilterAction, IgnoreBlankspace, parse_end, parse_node_with_trivia_filter,
        parse_node_with_trivia_until,
    },
    generators::node::gen_node_with_trivia,
    print_item_buffer::PrintItemBuffer,
    reporting::FormatDocumentResult,
};

pub fn gen_ident_expression(
    ident_expression: &ast::IdentExpression
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(ident_expression.syntax().children_with_tokens());
    let item_path = parse_node_with_trivia_until(&mut syntax, IgnoreBlankspace)
        .expect_kind(SyntaxKind::Path)?;
    let item_template = {
        let item = parse_node_with_trivia_until(&mut syntax, IgnoreBlankspace);
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
