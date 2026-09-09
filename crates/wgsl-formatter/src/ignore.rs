//! Code responsible for detecting ignore-pragmas.

use dprint_core_macros::sc;
use rowan::NodeOrToken;
use syntax::{SyntaxNode, SyntaxToken};

use crate::{
    generators::comments::{Comment, read_comment},
    print_item_buffer::PrintItemBuffer,
};

#[derive(Debug, Clone)]
pub(crate) struct IgnorePragma {
    pub(crate) block: bool,
    pub(crate) token: SyntaxToken,
}

impl IgnorePragma {
    #[must_use]
    pub(crate) fn syntax(
        &self
    ) -> NodeOrToken<
        rowan::SyntaxNode<syntax::WeslLanguage>,
        rowan::SyntaxToken<syntax::WeslLanguage>,
    > {
        NodeOrToken::Token(self.token.clone())
    }
}

/// Whether the `SyntaxNode`'s first interesting child is a [ignore-parent-pragma](`is_ignore_parent_pragma_comment`).
#[must_use]
pub(crate) fn is_ignored_from_within(content: &SyntaxNode) -> bool {
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
pub(crate) fn read_ignore_next_pragma_comment(
    node: &NodeOrToken<SyntaxNode, SyntaxToken>
) -> Option<IgnorePragma> {
    let as_comment = read_comment(node);
    match as_comment {
        Some(Comment::Block(syntax_token))
            if syntax_token.text().trim() == "/* @wgslfmt(ignore) */" =>
        {
            Some(IgnorePragma {
                block: true,
                token: syntax_token,
            })
        },
        Some(Comment::LineEnding(syntax_token))
            if syntax_token.text().trim() == "// @wgslfmt(ignore)" =>
        {
            Some(IgnorePragma {
                block: false,
                token: syntax_token,
            })
        },
        _ => None,
    }
}

/// Whether the given item is a comment with `@!wgslfmt(ignore)`.
#[must_use]
pub(crate) fn is_ignore_parent_pragma_comment(node: &NodeOrToken<SyntaxNode, SyntaxToken>) -> bool {
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

#[must_use]
pub(crate) fn gen_ignore_pragma(ignore_pragma: &IgnorePragma) -> PrintItemBuffer {
    let mut formatted = PrintItemBuffer::default();

    if ignore_pragma.block {
        formatted.push_sc(sc!("/* @wgslfmt(ignore) */"));
    } else {
        formatted.push_sc(sc!("// @wgslfmt(ignore)"));
    }
    // No newline - that is contained in the ignored code

    formatted
}
