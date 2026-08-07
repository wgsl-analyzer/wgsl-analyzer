use dprint_core_macros::sc;
use itertools::put_back;
use parser::{SyntaxKind, SyntaxNode};
use syntax::{
    AstNode as _,
    ast::{self, Expression},
};

use crate::{
    ast_parse::{IgnoreBlankspace, NoTrivia, parse_end, parse_node_with, syntax_iter},
    context_policies::statement_needs_semicolon_policy,
    generators::node::gen_node_with_trivia,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

pub fn gen_const_assert_statement(
    statement: &ast::AssertStatement
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = syntax_iter(statement.syntax());
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::ConstantAssert)?;
    let item_condition =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_castable_kind::<Expression>()?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Semicolon)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    formatted.push_sc(sc!("const_assert"));
    formatted.start_indent();
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_node_with_trivia(&item_condition)?);
    if statement_needs_semicolon_policy(statement.syntax()) {
        formatted.request(Request::discourage(RequestItem::Space));
        formatted.push_sc(sc!(";"));
    }
    formatted.finish_indent();

    Ok(formatted)
}

pub fn remove_const_assert_condition_parens_rule(node: &SyntaxNode) -> bool {
    let Some(parent) = node.parent() else {
        return false;
    };
    matches!(parent.kind(), SyntaxKind::AssertStatement)
}
