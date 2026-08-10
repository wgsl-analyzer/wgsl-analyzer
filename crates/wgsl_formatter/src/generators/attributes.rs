use std::{collections::BTreeMap, string::String};

use dprint_core::formatting::{PrintItem, PrintItems, StringContainer};
use dprint_core_macros::sc;
use itertools::{Itertools as _, Position};
use parser::{SyntaxKind, SyntaxNode};
use syntax::{
    AstNode as _,
    ast::{self, Attribute, AttributeList},
};

use crate::{
    ast_parse::{
        Chain, Filter, FilterAction, IgnoreBlankspace, IgnoreComma, NoTrivia,
        UntilSucceedingNewline, parse_end, parse_node_with, syntax_iter,
    },
    generators::node::{
        gen_node_content, gen_node_preceding_trivia, gen_node_succeeding_trivia,
        gen_node_with_trivia,
    },
    helpers::{LineSpacing, read_blankspace},
    multiline_group::MultilineGroup,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
    trivia::{NodeWithTrivia, NodeWithTriviaContent},
};

pub use standard_attributes::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttributeLayout {
    Inline,
    Multiline,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
// The order of the variants determines the order of the attribute groups in the output
enum AttributeGroup {
    Conditional,
    Diagnostics,
    BlendSrc,
    Id,
    Interpolate,
    Invariant,
    Location,
    OffsetAlignSize,
    BindingGroup,
    EarlyDepthTest,
    ComputeWorkgroup,
    Fragment,
    Vertex,
}
enum AttributeCategorization {
    Ungrouped(String),
    Grouped(AttributeGroup, usize),
    Inline(usize),
}

fn gen_attribute_group<T>(
    mut attributes: Vec<(T, &NodeWithTrivia)>,
    separator: &Request,
) -> FormatDocumentResult<PrintItemBuffer>
where
    T: Ord,
{
    attributes.sort_by(|(order_a, _), (order_b, _)| order_a.cmp(order_b));

    let mut formatted = PrintItemBuffer::default();
    // Ungrouped attributes go first
    for (pos, attribute) in attributes
        .iter()
        .map(|(_, attribute)| attribute)
        .with_position()
    {
        formatted.finish_new_line_group();
        formatted.extend(gen_node_preceding_trivia(attribute)?);
        formatted.extend(gen_node_content(attribute)?);
        formatted.start_new_line_group();
        formatted.extend(gen_node_succeeding_trivia(attribute)?);
        if pos != Position::Only && pos != Position::Last {
            formatted.request(separator.clone());
        }
    }
    Ok(formatted)
}

pub fn gen_attribute_list(attribute_list: &AttributeList) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = syntax_iter(attribute_list.syntax());
    let mut attributes = Vec::new();
    loop {
        let item_attribute = parse_node_with(&mut syntax, IgnoreBlankspace)
            .expect_ast_node_optional::<Attribute>()?;

        let is_end = item_attribute.is_end();
        if !item_attribute.is_whitespace() {
            attributes.push(item_attribute);
        }
        if is_end {
            break;
        }
    }
    parse_end(&mut syntax)?;

    // If we don't have any attributes, we early exit to avoid all the bureaucracy with newlines
    if attributes.is_empty() {
        return Ok(PrintItemBuffer::default());
    }

    // ==== Sort and Group the Attributes ====
    let mut ungrouped_attributes = Vec::new();
    let mut grouped_attributes = BTreeMap::<AttributeGroup, Vec<_>>::new();
    // Attributes that are inline with the target (like @const fn main()...)
    let mut attribute_group_inlined_with_target = Vec::new();

    for attribute_item in &attributes {
        use AttributeCategorization::{Grouped, Inline, Ungrouped};
        let attribute =
            Attribute::cast(attribute_item.content().unwrap().into_node().unwrap()).unwrap(); //TODO
        let cat = match &attribute {
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
            Attribute::IfAttribute(_) => Grouped(AttributeGroup::Conditional, 0),
            Attribute::ElifAttribute(_) => Grouped(AttributeGroup::Conditional, 1),
            Attribute::ElseAttribute(_) => Grouped(AttributeGroup::Conditional, 2),
            Attribute::EarlyDepthTestAttribute(_) => Grouped(AttributeGroup::EarlyDepthTest, 0),
        };

        match cat {
            Ungrouped(order) => ungrouped_attributes.push((order, attribute_item)),
            Grouped(attribute_group, order) => grouped_attributes
                .entry(attribute_group)
                .or_default()
                .push((order, attribute_item)),
            Inline(order) => attribute_group_inlined_with_target.push((order, attribute_item)),
        }
    }

    let expect_space_or_linebreak = Request::expect(RequestItem::Space).or_newline();

    let layout = if let Some(parent) = attribute_list.syntax().parent() {
        if matches!(
            parent.kind(),
            SyntaxKind::FunctionDeclaration | SyntaxKind::SwitchStatement | SyntaxKind::ReturnType
        ) {
            AttributeLayout::Inline
        } else {
            AttributeLayout::Multiline
        }
    } else {
        AttributeLayout::Multiline
    };

    let group_separator = match layout {
        AttributeLayout::Inline => expect_space_or_linebreak.clone(),
        AttributeLayout::Multiline => Request::expect(RequestItem::LineBreak),
    };

    let mut formatted = PrintItemBuffer::default();
    formatted.start_new_line_group();

    // Ungrouped attributes go first
    if !ungrouped_attributes.is_empty() {
        formatted.extend(gen_attribute_group(ungrouped_attributes, &group_separator)?);
        formatted.request(group_separator.clone());
    }

    // The grouped attributes in order
    // (They are ordered by the AttributeGroup enum's discriminator, because of the BTreeMap)
    for (_, attribute) in grouped_attributes {
        formatted.extend(gen_attribute_group(attribute, &expect_space_or_linebreak)?);
        formatted.request(group_separator.clone());
    }
    // Then attributes that should be inline with the target
    if !attribute_group_inlined_with_target.is_empty() {
        formatted.extend(gen_attribute_group(
            attribute_group_inlined_with_target,
            &expect_space_or_linebreak,
        )?);
        formatted.request(expect_space_or_linebreak);
    }

    // No final line break, these should be inline with the target
    formatted.finish_new_line_group();

    // We can discourage NewLines and Emptylines because finish_new_line_group
    // applies all the stuff beforehand already
    formatted.request(Request::discourage(RequestItem::LineBreak));
    formatted.request(Request::discourage(RequestItem::EmptyLine));
    formatted.request(Request::discourage(RequestItem::Space));

    Ok(formatted)
}

pub fn gen_attribute(attribute: &Attribute) -> FormatDocumentResult<PrintItemBuffer> {
    use Attribute::{
        AlignAttribute, BindingAttribute, BlendSrcAttribute, BuiltinAttribute, ComputeAttribute,
        ConstantAttribute, DiagnosticAttribute, EarlyDepthTestAttribute, ElifAttribute,
        ElseAttribute, FragmentAttribute, GroupAttribute, IdAttribute, IfAttribute,
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
        IfAttribute(if_attribute) => gen_if_attribute(if_attribute),
        ElifAttribute(elif_attribute) => gen_elif_attribute(elif_attribute),
        ElseAttribute(else_attribute) => gen_else_attribute(else_attribute),
        EarlyDepthTestAttribute(early_depth_test_attribute) => {
            gen_early_depth_test_attribute(early_depth_test_attribute)
        },
    }
}

pub fn gen_diagnostic_attribute(
    attribute: &ast::DiagnosticAttribute
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = syntax_iter(attribute.syntax());

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::AttributeOperator)?;
    let item_diagnostic = parse_node_with(&mut syntax, IgnoreBlankspace)
        .expect_kind(parser::SyntaxKind::Diagnostic)?;
    let item_control = parse_node_with(&mut syntax, IgnoreBlankspace)
        .expect_kind(SyntaxKind::DiagnosticControl)?;
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::default();
    formatted.push_sc(sc!("@"));
    formatted.extend(gen_node_with_trivia(&item_diagnostic)?);
    formatted.extend(gen_node_with_trivia(&item_control)?);
    Ok(formatted)
}

pub fn gen_interpolate_type_name(
    attribute: &ast::InterpolateTypeName
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = syntax_iter(attribute.syntax());
    let content = parse_node_with(&mut syntax, IgnoreBlankspace); // TODO It would be great to expect_kind here
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_node_with_trivia(&content)?);
    Ok(formatted)
}

pub fn gen_early_depth_test_mode(attribute: &SyntaxNode) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = syntax_iter(attribute.syntax());
    let content = parse_node_with(&mut syntax, IgnoreBlankspace); // TODO It would be great to expect_kind here
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_node_with_trivia(&content)?);
    Ok(formatted)
}

pub fn gen_interpolate_sampling_name(
    attribute: &ast::InterpolateSamplingName
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = syntax_iter(attribute.syntax());
    let content = parse_node_with(&mut syntax, IgnoreBlankspace); // TODO It would be great to expect_kind here
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_node_with_trivia(&content)?);
    Ok(formatted)
}
pub fn gen_interpolate_attribute(
    attribute: &ast::InterpolateAttribute
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = syntax_iter(attribute.syntax());

    let item_attr_operator =
        parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::AttributeOperator)?;
    let item_interpolate = parse_node_with(&mut syntax, IgnoreBlankspace)
        .expect_kind(parser::SyntaxKind::Interpolate)?;
    let item_paren_left =
        parse_node_with(&mut syntax, NoTrivia).expect_kind(parser::SyntaxKind::ParenthesisLeft)?;
    let interpolate_type_name = parse_node_with(&mut syntax, IgnoreBlankspace)
        .expect_kind(SyntaxKind::InterpolateTypeName)?;

    let item_comma =
        parse_node_with(&mut syntax, NoTrivia).only_if_kind(SyntaxKind::Comma, &mut syntax);
    let sampling = if item_comma.is_some() {
        let interpolate_sampling_name = parse_node_with(&mut syntax, IgnoreBlankspace)
            .expect_kind(SyntaxKind::InterpolateSamplingName)?;
        Some(interpolate_sampling_name)
    } else {
        None
    };
    parse_node_with(&mut syntax, NoTrivia).only_if_kind(SyntaxKind::Comma, &mut syntax);
    parse_node_with(&mut syntax, NoTrivia).expect_kind(parser::SyntaxKind::ParenthesisRight)?;
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_node_with_trivia(&item_attr_operator)?);
    formatted.extend(gen_node_with_trivia(&item_interpolate)?);
    formatted.extend(gen_node_with_trivia(&item_paren_left)?);
    formatted.extend(gen_node_with_trivia(&interpolate_type_name)?);
    if let Some(sampling) = sampling {
        formatted.push_sc(sc!(","));
        formatted.extend(gen_node_with_trivia(&sampling)?);
    }

    formatted.push_sc(sc!(")"));
    Ok(formatted)
}

pub fn gen_builtin_value_name(
    attribute: &ast::BuiltinValueName
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = syntax_iter(attribute.syntax());
    let content = parse_node_with(&mut syntax, IgnoreBlankspace); // TODO It would be great to expect_kind here
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_node_with_trivia(&content)?);
    Ok(formatted)
}
pub fn gen_builtin_attribute(
    attribute: &ast::BuiltinAttribute
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = syntax_iter(attribute.syntax());

    let item_attr_operator =
        parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::AttributeOperator)?;
    let item_builtin =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(parser::SyntaxKind::Builtin)?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind(parser::SyntaxKind::ParenthesisLeft)?;
    let item_builtin_value_name =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(SyntaxKind::BuiltinValueName)?;
    parse_node_with(&mut syntax, NoTrivia).only_if_kind(SyntaxKind::Comma, &mut syntax);
    parse_node_with(&mut syntax, NoTrivia).expect_kind(parser::SyntaxKind::ParenthesisRight)?;
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_node_with_trivia(&item_attr_operator)?);
    formatted.extend(gen_node_with_trivia(&item_builtin)?);
    formatted.push_sc(sc!("("));
    formatted.extend(gen_node_with_trivia(&item_builtin_value_name)?);
    formatted.push_sc(sc!(")"));
    Ok(formatted)
}

pub fn gen_other_attribute(
    attribute: &ast::OtherAttribute
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = syntax_iter(attribute.syntax());

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::AttributeOperator)?;
    let item_identifier = parse_node_with(&mut syntax, IgnoreBlankspace)
        .expect_kind(parser::SyntaxKind::Identifier)?;
    let item_arguments = parse_node_with(&mut syntax, IgnoreBlankspace)
        .only_if_kind(SyntaxKind::Arguments, &mut syntax);
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::default();
    formatted.push_sc(sc!("@"));
    formatted.extend(gen_node_with_trivia(&item_identifier)?);
    if let Some(item_arguments) = item_arguments {
        formatted.extend(gen_node_with_trivia(&item_arguments)?);
    }
    Ok(formatted)
}
#[rustfmt::skip]
#[expect(clippy::inline_modules, reason = "Its much neater this way, simply grouping them together.")]
mod standard_attributes {
    use super::gen_attr_standard_with_args;
    use dprint_core_macros::sc;
    use parser::{SyntaxKind};
    use syntax::{AstNode as _, ast};

    use crate::{print_item_buffer::PrintItemBuffer, reporting::FormatDocumentResult};


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

    // Naga
    pub fn gen_early_depth_test_attribute(attribute: &ast::EarlyDepthTestAttribute ) -> FormatDocumentResult<PrintItemBuffer> { gen_attr_standard_with_args(attribute.syntax(), SyntaxKind::EarlyDepthTest, sc!("early_depth_test")) }

    // WESL
    pub fn gen_if_attribute(attribute: &ast::IfAttribute ) -> FormatDocumentResult<PrintItemBuffer>                        { gen_attr_standard_with_args(attribute.syntax(), SyntaxKind::If, sc!("if")) }
    pub fn gen_elif_attribute(attribute: &ast::ElifAttribute ) -> FormatDocumentResult<PrintItemBuffer>                    { gen_attr_standard_with_args(attribute.syntax(), SyntaxKind::Elif, sc!("elif")) }
    pub fn gen_else_attribute(attribute: &ast::ElseAttribute ) -> FormatDocumentResult<PrintItemBuffer>                    { gen_attr_standard_with_args(attribute.syntax(), SyntaxKind::Else, sc!("else")) }
}

/// Attributes of the form:
/// `'expected_token' '(' expression [','] ')'`.
fn gen_attr_standard_with_args(
    syntax: &SyntaxNode,
    expected_token: SyntaxKind,
    attribute_name: &'static StringContainer,
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = syntax_iter(syntax);

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::AttributeOperator)?;
    let item_attribute_name =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(expected_token)?;

    let item_paren_left = parse_node_with(&mut syntax, NoTrivia)
        .only_if_kind(SyntaxKind::ParenthesisLeft, &mut syntax);
    let item_arguments = if item_paren_left.is_some() {
        let mut item_arguments = Vec::new();

        loop {
            let mut item = parse_node_with(
                &mut syntax,
                Chain(UntilSucceedingNewline, Chain(IgnoreBlankspace, IgnoreComma)),
            );

            // TODO This needs to be absorbed into parse_node..
            if matches!(item.kind(), Some(SyntaxKind::ParenthesisRight)) {
                let old_node = std::mem::replace(&mut item.node, NodeWithTriviaContent::End);
                syntax.put_back(old_node.into_option().unwrap()); //TODO
            }

            let is_end = item.is_end();
            if !item.is_whitespace() {
                item_arguments.push(item);
            }
            if is_end {
                break;
            }
        }

        parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::ParenthesisRight)?;
        Some(item_arguments)
    } else {
        None
    };

    parse_end(&mut syntax)?;

    // ==== Formatting

    let mut formatted = PrintItemBuffer::default();
    formatted.push_sc(sc!("@"));
    formatted.extend(gen_node_with_trivia(&item_attribute_name)?);
    if let Some(item_arguments) = item_arguments {
        let mut multiline_group = MultilineGroup::new(&mut formatted);
        multiline_group.push_sc(sc!("("));

        // If its blank we do not give the formatter the option to break within the ()
        if !item_arguments.is_empty() {
            multiline_group.start_indent();

            for (position, item) in item_arguments.into_iter().with_position() {
                multiline_group.grouped_newline_or_space();
                multiline_group.extend(gen_node_preceding_trivia(&item)?);
                if item.has_content() {
                    multiline_group.extend(gen_node_content(&item)?);
                    multiline_group.request(Request::discourage(RequestItem::Space));
                    if position == Position::Last || position == Position::Only {
                        multiline_group.extend_if_multi_line({
                            let mut pi = PrintItems::default();
                            pi.push_sc(sc!(","));
                            pi
                        });
                    } else {
                        multiline_group.push_sc(sc!(","));
                    }
                }
                multiline_group.extend(gen_node_succeeding_trivia(&item)?);
            }

            multiline_group.request(Request::discourage(RequestItem::Space));
            multiline_group.grouped_possible_newline();
            multiline_group.finish_indent();
        }

        multiline_group.push_sc(sc!(")"));
        multiline_group.end();
    }
    Ok(formatted)
}
