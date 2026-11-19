use dprint_core::formatting::Arena;
use syntax::{AstNode as _, ast};

use crate::{
    ast_parse::{DiscardBlankspace, NoTrivia, parse_end, parse_node_with, syntax_iter},
    generators::node::gen_node_with_trivia,
    print_item_buffer::{PrintItemBuffer, spacing_request::Request},
    reporting::FormatDocumentResult,
};

pub(crate) fn gen_field_expression<'buffer>(
    arena: &'buffer Arena,
    field_expression: &ast::FieldExpression,
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    // ==== Parse ====

    let mut syntax = syntax_iter(field_expression.syntax());
    let item_expression =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_ast_node::<ast::Expression>()?;
    let item_period =
        parse_node_with(&mut syntax, NoTrivia).expect_kind(syntax::SyntaxKind::Period)?;
    let item_target_ident = parse_node_with(&mut syntax, DiscardBlankspace)
        .expect_kind(syntax::SyntaxKind::Identifier)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new(arena);
    formatted.extend(arena, gen_node_with_trivia(arena, &item_expression)?);
    formatted.start_indent_before_requests(arena);
    formatted.start_new_line_group_before_requests(arena);
    formatted.request(Request::empty().or_newline());
    formatted.finish_new_line_group_after_requests(arena);
    formatted.extend(arena, gen_node_with_trivia(arena, &item_period)?);
    formatted.extend(arena, gen_node_with_trivia(arena, &item_target_ident)?);
    formatted.finish_indent_before_requests(arena);
    Ok(formatted)
}
