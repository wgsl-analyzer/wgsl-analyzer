use syntax::{AstNode as _, ast};

use crate::format::{
    helpers::todo_verbatim_wesl, print_item_buffer::PrintItemBuffer,
    reporting::FormatDocumentResult,
};

pub fn gen_import_statement(
    import: &ast::ImportStatement
) -> FormatDocumentResult<PrintItemBuffer> {
    todo_verbatim_wesl(import.syntax())
}
