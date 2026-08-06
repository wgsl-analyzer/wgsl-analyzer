use dprint_core_macros::sc;
use itertools::put_back;
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self, CompoundStatement},
};

use crate::{
    ast_parse::{
        FilterAction, IgnoreBlankspace, parse_end, parse_node, parse_node_with,
        parse_node_with_trivia_filter, parse_token,
    },
    generators::{
        comments::{gen_comments, parse_many_comments_and_blankspace},
        node::gen_node_with_trivia,
        statements::compound_statement::gen_compound_statement,
    },
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

pub fn gen_continuing_statement(
    statement: &ast::ContinuingStatement
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(statement.syntax().children_with_tokens());
    parse_token(&mut syntax, SyntaxKind::Continuing)?;
    let item_body = parse_node_with(&mut syntax, IgnoreBlankspace)
        .expect_kind(SyntaxKind::CompoundStatement)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.push_sc(sc!("continuing"));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_node_with_trivia(&item_body)?);
    formatted.request(Request::expect(RequestItem::LineBreak));

    Ok(formatted)
}
