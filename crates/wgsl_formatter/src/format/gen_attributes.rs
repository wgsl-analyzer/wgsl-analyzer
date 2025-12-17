use parser::SyntaxToken;
use syntax::{AstNode as _, ast};

use crate::format::{
    gen_comments::gen_comments, print_item_buffer::PrintItemBuffer, reporting::FormatDocumentResult,
};

//TODO Properly handle the comments, instead of just passing "syntaxtoken", which seems very untyped...
pub fn gen_attributes(
    attributes: Vec<(ast::Attribute, Vec<SyntaxToken>)>
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut formatted = PrintItemBuffer::new();

    //TODO Sort and order attributes
    for (attribute, comments_after_attribute) in attributes {
        formatted.extend(gen_attribute(&attribute)?);
        formatted.extend(gen_comments(comments_after_attribute));
        formatted.expect_line_break();
    }

    Ok(formatted)
}

pub fn gen_attribute(attribute: &ast::Attribute) -> FormatDocumentResult<PrintItemBuffer> {
    //formatted.push_sc(sc!("@"));
    super::helpers::todo_verbatim(attribute.syntax())
}
