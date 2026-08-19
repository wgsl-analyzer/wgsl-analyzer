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

    for line in verbatim_text.split_inclusive('\n') {
        if let Some(line) = line.strip_suffix('\n') {
            let line = line.strip_suffix('\r').unwrap_or(line);
            formatted.push_string(line.to_owned());
            formatted.request(Request::force(RequestItem::LineBreak));
        } else {
            formatted.push_string(line.to_owned());
        }
    }

    formatted.apply_end_request();
    formatted.finish_ignoring_indent_before_requests();

    Ok(formatted)
}
