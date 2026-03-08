use rowan::{GreenNode, GreenToken, NodeOrToken};
use syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};

use crate::FormattingOptions;

// "\n  fn" -> "\nfn"
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

pub(crate) fn is_whitespace_with_newline(maybe_whitespace: &SyntaxToken) -> bool {
    maybe_whitespace.kind().is_whitespace() && maybe_whitespace.text().contains('\n')
}

pub(crate) fn n_newlines_in_whitespace(maybe_whitespace: &SyntaxToken) -> Option<usize> {
    maybe_whitespace
        .kind()
        .is_whitespace()
        .then(|| maybe_whitespace.text().matches('\n').count())
}

pub(crate) fn remove_if_whitespace(maybe_whitespace: &SyntaxToken) {
    if maybe_whitespace.kind().is_whitespace() {
        remove_token(maybe_whitespace);
    }
}

pub(crate) fn remove_token(token: &SyntaxToken) {
    let index = token.index();
    token
        .parent()
        .unwrap()
        .splice_children(index..index + 1, Vec::new());
}

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

pub(crate) fn insert_after_syntax(
    node: &SyntaxNode,
    insert: SyntaxToken,
) {
    let index = node.index();
    node.parent()
        .unwrap()
        .splice_children((index + 1)..index + 1, vec![SyntaxElement::Token(insert)]);
}

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

pub(crate) fn whitespace_to_single_around(around: &SyntaxToken) {
    set_whitespace_single_before(around);
    set_whitespace_single_after(around);
}

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

pub(crate) fn set_whitespace_single_after(after: &SyntaxToken) -> Option<()> {
    set_whitespace_after(after, single_whitespace())
}

pub(crate) fn set_whitespace_single_before(before: &SyntaxToken) -> Option<()> {
    set_whitespace_before(before, single_whitespace())
}

pub(crate) fn single_whitespace() -> SyntaxToken {
    create_whitespace(" ")
}

pub(crate) fn create_whitespace(text: &str) -> SyntaxToken {
    create_syntax_token(SyntaxKind::Blankspace, text)
}

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

pub(crate) fn indent_after(
    token: &SyntaxToken,
    indent_level: usize,
    options: &FormattingOptions,
) -> Option<()> {
    let whitespace =
        create_whitespace(&format!("\n{}", options.indent_symbol.repeat(indent_level)));
    set_whitespace_after(token, whitespace)
}

pub(crate) fn indent_before(
    token: &SyntaxToken,
    indent_level: usize,
    options: &FormattingOptions,
) -> Option<()> {
    let whitespace =
        create_whitespace(&format!("\n{}", options.indent_symbol.repeat(indent_level)));
    set_whitespace_before(token, whitespace)
}
