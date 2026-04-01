//! Low-level helpers for manipulating whitespace and tokens in the syntax tree.
//!
//! These utilities operate directly on Rowan's mutable syntax tree, inserting,
//! removing, and replacing tokens to achieve the desired formatting.

use rowan::{GreenNode, GreenToken, NodeOrToken};
use syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};

use crate::FormattingOptions;

/// Trims trailing spaces from the whitespace token preceding `before`,
/// preserving any newlines. E.g. `"\n  fn"` → `"\nfn"`.
pub(crate) fn trim_whitespace_before_to_newline(before: &SyntaxToken) -> Option<()> {
    let maybe_whitespace = before.prev_token()?; // spellchecker:disable-line
    if maybe_whitespace.kind().is_whitespace() {
        let index = maybe_whitespace.index();

        let text = maybe_whitespace.text().trim_end_matches(' ');

        maybe_whitespace.parent().unwrap().splice_children(
            index..index + 1,
            vec![SyntaxElement::Token(create_whitespace(text))],
        );
    }
    Some(())
}

/// Returns `true` if the token is whitespace containing at least one newline.
pub(crate) fn is_whitespace_with_newline(maybe_whitespace: &SyntaxToken) -> bool {
    maybe_whitespace.kind().is_whitespace() && maybe_whitespace.text().contains('\n')
}

/// Counts the number of newline characters in a whitespace token.
/// Returns `None` if the token is not whitespace.
pub(crate) fn n_newlines_in_whitespace(maybe_whitespace: &SyntaxToken) -> Option<usize> {
    maybe_whitespace
        .kind()
        .is_whitespace()
        .then(|| maybe_whitespace.text().matches('\n').count())
}

/// Clamps the number of consecutive newlines in a whitespace string to `max`.
/// Preserves any trailing spaces/indentation after the last newline.
pub(crate) fn clamp_newlines(
    text: &str,
    max: usize,
) -> String {
    let mut result = String::new();
    let mut consecutive = 0;
    for ch in text.chars() {
        if ch == '\n' {
            consecutive += 1;
            if consecutive <= max {
                result.push(ch);
            }
        } else {
            consecutive = 0;
            result.push(ch);
        }
    }
    result
}

/// Removes the token from the tree if it is whitespace; otherwise does nothing.
pub(crate) fn remove_if_whitespace(maybe_whitespace: &SyntaxToken) {
    if maybe_whitespace.kind().is_whitespace() {
        remove_token(maybe_whitespace);
    }
}

/// Removes a token from its parent node.
pub(crate) fn remove_token(token: &SyntaxToken) {
    let index = token.index();
    token
        .parent()
        .unwrap()
        .splice_children(index..index + 1, Vec::new());
}

/// Replaces a token in the tree with a different token.
pub(crate) fn replace_token_with(
    token: &SyntaxToken,
    replacement: SyntaxToken,
) {
    let index = token.index();
    token
        .parent()
        .unwrap()
        .splice_children(index..index + 1, vec![SyntaxElement::Token(replacement)]);
}

/// Inserts a token immediately after the given token.
pub(crate) fn insert_after(
    token: &SyntaxToken,
    insert: SyntaxToken,
) {
    let index = token.index();
    token
        .parent()
        .unwrap()
        .splice_children((index + 1)..index + 1, vec![SyntaxElement::Token(insert)]);
}

/// Inserts a token immediately after the given syntax node.
pub(crate) fn insert_after_syntax(
    node: &SyntaxNode,
    insert: SyntaxToken,
) {
    let index = node.index();
    node.parent()
        .unwrap()
        .splice_children((index + 1)..index + 1, vec![SyntaxElement::Token(insert)]);
}

/// Inserts a token immediately before the given token.
pub(crate) fn insert_before(
    token: &SyntaxToken,
    insert: SyntaxToken,
) {
    let index = token.index();
    token
        .parent()
        .unwrap()
        .splice_children(index..index, vec![SyntaxElement::Token(insert)]);
}

/// Ensures exactly one space before and after the given token.
pub(crate) fn whitespace_to_single_around(around: &SyntaxToken) {
    set_whitespace_single_before(around);
    set_whitespace_single_after(around);
}

/// Sets the whitespace after `after` to `to`, replacing existing whitespace
/// or inserting if none exists.
pub(crate) fn set_whitespace_after(
    after: &SyntaxToken,
    to: SyntaxToken,
) -> Option<()> {
    let maybe_whitespace = after.next_token()?;
    if maybe_whitespace.kind().is_whitespace() {
        replace_token_with(&maybe_whitespace, to);
    } else {
        insert_after(after, to);
    }

    Some(())
}

/// Sets the whitespace before `before` to `to`, replacing existing whitespace
/// or inserting if none exists.
pub(crate) fn set_whitespace_before(
    before: &SyntaxToken,
    to: SyntaxToken,
) -> Option<()> {
    let maybe_whitespace = before.prev_token()?; // spellchecker:disable-line
    if maybe_whitespace.kind().is_whitespace() {
        replace_token_with(&maybe_whitespace, to);
    } else {
        insert_before(before, to);
    }

    Some(())
}

/// Shorthand: set exactly one space after the given token.
pub(crate) fn set_whitespace_single_after(after: &SyntaxToken) -> Option<()> {
    set_whitespace_after(after, single_whitespace())
}

/// Shorthand: set exactly one space before the given token.
pub(crate) fn set_whitespace_single_before(before: &SyntaxToken) -> Option<()> {
    set_whitespace_before(before, single_whitespace())
}

/// Creates a single-space whitespace token.
pub(crate) fn single_whitespace() -> SyntaxToken {
    create_whitespace(" ")
}

/// Creates a whitespace token with the given text content.
pub(crate) fn create_whitespace(text: &str) -> SyntaxToken {
    create_syntax_token(SyntaxKind::Blankspace, text)
}

/// Creates a detached syntax token with the given kind and text.
///
/// Wraps the token in a throwaway root node so Rowan can produce a
/// mutable token suitable for insertion into an existing tree.
pub(crate) fn create_syntax_token(
    kind: SyntaxKind,
    text: &str,
) -> SyntaxToken {
    let node = SyntaxNode::new_root(GreenNode::new(
        SyntaxKind::Error.into(),
        std::iter::once(NodeOrToken::Token(GreenToken::new(kind.into(), text))),
    ))
    .clone_for_update();
    node.first_token().unwrap()
}

/// Returns `true` if any token between `start` (exclusive) and `end` (exclusive)
/// contains a newline character.
pub(crate) fn has_newline_between(
    start: &SyntaxToken,
    end: &SyntaxToken,
) -> bool {
    let mut tok = start.next_token();
    while let Some(token) = tok {
        if token == *end {
            break;
        }
        if token.text().contains('\n') {
            return true;
        }
        tok = token.next_token();
    }
    false
}

/// Sets the whitespace after `token` to a newline followed by indentation.
pub(crate) fn indent_after(
    token: &SyntaxToken,
    indent_level: usize,
    options: &FormattingOptions,
) -> Option<()> {
    let whitespace =
        create_whitespace(&format!("\n{}", options.indent_symbol.repeat(indent_level)));
    set_whitespace_after(token, whitespace)
}

/// Sets the whitespace before `token` to a newline followed by indentation.
pub(crate) fn indent_before(
    token: &SyntaxToken,
    indent_level: usize,
    options: &FormattingOptions,
) -> Option<()> {
    let whitespace =
        create_whitespace(&format!("\n{}", options.indent_symbol.repeat(indent_level)));
    set_whitespace_before(token, whitespace)
}

/// Removes whitespace before and after all `::` tokens that are direct children
/// of the given node.
pub(crate) fn remove_whitespace_around_double_colon(syntax: &SyntaxNode) {
    for child in syntax.children_with_tokens() {
        if let rowan::NodeOrToken::Token(token) = &child
            && token.kind() == SyntaxKind::ColonColon
        {
            let preceding = token.prev_token(); // spellchecker:disable-line
            if let Some(preceding) = preceding {
                remove_if_whitespace(&preceding);
            }
            if let Some(following) = token.next_token() {
                remove_if_whitespace(&following);
            }
        }
    }
}
