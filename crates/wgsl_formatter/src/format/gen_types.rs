use syntax::{AstNode as _, ast};

use crate::format::{print_item_buffer::PrintItemBuffer, reporting::FormatDocumentResult};

pub fn gen_type_specifier(
    type_specifier: &ast::TypeSpecifier
) -> FormatDocumentResult<PrintItemBuffer> {
    //TODO
    super::helpers::todo_verbatim(type_specifier.syntax())
}
