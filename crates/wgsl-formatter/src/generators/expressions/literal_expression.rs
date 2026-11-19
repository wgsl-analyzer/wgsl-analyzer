use dprint_core::formatting::Arena;
use syntax::{AstNode as _, ast};

use crate::{print_item_buffer::PrintItemBuffer, reporting::FormatDocumentResult};
#[expect(
    clippy::unnecessary_wraps,
    reason = "Keep API uniform with other gen functions"
)]
pub(crate) fn gen_literal_expression<'buffer>(
    arena: &'buffer Arena,
    literal_expression: &ast::Literal,
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    // ==== Format ====
    let mut formatted = PrintItemBuffer::new(arena);
    formatted.push_string(arena, literal_expression.syntax().to_string());
    Ok(formatted)
}
