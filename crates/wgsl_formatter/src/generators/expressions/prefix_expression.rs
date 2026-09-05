use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::{
    ast_parse::{DiscardBlankspace, NoTrivia, parse_end, parse_node_with, syntax_iter},
    generators::node::gen_node_with_trivia,
    print_item_buffer::PrintItemBuffer,
    reporting::FormatDocumentResult,
};
pub fn gen_prefix_expression(
    infix_expression: &ast::PrefixExpression
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = syntax_iter(infix_expression.syntax());

    let item_operator = parse_node_with(&mut syntax, NoTrivia);
    let item_expr =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_ast_node::<ast::Expression>()?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    //I don't like to-stringing the operator here, would be better to special case on it,
    //In a benchmark on my machine however, removing this push_string() did not make any measurable impact - so it's fine.
    formatted.extend(gen_node_with_trivia(&item_operator)?);
    formatted.extend(gen_node_with_trivia(&item_expr)?);
    Ok(formatted)
}
