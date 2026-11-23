use syntax::{AstNode as _, ast};

use crate::{
    FormattingOptions, print_item_buffer::PrintItemBuffer, reporting::FormatDocumentResult,
};
#[expect(
    clippy::unnecessary_wraps,
    reason = "Keep API uniform with other gen functions"
)]
pub(crate) fn gen_literal_expression(
    #[expect(unused, reason = "no config is necessary here (yet)")] options: &FormattingOptions,
    literal_expression: &ast::Literal,
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.push_string(literal_expression.syntax().to_string());
    Ok(formatted)
}
