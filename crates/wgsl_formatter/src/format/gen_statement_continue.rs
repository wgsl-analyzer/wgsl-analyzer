use dprint_core_macros::sc;
use itertools::put_back;
use parser::SyntaxKind;
use syntax::{AstNode as _, ast};

use crate::format::{
    ast_parse::{parse_end, parse_many_comments_and_blankspace, parse_token, parse_token_optional},
    gen_comments::gen_comments,
    print_item_buffer::{PrintItemBuffer, request_folder::RequestItem},
    reporting::FormatDocumentResult,
};

pub fn gen_continue_statement(
    node: &ast::ContinueStatement,
    include_semicolon: bool,
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    // We still parse through the discard syntax even tho there is no information for
    // the formatter to get out of it. This exists to ensure we don't accidentally delete
    // user's code should future changes to wgsl allow more complex continue statements.
    let mut syntax = put_back(node.syntax().children_with_tokens());
    parse_token(&mut syntax, SyntaxKind::Continue)?;
    let comments_after_continue = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token_optional(&mut syntax, SyntaxKind::Semicolon);
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("continue"));
    if include_semicolon {
        formatted.push_sc(sc!(";"));
    }
    formatted.expect(RequestItem::LineBreak);
    formatted.extend(gen_comments(&comments_after_continue));
    Ok(formatted)
}
