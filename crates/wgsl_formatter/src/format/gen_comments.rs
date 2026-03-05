use parser::SyntaxKind;
use rowan::NodeOrToken;

use crate::format::{
    ast_parse::{SyntaxIter, parse_token_optional},
    print_item_buffer::{PrintItemBuffer, SeparationPolicy, SeparationRequest},
    reporting::FormatDocumentResult,
};

// We don't have a Comment SyntaxNode in the AST yet, so we use a custom enum and parser function
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

// TODO(MonaMayrhofer) Search for all usages of gen_comment(s) and see if they use the more modern parse_comment to parse them
pub fn gen_comment(item: &Comment) -> PrintItemBuffer {
    let mut formatted = PrintItemBuffer::new();
    match item {
        Comment::Block(content) => {
            formatted.expect_single_space();
            formatted.push_string(content.clone());
            formatted.expect_single_space();
        },
        Comment::LineEnding(content) => {
            formatted.expect_single_space();
            formatted.push_string(content.clone());
            //TODO(MonaMayrhofer) This should be a request, but for now we have no way of encoding a "forced newline no matter what"
            formatted.request(SeparationRequest {
                line_break: SeparationPolicy::Forced,
                ..Default::default()
            });
        },
    }
    formatted
}
