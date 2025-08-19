use parser::SyntaxNode;
use pretty::{DocAllocator, DocBuilder};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatDocumentErrorKind {
    MissingFnName,
    MissingFnParams,
    MissingFnParamName,
    MissingFnParamType,
    UnexpectedModuleNode,
}

impl FormatDocumentErrorKind {
    pub const fn at(
        self,
        syntax_node: SyntaxNode,
    ) -> FormatDocumentError {
        FormatDocumentError {
            error_kind: self,
            syntax_node,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FormatDocumentError {
    pub error_kind: FormatDocumentErrorKind,
    pub syntax_node: SyntaxNode,
}

pub type FormatDocumentResult<T> = Result<T, FormatDocumentError>;
