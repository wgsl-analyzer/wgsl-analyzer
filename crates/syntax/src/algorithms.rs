use itertools::Itertools as _;
use parser::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};
use rowan::{Direction, NodeOrToken, TextRange, TextSize};

use crate::AstNode;

/// Returns ancestors of the node at the offset, sorted by length.
///
/// This should do the right thing at an edge, for example, when searching
/// for expressions at `{ $0foo }` we will get the name reference instead
/// of the whole block, which we would get if we just did
/// `find_token_at_offset(...).flat_map(|t| t.parent().ancestors())`.
pub fn ancestors_at_offset(
    node: &SyntaxNode,
    offset: TextSize,
) -> impl Iterator<Item = SyntaxNode> {
    node.token_at_offset(offset)
        .map(|token| token.parent_ancestors())
        .kmerge_by(|node1, node2| node1.text_range().len() < node2.text_range().len())
}

/// Finds a node of specific Ast type at offset. Note that this is slightly
/// imprecise: if the cursor is strictly between two nodes of the desired type,
/// as in:
///
/// ```no_run,ignore
/// struct Foo {}|struct Bar;
/// ```
///
/// then the shorter node will be silently preferred.
pub fn find_node_at_offset<N: AstNode>(
    syntax: &SyntaxNode,
    offset: TextSize,
) -> Option<N> {
    ancestors_at_offset(syntax, offset).find_map(N::cast)
}

pub fn find_node_at_range<N: AstNode>(
    syntax: &SyntaxNode,
    range: TextRange,
) -> Option<N> {
    syntax.covering_element(range).ancestors().find_map(N::cast)
}

/// Skip to next non `trivia` token.
#[must_use]
pub fn skip_trivia_token(
    mut token: SyntaxToken,
    direction: Direction,
) -> Option<SyntaxToken> {
    while token.kind().is_trivia() {
        token = match direction {
            Direction::Next => token.next_token()?,
            Direction::Prev => token.prev_token()?, // spellchecker:disable-line
        }
    }
    Some(token)
}

/// Skip to next non `whitespace` token.
#[must_use]
pub fn skip_whitespace_token(
    mut token: SyntaxToken,
    direction: Direction,
) -> Option<SyntaxToken> {
    while token.kind() == SyntaxKind::Blankspace {
        token = match direction {
            Direction::Next => token.next_token()?,
            Direction::Prev => token.prev_token()?, // spellchecker:disable-line
        }
    }
    Some(token)
}

/// Finds the first sibling in the given direction which is not `trivia`.
pub fn non_trivia_sibling(
    element: SyntaxElement,
    direction: Direction,
) -> Option<SyntaxElement> {
    return match element {
        NodeOrToken::Node(node) => node
            .siblings_with_tokens(direction)
            .skip(1)
            .find(not_trivia),
        NodeOrToken::Token(token) => token
            .siblings_with_tokens(direction)
            .skip(1)
            .find(not_trivia),
    };

    fn not_trivia(element: &SyntaxElement) -> bool {
        match element {
            NodeOrToken::Node(_) => true,
            NodeOrToken::Token(token) => !token.kind().is_trivia(),
        }
    }
}

#[must_use]
pub fn least_common_ancestor(
    first: &SyntaxNode,
    second: &SyntaxNode,
) -> Option<SyntaxNode> {
    if first == second {
        return Some(first.clone());
    }

    let first_depth = first.ancestors().count();
    let second_depth = second.ancestors().count();
    let keep = first_depth.min(second_depth);

    let first_candidates = first.ancestors().skip(first_depth - keep);
    let second_candidates = second.ancestors().skip(second_depth - keep);
    let (result, _) = first_candidates
        .zip(second_candidates)
        .find(|(first, second)| first == second)?;
    Some(result)
}

pub fn neighbor<T: AstNode>(
    me: &T,
    direction: Direction,
) -> Option<T> {
    me.syntax().siblings(direction).skip(1).find_map(T::cast)
}

#[must_use]
pub fn has_errors(node: &SyntaxNode) -> bool {
    node.children().any(|node| node.kind() == SyntaxKind::Error)
}
