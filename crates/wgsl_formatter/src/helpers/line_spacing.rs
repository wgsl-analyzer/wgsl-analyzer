use parser::{SyntaxKind, SyntaxNode, SyntaxToken};
use rowan::NodeOrToken;

use crate::{
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

#[derive(Clone, Debug)]
pub enum LineSpacing {
    LineBreak(SyntaxToken),
    EmptyLine(SyntaxToken),
    OnelineBlankspace(SyntaxToken),
}

#[must_use]
pub fn read_blankspace(blankspace: &NodeOrToken<SyntaxNode, SyntaxToken>) -> Option<LineSpacing> {
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
        0 => Some(LineSpacing::OnelineBlankspace(blankspace.clone())),
        1 => Some(LineSpacing::LineBreak(blankspace.clone())),
        _ => Some(LineSpacing::EmptyLine(blankspace.clone())),
    }
}

#[expect(
    clippy::unnecessary_wraps,
    reason = "Keep the API homogeneous with all gen_* functions"
)]
pub fn gen_line_spacing(line_spacing: &LineSpacing) -> FormatDocumentResult<PrintItemBuffer> {
    let mut formatted = PrintItemBuffer::default();
    match line_spacing {
        LineSpacing::EmptyLine(_) => {
            //There was an empty line in the source
            formatted.request(Request::expect(RequestItem::EmptyLine));
        },
        LineSpacing::LineBreak(_) => {
            //There was a newline in the source
            formatted.request(Request::expect(RequestItem::LineBreak));
        },
        LineSpacing::OnelineBlankspace(_) => {
            // There was blankspace in the source which we ignore
        },
    }
    Ok(formatted)
}
