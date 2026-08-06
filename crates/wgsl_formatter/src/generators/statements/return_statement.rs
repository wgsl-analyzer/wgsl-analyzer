use dprint_core_macros::sc;
use itertools::put_back;
use parser::{SyntaxKind, SyntaxNode};
use syntax::{
    AstNode as _,
    ast::{self, Expression},
};

use crate::{
    ast_parse::{
        NoTrivia, parse_end, parse_node_optional, parse_node_with, parse_token,
        parse_token_optional,
    },
    context_policies::statement_needs_semicolon_policy,
    generators::{
        comments::{gen_comments, parse_many_comments_and_blankspace},
        expressions::gen_expression,
    },
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
    let mut syntax = put_back(statement.syntax().children_with_tokens());
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Return)?;
    let comments_after_return = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_expression = parse_node_optional::<Expression>(&mut syntax);
    let comments_after_expression = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind_optional(SyntaxKind::Semicolon)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.push_sc(sc!("return"));
    formatted.start_indent();
    formatted.extend(gen_comments(&comments_after_return));
    if let Some(item_expression) = item_expression {
        formatted.request(Request::expect(RequestItem::Space));
        formatted.extend(gen_expression(&item_expression)?);
    }
    formatted.extend(gen_comments(&comments_after_expression));

    if statement_needs_semicolon_policy(statement.syntax()) {
        formatted.request(Request::discourage(RequestItem::Space));
        formatted.push_sc(sc!(";"));
    }
    formatted.finish_indent();
    Ok(formatted)
}

pub fn remove_return_value_parens_rule(node: &SyntaxNode) -> bool {
    let Some(parent) = node.parent() else {
        return false;
    };
    matches!(parent.kind(), SyntaxKind::ReturnStatement)
}
