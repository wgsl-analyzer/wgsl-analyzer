use itertools::{Itertools as _, Position};
use parser::{SyntaxKind, SyntaxNode, SyntaxToken};
use rowan::NodeOrToken;

use crate::print_item_buffer::{
    PrintItemBuffer,
    spacing_request::{Request, RequestItem},
};

// We don't have a Comment SyntaxNode in the AST yet, so we use a custom enum and parser function
#[derive(Clone, Debug)]
pub enum Comment {
    Block(SyntaxToken),
    LineEnding(SyntaxToken),
}

pub fn read_comment(item: &NodeOrToken<SyntaxNode, SyntaxToken>) -> Option<Comment> {
    if let NodeOrToken::Token(child) = &item {
        #[expect(
            clippy::wildcard_enum_match_arm,
            reason = "We don't care about future enum variants."
        )]
        match child.kind() {
            SyntaxKind::BlockComment => Some(Comment::Block(child.clone())),
            SyntaxKind::LineEndingComment => Some(Comment::LineEnding(child.clone())),
            _ => None,
        }
    } else {
        None
    }
}

pub fn gen_comment(item: &Comment) -> PrintItemBuffer {
    let mut formatted = PrintItemBuffer::default();
    match item {
        Comment::Block(content) => {
            formatted.request(Request::expect(RequestItem::Space));

            let mut lines = content.text().lines().with_position();
            if let Some((pos, line)) = lines.next() {
                formatted.push_string(line.to_owned());
                if pos != Position::Only && pos != Position::Last {
                    formatted.request(Request::expect(RequestItem::LineBreak));
                }
            }

            formatted.start_ignoring_indent_before_requests();
            for (pos, line) in lines {
                formatted.push_string(line.to_owned());
                if pos != Position::Only && pos != Position::Last {
                    formatted.request(Request::expect(RequestItem::LineBreak));
                }
            }
            formatted.finish_ignoring_indent_before_requests();
            formatted.request(Request::expect(RequestItem::Space));
        },
        Comment::LineEnding(content) => {
            formatted.request(Request::expect(RequestItem::Space));
            // Line ending comments may not contain newlines - otherwise push_string will
            // run into a debug_assert down the line where its a lot harder to debug.
            debug_assert!(
                content.text().lines().count() == 1,
                "line ending comment may not contain newlines."
            );
            formatted.push_string(content.text().to_owned());
            formatted.request(Request::force(RequestItem::LineBreak));
        },
    }
    formatted
}
