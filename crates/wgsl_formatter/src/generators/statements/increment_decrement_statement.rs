use dprint_core_macros::sc;
use parser::SyntaxKind;
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
    reporting::FormatDocumentError,
};

pub fn gen_increment_decrement_statement(
    increment_decrement_statement: &ast::IncrementDecrementStatement
) -> Result<PrintItemBuffer, FormatDocumentError> {
    // ==== Parse ====
    let mut syntax = syntax_iter(increment_decrement_statement.syntax());

    let item_ident =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_ast_node::<Expression>()?;
    let item_operator = parse_node_with(&mut syntax, IgnoreBlankspace); // TODO(MonaMayrhofer,outdated) its fine - nothing surprising will appear here
    parse_node_with(&mut syntax, NoTrivia).expect_kind_optional(SyntaxKind::Semicolon)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_node_with_trivia(&item_ident)?);
    formatted.extend(gen_node_with_trivia(&item_operator)?);

    if statement_needs_semicolon_policy(increment_decrement_statement.syntax()) {
        formatted.request(Request::discourage(RequestItem::Space));
        formatted.push_sc(sc!(";"));
    }
    Ok(formatted)
}
