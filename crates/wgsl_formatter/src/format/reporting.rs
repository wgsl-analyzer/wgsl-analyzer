use parser::{SyntaxKind, SyntaxNode, SyntaxToken, WeslLanguage};
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

#[derive(Debug, Clone)]
pub struct ErrorSource {
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

macro_rules! err_src {
    () => {
        $crate::format::reporting::ErrorSource {
            file: file!(),
            line: line!(),
            column: column!(),
        }
    };
}

pub(crate) use err_src;

impl FormatDocumentErrorKind {
    pub const fn at(
        self,
        text_range: TextRange,
        rust_error_source: ErrorSource,
    ) -> FormatDocumentError {
        FormatDocumentError {
            error_kind: self,
            text_range: Some(text_range),
            rust_error_source,
        }
    }

    #[deprecated(note = "Only exists while prototyping")]
    pub const fn without_range(
        self,
        rust_error_source: ErrorSource,
    ) -> FormatDocumentError {
        FormatDocumentError {
            error_kind: self,
            text_range: None,
            rust_error_source,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FormatDocumentError {
    pub error_kind: FormatDocumentErrorKind,
    pub text_range: Option<TextRange>,
    pub rust_error_source: ErrorSource,
}

#[must_use]
pub type FormatDocumentResult<T> = Result<T, FormatDocumentError>;
