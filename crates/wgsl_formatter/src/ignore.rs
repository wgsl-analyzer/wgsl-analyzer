//! Code responsible for detecting ignore-pragmas.

use parser::{SyntaxNode, SyntaxToken};
use rowan::NodeOrToken;

use crate::generators::comments::{Comment, read_comment};

/// Whether the `SyntaxNode`'s first interesting child is a [ignore-parent-pragma](`is_ignore_parent_pragma_comment`).
#[must_use]
pub fn is_ignored_from_within(content: &SyntaxNode) -> bool {
    content
        .children_with_tokens()
        .take_while(|child| match child {
            NodeOrToken::Node(_) => false,
            NodeOrToken::Token(_) => true,
        })
        .any(|child| is_ignore_parent_pragma_comment(&child))
}

/// Whether the given item is a comment with `@wgslfmt(ignore)`.
#[must_use]
pub fn is_ignore_next_pragma_comment(node: &NodeOrToken<SyntaxNode, SyntaxToken>) -> bool {
    let as_comment = read_comment(node);
    match as_comment {
        Some(Comment::Block(syntax_token))
            if syntax_token.text().trim() == "/* @wgslfmt(ignore) */" =>
        {
            true
        },
        Some(Comment::LineEnding(syntax_token))
            if syntax_token.text().trim() == "// @wgslfmt(ignore)" =>
        {
            true
        },
        _ => false,
    }
}

/// Whether the given item is a comment with `@!wgslfmt(ignore)`.
#[must_use]
pub fn is_ignore_parent_pragma_comment(node: &NodeOrToken<SyntaxNode, SyntaxToken>) -> bool {
    let as_comment = read_comment(node);
    match as_comment {
        Some(Comment::Block(syntax_token))
            if syntax_token.text().trim() == "/* @!wgslfmt(ignore) */" =>
        {
            true
        },
        Some(Comment::LineEnding(syntax_token))
            if syntax_token.text().trim() == "// @!wgslfmt(ignore)" =>
        {
            true
        },
        _ => false,
    }
}
