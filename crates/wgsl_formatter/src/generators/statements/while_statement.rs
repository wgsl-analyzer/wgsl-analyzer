use dprint_core_macros::sc;
use parser::{SyntaxKind, SyntaxNode};
use syntax::{
    AstNode as _,
    ast::{self, CompoundStatement, Expression},
};

use crate::{
    ast_parse::{DiscardBlankspace, NoTrivia, parse_end, parse_node_with, syntax_iter},
    generators::node::gen_node_with_trivia,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

pub fn gen_while_statement(
    statement: &ast::WhileStatement
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = syntax_iter(statement.syntax());
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::While)?;
    let item_condition =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_ast_node::<Expression>()?;
    let item_body =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_ast_node::<CompoundStatement>()?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    // formatted.extend(gen_attributes(
    //     &item_attributes,
    //     AttributeLayout::Multiline,
    // )?);
    formatted.push_sc(sc!("while"));
    formatted.start_indent_before_requests();
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_node_with_trivia(&item_condition)?);
    formatted.finish_indent_before_requests();
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_node_with_trivia(&item_body)?);
    formatted.request(Request::expect(RequestItem::LineBreak));

    Ok(formatted)
}

#[must_use]
pub fn remove_while_condition_parens_rule(node: &SyntaxNode) -> bool {
    let Some(parent) = node.parent() else {
        return false;
    };
    matches!(parent.kind(), SyntaxKind::WhileStatement)
}
