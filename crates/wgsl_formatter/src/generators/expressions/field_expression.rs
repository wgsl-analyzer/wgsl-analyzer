use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::{
    ast_parse::{IgnoreBlankspace, NoTrivia, parse_end, parse_node_with, syntax_iter},
    generators::node::gen_node_with_trivia,
    print_item_buffer::{PrintItemBuffer, spacing_request::Request},
    reporting::FormatDocumentResult,
};

pub fn gen_field_expression(
    field_expression: &ast::FieldExpression
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = syntax_iter(field_expression.syntax());
    let item_struct_expr =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_castable_kind::<ast::Expression>()?;
    let item_period =
        parse_node_with(&mut syntax, NoTrivia).expect_kind(parser::SyntaxKind::Period)?;
    let item_target_ident = parse_node_with(&mut syntax, IgnoreBlankspace)
        .expect_kind(parser::SyntaxKind::Identifier)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_node_with_trivia(&item_struct_expr)?);
    formatted.start_indent();
    formatted.request(Request::empty().or_newline());
    formatted.extend(gen_node_with_trivia(&item_period)?);
    formatted.extend(gen_node_with_trivia(&item_target_ident)?);
    formatted.finish_indent();
    Ok(formatted)
}
