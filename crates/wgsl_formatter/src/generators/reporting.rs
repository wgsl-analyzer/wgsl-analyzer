use std::fmt::Display;

use parser::{SyntaxKind, SyntaxNode, SyntaxToken};
use rowan::NodeOrToken;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormatDocumentError {
    UnexpectedNodeOrToken {
        received: NodeOrToken<SyntaxNode, SyntaxToken>,
    },
    UnsupportedNodeOrToken {
        received: NodeOrToken<SyntaxNode, SyntaxToken>,
    },
    MissingTokens {
        expected: Option<SyntaxKind>,
    },
    MissingNode,
}

impl Display for FormatDocumentError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::UnexpectedNodeOrToken { received } => {
                write!(f, "Unexpected node or token: {received:?}")
            },
            Self::UnsupportedNodeOrToken { received } => write!(
                f,
                "Node/Token found at an unsupported location: {received:?}",
            ),
            Self::MissingTokens { expected } => {
                write!(f, "Expected to find a token {expected:?} but found none")
            },
            Self::MissingNode => write!(f, "Expected to find a node but found none"),
        }
    }
}

pub type FormatDocumentResult<T> = Result<T, FormatDocumentError>;

pub trait UnwrapIfPreferCrash {
    fn expect_if_prefer_crash(self) -> Self;
}

impl<T> UnwrapIfPreferCrash for FormatDocumentResult<T> {
    #[inline]
    fn expect_if_prefer_crash(self) -> Self {
        #[cfg(feature = "prefer-immediate-crash")]
        {
            Ok(self.expect(
                "Compiled with --features=prefer-immediate-crash, thus immediately crashing.",
            ))
        }
        #[cfg(not(feature = "prefer-immediate-crash"))]
        {
            self
        }
    }
}
