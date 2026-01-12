use parser::{SyntaxKind, SyntaxToken};

use crate::format::print_item_buffer::{PrintItemBuffer, SeparationPolicy, SeparationRequest};

pub fn gen_comments(comments: Vec<SyntaxToken>) -> PrintItemBuffer {
    let mut formatted = PrintItemBuffer::new();
    for item in comments {
        formatted.extend(gen_comment(&item));
    }
    formatted
}
pub fn gen_comment(item: &SyntaxToken) -> PrintItemBuffer {
    let mut formatted = PrintItemBuffer::new();
    if item.kind() == SyntaxKind::BlockComment {
        formatted.expect_single_space();
        formatted.push_string(item.to_string());
        formatted.expect_single_space();
    } else if item.kind() == SyntaxKind::LineEndingComment {
        formatted.expect_single_space();
        formatted.push_string(item.to_string());
        //TODO This should be a request, but for now we have no way of encoding a "forced newline no matter what"
        formatted.request(SeparationRequest {
            line_break: SeparationPolicy::Forced,
            ..Default::default()
        });
    } else {
        //TODO Make this unrepresentable
        unreachable!("Non comment entry found in comments Vec");
    }
    formatted
}
