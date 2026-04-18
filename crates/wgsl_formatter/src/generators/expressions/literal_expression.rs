use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::{print_item_buffer::PrintItemBuffer, reporting::FormatDocumentResult};
#[expect(
    clippy::unnecessary_wraps,
    reason = "Keep API uniform with other gen functions"
)]
pub fn gen_literal_expression(
    literal_expression: &ast::Literal
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.push_string(literal_expression.syntax().to_string());
    Ok(formatted)
}
