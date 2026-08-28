use itertools::{Itertools, Position};
use parser::{SyntaxNode, SyntaxToken};
use rowan::NodeOrToken;

use crate::{
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

pub fn gen_node_syntax_verbatim(
    node: &NodeOrToken<SyntaxNode, SyntaxToken>
) -> FormatDocumentResult<PrintItemBuffer> {
    let verbatim_text = node.to_string();

    let mut formatted = PrintItemBuffer::default();

    formatted.start_ignoring_indent_before_requests();
    formatted.apply_end_request();

    for (position, line) in verbatim_text.split_inclusive('\n').with_position() {
        let (line, has_break) = line.strip_suffix('\n').map_or((line, false), |line| {
            (line.strip_suffix('\r').unwrap_or(line), true)
        });
        if !line.is_empty() || position == Position::Middle {
            push_string_with_tabs(&mut formatted, line);
        }
        if has_break {
            formatted.request(Request::force(RequestItem::LineBreak));
            if position != Position::Last && position != Position::Only {
                formatted.apply_end_request();
                formatted.push_sc(dprint_core_macros::sc!("")); //TODO Why is this necessary? This should absolutely not be needed here??
            }
        }
    }

    formatted.apply_end_request();
    formatted.finish_ignoring_indent_before_requests();

    Ok(formatted)
}

fn push_string_with_tabs(
    formatted: &mut PrintItemBuffer,
    text: &str,
) {
    for part in text.split_inclusive('\t') {
        if let Some(part) = part.strip_suffix('\t') {
            formatted.push_string(part.to_owned());
            formatted.push_tab();
        } else {
            formatted.push_string(part.to_owned());
        }
    }
}
