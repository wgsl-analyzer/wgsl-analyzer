//! Utils to deal with trivia (comments, attributes and blank space).
use parser::{SyntaxKind, SyntaxNode, SyntaxToken};
use rowan::NodeOrToken;
use syntax::{AstNode, AstToken, ast::AttributeList};

use crate::{
    ast_parse::SyntaxIter,
    generators::comments::Comment,
    helpers::LineSpacing,
    reporting::{FormatDocumentError, FormatDocumentResult, UnwrapIfPreferCrash as _},
};

/// A piece of "trivia" that is associated to a content inside a [`NodeWithTrivia`].
#[derive(Clone, Debug)]
pub enum NodeTriviaItem {
    /// A piece of the AST that should not be included in the formatted output.
    Discarded(NodeOrToken<SyntaxNode, SyntaxToken>),
    /// Some amount of blank space.
    LineSpacing(LineSpacing),
    /// A comment that was not followed by a newline in the source.
    Comment(Comment),
    /// A comment that was followed by a newline in the source.
    ///
    /// This can be both a block-comment and a line ending comment.
    NewlinedComment(Comment),
    /// A list of attributes.
    AttributeList(AttributeList),
}

impl NodeTriviaItem {
    /// Add this item back onto the provided [`SyntaxIter`].
    pub fn put_back(
        self,
        syntax: &mut SyntaxIter,
    ) {
        match self {
            Self::LineSpacing(next_gen_line_spacing) => match next_gen_line_spacing {
                LineSpacing::LineBreak(syntax_token)
                | LineSpacing::EmptyLine(syntax_token)
                | LineSpacing::OnelineBlankspace(syntax_token) => {
                    syntax.put_back(NodeOrToken::Token(syntax_token));
                },
            },
            Self::Comment(comment) => match comment {
                Comment::Block(node) | Comment::LineEnding(node) => {
                    syntax.put_back(NodeOrToken::Token(node));
                },
            },
            Self::NewlinedComment(comment) => match comment {
                Comment::Block(node) | Comment::LineEnding(node) => {
                    syntax.put_back(NodeOrToken::Token(node));
                },
            },
            Self::AttributeList(attribute_list) => {
                syntax.put_back(NodeOrToken::Node(attribute_list.syntax().clone()));
            },
            Self::Discarded(content) => {
                syntax.put_back(content);
            },
        }
    }
}

/// The "content" that is associated with trivia inside a [`NodeWithTrivia`].
#[derive(Clone, Debug)]
pub enum NodeWithTriviaContent {
    /// There is no content, just trivia.
    ///
    /// This is useful e.g for freestanding comments that aren't attached to anything.
    NoContent,

    /// The content is some piece of the AST.
    Content(NodeOrToken<SyntaxNode, SyntaxToken>),

    /// The content is the "end".
    ///
    /// This can either mark the end of the code that needs formatting, or
    /// it can be the end of some sub-piece of code (like the end of a function parameter).
    ///
    /// This is very useful for [`crate::ast_parse::parse_many_nodes_with`].
    End,
}

impl NodeWithTriviaContent {
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::NoContent | Self::End)
    }

    #[must_use]
    pub const fn as_ref(&self) -> Option<&NodeOrToken<SyntaxNode, SyntaxToken>> {
        match self {
            Self::Content(node_or_token) => Some(node_or_token),
            Self::NoContent | Self::End => None,
        }
    }

    #[must_use]
    pub fn into_option(self) -> Option<NodeOrToken<SyntaxNode, SyntaxToken>> {
        match self {
            Self::Content(node_or_token) => Some(node_or_token),
            Self::NoContent | Self::End => None,
        }
    }
}

/// Some piece of "content" that is associated with preceding and succeeding trivia.
///
/// The "content" can be a `SyntaxNode`, empty, or the end of input. See [`NodeWithTriviaContent`] for details.
///
/// See [`crate::ast_parse::parse_node_with`] for details on how trivia gets associated with content.
#[derive(Clone, Debug)]
pub struct NodeWithTrivia {
    /// Any trivia associated with the content, that preceded it in the source.
    pub preceding_trivia: Vec<NodeTriviaItem>,
    //TODO rename to content
    /// The content that the trivia is associated with.
    pub node: NodeWithTriviaContent,
    /// Any trivia associated with the content, that succeeded it in the source.
    pub succeeding_trivia: Vec<NodeTriviaItem>,
    /// Whether this [`NodeWithTrivia`] wants to be formatted.
    ///
    /// This tells us if the content was preceded by, or contains a ignore-pragma that
    /// would exempt it from being formatted. See [`crate::ignore`] for details.
    pub format: bool,
}

impl NodeWithTrivia {
    /// Get the `SyntaxKind` of self, or [`None`] if self did not contain a content node.
    pub fn kind(&self) -> Option<SyntaxKind> {
        self.node
            .as_ref()
            .map(NodeOrToken::<SyntaxNode, SyntaxToken>::kind)
    }

    /// Adds any syntax-nodes within self back onto the [`SyntaxIter`].
    pub fn put_back(
        self,
        syntax: &mut SyntaxIter,
    ) {
        for item in self.succeeding_trivia.into_iter().rev() {
            item.put_back(syntax);
        }
        match self.node {
            NodeWithTriviaContent::Content(node_or_token) => {
                syntax.put_back(node_or_token);
            },
            NodeWithTriviaContent::NoContent | NodeWithTriviaContent::End => {},
        }
        for item in self.preceding_trivia.into_iter().rev() {
            item.put_back(syntax);
        }
    }

    /// Returns `None` and [puts self back](Self::put_back) onto the
    /// [`SyntaxIter`] if [`Self::kind`] did not match the provided `SyntaxKind`.
    pub fn only_if_kind(
        self,
        kind: SyntaxKind,
        syntax: &mut SyntaxIter,
    ) -> Option<Self> {
        if self.kind().is_some_and(|node| node == kind) {
            Some(self)
        } else {
            self.put_back(syntax);
            None
        }
    }

    /// Returns `None` and [puts self back](Self::put_back) onto the
    /// [`SyntaxIter`] if the content node of self cannot be cast into the
    /// specified `AstNode`.
    pub fn only_if_ast_node<T>(
        self,
        syntax: &mut SyntaxIter,
    ) -> Option<Self>
    where
        T: AstNode,
    {
        if self
            .node
            .as_ref()
            .is_some_and(|node| !T::can_cast(node.kind()))
        {
            self.put_back(syntax);
            None
        } else {
            Some(self)
        }
    }

    /// Like [`Self::expect_kind`] but does not error if self does not have content.
    #[track_caller]
    pub fn expect_kind_optional(
        self,
        kind: SyntaxKind,
    ) -> FormatDocumentResult<Self> {
        if self.node.as_ref().is_some_and(|node| node.kind() != kind) {
            Err(FormatDocumentError::UnexpectedNodeOrToken {
                received: self.node.into_option(),
            })
            .expect_if_prefer_crash()
        } else {
            Ok(self)
        }
    }

    /// Returns a [`FormatDocumentError`] if self did not have a content node or that node
    /// did not match the given `SyntaxKind`.
    #[track_caller]
    pub fn expect_kind(
        self,
        kind: SyntaxKind,
    ) -> FormatDocumentResult<Self> {
        if self.node.as_ref().is_some_and(|node| node.kind() == kind) {
            Ok(self)
        } else {
            Err(FormatDocumentError::UnexpectedNodeOrToken {
                received: self.node.into_option(),
            })
            .expect_if_prefer_crash()
        }
    }

    /// Returns a [`FormatDocumentError`] if self did not have a content node or that node
    /// could not be cast into the given `AstNode`.
    #[track_caller]
    pub fn expect_ast_node<T>(self) -> FormatDocumentResult<Self>
    where
        T: AstNode,
    {
        if let NodeWithTriviaContent::Content(NodeOrToken::Node(node)) = &self.node
            && T::cast(node.clone()).is_some()
        {
            return Ok(self);
        }
        Err(FormatDocumentError::UnexpectedNodeOrToken {
            received: self.node.into_option(),
        })
        .expect_if_prefer_crash()
    }

    /// Like [`Self::expect_ast_node`] but does not error if self does not have content.
    #[track_caller]
    pub fn expect_ast_node_optional<T>(self) -> FormatDocumentResult<Self>
    where
        T: AstNode,
    {
        match &self.node {
            NodeWithTriviaContent::NoContent | NodeWithTriviaContent::End => Ok(self),
            NodeWithTriviaContent::Content(node_or_token) => {
                if let NodeOrToken::Node(node) = node_or_token
                    && T::can_cast(node.kind())
                {
                    Ok(self)
                } else {
                    Err(FormatDocumentError::UnexpectedNodeOrToken {
                        received: self.node.into_option(),
                    })
                    .expect_if_prefer_crash()
                }
            },
        }
    }

    /// Returns a [`FormatDocumentError`] if self did not have a content node or that node
    /// could not be cast into the given `AstToken`.
    #[track_caller]
    pub fn expect_ast_token<T>(self) -> FormatDocumentResult<Self>
    where
        T: AstToken,
    {
        if let NodeWithTriviaContent::Content(NodeOrToken::Token(node)) = &self.node
            && T::cast(node.clone()).is_some()
        {
            return Ok(self);
        }
        Err(FormatDocumentError::UnexpectedNodeOrToken {
            received: self.node.into_option(),
        })
        .expect_if_prefer_crash()
    }

    /// Is the content of this node [`NodeWithTriviaContent::End`].
    #[must_use]
    pub const fn is_end(&self) -> bool {
        matches!(self.node, NodeWithTriviaContent::End)
    }

    /// Is the content of this node and any associated trivia purely made up of whitespace?
    #[must_use]
    pub fn is_whitespace(&self) -> bool {
        self.node.is_empty()
            && self.preceding_trivia.iter().all(|trivia| {
                matches!(
                    trivia,
                    NodeTriviaItem::LineSpacing(_) | NodeTriviaItem::Discarded(_)
                )
            })
            && self.succeeding_trivia.iter().all(|trivia| {
                matches!(
                    trivia,
                    NodeTriviaItem::LineSpacing(_) | NodeTriviaItem::Discarded(_)
                )
            })
    }

    /// Does this node have a nonempty content?
    #[must_use]
    pub const fn has_content(&self) -> bool {
        matches!(self.node, NodeWithTriviaContent::Content(_))
    }

    #[must_use]
    pub fn content(&self) -> Option<NodeOrToken<SyntaxNode, SyntaxToken>> {
        match &self.node {
            NodeWithTriviaContent::Content(node_or_token) => Some(node_or_token.clone()),
            NodeWithTriviaContent::NoContent | NodeWithTriviaContent::End => None,
        }
    }

    /// Trims off any linebreaks that would be at the beginning of the preceding trivia.
    #[must_use]
    pub fn trim_starting_linebreaks(mut self) -> Self {
        for item in &mut self.preceding_trivia {
            match item {
                NodeTriviaItem::LineSpacing(LineSpacing::LineBreak(content)) => {
                    *item = NodeTriviaItem::Discarded(NodeOrToken::Token(content.clone()));
                },
                NodeTriviaItem::Discarded(_) => {},
                NodeTriviaItem::LineSpacing(_)
                | NodeTriviaItem::Comment(_)
                | NodeTriviaItem::NewlinedComment(_)
                | NodeTriviaItem::AttributeList(_) => {
                    break;
                },
            }
        }
        self
    }
}
