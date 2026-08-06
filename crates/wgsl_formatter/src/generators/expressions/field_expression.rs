use itertools::put_back;
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::{
    ast_parse::{
        FilterAction, IgnoreBlankspace, NoTrivia, parse_end, parse_node_with,
        parse_node_with_trivia_filter,
    },
    generators::node::gen_node_with_trivia,
    print_item_buffer::{PrintItemBuffer, spacing_request::Request},
    reporting::FormatDocumentResult,
};

pub fn gen_field_expression(
    field_expression: &ast::FieldExpression
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(field_expression.syntax().children_with_tokens());
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
