use dprint_core_macros::sc;
use itertools::put_back;
use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::format::{
    ast_parse::{parse_end, parse_node, parse_token},
    expressions::gen_expression::gen_expression,
    gen_comments::{gen_comments, parse_many_comments_and_blankspace},
    print_item_buffer::{PrintItemBuffer, request_folder::RequestItem},
    reporting::FormatDocumentResult,
};

pub fn gen_parenthesis_expression(
    parenthesis_expression: &ast::ParenthesisExpression,
    remove_parentheses: bool,
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(parenthesis_expression.syntax().children_with_tokens());
    parse_token(&mut syntax, parser::SyntaxKind::ParenthesisLeft)?;
    let item_comment_after_left_paren = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_content = parse_node::<ast::Expression>(&mut syntax)?;
    let item_comment_after_content = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, parser::SyntaxKind::ParenthesisRight)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    if remove_parentheses {
        formatted.expect(RequestItem::Space);
    } else {
        formatted.push_sc(sc!("("));
        formatted.start_new_line_group();
        formatted.start_indent();

        formatted.discourage(RequestItem::Space);
    }
    formatted.extend(gen_comments(&item_comment_after_left_paren));
    formatted.extend(gen_expression(&item_content, true)?);
    formatted.extend(gen_comments(&item_comment_after_content));

    if remove_parentheses {
        formatted.expect(RequestItem::Space);
    } else {
        formatted.discourage(RequestItem::Space);
        formatted.finish_indent();
        formatted.finish_new_line_group();
        formatted.push_sc(sc!(")"));
    }
    Ok(formatted)
}
