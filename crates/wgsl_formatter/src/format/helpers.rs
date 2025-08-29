use std::{alloc::alloc, iter::repeat_with};

use dprint_core::formatting::{PrintItems, PrintOptions, Signal, StringContainer};
use parser::{SyntaxKind, SyntaxNode, SyntaxToken};
use rowan::NodeOrToken;
use syntax::{
    AstNode as _, HasName as _,
    ast::{self},
};

use crate::{
    FormattingOptions,
    format::{
        print_item_buffer::{PrintItemBuffer, PrintItemRequest, SeparationPolicy},
        reporting::{FormatDocumentErrorKind, FormatDocumentResult, err_src},
    },
};

/// Lays out the children of a node in a way so that
/// - after every node is exactly 1 or 2 newlines (aka. 0 to 1 blank lines)
/// - there are no newlines before the first node
pub fn gen_spaced_lines<F>(
    node: &parser::SyntaxNode,
    mut pretty_item: F,
) -> FormatDocumentResult<PrintItemBuffer>
where
    F: FnMut(&NodeOrToken<SyntaxNode, SyntaxToken>) -> FormatDocumentResult<PrintItemBuffer>,
{
    let mut result = PrintItemBuffer::new();

    enum NewLineState {
        AtStartOfBlock,
        NewLinesAfterItem(usize),
    }

    result.request(PrintItemRequest::discouraged());

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
            if newlines >= 2 {
                //There was an empty line in the source
                result.request(PrintItemRequest {
                    empty_line: SeparationPolicy::Expected,
                    ..Default::default()
                });
            }
        } else {
            result.extend(pretty_item(&child)?);
            result.request(PrintItemRequest {
                line_break: SeparationPolicy::Expected,
                ..Default::default()
            });
        }
    }

    //There should be a newline at the end of the file
    result.request(PrintItemRequest {
        line_break: SeparationPolicy::Expected,
        ..Default::default()
    });

    Ok(result)
}

#[inline]
pub fn into_items(sc: &'static StringContainer) -> PrintItemBuffer {
    let mut pi = PrintItemBuffer::new();
    pi.push_sc(sc);
    pi
}
