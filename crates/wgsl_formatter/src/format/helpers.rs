use std::{alloc::alloc, iter::repeat_with};

use parser::SyntaxNode;
use pretty::{BoxAllocator, DocAllocator, DocBuilder};
use syntax::{
    AstNode as _, HasName as _,
    ast::{self},
};

use crate::{FormattingOptions, format::reporting::FormatDocumentResult};

/// Lays out the children of a node in a way so that
/// - after every node is exactly 1 or 2 newlines (aka. 0 to 1 blank lines)
/// - there are no newlines before the first node
pub fn pretty_spaced_lines<'ann, D, TAnnotation, F>(
    node: &parser::SyntaxNode,
    allocator: &'ann D,
    mut pretty_node: F,
) -> FormatDocumentResult<DocBuilder<'ann, D, TAnnotation>>
where
    D: DocAllocator<'ann, TAnnotation>,
    F: FnMut(SyntaxNode) -> FormatDocumentResult<DocBuilder<'ann, D, TAnnotation>>,
{
    let mut result = allocator.nil();

    enum NewLineState {
        AtStartOfBlock,
        NewLinesAfterNode(usize),
    }

    let mut new_line_state = NewLineState::AtStartOfBlock;

    for child in node.children_with_tokens() {
        match child {
            rowan::NodeOrToken::Token(token) => {
                if token.kind() == parser::SyntaxKind::Blankspace {
                    //TODO Think a bit more about different types of newlines
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
                }
            },
            rowan::NodeOrToken::Node(node) => {
                match new_line_state {
                    NewLineState::AtStartOfBlock => {},
                    NewLineState::NewLinesAfterNode(count) => {
                        result =
                            result.append(allocator.concat(
                                repeat_with(|| allocator.hardline()).take(count.clamp(1, 2)),
                            ));
                    },
                }

                result = result.append(pretty_node(node)?);
                new_line_state = NewLineState::NewLinesAfterNode(0);
            },
        }
    }

    match new_line_state {
        NewLineState::AtStartOfBlock => {},
        NewLineState::NewLinesAfterNode(count) => {
            //There should be a newline at the end of the file
            result = result.append(allocator.hardline());
        },
    }

    Ok(result)
}
