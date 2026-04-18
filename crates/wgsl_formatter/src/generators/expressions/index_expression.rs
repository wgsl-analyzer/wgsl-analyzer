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
    multiline_group::MultilineGroup,
    print_item_buffer::PrintItemBuffer,
    reporting::FormatDocumentResult,
};

pub fn gen_index_expression(
    index_expression: &ast::IndexExpression
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(index_expression.syntax().children_with_tokens());
    let item_array_expr = parse_node::<ast::Expression>(&mut syntax)?;
    let comments_after_ident_expr = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, parser::SyntaxKind::BracketLeft)?;
    let comments_after_open_bracket = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_actual_index = parse_node::<ast::Expression>(&mut syntax)?;
    let comments_after_index_expr = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, parser::SyntaxKind::BracketRight)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();

    formatted.extend(gen_expression(&item_array_expr, false)?);
    formatted.extend(gen_comments(&comments_after_ident_expr));

    let mut multiline_group = MultilineGroup::new(&mut formatted);

    multiline_group.push_sc(sc!("["));

    multiline_group.start_indent();

    multiline_group.extend(gen_comments(&comments_after_open_bracket));
    multiline_group.extend(gen_expression(&item_actual_index, true)?);
    multiline_group.extend(gen_comments(&comments_after_index_expr));
    multiline_group.grouped_newline_or_space();

    multiline_group.finish_indent();

    multiline_group.push_sc(sc!("]"));

    multiline_group.end();

    Ok(formatted)
}
