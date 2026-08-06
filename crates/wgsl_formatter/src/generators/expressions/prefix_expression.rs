use std::iter::Filter;

use itertools::put_back;
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::{
    ast_parse::{
        FilterAction, parse_end, parse_node, parse_node_with_trivia_filter,
        parse_node_with_trivia_filter_2, parse_token_any,
    },
    generators::{
        comments::{gen_comments, parse_many_comments_and_blankspace},
        expressions::gen_expression,
        node::{gen_node, gen_node_with_trivia},
    },
    print_item_buffer::PrintItemBuffer,
    reporting::FormatDocumentResult,
};
pub fn gen_prefix_expression(
    infix_expression: &ast::PrefixExpression
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(infix_expression.syntax().children_with_tokens());

    let item_operator = parse_node_with_trivia_filter_2(
        &mut syntax,
        |node| matches!(node.kind(), SyntaxKind::Blankspace).then_some(FilterAction::Ignored),
        |_| Some(FilterAction::Stop),
    );
    let item_expr = parse_node_with_trivia_filter(&mut syntax, |node| {
        matches!(node.kind(), SyntaxKind::Blankspace).then_some(FilterAction::Ignored)
    })
    .expect_castable_kind::<ast::Expression>()?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    //I don't like to-stringing the operator here, would be better to special case on it,
    //In a benchmark on my machine however, removing this push_string() did not make any measurable impact - so it's fine.
    formatted.extend(gen_node_with_trivia(&item_operator)?);
    formatted.extend(gen_node_with_trivia(&item_expr)?);
    Ok(formatted)
}
