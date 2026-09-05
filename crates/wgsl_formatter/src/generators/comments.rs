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
impl Comment {
    #[must_use]
    pub fn syntax(
        &self
    ) -> NodeOrToken<
        rowan::SyntaxNode<parser::WeslLanguage>,
        rowan::SyntaxToken<parser::WeslLanguage>,
    > {
        match self {
            Self::Block(token) | Self::LineEnding(token) => NodeOrToken::Token(token.clone()),
        }
    }
}

#[must_use]
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

#[must_use]
pub fn gen_comment(item: &Comment) -> PrintItemBuffer {
    let mut formatted = PrintItemBuffer::default();
    match item {
        Comment::Block(content) => {
            formatted.request(Request::expect(RequestItem::Space));

            let mut lines = content.text().lines().with_position();
            if let Some((line_pos, line)) = lines.next() {
                for (tab_pos, tab_part) in line.split('\t').with_position() {
                    formatted.push_string(tab_part.to_owned());
                    if tab_pos != Position::Last && tab_pos != Position::Only {
                        formatted.push_tab();
                    }
                }
                if line_pos != Position::Only && line_pos != Position::Last {
                    formatted.request(Request::expect(RequestItem::LineBreak));
                }
            }

            formatted.start_ignoring_indent_before_requests();
            for (line_pos, line) in lines {
                for (tab_pos, tab_part) in line.split('\t').with_position() {
                    formatted.push_string(tab_part.to_owned());
                    if tab_pos != Position::Last && tab_pos != Position::Only {
                        formatted.push_tab();
                    }
                }
                if line_pos != Position::Only && line_pos != Position::Last {
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
            for (tab_pos, tab_part) in content.text().split('\t').with_position() {
                formatted.push_string(tab_part.to_owned());
                if tab_pos != Position::Last && tab_pos != Position::Only {
                    formatted.push_tab();
                }
            }
            formatted.request(Request::force(RequestItem::LineBreak));
        },
    }
    formatted
}
