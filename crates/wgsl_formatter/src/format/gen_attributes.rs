use std::string::String;

use dprint_core_macros::sc;
use itertools::put_back;
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self, Arguments, Attribute, DiagnosticControl},
};

use crate::format::{
    ast_parse::{
        SyntaxIter, parse_end, parse_many_comments_and_blankspace, parse_node, parse_node_optional,
        parse_token,
    },
    gen_comments::{Comment, gen_comments},
    gen_diagnostic::gen_diagnostic_control,
    gen_function_call::gen_function_call_arguments,
    print_item_buffer::PrintItemBuffer,
    reporting::FormatDocumentResult,
};

use super::print_item_buffer::request_folder::RequestItem;

pub struct ParsedAttribute {
    attribute: Attribute,
    comments_after_attribute: Vec<Comment>,
}
pub struct ParsedAttributes {
    attributes: Vec<ParsedAttribute>,
}

pub fn parse_many_attributes(syntax: &mut SyntaxIter) -> FormatDocumentResult<ParsedAttributes> {
    // TODO(MonaMayrhofer) Think about whether this is the correct way to abstract this.
    // Maybe there should even a "many with comments" combinator, to also deduplicate code from fn parameters/struct members
    // Also this is very similar to parse_many_comments_and_blankspace
    let mut attributes = Vec::new();
    loop {
        let Some(item_attribute) = parse_node_optional::<Attribute>(syntax) else {
            break;
        };
        let item_comments_after_attribute = parse_many_comments_and_blankspace(syntax)?;

        attributes.push(ParsedAttribute {
            attribute: item_attribute,
            comments_after_attribute: item_comments_after_attribute,
        });
    }
    Ok(ParsedAttributes { attributes })
}

#[derive(Clone, Copy, Debug)]
pub enum AttributeLayout {
    Inline,
    Multiline,
}

pub fn gen_attributes(
    attributes: &ParsedAttributes,
    layout: AttributeLayout,
) -> FormatDocumentResult<PrintItemBuffer> {
    // If we don't have any attributes, we early exit to avoid all the bureaucracy with newlines
    if attributes.attributes.is_empty() {
        return Ok(PrintItemBuffer::new());
    }

    // ==== Sort and Group the Attributes ====
    let mut ungrouped_attributes = Vec::new();
    let mut attribute_group_diagnostics = Vec::new();
    let mut attribute_group_pre_fn_inlined = Vec::new();
    let mut attribute_group_offset_align_size = Vec::new();
    let mut attribute_group_binding_group = Vec::new();
    let mut attribute_group_compute_workgroup = Vec::new();

    for attribute in &attributes.attributes {
        match &attribute.attribute {
            Attribute::ConstantAttribute(_) => {
                attribute_group_pre_fn_inlined.push((0, attribute));
            },
            Attribute::DiagnosticAttribute(_) => {
                attribute_group_diagnostics.push((0, attribute));
            },
            Attribute::OtherAttribute(attrib) => {
                let name = attrib.name().map(|identifier| identifier.text().to_owned());
                let name = name.as_deref();
                match name {
                    Some("offset") => attribute_group_offset_align_size.push((0, attribute)),
                    Some("align") => attribute_group_offset_align_size.push((1, attribute)),
                    Some("size") => attribute_group_offset_align_size.push((2, attribute)),

                    Some("must_use") => attribute_group_pre_fn_inlined.push((1, attribute)),

                    Some("group") => attribute_group_binding_group.push((0, attribute)),
                    Some("binding") => attribute_group_binding_group.push((1, attribute)),

                    Some("compute") => attribute_group_compute_workgroup.push((0, attribute)),
                    Some("workgroup_size") => {
                        attribute_group_compute_workgroup.push((1, attribute));
                    },

                    Some(name) => ungrouped_attributes.push((name.to_owned(), attribute)),
                    None => ungrouped_attributes.push((String::new(), attribute)),
                }
            },
        }
    }

    fn gen_attribute_group<T: Ord>(
        mut attributes: Vec<(T, &ParsedAttribute)>,
        separator: RequestItem,
    ) -> FormatDocumentResult<PrintItemBuffer> {
        attributes.sort_by(|(order_a, _), (order_b, _)| order_a.cmp(order_b));

        let mut formatted = PrintItemBuffer::new();
        // Ungrouped attributes go first
        for ParsedAttribute {
            attribute,
            comments_after_attribute,
        } in attributes.iter().map(|(_, attribute)| attribute)
        {
            formatted.extend(gen_attribute(attribute)?);
            formatted.extend(gen_comments(comments_after_attribute));
            formatted.expect(separator);
        }
        Ok(formatted)
    }

    let group_separator = match layout {
        AttributeLayout::Inline => RequestItem::Space,
        AttributeLayout::Multiline => RequestItem::LineBreak,
    };

    let mut formatted = PrintItemBuffer::new();
    // Ungrouped attributes go first
    formatted.extend(gen_attribute_group(ungrouped_attributes, group_separator)?);
    formatted.extend(gen_attribute_group(
        attribute_group_binding_group,
        RequestItem::Space,
    )?);
    formatted.expect(group_separator);
    formatted.extend(gen_attribute_group(
        attribute_group_offset_align_size,
        RequestItem::Space,
    )?);
    formatted.expect(group_separator);
    formatted.extend(gen_attribute_group(
        attribute_group_compute_workgroup,
        RequestItem::Space,
    )?);
    formatted.expect(group_separator);
    formatted.extend(gen_attribute_group(
        attribute_group_diagnostics,
        RequestItem::Space,
    )?);
    formatted.expect(group_separator);
    formatted.extend(gen_attribute_group(
        attribute_group_pre_fn_inlined,
        RequestItem::Space,
    )?);
    // No final line break, these should be inline with the fn

    Ok(formatted)
}

pub fn gen_attribute(attribute: &Attribute) -> FormatDocumentResult<PrintItemBuffer> {
    match attribute {
        Attribute::ConstantAttribute(constant_attribute) => gen_const_attribute(constant_attribute),
        Attribute::DiagnosticAttribute(diagnostic_attribute) => {
            gen_diagnostic_attribute(diagnostic_attribute)
        },
        Attribute::OtherAttribute(other_attribute) => gen_other_attribute(other_attribute),
    }
}

//TODO(MonaMayrhofer) Collapse this attribute gen logic into something more generic
pub fn gen_diagnostic_attribute(
    attribute: &ast::DiagnosticAttribute
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = put_back(attribute.syntax().children_with_tokens());

    parse_token(&mut syntax, SyntaxKind::AttributeOperator)?;
    let item_comments_after_operator = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, parser::SyntaxKind::Diagnostic)?;
    let item_comments_after_identifier = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_control = parse_node::<DiagnosticControl>(&mut syntax)?;
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("@"));
    formatted.extend(gen_comments(&item_comments_after_operator));
    formatted.push_sc(sc!("diagnostic"));
    formatted.extend(gen_comments(&item_comments_after_identifier));
    formatted.extend(gen_diagnostic_control(&item_control)?);
    Ok(formatted)
}

pub fn gen_other_attribute(
    attribute: &ast::OtherAttribute
) -> FormatDocumentResult<PrintItemBuffer> {
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

pub fn gen_const_attribute(
    attribute: &ast::ConstantAttribute
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = put_back(attribute.syntax().children_with_tokens());

    parse_token(&mut syntax, SyntaxKind::AttributeOperator)?;
    let item_comments_after_operator = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, parser::SyntaxKind::Const)?;
    let item_comments_after_identifier = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_arguments = parse_node_optional::<Arguments>(&mut syntax);
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("@"));
    formatted.extend(gen_comments(&item_comments_after_operator));
    formatted.push_sc(sc!("const"));
    formatted.extend(gen_comments(&item_comments_after_identifier));
    if let Some(item_arguments) = item_arguments {
        formatted.extend(gen_function_call_arguments(&item_arguments)?);
    }
    Ok(formatted)
}
