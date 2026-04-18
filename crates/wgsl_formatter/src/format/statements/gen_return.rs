use dprint_core_macros::sc;
use itertools::put_back;
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self, Expression},
};

use crate::format::{
    ast_parse::{parse_end, parse_node_optional, parse_token, parse_token_optional},
    expressions::gen_expression::gen_expression,
    gen_comments::{gen_comments, parse_many_comments_and_blankspace},
    print_item_buffer::{PrintItemBuffer, request_folder::RequestItem},
    reporting::FormatDocumentResult,
};

pub fn gen_return_statement(
    statement: &ast::ReturnStatement,
    include_semicolon: bool,
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(statement.syntax().children_with_tokens());
    parse_token(&mut syntax, SyntaxKind::Return)?;
    let comments_after_return = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_expression = parse_node_optional::<Expression>(&mut syntax);
    let comments_after_expression = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token_optional(&mut syntax, SyntaxKind::Semicolon);
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("return"));
    formatted.start_indent();
    formatted.extend(gen_comments(&comments_after_return));
    if let Some(item_expression) = item_expression {
        formatted.expect(RequestItem::Space);
        formatted.extend(gen_expression(&item_expression, true)?);
    }
    formatted.extend(gen_comments(&comments_after_expression));

    if include_semicolon {
        formatted.discourage(RequestItem::Space);
        formatted.push_sc(sc!(";"));
    }
    formatted.finish_indent();
    Ok(formatted)
}
