use dprint_core::formatting::Signal;
use dprint_core_macros::sc;
use itertools::put_back;
use parser::SyntaxKind;
use rowan::NodeOrToken;
use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::format::{
    gen_attributes::{gen_attributes, parse_many_attributes},
    gen_comments::gen_comment,
    gen_statement::gen_statement_maybe_semicolon,
    helpers::gen_spaced_lines,
    print_item_buffer::{PrintItemBuffer, SeparationPolicy, SeparationRequest},
    reporting::{FormatDocumentErrorKind, FormatDocumentResult},
};

pub fn gen_compound_statement(
    syntax: &ast::CompoundStatement
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    //TODO I don't really like this, but its an easy way for now
    let body_empty = syntax.syntax().children_with_tokens().all(|child| {
        matches!(
            child.kind(),
            SyntaxKind::BraceLeft | SyntaxKind::BraceRight | SyntaxKind::Blankspace
        )
    });

    // First parse off the attributes
    let mut syntax = put_back(syntax.syntax().children_with_tokens());
    let item_attributes = parse_many_attributes(&mut syntax)?;

    let lines = gen_spaced_lines(&mut syntax, |child| {
        //TODO This clone is unnecessary if we had a cast that returned the passed in node
        // on a failure like std::any::Any (SyntaxNode -> Result<Item, Syntaxnode>)
        if let NodeOrToken::Node(child) = child
            && let Some(statement) = ast::Statement::cast(child.clone())
        {
            let mut formatted = PrintItemBuffer::new();
            formatted.request_line_break(SeparationPolicy::Expected);
            formatted.extend(gen_statement_maybe_semicolon(&statement, true)?);
            Ok(formatted)
        } else if let NodeOrToken::Token(child) = child
            && matches!(
                child.kind(),
                SyntaxKind::BlockComment | SyntaxKind::LineEndingComment
            )
        {
            Ok(gen_comment(child))
        } else if let NodeOrToken::Token(child) = child
            && matches!(child.kind(), SyntaxKind::BraceLeft | SyntaxKind::BraceRight)
        {
            //The braces are expected, we could pop them from the syntax before passing them to gen_spaced_lines,
            // but this way is just as fine
            Ok(PrintItemBuffer::new())
        } else {
            Err(FormatDocumentErrorKind::UnexpectedToken {
                received: child.clone(),
            }
            .at(child.text_range()))
        }
    })?;

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
        formatted.extend(lines);
        formatted.request(SeparationRequest {
            empty_line: SeparationPolicy::Discouraged,
            line_break: SeparationPolicy::Expected,
            ..Default::default()
        });
        formatted.push_signal(Signal::FinishIndent);
    }

    formatted.push_sc(sc!("}"));
    Ok(formatted)
}
