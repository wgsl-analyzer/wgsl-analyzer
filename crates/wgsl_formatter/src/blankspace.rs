//! Various helpers to classify blankspaces into their types.
use parser::{SyntaxKind, SyntaxNode, SyntaxToken};
use rowan::NodeOrToken;

use crate::{
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

/// A `SyntaxToken` containing only blank space.
///
/// ... and newlines, and tabs, and similar stuff.
#[derive(Clone, Debug)]
pub enum Blankspace {
    /// A blankspace that contains exactly one newline (and possibly spaces and tabs).
    LineBreak(SyntaxToken),
    /// A blankspace that contains exactly two newlines (and possibly spaces and tabs).
    EmptyLine(SyntaxToken),
    /// A blankspace that contains no newlines (but possibly spaces and tabs).
    Inline(SyntaxToken),
}
impl Blankspace {
    #[must_use]
    pub fn syntax(&self) -> NodeOrToken<SyntaxNode, SyntaxToken> {
        match self {
            Self::LineBreak(syntax_token)
            | Self::EmptyLine(syntax_token)
            | Self::Inline(syntax_token) => NodeOrToken::Token(syntax_token.clone()),
        }
    }
}

#[must_use]
pub fn read_blankspace(blankspace: &NodeOrToken<SyntaxNode, SyntaxToken>) -> Option<Blankspace> {
    let NodeOrToken::Token(blankspace) = blankspace else {
        return None;
    };

    if blankspace.kind() != SyntaxKind::Blankspace {
        return None;
    }

    let newlines = blankspace
        .text()
        .chars()
        .filter(|item| *item == '\n')
        .count();
    match newlines {
        0 => Some(Blankspace::Inline(blankspace.clone())),
        1 => Some(Blankspace::LineBreak(blankspace.clone())),
        _ => Some(Blankspace::EmptyLine(blankspace.clone())),
    }
}

#[expect(
    clippy::unnecessary_wraps,
    reason = "Keep the API homogeneous with all gen_* functions"
)]
pub fn gen_blankspace(blankspace: &Blankspace) -> FormatDocumentResult<PrintItemBuffer> {
    let mut formatted = PrintItemBuffer::default();
    match blankspace {
        Blankspace::EmptyLine(_) => {
            //There was an empty line in the source
            formatted.request(Request::expect(RequestItem::EmptyLine));
        },
        Blankspace::LineBreak(_) => {
            //There was a newline in the source
            formatted.request(Request::expect(RequestItem::LineBreak));
        },
        Blankspace::Inline(_) => {
            // There was blankspace in the source which we ignore
        },
    }
    Ok(formatted)
}
