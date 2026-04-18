use itertools::put_back;
use syntax::{
    AstNode as _,
    ast::{self, TemplateList},
};

use crate::{
    ast_parse::{parse_end, parse_node, parse_node_optional},
    generators::{
        comments::{gen_comments, parse_many_comments_and_blankspace},
        path::gen_path,
        types::gen_template_list,
    },
    print_item_buffer::PrintItemBuffer,
    reporting::FormatDocumentResult,
};

pub fn gen_ident_expression(
    ident_expression: &ast::IdentExpression
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(ident_expression.syntax().children_with_tokens());
    let item_path = parse_node::<ast::Path>(&mut syntax)?;
    let item_comments_after_name_reference = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_template = parse_node_optional::<TemplateList>(&mut syntax);
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.extend(gen_path(&item_path)?);
    formatted.extend(gen_comments(&item_comments_after_name_reference));
    if let Some(item_template) = item_template {
        formatted.extend(gen_template_list(&item_template)?);
    }
    Ok(formatted)
}
