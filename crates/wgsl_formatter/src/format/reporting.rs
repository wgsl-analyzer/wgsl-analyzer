use parser::SyntaxNode;
use rowan::TextRange;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatDocumentErrorKind {
    UnexpectedToken,
    UnexpectedModuleNode,
    MissingTokens,
}

impl FormatDocumentErrorKind {
    pub const fn at(
        self,
        text_range: TextRange,
    ) -> FormatDocumentError {
        FormatDocumentError {
            error_kind: self,
            text_range,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FormatDocumentError {
    pub error_kind: FormatDocumentErrorKind,
    pub text_range: TextRange,
}

pub type FormatDocumentResult<T> = Result<T, FormatDocumentError>;
