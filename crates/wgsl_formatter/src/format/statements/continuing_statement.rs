use dprint_core_macros::sc;
use itertools::put_back;
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self, CompoundStatement},
};

use crate::format::{
    ast_parse::{parse_end, parse_node, parse_token},
    gen_comments::{gen_comments, parse_many_comments_and_blankspace},
    print_item_buffer::{PrintItemBuffer, request_folder::RequestItem},
    reporting::FormatDocumentResult,
    statements::compound_statement::gen_compound_statement,
};

pub fn gen_continuing_statement(
    statement: &ast::ContinuingStatement
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(statement.syntax().children_with_tokens());
    parse_token(&mut syntax, SyntaxKind::Continuing)?;
    let comments_after_continuing = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_body = parse_node::<CompoundStatement>(&mut syntax)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("continuing"));
    formatted.extend(gen_comments(&comments_after_continuing));
    formatted.expect(RequestItem::Space);
    formatted.extend(gen_compound_statement(&item_body)?);
    formatted.expect(RequestItem::LineBreak);

    Ok(formatted)
}
