use dprint_core_macros::sc;
use itertools::put_back;
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self, Expression},
};

use crate::format::{
    ast_parse::{parse_end, parse_node, parse_token},
    expressions::gen_expression::gen_expression,
    gen_comments::{gen_comments, parse_many_comments_and_blankspace},
    print_item_buffer::{PrintItemBuffer, request_folder::RequestItem},
    reporting::FormatDocumentResult,
};

pub fn gen_const_assert_statement(
    statement: &ast::AssertStatement,
    include_semicolon: bool,
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(statement.syntax().children_with_tokens());
    parse_token(&mut syntax, SyntaxKind::ConstantAssert)?;
    let comments_after_const_assert = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_condition = parse_node::<Expression>(&mut syntax)?;
    let comments_after_condition = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::Semicolon)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();

    formatted.push_sc(sc!("const_assert"));
    formatted.start_indent();
    formatted.expect(RequestItem::Space);
    formatted.extend(gen_comments(&comments_after_const_assert));
    formatted.extend(gen_expression(&item_condition, true)?);
    formatted.extend(gen_comments(&comments_after_condition));
    if include_semicolon {
        formatted.discourage(RequestItem::Space);
        formatted.push_sc(sc!(";"));
    }
    formatted.finish_indent();

    Ok(formatted)
}
