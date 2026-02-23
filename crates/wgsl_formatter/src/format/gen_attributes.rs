use parser::SyntaxToken;
use syntax::{
    AstNode as _,
    ast::{self, Attribute},
};

use crate::format::{
    ast_parse::{SyntaxIter, parse_many_comments_and_blankspace, parse_node_optional},
    gen_comments::gen_comments,
    print_item_buffer::PrintItemBuffer,
    reporting::FormatDocumentResult,
};

pub struct ParsedAttributes {
    attributes: Vec<(ast::Attribute, Vec<SyntaxToken>)>,
}

pub fn parse_many_attributes(syntax: &mut SyntaxIter) -> FormatDocumentResult<ParsedAttributes> {
    // TODO(MonaMayrhofer) Think about whether this is the correct way to abstract this.
    // Maybe there should even a "many with commments" combinator, to also deduplicate code from fn parameters/struct members
    // Also this is very similar to parse_many_comments_and_blankspace
    let mut attributes = Vec::new();
    loop {
        let Some(item_attribute) = parse_node_optional::<ast::Attribute>(syntax) else {
            break;
        };
        let item_comments_after_attribute = parse_many_comments_and_blankspace(syntax)?;

        attributes.push((item_attribute, item_comments_after_attribute));
    }
    Ok(ParsedAttributes { attributes })
}

//TODO Properly handle the comments, instead of just passing "syntaxtoken", which seems very untyped...
pub fn gen_attributes(attributes: &ParsedAttributes) -> FormatDocumentResult<PrintItemBuffer> {
    let mut formatted = PrintItemBuffer::new();

    //TODO Sort and order attributes
    for (attribute, comments_after_attribute) in &attributes.attributes {
        formatted.extend(gen_attribute(attribute)?);
        formatted.extend(gen_comments(comments_after_attribute));
        formatted.expect_line_break();
    }

    Ok(formatted)
}

pub fn gen_attribute(attribute: &ast::Attribute) -> FormatDocumentResult<PrintItemBuffer> {
    //formatted.push_sc(sc!("@"));
    super::helpers::todo_verbatim(attribute.syntax())
}
