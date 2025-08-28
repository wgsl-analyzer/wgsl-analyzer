use std::{alloc::alloc, iter::repeat_with};

use dprint_core::formatting::{PrintItems, PrintOptions, Signal, StringContainer};
use parser::SyntaxNode;
use syntax::{
    AstNode as _, HasName as _,
    ast::{self},
};

use crate::{
    FormattingOptions,
    format::{
        handle_comments,
        reporting::{FormatDocumentErrorKind, FormatDocumentResult},
    },
};

/// Lays out the children of a node in a way so that
/// - after every node is exactly 1 or 2 newlines (aka. 0 to 1 blank lines)
/// - there are no newlines before the first node
pub fn gen_spaced_lines<F>(
    node: &parser::SyntaxNode,
    mut pretty_node: F,
) -> FormatDocumentResult<PrintItems>
where
    F: FnMut(&SyntaxNode) -> FormatDocumentResult<PrintItems>,
{
    let mut result = PrintItems::new();

    enum NewLineState {
        AtStartOfBlock,
        NewLinesAfterNode(usize),
    }

    let mut new_line_state = NewLineState::AtStartOfBlock;

    for child in node.children_with_tokens() {
        if let rowan::NodeOrToken::Token(token) = &child
            && token.kind() == parser::SyntaxKind::Blankspace
        {
            // if the token is a blankspace, collapse the newlines according to the rules.

            //TODO Think a bit more about different types of newlines (\c\n etc.)
            //TODO child.to_string() here surely is wasteful - there must be a better way.

            let newlines = token
                .to_string()
                .chars()
                .filter(|item| *item == '\n')
                .count();
            new_line_state = match new_line_state {
                //no newlines at start of block
                NewLineState::AtStartOfBlock => NewLineState::AtStartOfBlock,
                NewLineState::NewLinesAfterNode(count) => {
                    NewLineState::NewLinesAfterNode(count + newlines)
                },
            };
        } else {
            // else the child is something to be formatted, so print out the collapsed newlines
            match new_line_state {
                NewLineState::AtStartOfBlock => {},
                NewLineState::NewLinesAfterNode(count) => {
                    for _ in (0..count.clamp(1, 2)) {
                        result.push_signal(Signal::NewLine);
                    }
                },
            }
            if let Some(items) = handle_comments(&child, &mut true) {
                result.extend(items?);
            } else if let rowan::NodeOrToken::Node(node) = &child {
                result.extend(pretty_node(node)?);
            } else {
                return Err(FormatDocumentErrorKind::UnexpectedToken.at(child.text_range()));
            }
            new_line_state = NewLineState::NewLinesAfterNode(0);
        }
    }

    match new_line_state {
        NewLineState::AtStartOfBlock => {},
        NewLineState::NewLinesAfterNode(count) => {
            //There should be a newline at the end of the file
            result.push_signal(Signal::NewLine);
        },
    }

    Ok(result)
}

#[inline]
pub fn into_items(sc: &'static StringContainer) -> PrintItems {
    let mut pi = PrintItems::default();
    pi.push_sc(sc);
    pi
}
