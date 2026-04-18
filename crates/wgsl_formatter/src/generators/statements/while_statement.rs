use dprint_core_macros::sc;
use itertools::put_back;
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self, CompoundStatement, Expression},
};

use crate::{
    ast_parse::{parse_end, parse_node, parse_token},
    generators::{
        expressions::gen_expression,
        gen_attributes::{AttributeLayout, gen_attributes, parse_many_attributes},
        gen_comments::{gen_comments, parse_many_comments_and_blankspace},
        statements::compound_statement::gen_compound_statement,
    },
    print_item_buffer::{PrintItemBuffer, request_folder::RequestItem},
    reporting::FormatDocumentResult,
};

pub fn gen_while_statement(
    statement: &ast::WhileStatement
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(statement.syntax().children_with_tokens());
    let item_attributes = parse_many_attributes(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::While)?;
    let comments_after_while = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_condition = parse_node::<Expression>(&mut syntax)?;
    let comments_after_condition = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_body = parse_node::<CompoundStatement>(&mut syntax)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.extend(gen_attributes(
        &item_attributes,
        AttributeLayout::Multiline,
    )?);
    formatted.push_sc(sc!("while"));
    formatted.extend(gen_comments(&comments_after_while));
    formatted.expect(RequestItem::Space);
    formatted.extend(gen_expression(&item_condition, true)?);
    formatted.expect(RequestItem::Space);
    formatted.extend(gen_comments(&comments_after_condition));
    formatted.extend(gen_compound_statement(&item_body)?);
    formatted.expect(RequestItem::LineBreak);

    Ok(formatted)
}
