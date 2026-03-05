use dprint_core::formatting::Signal;
use dprint_core_macros::sc;
use itertools::put_back;
use parser::{SyntaxKind, WeslLanguage};
use rowan::{NodeOrToken, SyntaxToken};
use syntax::{
    AstNode as _,
    ast::{self, Statement},
};

use crate::format::{
    self,
    ast_parse::{parse_end, parse_node_optional, parse_token, parse_token_optional},
    gen_attributes::{gen_attributes, parse_many_attributes},
    gen_comments::{Comment, gen_comment, parse_comment_optional},
    gen_statement::gen_statement_maybe_semicolon,
    helpers::{LineSpacing, gen_line_spacing, line_spacing},
    print_item_buffer::{PrintItemBuffer, SeparationPolicy, SeparationRequest},
    reporting::{FormatDocumentErrorKind, FormatDocumentResult},
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
        if let Some(spacing) = line_spacing(&mut syntax) {
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

    formatted.extend(gen_attributes(&item_attributes)?);
    formatted.push_sc(sc!("{"));

    if !body_empty {
        formatted.push_signal(Signal::StartIndent);
        formatted.request(SeparationRequest {
            empty_line: SeparationPolicy::Discouraged,
            line_break: SeparationPolicy::Expected,
            ..Default::default()
        });
        for line in lines {
            match line {
                CompoundStatementItem::Statement(statement) => {
                    formatted.expect_line_break();
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
        formatted.request(SeparationRequest {
            empty_line: SeparationPolicy::Discouraged,
            line_break: SeparationPolicy::Expected,
            ..Default::default()
        });
        formatted.push_signal(Signal::FinishIndent);
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
        formatted.expect_line_break();
    }

    Ok(formatted)
}
