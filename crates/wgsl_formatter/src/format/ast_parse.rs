#![expect(
    clippy::wildcard_enum_match_arm,
    reason = "Most match statements require us to put back the unmatched variant"
)]
#![expect(
    clippy::unnecessary_wraps,
    reason = "It is a conscious API choice that all the parse_* fns return a result, in order to keep a unified api"
)]
//! A minimal parser toolbox used by the formatter
//! to parse the AST into a structure usable for the formatter itself

//TODO Make ***_optional functions into parser-combinator like things

use itertools::PutBack;
use parser::{SyntaxElementChildren, SyntaxKind, SyntaxNode, SyntaxToken};
use rowan::NodeOrToken;
use syntax::{
    AstNode,
    ast::{self, Attribute},
};

use crate::format::reporting::{
    FormatDocumentErrorKind, FormatDocumentResult, UnwrapIfPreferCrash,
};

pub type SyntaxIter = PutBack<SyntaxElementChildren>;
pub fn parse_token(
    syntax: &mut SyntaxIter,
    expected: SyntaxKind,
) -> FormatDocumentResult<SyntaxToken> {
    match syntax.next() {
        Some(NodeOrToken::Token(child)) if child.kind() == expected => Ok(child),
        Some(other) => {
            syntax.put_back(other.clone());
            Err(FormatDocumentErrorKind::UnexpectedToken { received: other }.without_range())
        },
        None => Err(FormatDocumentErrorKind::MissingTokens {
            expected: Some(expected),
        }
        .without_range()),
    }
    .expect_if_prefer_crash()
}

pub fn parse_node_by_kind(
    syntax: &mut SyntaxIter,
    expected: SyntaxKind,
) -> FormatDocumentResult<SyntaxNode> {
    match syntax.next() {
        Some(NodeOrToken::Node(child)) if child.kind() == expected => Ok(child),
        Some(other) => {
            syntax.put_back(other.clone());
            Err(FormatDocumentErrorKind::UnexpectedToken { received: other }.without_range())
        },
        None => Err(FormatDocumentErrorKind::MissingTokens {
            expected: Some(expected),
        }
        .without_range()),
    }
    .expect_if_prefer_crash()
}

pub fn parse_node_by_kind_optional(
    syntax: &mut SyntaxIter,
    expected: SyntaxKind,
) -> Option<SyntaxNode> {
    match syntax.next() {
        Some(NodeOrToken::Node(child)) if child.kind() == expected => Some(child),
        Some(other) => {
            syntax.put_back(other);
            None
        },
        None => None,
    }
}

pub fn parse_token_any(syntax: &mut SyntaxIter) -> FormatDocumentResult<SyntaxToken> {
    match syntax.next() {
        Some(NodeOrToken::Token(child)) => Ok(child),
        Some(other) => {
            syntax.put_back(other.clone());
            Err(FormatDocumentErrorKind::UnexpectedToken { received: other }.without_range())
        },
        None => Err(FormatDocumentErrorKind::MissingTokens { expected: None }.without_range()),
    }
    .expect_if_prefer_crash()
}

pub fn parse_token_optional(
    syntax: &mut SyntaxIter,
    expected: SyntaxKind,
) -> Option<SyntaxToken> {
    match syntax.next() {
        Some(NodeOrToken::Token(child)) if child.kind() == expected => Some(child),
        Some(other) => {
            syntax.put_back(other);
            None
        },
        None => None,
    }
}

pub fn parse_end(syntax: &mut SyntaxIter) -> FormatDocumentResult<()> {
    match syntax.next() {
        None => Ok(()),
        Some(other) => {
            syntax.put_back(other.clone());
            Err(FormatDocumentErrorKind::UnexpectedToken { received: other }.without_range())
        },
    }
    .expect_if_prefer_crash()
}

pub fn parse_end_optional(syntax: &mut SyntaxIter) -> Option<()> {
    match syntax.next() {
        None => Some(()),
        Some(remaining) => {
            syntax.put_back(remaining);
            None
        },
    }
}

pub fn parse_node_optional<T: AstNode>(syntax: &mut SyntaxIter) -> Option<T> {
    match syntax.next() {
        Some(NodeOrToken::Node(child)) => {
            //TODO This clone wouldn't be needed if T::cast returned the item on a failure
            if let Some(child) = T::cast(child.clone()) {
                Some(child)
            } else {
                syntax.put_back(NodeOrToken::Node(child));
                None
            }
        },
        Some(other) => {
            syntax.put_back(other);
            None
        },
        None => None,
    }
}
pub fn parse_node<T: AstNode>(syntax: &mut SyntaxIter) -> FormatDocumentResult<T> {
    match syntax.next() {
        Some(NodeOrToken::Node(child)) => {
            //TOCO This clone wouldn't be necessary if T::cast returned the item on failure
            if let Some(child) = T::cast(child.clone()) {
                Ok(child)
            } else {
                Err(FormatDocumentErrorKind::UnexpectedToken {
                    received: NodeOrToken::Node(child),
                }
                .without_range())
            }
        },
        Some(other) => {
            syntax.put_back(other.clone());
            Err(FormatDocumentErrorKind::UnexpectedToken { received: other }.without_range())
        },
        None => {
            todo!();
            Err(FormatDocumentErrorKind::MissingNode.without_range())
        },
    }
    .expect_if_prefer_crash()
}

pub fn parse_many_comments_and_blankspace(
    syntax: &mut SyntaxIter
) -> FormatDocumentResult<Vec<SyntaxToken>> {
    let mut comments = Vec::new();

    while let Some(token) = syntax.next() {
        match token {
            NodeOrToken::Token(child) if child.kind() == SyntaxKind::Blankspace => {
                //Allowed, we ignore it
            },
            NodeOrToken::Token(child) if child.kind() == SyntaxKind::BlockComment => {
                comments.push(child);
            },
            NodeOrToken::Token(child) if child.kind() == SyntaxKind::LineEndingComment => {
                comments.push(child);
            },
            other => {
                syntax.put_back(other);
                return Ok(comments);
            },
        }
    }
    Ok(comments)
}
