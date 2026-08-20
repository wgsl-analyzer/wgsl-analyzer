use dprint_core_macros::sc;
use parser::{SyntaxKind, SyntaxNode};
use syntax::{
    AstNode as _,
    ast::{self},
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

pub fn gen_return_statement(
    statement: &ast::ReturnStatement
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = syntax_iter(statement.syntax());
    let item_return =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(SyntaxKind::Return)?;
    let item_expression = parse_node_with(&mut syntax, IgnoreBlankspace)
        .only_if_ast_node::<ast::Expression>(&mut syntax);
    parse_node_with(&mut syntax, NoTrivia).expect_kind_optional(SyntaxKind::Semicolon)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_node_with_trivia(&item_return)?);
    formatted.start_indent_before_requests();
    if let Some(item_expression) = item_expression {
        formatted.request(Request::expect(RequestItem::Space));
        formatted.extend(gen_node_with_trivia(&item_expression)?);
    }

    if statement_needs_semicolon_policy(statement.syntax()) {
        formatted.request(Request::discourage(RequestItem::Space));
        formatted.push_sc(sc!(";"));
    }
    formatted.finish_indent_before_requests();
    Ok(formatted)
}

pub fn remove_return_value_parens_rule(node: &SyntaxNode) -> bool {
    let Some(parent) = node.parent() else {
        return false;
    };
    matches!(parent.kind(), SyntaxKind::ReturnStatement)
}
