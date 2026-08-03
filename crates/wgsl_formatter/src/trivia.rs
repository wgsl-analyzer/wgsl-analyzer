use parser::{SyntaxKind, SyntaxNode, SyntaxToken};
use rowan::NodeOrToken;
use syntax::ast::{Attribute, AttributeList};

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
    pub fn is_empty(&self) -> bool {
        matches!(
            self,
            NodeWithTriviaContent::NoContent | NodeWithTriviaContent::End
        )
    }

    pub fn as_ref(&self) -> Option<&NodeOrToken<SyntaxNode, SyntaxToken>> {
        match self {
            NodeWithTriviaContent::NoContent => todo!(),
            NodeWithTriviaContent::Content(node_or_token) => Some(&node_or_token),
            NodeWithTriviaContent::End => todo!(),
        }
    }

    pub fn unwrap(self) -> NodeOrToken<SyntaxNode, SyntaxToken> {
        match self {
            NodeWithTriviaContent::NoContent => todo!(),
            NodeWithTriviaContent::Content(node_or_token) => node_or_token,
            NodeWithTriviaContent::End => todo!(),
        }
    }

    pub fn as_option(self) -> Option<NodeOrToken<SyntaxNode, SyntaxToken>> {
        match self {
            NodeWithTriviaContent::NoContent => None,
            NodeWithTriviaContent::Content(node_or_token) => Some(node_or_token),
            NodeWithTriviaContent::End => None,
        }
    }
}

/// A syntax node (or a point of code) that is preceded by whitespaces, comments and attributes
#[derive(Clone, Debug)]
pub struct NodeWithTrivia {
    pub preceding_trivia: Vec<NodeTriviaItem>,
    pub node: NodeWithTriviaContent,
}

impl NodeWithTrivia {
    pub fn expect_kind(
        self,
        kind: SyntaxKind,
    ) -> FormatDocumentResult<Self> {
        if self.node.as_ref().is_some_and(|node| node.kind() == kind) {
            Ok(self)
        } else {
            //TODO Better error here
            Err(FormatDocumentError::UnexpectedNodeOrToken {
                received: self.node.unwrap(),
            })
        }
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
    }
}
