//! A minimal parser toolbox used by the formatter
//! to parse the AST into a structure usable for the formatter itself.

//TODO Make ***_optional functions into parser-combinator like things

use itertools::PutBack;
use parser::{SyntaxElementChildren, SyntaxKind, SyntaxNode, SyntaxToken};
use rowan::NodeOrToken;
use syntax::{AstNode, AstToken};

use crate::generators::reporting::{
    FormatDocumentError, FormatDocumentResult, UnwrapIfPreferCrash as _,
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
            Err(FormatDocumentError::UnexpectedNodeOrToken { received: other })
        },
        None => Err(FormatDocumentError::MissingTokens {
            expected: Some(expected),
        }),
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
            Err(FormatDocumentError::UnexpectedNodeOrToken { received: other })
        },
        None => Err(FormatDocumentError::MissingTokens {
            expected: Some(expected),
        }),
    }
    .expect_if_prefer_crash()
}

pub fn parse_any_node_optional(syntax: &mut SyntaxIter) -> Option<SyntaxNode> {
    match syntax.next() {
        Some(NodeOrToken::Node(child)) => Some(child),
        Some(other) => {
            syntax.put_back(other);
            None
        },
        None => None,
    }
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
            Err(FormatDocumentError::UnexpectedNodeOrToken { received: other })
        },
        None => Err(FormatDocumentError::MissingTokens { expected: None }),
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
            Err(FormatDocumentError::UnexpectedNodeOrToken { received: other })
        },
    }
    .expect_if_prefer_crash()
}

#[expect(unused, reason = "TODO")]
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
                Err(FormatDocumentError::UnexpectedNodeOrToken {
                    received: NodeOrToken::Node(child),
                })
            }
        },
        Some(other) => {
            syntax.put_back(other.clone());
            Err(FormatDocumentError::UnexpectedNodeOrToken { received: other })
        },
        None => Err(FormatDocumentError::MissingNode),
    }
    .expect_if_prefer_crash()
}

pub fn parse_ast_token<T: AstToken>(syntax: &mut SyntaxIter) -> FormatDocumentResult<T> {
    match syntax.next() {
        Some(NodeOrToken::Token(child)) => {
            //TOCO This clone wouldn't be necessary if T::cast returned the item on failure
            if let Some(child) = T::cast(child.clone()) {
                Ok(child)
            } else {
                Err(FormatDocumentError::UnexpectedNodeOrToken {
                    received: NodeOrToken::Token(child),
                })
            }
        },
        Some(other) => {
            syntax.put_back(other.clone());
            Err(FormatDocumentError::UnexpectedNodeOrToken { received: other })
        },
        None => Err(FormatDocumentError::MissingNode),
    }
    .expect_if_prefer_crash()
}
