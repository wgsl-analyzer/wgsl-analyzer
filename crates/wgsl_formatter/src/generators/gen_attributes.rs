use std::{collections::BTreeMap, string::String};

use dprint_core::formatting::StringContainer;
use dprint_core_macros::sc;
use itertools::put_back;
use parser::{SyntaxKind, SyntaxNode};
use syntax::{
    AstNode as _,
    ast::{
        self, Arguments, Attribute, BuiltinValueName, DiagnosticControl, InterpolateSamplingName,
        InterpolateTypeName,
    },
};

use crate::generators::{
    ast_parse::{
        SyntaxIter, parse_end, parse_node, parse_node_optional, parse_token, parse_token_any,
        parse_token_optional,
    },
    gen_comments::{Comment, gen_comments, parse_many_comments_and_blankspace},
    gen_diagnostic::gen_diagnostic_control,
    print_item_buffer::PrintItemBuffer,
    reporting::FormatDocumentResult,
    statements::function_call_statement::{
        gen_function_call_arguments, gen_function_call_like_comma_separated_values,
    },
};

use super::print_item_buffer::request_folder::RequestItem;

pub use standard_attributes::*;

#[derive(Debug)]
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    // The order of the variants determines the order of the attribute groups in the output
    enum AttributeGroup {
        Diagnostics,
        BlendSrc,
        Id,
        Interpolate,
        Invariant,
        Location,
        OffsetAlignSize,
        BindingGroup,
        ComputeWorkgroup,
        Fragment,
        Vertex,
    }
    enum AttributeCategorization {
        Ungrouped(String),
        Grouped(AttributeGroup, usize),
        Inline(usize),
    }

    // ==== Sort and Group the Attributes ====
    let mut ungrouped_attributes = Vec::new();
    let mut grouped_attributes = BTreeMap::<AttributeGroup, Vec<_>>::new();
    // Attributes that are inline with the target (like @const fn main()...)
    let mut attribute_group_inlined_with_target = Vec::new();

    for attribute in &attributes.attributes {
        use AttributeCategorization::{Grouped, Inline, Ungrouped};
        let cat = match &attribute.attribute {
            Attribute::DiagnosticAttribute(_) => Grouped(AttributeGroup::Diagnostics, 0),
            Attribute::SizeAttribute(_) => Grouped(AttributeGroup::OffsetAlignSize, 2),
            Attribute::AlignAttribute(_) => Grouped(AttributeGroup::OffsetAlignSize, 1),
            Attribute::GroupAttribute(_) => Grouped(AttributeGroup::BindingGroup, 0),
            Attribute::BindingAttribute(_) => Grouped(AttributeGroup::BindingGroup, 1),
            Attribute::ComputeAttribute(_) => Grouped(AttributeGroup::ComputeWorkgroup, 0),
            Attribute::WorkgroupSizeAttribute(_) => Grouped(AttributeGroup::ComputeWorkgroup, 1),
            Attribute::VertexAttribute(_) => Grouped(AttributeGroup::Vertex, 0),
            Attribute::FragmentAttribute(_) => Grouped(AttributeGroup::Fragment, 0),
            Attribute::BlendSrcAttribute(_) => Grouped(AttributeGroup::BlendSrc, 0),
            Attribute::IdAttribute(_) => Grouped(AttributeGroup::Id, 0),
            Attribute::InterpolateAttribute(_) => Grouped(AttributeGroup::Interpolate, 0),
            Attribute::InvariantAttribute(_) => Grouped(AttributeGroup::Invariant, 0),
            Attribute::LocationAttribute(_) => Grouped(AttributeGroup::Location, 0),

            Attribute::OtherAttribute(attrib) => {
                let name = attrib.name().map(|identifier| identifier.text().to_owned());
                let name = name.as_deref();
                match name {
                    Some("offset") => Grouped(AttributeGroup::OffsetAlignSize, 0),

                    Some(name) => Ungrouped(name.to_owned()),
                    //ungrouped_attributes.push((name.to_owned(), attribute)),
                    None => Ungrouped(String::new()),
                    //ungrouped_attributes.push((String::new(), attribute)),
                }
            },
            Attribute::BuiltinAttribute(_) => Inline(2),
            Attribute::MustUseAttribute(_) => Inline(1),
            Attribute::ConstantAttribute(_) => Inline(0),
        };

        match cat {
            Ungrouped(order) => ungrouped_attributes.push((order, attribute)),
            Grouped(attribute_group, order) => grouped_attributes
                .entry(attribute_group)
                .or_default()
                .push((order, attribute)),
            Inline(order) => attribute_group_inlined_with_target.push((order, attribute)),
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

    // The grouped attributes in order
    // (They are ordered by the AttributeGroup enum's discriminator, because of the BTreeMap)
    for (_, attribute) in grouped_attributes {
        formatted.extend(gen_attribute_group(attribute, RequestItem::Space)?);
        formatted.expect(group_separator);
    }
    // Then attributes that should be inline with the target
    formatted.extend(gen_attribute_group(
        attribute_group_inlined_with_target,
        RequestItem::Space,
    )?);
    // No final line break, these should be inline with the target

    Ok(formatted)
}

pub fn gen_attribute(attribute: &Attribute) -> FormatDocumentResult<PrintItemBuffer> {
    use Attribute::{
        AlignAttribute, BindingAttribute, BlendSrcAttribute, BuiltinAttribute, ComputeAttribute,
        ConstantAttribute, DiagnosticAttribute, FragmentAttribute, GroupAttribute, IdAttribute,
        InterpolateAttribute, InvariantAttribute, LocationAttribute, MustUseAttribute,
        OtherAttribute, SizeAttribute, VertexAttribute, WorkgroupSizeAttribute,
    };
    match attribute {
        OtherAttribute(other_attribute) => gen_other_attribute(other_attribute),
        // === Standard Attributes ===
        ConstantAttribute(constant_attribute) => gen_const_attribute(constant_attribute),
        DiagnosticAttribute(diagnostic_attribute) => gen_diagnostic_attribute(diagnostic_attribute),
        AlignAttribute(align_attribute) => gen_align_attribute(align_attribute),
        BindingAttribute(binding_attribute) => gen_binding_attribute(binding_attribute),
        BlendSrcAttribute(blend_src_attribute) => gen_blend_src_attribute(blend_src_attribute),
        BuiltinAttribute(builtin_attribute) => gen_builtin_attribute(builtin_attribute),
        GroupAttribute(group_attribute) => gen_group_attribute(group_attribute),
        IdAttribute(id_attribute) => gen_id_attribute(id_attribute),
        InterpolateAttribute(interpolate_attribute) => {
            gen_interpolate_attribute(interpolate_attribute)
        },
        InvariantAttribute(invariant_attribute) => gen_invariant_attribute(invariant_attribute),
        LocationAttribute(location_attribute) => gen_location_attribute(location_attribute),
        MustUseAttribute(must_use_attribute) => gen_must_use_attribute(must_use_attribute),
        SizeAttribute(size_attribute) => gen_size_attribute(size_attribute),
        WorkgroupSizeAttribute(workgroup_size_attribute) => {
            gen_workgroup_size_attribute(workgroup_size_attribute)
        },
        VertexAttribute(vertex_attribute) => gen_vertex_attribute(vertex_attribute),
        FragmentAttribute(fragment_attribute) => gen_fragment_attribute(fragment_attribute),
        ComputeAttribute(compute_attribute) => gen_compute_attribute(compute_attribute),
    }
}

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

pub fn gen_interpolate_type_name(
    attribute: &ast::InterpolateTypeName
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = put_back(attribute.syntax().children_with_tokens());
    let content = parse_token_any(&mut syntax)?;
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::new();
    formatted.push_string(content.text().to_owned());
    Ok(formatted)
}
pub fn gen_interpolate_sampling_name(
    attribute: &ast::InterpolateSamplingName
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = put_back(attribute.syntax().children_with_tokens());
    let content = parse_token_any(&mut syntax)?;
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::new();
    formatted.push_string(content.text().to_owned());
    Ok(formatted)
}
pub fn gen_interpolate_attribute(
    attribute: &ast::InterpolateAttribute
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = put_back(attribute.syntax().children_with_tokens());

    parse_token(&mut syntax, SyntaxKind::AttributeOperator)?;
    let item_comments_after_operator = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, parser::SyntaxKind::Interpolate)?;
    let item_comments_after_identifier = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, parser::SyntaxKind::ParenthesisLeft)?;
    let item_comments_after_open_paren = parse_many_comments_and_blankspace(&mut syntax)?;
    let interpolate_type_name = parse_node::<InterpolateTypeName>(&mut syntax)?;
    let item_comments_after_itn = parse_many_comments_and_blankspace(&mut syntax)?;

    // TODO(MonaMayrhofer) i am not really proud of this code, there must be a cleaner way
    let sampling = if parse_token_optional(&mut syntax, SyntaxKind::Comma).is_some() {
        let item_comments_after_comma = parse_many_comments_and_blankspace(&mut syntax)?;
        let interpolate_sampling_name = parse_node::<InterpolateSamplingName>(&mut syntax)?;
        let item_comments_after_isn = parse_many_comments_and_blankspace(&mut syntax)?;
        Some((
            item_comments_after_comma,
            interpolate_sampling_name,
            item_comments_after_isn,
        ))
    } else {
        None
    };
    parse_token_optional(&mut syntax, SyntaxKind::Comma);
    parse_token(&mut syntax, parser::SyntaxKind::ParenthesisRight)?;
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("@"));
    formatted.extend(gen_comments(&item_comments_after_operator));
    formatted.push_sc(sc!("interpolate"));
    formatted.extend(gen_comments(&item_comments_after_identifier));
    formatted.push_sc(sc!("("));
    formatted.extend(gen_comments(&item_comments_after_open_paren));
    formatted.extend(gen_interpolate_type_name(&interpolate_type_name)?);
    formatted.extend(gen_comments(&item_comments_after_itn));
    if let Some((item_comments_after_comma, interpolate_sampling_name, item_comments_after_isn)) =
        sampling
    {
        formatted.push_sc(sc!(","));
        formatted.extend(gen_comments(&item_comments_after_comma));
        formatted.extend(gen_interpolate_sampling_name(&interpolate_sampling_name)?);
        formatted.extend(gen_comments(&item_comments_after_isn));
    }

    formatted.push_sc(sc!(")"));
    Ok(formatted)
}

pub fn gen_builtin_value_name(
    attribute: &ast::BuiltinValueName
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = put_back(attribute.syntax().children_with_tokens());
    let content = parse_token_any(&mut syntax)?;
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::new();
    formatted.push_string(content.text().to_owned());
    Ok(formatted)
}
pub fn gen_builtin_attribute(
    attribute: &ast::BuiltinAttribute
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = put_back(attribute.syntax().children_with_tokens());

    parse_token(&mut syntax, SyntaxKind::AttributeOperator)?;
    let item_comments_after_operator = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, parser::SyntaxKind::Builtin)?;
    let item_comments_after_identifier = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, parser::SyntaxKind::ParenthesisLeft)?;
    let item_comments_after_open_paren = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_builtin_value_name = parse_node::<BuiltinValueName>(&mut syntax)?;
    let item_comments_after_itn = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token_optional(&mut syntax, SyntaxKind::Comma);
    parse_token(&mut syntax, parser::SyntaxKind::ParenthesisRight)?;
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("@"));
    formatted.extend(gen_comments(&item_comments_after_operator));
    formatted.push_sc(sc!("builtin"));
    formatted.extend(gen_comments(&item_comments_after_identifier));
    formatted.push_sc(sc!("("));
    formatted.extend(gen_comments(&item_comments_after_open_paren));
    formatted.extend(gen_builtin_value_name(&item_builtin_value_name)?);
    formatted.extend(gen_comments(&item_comments_after_itn));
    formatted.push_sc(sc!(")"));
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
#[rustfmt::skip]
mod standard_attributes {
    use super::gen_attr_standard_with_args;
    use dprint_core_macros::sc;
    use parser::SyntaxKind;
    use syntax::{AstNode as _, ast};

    use crate::generators::{print_item_buffer::PrintItemBuffer, reporting::FormatDocumentResult};


    pub fn gen_align_attribute(attribute: &ast::AlignAttribute) -> FormatDocumentResult<PrintItemBuffer>                   { gen_attr_standard_with_args(attribute.syntax(), SyntaxKind::Align, sc!("align")) }
    pub fn gen_const_attribute(attribute: &ast::ConstantAttribute ) -> FormatDocumentResult<PrintItemBuffer>               { gen_attr_standard_with_args(attribute.syntax(), SyntaxKind::Const, sc!("const")) }
    pub fn gen_binding_attribute(attribute: &ast::BindingAttribute ) -> FormatDocumentResult<PrintItemBuffer>              { gen_attr_standard_with_args(attribute.syntax(), SyntaxKind::Binding, sc!("binding")) }
    pub fn gen_blend_src_attribute(attribute: &ast::BlendSrcAttribute ) -> FormatDocumentResult<PrintItemBuffer>           { gen_attr_standard_with_args(attribute.syntax(), SyntaxKind::BlendSrc, sc!("blend_src")) }
    pub fn gen_group_attribute(attribute: &ast::GroupAttribute ) -> FormatDocumentResult<PrintItemBuffer>                  { gen_attr_standard_with_args(attribute.syntax(), SyntaxKind::Group, sc!("group")) }
    pub fn gen_id_attribute(attribute: &ast::IdAttribute) -> FormatDocumentResult<PrintItemBuffer>                         { gen_attr_standard_with_args(attribute.syntax(), SyntaxKind::Id, sc!("id")) }
    pub fn gen_invariant_attribute(attribute: &ast::InvariantAttribute ) -> FormatDocumentResult<PrintItemBuffer>          { gen_attr_standard_with_args(attribute.syntax(), SyntaxKind::Invariant, sc!("invariant")) }
    pub fn gen_location_attribute(attribute: &ast::LocationAttribute ) -> FormatDocumentResult<PrintItemBuffer>            { gen_attr_standard_with_args(attribute.syntax(), SyntaxKind::Location, sc!("location")) }
    pub fn gen_must_use_attribute(attribute: &ast::MustUseAttribute ) -> FormatDocumentResult<PrintItemBuffer>             { gen_attr_standard_with_args(attribute.syntax(), SyntaxKind::MustUse, sc!("must_use")) }
    pub fn gen_size_attribute(attribute: &ast::SizeAttribute ) -> FormatDocumentResult<PrintItemBuffer>                    { gen_attr_standard_with_args(attribute.syntax(), SyntaxKind::Size, sc!("size")) }
    pub fn gen_workgroup_size_attribute(attribute: &ast::WorkgroupSizeAttribute ) -> FormatDocumentResult<PrintItemBuffer> { gen_attr_standard_with_args(attribute.syntax(), SyntaxKind::WorkgroupSize, sc!("workgroup_size"), ) }
    pub fn gen_vertex_attribute(attribute: &ast::VertexAttribute ) -> FormatDocumentResult<PrintItemBuffer>                { gen_attr_standard_with_args(attribute.syntax(), SyntaxKind::Vertex, sc!("vertex")) }
    pub fn gen_fragment_attribute(attribute: &ast::FragmentAttribute ) -> FormatDocumentResult<PrintItemBuffer>            { gen_attr_standard_with_args(attribute.syntax(), SyntaxKind::Fragment, sc!("fragment")) }
    pub fn gen_compute_attribute(attribute: &ast::ComputeAttribute ) -> FormatDocumentResult<PrintItemBuffer>              { gen_attr_standard_with_args(attribute.syntax(), SyntaxKind::Compute, sc!("compute")) }
}

/// Attributes of the form:
/// `'expected_token' '(' expression [','] ')'`.
fn gen_attr_standard_with_args(
    syntax: &SyntaxNode,
    expected_token: SyntaxKind,
    attribute_name: &'static StringContainer,
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = put_back(syntax.children_with_tokens());

    parse_token(&mut syntax, SyntaxKind::AttributeOperator)?;
    let item_comments_after_operator = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, expected_token)?;
    let item_comments_after_identifier = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_arguments = gen_function_call_like_comma_separated_values(&mut syntax)?;
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("@"));
    formatted.extend(gen_comments(&item_comments_after_operator));
    formatted.push_sc(attribute_name);
    formatted.extend(gen_comments(&item_comments_after_identifier));
    formatted.extend(item_arguments);
    Ok(formatted)
}
