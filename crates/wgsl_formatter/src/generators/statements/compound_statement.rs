use dprint_core_macros::sc;
use itertools::put_back;
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self, Statement},
};

use crate::{
    ast_parse::{parse_end, parse_node_optional, parse_token, parse_token_optional},
    context_policies::collapse_one_liner_compound_statement_policy,
    generators::{
        attributes::{AttributeLayout, gen_attributes, parse_many_attributes},
        comments::{Comment, gen_comment, parse_comment_optional},
        statements::gen_statement_maybe_semicolon,
    },
    helpers::{LineSpacing, gen_line_spacing, parse_line_spacing},
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

pub fn gen_compound_statement(
    node: &ast::CompoundStatement
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Context ====
    let starting_attribute_layout = if let Some(parent) = node.syntax().parent() {
        if parent.kind() == SyntaxKind::FunctionDeclaration {
            AttributeLayout::Inline
        } else {
            AttributeLayout::Multiline
        }
    } else {
        AttributeLayout::Multiline
    };

    // ==== Parse ====

    let mut syntax = put_back(node.syntax().children_with_tokens());
    let item_attributes = parse_many_attributes(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::BraceLeft)?;

    enum CompoundStatementItem {
        Statement(ast::Statement),
        Comment(Comment),
        LineSpacing(LineSpacing),
    }
    let mut lines = Vec::new();
    let mut body_empty = true;

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

    let is_one_liner = lines
        .iter()
        .filter(|it| {
            matches!(
                it,
                CompoundStatementItem::Comment(_) | CompoundStatementItem::Statement(_)
            )
        })
        .count()
        == 1;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    formatted.extend(gen_attributes(&item_attributes, starting_attribute_layout)?);
    formatted.push_sc(sc!("{"));

    if !body_empty {
        formatted.start_indent();
        if is_one_liner && collapse_one_liner_compound_statement_policy(node.syntax()) {
            formatted.request(Request::discourage(RequestItem::LineBreak));
            formatted.request(Request::expect(RequestItem::Space));
        } else {
            formatted.request(Request::discourage(RequestItem::EmptyLine));
            formatted.request(Request::expect(RequestItem::LineBreak));
        }

        for line in lines {
            match line {
                CompoundStatementItem::Statement(statement) => {
                    formatted.request(Request::expect(RequestItem::LineBreak));
                    formatted.extend(gen_statement_maybe_semicolon(&statement)?);
                },
                CompoundStatementItem::Comment(comment) => {
                    formatted.extend(gen_comment(&comment));
                },
                CompoundStatementItem::LineSpacing(line_spacing) => {
                    formatted.extend(gen_line_spacing(&line_spacing)?);
                },
            }
        }
        if is_one_liner && collapse_one_liner_compound_statement_policy(node.syntax()) {
            formatted.request(Request::discourage(RequestItem::LineBreak));
            formatted.request(Request::expect(RequestItem::Space));
        } else {
            formatted.request(Request::discourage(RequestItem::EmptyLine));
            formatted.request(Request::expect(RequestItem::LineBreak));
        }
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
        formatted.request(Request::expect(RequestItem::LineBreak));
    }

    Ok(formatted)
}
