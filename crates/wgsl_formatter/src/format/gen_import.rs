use itertools::put_back;
use syntax::{AstNode as _, ast};

use crate::format::{
    ast_parse::parse_end, print_item_buffer::PrintItemBuffer, reporting::FormatDocumentResult,
};

pub fn gen_import_package_relative(
    node: &ast::ImportPackageRelative
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(node.syntax().children_with_tokens());
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    Ok(formatted)
}
pub fn gen_import_super_relative(
    node: &ast::ImportSuperRelative
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(node.syntax().children_with_tokens());
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    Ok(formatted)
}
pub fn gen_import_item(node: &ast::ImportItem) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(node.syntax().children_with_tokens());
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    Ok(formatted)
}
pub fn gen_import_path(node: &ast::ImportPath) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(node.syntax().children_with_tokens());
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    Ok(formatted)
}
pub fn gen_import_collection(
    node: &ast::ImportCollection
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(node.syntax().children_with_tokens());
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    Ok(formatted)
}
