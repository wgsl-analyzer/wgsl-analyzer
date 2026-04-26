use rowan::NodeOrToken;

use crate::{
    ast_parse::{SyntaxIter, parse_token_optional},
    print_item_buffer::{
        PrintItemBuffer,
        request_folder::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

pub enum LineSpacing {
    LineBreak,
    EmptyLine,
}

pub fn parse_line_spacing(syntax: &mut SyntaxIter) -> Option<LineSpacing> {
    let blankspace = parse_token_optional(syntax, parser::SyntaxKind::Blankspace)?;

    //TODO(MonaMayrhofer) Think a bit more about different types of newlines (\c\n etc.)
    //TODO(MonaMayrhofer) child.to_string() here surely is wasteful - there must be a better way.
    let newlines = blankspace
        .to_string()
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
