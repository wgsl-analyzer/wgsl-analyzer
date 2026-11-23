use dprint_core::formatting::Arena;
use itertools::Itertools as _;
use rowan::NodeOrToken;
use syntax::{SyntaxNode, SyntaxToken};

use crate::{
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

#[expect(clippy::unnecessary_wraps, reason = "Intentional api for generators")]
pub(crate) fn gen_node_syntax_verbatim<'buffer>(
    arena: &'buffer Arena,
    node: &NodeOrToken<SyntaxNode, SyntaxToken>,
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    let verbatim_text = node.to_string();

    let mut formatted = PrintItemBuffer::new(arena);

    formatted.start_ignoring_indent_before_requests(arena);
    formatted.apply_end_request(arena);

    for (position, line) in verbatim_text.split_inclusive('\n').with_position() {
        let (line, has_break) = line.strip_suffix('\n').map_or((line, false), |line| {
            (line.strip_suffix('\r').unwrap_or(line), true)
        });
        if !line.is_empty() || position.is_middle() {
            push_string_with_tabs(arena, &mut formatted, line);
        }
        if has_break {
            formatted.request(Request::force(RequestItem::LineBreak));
            if !position.is_last && !position.is_exactly_one() {
                formatted.apply_end_request(arena);
            }
        }
    }

    formatted.apply_end_request(arena);
    formatted.finish_ignoring_indent_before_requests(arena);

    Ok(formatted)
}

fn push_string_with_tabs<'buffer>(
    arena: &'buffer Arena,
    formatted: &mut PrintItemBuffer<'buffer>,
    text: &str,
) {
    for part in text.split_inclusive('\t') {
        if let Some(part) = part.strip_suffix('\t') {
            formatted.push_string(arena, part.to_owned());
            formatted.push_tab(arena);
        } else {
            formatted.push_string(arena, part.to_owned());
        }
    }
}
