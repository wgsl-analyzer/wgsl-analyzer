//! A minimal parser toolbox used by the formatter
//! to parse the AST into a structure usable for the formatter itself.

use itertools::PutBack;
use parser::{SyntaxElementChildren, SyntaxKind, SyntaxNode, SyntaxToken};
use rowan::NodeOrToken;
use syntax::{AstNode, AstToken, ast::AttributeList};

use crate::{
    generators::comments::parse_comment_optional,
    helpers::{NextGenLineSpacing, parse_next_gen_line_spacing},
    reporting::{FormatDocumentError, FormatDocumentResult, UnwrapIfPreferCrash as _},
    trivia::{NodeTriviaItem, NodeWithTrivia, NodeWithTriviaContent},
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

#[deprecated]
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

pub fn parse_end_optional(syntax: &mut SyntaxIter) -> Option<()> {
    match syntax.next() {
        None => Some(()),
        Some(remaining) => {
            syntax.put_back(remaining);
            None
        },
    }
}

pub fn parse_node_optional<T>(syntax: &mut SyntaxIter) -> Option<T>
where
    T: AstNode,
{
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

pub fn parse_node_with_trivia(syntax: &mut SyntaxIter) -> NodeWithTrivia {
    let mut preceding_trivia = Vec::new();
    let mut succeeding_trivia = Vec::new();

    loop {
        // We allow line spacing at the very top of trivia
        if let Some(spacing) = parse_next_gen_line_spacing(syntax) {
            preceding_trivia.push(NodeTriviaItem::LineSpacing(spacing));
        } else {
            break;
        }
    }

    loop {
        if let Some(line_spacing) = parse_next_gen_line_spacing(syntax) {
            match line_spacing {
                NextGenLineSpacing::EmptyLine(blankspace) => {
                    syntax.put_back(NodeOrToken::Token(blankspace));
                    return NodeWithTrivia {
                        preceding_trivia,
                        node: NodeWithTriviaContent::NoContent,
                        succeeding_trivia,
                    };
                },
                NextGenLineSpacing::LineBreak(_) => {
                    preceding_trivia.push(NodeTriviaItem::LineSpacing(line_spacing));
                },
                NextGenLineSpacing::OnelineBlankspace(_) => {
                    // Line breaks and oneline blankspace never carry any meaning
                },
            }
        } else if let Some(comment) = parse_comment_optional(syntax) {
            preceding_trivia.push(NodeTriviaItem::Comment(comment));
        } else if let Some(attributes) = parse_node_optional::<AttributeList>(syntax) {
            preceding_trivia.push(NodeTriviaItem::AttributeList(attributes));
        } else {
            break;
        }
    }
    let node = match syntax.next() {
        Some(next) => NodeWithTriviaContent::Content(next),
        None => NodeWithTriviaContent::End,
    };

    loop {
        if let Some(trivia) = parse_comment_optional(syntax) {
            succeeding_trivia.push(NodeTriviaItem::Comment(trivia));
        } else if let Some(line_spacing) = parse_next_gen_line_spacing(syntax) {
            match line_spacing {
                NextGenLineSpacing::OnelineBlankspace(syntax_token) => {
                    // Oneline blankspace does not differentiate between where trivia belongs to
                },
                NextGenLineSpacing::LineBreak(blankspace)
                | NextGenLineSpacing::EmptyLine(blankspace) => {
                    // Any meaningful line spacing ends succeeding trivia
                    syntax.put_back(NodeOrToken::Token(blankspace));
                    break;
                },
            }
        } else {
            break;
        }
    }

    NodeWithTrivia {
        preceding_trivia,
        node,
        succeeding_trivia,
    }
}

#[deprecated]
pub fn parse_node<T>(syntax: &mut SyntaxIter) -> FormatDocumentResult<T>
where
    T: AstNode,
{
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

pub fn parse_ast_token<T>(syntax: &mut SyntaxIter) -> FormatDocumentResult<T>
where
    T: AstToken,
{
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
