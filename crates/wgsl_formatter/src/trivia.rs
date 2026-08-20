use parser::{SyntaxKind, SyntaxNode, SyntaxToken};
use rowan::NodeOrToken;
use syntax::{AstNode, AstToken, ast::AttributeList};

use crate::{
    ast_parse::SyntaxIter,
    generators::comments::Comment,
    helpers::LineSpacing,
    reporting::{FormatDocumentError, FormatDocumentResult, UnwrapIfPreferCrash as _},
};

#[derive(Clone, Debug)]
pub enum NodeTriviaItem {
    LineSpacing(LineSpacing),
    Comment(Comment),
    NewlinedComment(Comment),
    AttributeList(AttributeList),
}

impl NodeTriviaItem {
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
        }
    }
}

#[derive(Clone, Debug)]
pub enum NodeWithTriviaContent {
    NoContent,
    Content(NodeOrToken<SyntaxNode, SyntaxToken>),
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

/// A syntax node (or a point of code) that is preceded by whitespaces, comments and attributes.
#[derive(Clone, Debug)]
pub struct NodeWithTrivia {
    pub preceding_trivia: Vec<NodeTriviaItem>,
    pub node: NodeWithTriviaContent,
    pub succeeding_trivia: Vec<NodeTriviaItem>,
}

impl NodeWithTrivia {
    pub fn kind(&self) -> Option<SyntaxKind> {
        self.node
            .as_ref()
            .map(NodeOrToken::<SyntaxNode, SyntaxToken>::kind)
    }

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

    pub fn only_if_kind(
        self,
        kind: SyntaxKind,
        syntax: &mut SyntaxIter,
    ) -> Option<Self> {
        if self.node.as_ref().is_some_and(|node| node.kind() == kind) {
            Some(self)
        } else {
            self.put_back(syntax);
            None
        }
    }

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

    #[must_use]
    pub const fn is_end(&self) -> bool {
        matches!(self.node, NodeWithTriviaContent::End)
    }

    #[must_use]
    pub fn is_whitespace(&self) -> bool {
        self.node.is_empty()
            && self
                .preceding_trivia
                .iter()
                .all(|trivia| matches!(trivia, NodeTriviaItem::LineSpacing(_)))
            && self
                .succeeding_trivia
                .iter()
                .all(|trivia| matches!(trivia, NodeTriviaItem::LineSpacing(_)))
    }

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
}
