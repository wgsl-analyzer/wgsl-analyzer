use syntax::{AstNode as _, ast};

use crate::format::{
    helpers::todo_verbatim_wesl, print_item_buffer::PrintItemBuffer,
    reporting::FormatDocumentResult,
};

pub fn gen_path(path: &ast::Path) -> FormatDocumentResult<PrintItemBuffer> {
    todo_verbatim_wesl(path.syntax())
}
