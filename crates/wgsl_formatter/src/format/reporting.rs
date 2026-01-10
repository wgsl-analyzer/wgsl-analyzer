use parser::{SyntaxKind, SyntaxNode, SyntaxToken};
use rowan::{NodeOrToken, TextRange};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormatDocumentErrorKind {
    UnexpectedToken {
        received: NodeOrToken<SyntaxNode, SyntaxToken>,
    },
    UnexpectedModuleNode,
    MissingTokens {
        expected: Option<SyntaxKind>,
    },
    MissingNode,
}

impl FormatDocumentErrorKind {
    pub const fn at(
        self,
        text_range: TextRange,
    ) -> FormatDocumentError {
        FormatDocumentError {
            error_kind: self,
            text_range: Some(text_range),
        }
    }

    #[deprecated(note = "Only exists while prototyping")]
    pub const fn without_range(self) -> FormatDocumentError {
        FormatDocumentError {
            error_kind: self,
            text_range: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FormatDocumentError {
    pub error_kind: FormatDocumentErrorKind,
    pub text_range: Option<TextRange>,
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
