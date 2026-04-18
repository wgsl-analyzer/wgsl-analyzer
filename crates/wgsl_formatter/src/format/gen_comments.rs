use itertools::{Itertools as _, Position};
use parser::SyntaxKind;
use rowan::NodeOrToken;

use crate::format::{
    ast_parse::{SyntaxIter, parse_token_optional},
    print_item_buffer::PrintItemBuffer,
    reporting::FormatDocumentResult,
};

use super::print_item_buffer::request_folder::RequestItem;

// We don't have a Comment SyntaxNode in the AST yet, so we use a custom enum and parser function
#[derive(Clone, Debug)]
pub enum Comment {
    Block(String),
    LineEnding(String),
}

pub fn parse_comment_optional(syntax: &mut SyntaxIter) -> Option<Comment> {
    let item = syntax.next()?;
    if let NodeOrToken::Token(child) = &item {
        #[expect(
            clippy::wildcard_enum_match_arm,
            reason = "We don't care about future enum variants."
        )]
        match child.kind() {
            SyntaxKind::BlockComment => Some(Comment::Block(child.text().to_owned())),
            SyntaxKind::LineEndingComment => Some(Comment::LineEnding(child.text().to_owned())),

            _ => {
                syntax.put_back(item);
                None
            },
        }
    } else {
        syntax.put_back(item);
        None
    }
}

#[expect(
    clippy::unnecessary_wraps,
    reason = "Keep the API homogeneous with all gen_* functions"
)]
#[expect(
    clippy::redundant_pattern_matching,
    reason = "Make it more obvious that the syntax token is consumed"
)]
pub fn parse_many_comments_and_blankspace(
    syntax: &mut SyntaxIter
) -> FormatDocumentResult<Vec<Comment>> {
    let mut comments = Vec::new();
    loop {
        if let Some(comment) = parse_comment_optional(syntax) {
            comments.push(comment);
        } else if let Some(_) = parse_token_optional(syntax, SyntaxKind::Blankspace) {
            //Allowed, we ignore and consume it
        } else {
            break;
        }
    }
    Ok(comments)
}

pub fn gen_comments(comments: &[Comment]) -> PrintItemBuffer {
    let mut formatted = PrintItemBuffer::new();
    for item in comments {
        formatted.extend(gen_comment(item));
    }
    formatted
}

pub fn gen_comment(item: &Comment) -> PrintItemBuffer {
    let mut formatted = PrintItemBuffer::new();
    match item {
        Comment::Block(content) => {
            formatted.expect(RequestItem::Space);

            let mut lines = content.lines().with_position();
            if let Some((pos, line)) = lines.next() {
                formatted.push_string(line.to_owned());
                if pos != Position::Only && pos != Position::Last {
                    formatted.expect(RequestItem::LineBreak);
                }
            }

            formatted.start_ignoring_indent();
            for (pos, line) in lines {
                formatted.push_string(line.to_owned());
                if pos != Position::Only && pos != Position::Last {
                    formatted.expect(RequestItem::LineBreak);
                }
            }
            formatted.discourage(RequestItem::LineBreak);
            formatted.finish_ignoring_indent();
            formatted.expect(RequestItem::Space);
        },
        Comment::LineEnding(content) => {
            formatted.expect(RequestItem::Space);
            // TODO There should never be newlinees in a line ending comment...Right?
            formatted.push_string(content.clone());
            formatted.force(RequestItem::LineBreak);
        },
    }
    formatted
}
