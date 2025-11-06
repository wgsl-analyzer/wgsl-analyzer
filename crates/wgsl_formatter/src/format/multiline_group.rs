use dprint_core::formatting::{LineNumber, LineNumberAnchor};

use crate::format::{
    helpers::create_is_multiple_lines_resolver, print_item_buffer::PrintItemBuffer,
};

fn gen_multiline_group(lines: &[PrintItemBuffer]) -> PrintItemBuffer {
    let mut formatted = PrintItemBuffer::new();

    let start_ln = LineNumber::new("start");
    let end_ln = LineNumber::new("end");
    let is_multiple_lines = create_is_multiple_lines_resolver(start_ln, end_ln);

    formatted.push_info(start_ln);
    formatted.push_anchor(LineNumberAnchor::new(end_ln));
    formatted
}
