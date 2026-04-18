use dprint_core_macros::sc;
use itertools::put_back;
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self, Expression, IncrementDecrement},
};

use crate::format::{
    ast_parse::{parse_end, parse_node, parse_token, parse_token_optional},
    gen_comments::{gen_comments, parse_many_comments_and_blankspace},
    gen_expression::gen_expression,
    print_item_buffer::{PrintItemBuffer, request_folder::RequestItem},
    reporting::FormatDocumentError,
};

pub fn gen_increment_decrement_statement(
    increment_decrement_statement: &ast::IncrementDecrementStatement,
    include_semicolon: bool,
) -> Result<PrintItemBuffer, FormatDocumentError> {
    // NOTE!! - When changing this function, make sure to also update gen_phony_assignment_statement.
    // This is non-dry code, but when inevitably at some point there will be some differences between
    // the two, this should clearly communicate that they should be split up and not
    // continue to be one function with a whole lot of parameters and ifs.

    // ==== Parse ====
    let mut syntax = put_back(
        increment_decrement_statement
            .syntax()
            .children_with_tokens(),
    );

    let item_ident = parse_node::<Expression>(&mut syntax)?;
    let item_comments_after_ident = parse_many_comments_and_blankspace(&mut syntax)?;
    let inc_dec = if parse_token_optional(&mut syntax, SyntaxKind::PlusPlus).is_some() {
        IncrementDecrement::Increment
    } else {
        parse_token(&mut syntax, SyntaxKind::MinusMinus)?;
        IncrementDecrement::Decrement
    };
    let item_comments_after_inc_dec = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token_optional(&mut syntax, SyntaxKind::Semicolon);
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.extend(gen_expression(&item_ident, true)?);
    formatted.extend(gen_comments(&item_comments_after_ident));

    match inc_dec {
        IncrementDecrement::Increment => {
            formatted.push_sc(sc!("++"));
        },
        IncrementDecrement::Decrement => {
            formatted.push_sc(sc!("--"));
        },
    }

    formatted.extend(gen_comments(&item_comments_after_inc_dec));

    if include_semicolon {
        formatted.discourage(RequestItem::Space);
        formatted.push_sc(sc!(";"));
    }
    Ok(formatted)
}
