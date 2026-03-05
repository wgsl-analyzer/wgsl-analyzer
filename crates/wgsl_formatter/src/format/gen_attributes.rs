use dprint_core_macros::sc;
use itertools::put_back;
use parser::{SyntaxKind, SyntaxToken};
use syntax::{
    AstNode as _,
    ast::{self, Arguments, Attribute, IdentExpression},
};

use crate::format::{
    ast_parse::{
        SyntaxIter, parse_end, parse_many_comments_and_blankspace, parse_node, parse_node_optional,
        parse_token,
    },
    gen_comments::{Comment, gen_comments},
    gen_function_call::gen_function_call_arguments,
    print_item_buffer::PrintItemBuffer,
    reporting::FormatDocumentResult,
};

pub struct ParsedAttributes {
    attributes: Vec<(ast::Attribute, Vec<Comment>)>,
}

pub fn parse_many_attributes(syntax: &mut SyntaxIter) -> FormatDocumentResult<ParsedAttributes> {
    // TODO(MonaMayrhofer) Think about whether this is the correct way to abstract this.
    // Maybe there should even a "many with comments" combinator, to also deduplicate code from fn parameters/struct members
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
    let mut syntax = put_back(attribute.syntax().children_with_tokens());

    parse_token(&mut syntax, SyntaxKind::AttributeOperator)?;
    let item_comments_after_operator = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_identifier = parse_token(&mut syntax, parser::SyntaxKind::Identifier)?;
    let item_comments_after_identifier = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_arguments = parse_node_optional::<Arguments>(&mut syntax);
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("@"));
    formatted.extend(gen_comments(&item_comments_after_operator));
    formatted.push_string(item_identifier.to_string());
    formatted.extend(gen_comments(&item_comments_after_identifier));
    if let Some(item_arguments) = item_arguments {
        formatted.extend(gen_function_call_arguments(&item_arguments)?);
    }
    Ok(formatted)
}
