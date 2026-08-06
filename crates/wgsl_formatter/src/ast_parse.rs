//! A minimal parser toolbox used by the formatter
//! to parse the AST into a structure usable for the formatter itself.

use itertools::PutBack;
use parser::{SyntaxElementChildren, SyntaxKind, SyntaxNode, SyntaxToken};
use rowan::NodeOrToken;
use syntax::{AstNode, AstToken, ast::AttributeList};

use crate::{
    generators::comments::{parse_comment_optional, read_comment},
    helpers::{NextGenLineSpacing, parse_next_gen_line_spacing, read_blankspace},
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

pub trait UntilFilter {
    fn filter(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<FilterAction>;
}

pub struct UntilEmptyLine;

impl UntilFilter for UntilEmptyLine {
    fn filter(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<FilterAction> {
        match read_blankspace(node) {
            Some(NextGenLineSpacing::EmptyLine(_)) => Some(FilterAction::Stop),
            _ => None,
        }
    }
}

pub struct UntilSyntaxKind(pub SyntaxKind);
impl UntilFilter for UntilSyntaxKind {
    fn filter(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<FilterAction> {
        (node.kind() == self.0).then_some(FilterAction::Stop)
    }
}

pub struct BareSyntaxKind(pub SyntaxKind);
impl UntilFilter for BareSyntaxKind {
    fn filter(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<FilterAction> {
        (node.kind() != self.0).then_some(FilterAction::Stop)
    }
}

pub fn parse_node_with_trivia_until<F>(
    syntax: &mut SyntaxIter,
    until: F,
) -> NodeWithTrivia
where
    F: UntilFilter,
{
    parse_node_with_trivia_filter(syntax, |node| until.filter(node))
}

pub fn parse_node_with_trivia(syntax: &mut SyntaxIter) -> NodeWithTrivia {
    // TODO Remove Empty Line Handling from here
    parse_node_with_trivia_until(syntax, UntilEmptyLine)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FilterAction {
    Ignored,
    // TODO I think we can do without Content, as that is just a worse None in the filter
    Content,
    Stop,
}

pub fn parse_node_with_trivia_filter<F>(
    syntax: &mut SyntaxIter,
    filter: F,
) -> NodeWithTrivia
where
    F: Fn(&NodeOrToken<SyntaxNode, SyntaxToken>) -> Option<FilterAction>,
{
    let mut preceding_trivia = Vec::new();
    let mut succeeding_trivia = Vec::new();

    // TODO Remove this
    loop {
        // We allow line spacing at the very top of trivia
        if let Some(spacing) = parse_next_gen_line_spacing(syntax) {
            preceding_trivia.push(NodeTriviaItem::LineSpacing(spacing));
        } else {
            break;
        }
    }

    let content = loop {
        // I wish we had linear types...
        // NOTE: Make sure node is either put_back onto syntax or consumed in a meaningful way
        if let Some(node) = syntax.next() {
            let action = filter(&node);
            match action {
                Some(FilterAction::Ignored) => {},
                Some(FilterAction::Content) => {
                    break NodeWithTriviaContent::Content(node);
                },
                Some(FilterAction::Stop) => {
                    syntax.put_back(node);
                    break NodeWithTriviaContent::NoContent;
                },
                None => {
                    if let Some(line_spacing) = read_blankspace(&node) {
                        preceding_trivia.push(NodeTriviaItem::LineSpacing(line_spacing));
                    } else if let Some(comment) = read_comment(&node) {
                        preceding_trivia.push(NodeTriviaItem::Comment(comment));
                    } else if let NodeOrToken::Node(node) = &node
                        && let Some(attributes) = AttributeList::cast(node.clone())
                    {
                        preceding_trivia.push(NodeTriviaItem::AttributeList(attributes));
                    } else {
                        break NodeWithTriviaContent::Content(node);
                    }
                },
            }
        } else {
            break NodeWithTriviaContent::End;
        }
    };

    while let Some(node) = syntax.next() {
        let action = filter(&node);
        match action {
            Some(FilterAction::Ignored) => {},
            Some(FilterAction::Content) => {
                // This belongs into the "content" of the next call to parse_node_...
                syntax.put_back(node);
                break;
            },
            Some(FilterAction::Stop) => {
                // We want to stop parsing succeeding trivia
                syntax.put_back(node);
                break;
            },
            None => {
                if let Some(line_spacing) = read_blankspace(&node) {
                    succeeding_trivia.push(NodeTriviaItem::LineSpacing(line_spacing));
                } else if let Some(comment) = read_comment(&node) {
                    succeeding_trivia.push(NodeTriviaItem::Comment(comment));
                } else if node.kind() == SyntaxKind::AttributeList {
                    // Attributes are always "before" the item they are attached to
                    syntax.put_back(node);
                    break;
                } else {
                    // This belongs into the "content" of the next call to parse_node_...
                    syntax.put_back(node);
                    break;
                }
            },
        }
    }

    NodeWithTrivia {
        preceding_trivia,
        node: content,
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
