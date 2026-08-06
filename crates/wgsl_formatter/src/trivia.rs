use parser::{SyntaxKind, SyntaxNode, SyntaxToken};
use rowan::NodeOrToken;
use syntax::{
    AstNode, AstToken,
    ast::{Attribute, AttributeList},
};

use crate::{
    generators::comments::Comment,
    helpers::{LineSpacing, NextGenLineSpacing},
    reporting::{FormatDocumentError, FormatDocumentResult},
};

#[derive(Clone, Debug)]
pub enum NodeTriviaItem {
    LineSpacing(NextGenLineSpacing),
    Comment(Comment),
    AttributeList(AttributeList),
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

/// A syntax node (or a point of code) that is preceded by whitespaces, comments and attributes
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

    pub fn expect_kind_optional(
        self,
        kind: SyntaxKind,
    ) -> FormatDocumentResult<Self> {
        if self.node.as_ref().is_some_and(|node| node.kind() != kind) {
            //TODO Better error here
            Err(FormatDocumentError::UnexpectedNodeOrToken {
                received: self.node.into_option().unwrap(),
            })
        } else {
            Ok(self)
        }
    }

    pub fn expect_kind(
        self,
        kind: SyntaxKind,
    ) -> FormatDocumentResult<Self> {
        if self.node.as_ref().is_some_and(|node| node.kind() == kind) {
            Ok(self)
        } else {
            //TODO Better error here
            Err(FormatDocumentError::UnexpectedNodeOrToken {
                received: self.node.into_option().unwrap(),
            })
        }
    }

    // TODO Rename to expect_ast_node
    pub fn expect_castable_kind<T>(self) -> FormatDocumentResult<Self>
    where
        T: AstNode,
    {
        if let NodeWithTriviaContent::Content(NodeOrToken::Node(node)) = &self.node {
            if T::cast(node.clone()).is_some() {
                return Ok(self);
            }
        }
        //TODO Better error here
        Err(FormatDocumentError::UnexpectedNodeOrToken {
            received: self.node.into_option().unwrap(),
        })
    }

    pub fn expect_ast_token<T>(self) -> FormatDocumentResult<Self>
    where
        T: AstToken,
    {
        if let NodeWithTriviaContent::Content(NodeOrToken::Token(node)) = &self.node {
            if T::cast(node.clone()).is_some() {
                return Ok(self);
            }
        }
        //TODO Better error here
        Err(FormatDocumentError::UnexpectedNodeOrToken {
            received: self.node.into_option().unwrap(),
        })
    }

    pub fn is_end(&self) -> bool {
        matches!(self.node, NodeWithTriviaContent::End)
    }

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
}
