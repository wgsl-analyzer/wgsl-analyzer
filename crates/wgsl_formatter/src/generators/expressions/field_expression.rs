use dprint_core_macros::sc;
use itertools::put_back;
use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::{
    ast_parse::{parse_end, parse_node, parse_token},
    generators::{
        comments::{gen_comments, parse_many_comments_and_blankspace},
        expressions::gen_expression,
    },
    print_item_buffer::PrintItemBuffer,
    reporting::FormatDocumentResult,
};

pub fn gen_field_expression(
    field_expression: &ast::FieldExpression
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(field_expression.syntax().children_with_tokens());
    let item_struct_expr = parse_node::<ast::Expression>(&mut syntax)?;
    let comments_after_ident_expr = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, parser::SyntaxKind::Period)?;
    let comments_after_period = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_target_ident = parse_token(&mut syntax, parser::SyntaxKind::Identifier)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_expression(&item_struct_expr, false)?);
    formatted.extend(gen_comments(&comments_after_ident_expr));
    formatted.push_sc(sc!("."));
    formatted.extend(gen_comments(&comments_after_period));
    formatted.push_string(item_target_ident.text().to_owned());
    Ok(formatted)
}
