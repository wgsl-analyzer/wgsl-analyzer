//! A minimal parser toolbox used by the formatter
//! to parse the AST into a structure usable for the formatter itself.

use itertools::{PutBackN, put_back_n};
use parser::{SyntaxElementChildren, SyntaxKind, SyntaxNode, SyntaxToken};
use rowan::NodeOrToken;
use syntax::{AstNode, AstToken, ast::AttributeList};

use crate::{
    generators::comments::read_comment,
    helpers::{NextGenLineSpacing, read_blankspace},
    reporting::{FormatDocumentError, FormatDocumentResult, UnwrapIfPreferCrash as _},
    trivia::{NodeTriviaItem, NodeWithTrivia, NodeWithTriviaContent},
};

pub type SyntaxIter = PutBackN<SyntaxElementChildren>;
pub fn syntax_iter(syntax: &SyntaxNode) -> SyntaxIter {
    put_back_n(syntax.children_with_tokens())
}

#[deprecated]
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

#[deprecated]
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

#[deprecated]
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

#[deprecated]
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

#[deprecated]
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

#[deprecated]
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

#[deprecated]
pub fn parse_end_optional(syntax: &mut SyntaxIter) -> Option<()> {
    match syntax.next() {
        None => Some(()),
        Some(remaining) => {
            syntax.put_back(remaining);
            None
        },
    }
}

#[deprecated]
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

// TODO I think the default should be to ignore blankspace and *including* it should be explicit (in struct body and compound statements)
pub struct IgnoreBlankspace;
impl UntilFilter for IgnoreBlankspace {
    fn filter(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<FilterAction> {
        (node.kind() == SyntaxKind::Blankspace).then_some(FilterAction::Ignored)
    }
}

pub struct NoTrivia;
impl UntilFilter for NoTrivia {
    fn filter(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<FilterAction> {
        Some(FilterAction::Content)
    }
}

pub struct Oneline;
impl UntilFilter for Oneline {
    fn filter(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<FilterAction> {
        match read_blankspace(node) {
            Some(NextGenLineSpacing::EmptyLine(_)) | Some(NextGenLineSpacing::LineBreak(_)) => {
                Some(FilterAction::Stop)
            },
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct Filter<T>(pub T)
where
    T: Fn(&NodeOrToken<SyntaxNode, SyntaxToken>) -> Option<FilterAction>;
impl<T> UntilFilter for Filter<T>
where
    T: Fn(&NodeOrToken<SyntaxNode, SyntaxToken>) -> Option<FilterAction>,
{
    fn filter(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<FilterAction> {
        self.0(node)
    }
}

/// Parses a node with surrounding trivia, based on the given strategy.
pub fn parse_node_with<F>(
    syntax: &mut SyntaxIter,
    until: F,
) -> NodeWithTrivia
where
    F: UntilFilter,
{
    parse_node_with_trivia_filter(syntax, |node| until.filter(node))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FilterAction {
    Ignored,
    // TODO I think we can do without Content, as that is just a worse None in the filter
    Content,
    Stop,
    IgnoreAndStop,
}

#[deprecated]
pub fn parse_node_with_trivia_filter<F>(
    syntax: &mut SyntaxIter,
    filter: F,
) -> NodeWithTrivia
where
    F: Fn(&NodeOrToken<SyntaxNode, SyntaxToken>) -> Option<FilterAction>,
{
    parse_node_with_trivia_filter_2(syntax, &filter, &filter)
}

// TODO Rename this... .... WIP api naming is hard
pub fn parse_node_with_trivia_filter_2<FPre, FPost>(
    syntax: &mut SyntaxIter,
    filter_pre: FPre,
    filter_post: FPost,
) -> NodeWithTrivia
where
    FPre: Fn(&NodeOrToken<SyntaxNode, SyntaxToken>) -> Option<FilterAction>,
    FPost: Fn(&NodeOrToken<SyntaxNode, SyntaxToken>) -> Option<FilterAction>,
{
    let mut preceding_trivia = Vec::new();
    let mut succeeding_trivia = Vec::new();

    // TODO Remove this

    let content = loop {
        // I wish we had linear types...
        // NOTE: Make sure node is either put_back onto syntax or consumed in a meaningful way
        if let Some(node) = syntax.next() {
            let action = filter_pre(&node);
            println!("Pre {node:?} {action:?}");
            match action {
                Some(FilterAction::Ignored) => {},
                Some(FilterAction::Content) => {
                    break NodeWithTriviaContent::Content(node);
                },
                Some(FilterAction::Stop) => {
                    syntax.put_back(node);
                    break NodeWithTriviaContent::NoContent;
                },
                Some(FilterAction::IgnoreAndStop) => {
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
        let action = filter_post(&node);
        println!("Post {node:?} {action:?}");
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
            Some(FilterAction::IgnoreAndStop) => {
                // We want to stop parsing succeeding trivia
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

#[deprecated]
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
