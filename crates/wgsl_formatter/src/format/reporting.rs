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
