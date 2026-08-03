use parser::SyntaxToken;
use rowan::NodeOrToken;

use crate::{
    ast_parse::{SyntaxIter, parse_token_optional},
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

#[derive(Clone, Debug)]
//TODO rename to LineSpacing
pub enum NextGenLineSpacing {
    LineBreak(SyntaxToken),
    EmptyLine(SyntaxToken),
    OnelineBlankspace(SyntaxToken),
}

#[derive(Clone, Copy, Debug)]
#[deprecated]
pub enum LineSpacing {
    LineBreak,
    EmptyLine,
}

//TODO Rename to parse_line_spacing
pub fn parse_next_gen_line_spacing(syntax: &mut SyntaxIter) -> Option<NextGenLineSpacing> {
    let blankspace = parse_token_optional(syntax, parser::SyntaxKind::Blankspace)?;

    let newlines = blankspace
        .text()
        .chars()
        .filter(|item| *item == '\n')
        .count();
    match newlines {
        0 => Some(NextGenLineSpacing::OnelineBlankspace(blankspace)),
        1 => Some(NextGenLineSpacing::LineBreak(blankspace)),
        _ => Some(NextGenLineSpacing::EmptyLine(blankspace)),
    }
}

#[deprecated]
pub fn parse_line_spacing(syntax: &mut SyntaxIter) -> Option<LineSpacing> {
    let blankspace = parse_token_optional(syntax, parser::SyntaxKind::Blankspace)?;

    let newlines = blankspace
        .text()
        .chars()
        .filter(|item| *item == '\n')
        .count();
    match newlines {
        0 => {
            syntax.put_back(NodeOrToken::Token(blankspace));
            None
        },
        1 => Some(LineSpacing::LineBreak),
        _ => Some(LineSpacing::EmptyLine),
    }
}

#[expect(
    clippy::unnecessary_wraps,
    reason = "Keep the API homogeneous with all gen_* functions"
)]
pub fn gen_line_spacing(line_spacing: &LineSpacing) -> FormatDocumentResult<PrintItemBuffer> {
    let mut formatted = PrintItemBuffer::default();
    match line_spacing {
        LineSpacing::EmptyLine => {
            //There was an empty line in the source
            formatted.request(Request::expect(RequestItem::EmptyLine));
        },
        LineSpacing::LineBreak => {
            //There was a newline in the source
            formatted.request(Request::expect(RequestItem::LineBreak));
        },
    }
    Ok(formatted)
}

#[expect(
    clippy::unnecessary_wraps,
    reason = "Keep the API homogeneous with all gen_* functions"
)]
//TODO Rename to gen_line_spacing
pub fn gen_next_gen_line_spacing(
    line_spacing: &NextGenLineSpacing
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut formatted = PrintItemBuffer::default();
    match line_spacing {
        NextGenLineSpacing::EmptyLine(_) => {
            //There was an empty line in the source
            formatted.request(Request::expect(RequestItem::EmptyLine));
        },
        NextGenLineSpacing::LineBreak(_) => {
            //There was a newline in the source
            formatted.request(Request::expect(RequestItem::LineBreak));
        },
        NextGenLineSpacing::OnelineBlankspace(_) => {
            // There was blankspace in the source which we ignore
        },
    }
    Ok(formatted)
}
