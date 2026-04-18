use dprint_core_macros::sc;
use itertools::put_back;
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self, Statement},
};

use crate::{
    ast_parse::{parse_end, parse_node_optional, parse_token, parse_token_optional},
    generators::{
        attributes::{AttributeLayout, gen_attributes, parse_many_attributes},
        comments::{Comment, gen_comment, parse_comment_optional},
        statements::gen_statement_maybe_semicolon,
    },
    helpers::{LineSpacing, gen_line_spacing, parse_line_spacing},
    print_item_buffer::{PrintItemBuffer, request_folder::RequestItem},
    reporting::FormatDocumentResult,
};

enum CompoundStatementItem {
    Statement(ast::Statement),
    Comment(Comment),
    LineSpacing(LineSpacing),
}

pub fn gen_compound_statement(
    syntax: &ast::CompoundStatement
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====

    let mut syntax = put_back(syntax.syntax().children_with_tokens());
    let item_attributes = parse_many_attributes(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::BraceLeft)?;

    let mut lines = Vec::new();
    let mut body_empty = true; //TODO (MonaMayrhofer) This annoys me, brittle, easy to forget

    loop {
        if let Some(spacing) = parse_line_spacing(&mut syntax) {
            lines.push(CompoundStatementItem::LineSpacing(spacing));
        } else if let Some(_statement) = parse_token_optional(&mut syntax, SyntaxKind::Blankspace) {
            // If its not a line_spacing blankspace, then we simply discard it
        } else if let Some(statement) = parse_node_optional::<Statement>(&mut syntax) {
            body_empty = false;
            lines.push(CompoundStatementItem::Statement(statement));
        } else if let Some(comment) = parse_comment_optional(&mut syntax) {
            body_empty = false;
            lines.push(CompoundStatementItem::Comment(comment));
        } else {
            break;
        }
    }
    parse_token(&mut syntax, SyntaxKind::BraceRight)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();

    formatted.extend(gen_attributes(
        &item_attributes,
        AttributeLayout::Multiline,
    )?);
    formatted.push_sc(sc!("{"));

    if !body_empty {
        formatted.start_indent();
        formatted.discourage(RequestItem::EmptyLine);
        formatted.expect(RequestItem::LineBreak);
        for line in lines {
            match line {
                CompoundStatementItem::Statement(statement) => {
                    formatted.expect(RequestItem::LineBreak);
                    formatted.extend(gen_statement_maybe_semicolon(&statement, true)?);
                },
                CompoundStatementItem::Comment(comment) => {
                    formatted.extend(gen_comment(&comment));
                },
                CompoundStatementItem::LineSpacing(line_spacing) => {
                    formatted.extend(gen_line_spacing(&line_spacing)?);
                },
            }
        }
        formatted.discourage(RequestItem::EmptyLine);
        formatted.expect(RequestItem::LineBreak);
        formatted.finish_indent();
    }

    formatted.push_sc(sc!("}"));

    if !body_empty {
        // This exists mainly for things like
        // fn a { let a = 1; } // Thing
        // ==>
        // fn a {
        //   let a = 1;
        // }
        // // Thing
        // So the comment is not on the same line as the closing brace.
        formatted.expect(RequestItem::LineBreak);
    }

    Ok(formatted)
}
