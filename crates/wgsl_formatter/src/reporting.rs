//! Various bare-bones error handling.
use std::fmt::Display;

use parser::{SyntaxNode, SyntaxToken};
use rowan::NodeOrToken;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormatDocumentError {
    UnexpectedNodeOrToken {
        received: Option<NodeOrToken<SyntaxNode, SyntaxToken>>,
    },
    UnsupportedNodeOrToken {
        received: NodeOrToken<SyntaxNode, SyntaxToken>,
    },
    MissingNode,
}

impl Display for FormatDocumentError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        #[expect(
            clippy::use_debug,
            reason = "We want the debug information here, as this is an error we don't rely on a stable format"
        )]
        match self {
            Self::UnexpectedNodeOrToken {
                received: Some(received),
            } => {
                write!(
                    f,
                    "Unexpected node or token {:?} at {:?}. {received:?}",
                    received.kind(),
                    received.text_range()
                )
            },
            Self::UnexpectedNodeOrToken { received: None } => {
                write!(f, "Expected node or token but found None")
            },
            Self::UnsupportedNodeOrToken { received } => {
                write!(f, "Encountered unsupported Node or Token: {received:?}")
            },
            Self::MissingNode => write!(f, "Expected to find a node but found none"),
        }
    }
}

pub type FormatDocumentResult<T> = Result<T, FormatDocumentError>;

pub trait UnwrapIfPreferCrash {
    #[must_use]
    fn expect_if_prefer_crash(self) -> Self;
}

impl<T> UnwrapIfPreferCrash for FormatDocumentResult<T> {
    #[inline]
    #[track_caller]
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
