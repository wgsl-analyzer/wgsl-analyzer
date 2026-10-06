use std::{collections::BTreeMap, string::String};

use dprint_core::formatting::PrintItems;
use dprint_core_macros::sc;
use itertools::{Itertools as _, Position};
use syntax::{
    AstNode as _, SyntaxKind,
    ast::{self, AttributeKind, AttributeList},
};

use crate::{
    ast_parse::{
        DiscardBlankspace, DiscardComma, DiscardParenthesis, NoTrivia, StopAtNewline, Succeeding,
        parse_end, parse_many_nodes_with, parse_node_with, syntax_iter,
    },
    generators::node::{
        gen_node_content, gen_node_preceding_trivia, gen_node_succeeding_trivia,
        gen_node_with_trivia,
    },
    multiline_group::MultilineGroup,
    options::TEMP_EXPERIMENTAL_CONDCOMP_MODE,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::{FormatDocumentError, FormatDocumentResult},
    trivia::{NodeTriviaItem, NodeWithTrivia},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AttributeLayout {
    Inline,
    Multiline,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
// The order of the variants determines the order of the attribute groups in the output
pub(crate) enum AttributeGroup {
    Conditional,
    Diagnostics,
    BlendSrc,
    Id,
    Interpolate,
    Invariant,
    OffsetAlignSize,
    BindingGroup,
    ComputeWorkgroup,
    Fragment,
    Vertex,
}
pub(crate) enum AttributeCategorization {
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
    for (pos, attribute) in attributes
        .iter()
        .map(|(_, attribute)| attribute)
        .with_position()
    {
        formatted.extend(gen_node_preceding_trivia(attribute)?);
        formatted.extend(gen_node_content(attribute)?);
        formatted.extend(gen_node_succeeding_trivia(attribute)?);
        if pos != Position::Only && pos != Position::Last {
            formatted.request(separator.clone());
        }
    }
    Ok(formatted)
}

pub(crate) fn categorize_attribute(attribute: &ast::Attribute) -> AttributeCategorization {
    use AttributeCategorization::{Grouped, Inline, Ungrouped};
    use ast::{ConditionalAttributeKind, EntrypointAttributeKind};

    match attribute.kind() {
        AttributeKind::Diagnostic => Grouped(AttributeGroup::Diagnostics, 0),
        AttributeKind::Size => Grouped(AttributeGroup::OffsetAlignSize, 2),
        AttributeKind::Align => Grouped(AttributeGroup::OffsetAlignSize, 1),
        AttributeKind::Group => Grouped(AttributeGroup::BindingGroup, 0),
        AttributeKind::Binding => Grouped(AttributeGroup::BindingGroup, 1),
        AttributeKind::Entrypoint(EntrypointAttributeKind::Compute) => {
            Grouped(AttributeGroup::ComputeWorkgroup, 0)
        },
        AttributeKind::SubgroupSize => Grouped(AttributeGroup::ComputeWorkgroup, 2),
        AttributeKind::WorkgroupSize => Grouped(AttributeGroup::ComputeWorkgroup, 1),
        AttributeKind::Entrypoint(EntrypointAttributeKind::Vertex) => {
            Grouped(AttributeGroup::Vertex, 0)
        },
        AttributeKind::Entrypoint(EntrypointAttributeKind::Fragment) => {
            Grouped(AttributeGroup::Fragment, 0)
        },
        AttributeKind::BlendSrc => Grouped(AttributeGroup::BlendSrc, 0),
        AttributeKind::Id => Grouped(AttributeGroup::Id, 0),
        AttributeKind::Interpolate => Grouped(AttributeGroup::Interpolate, 0),
        AttributeKind::Invariant => Grouped(AttributeGroup::Invariant, 0),
        AttributeKind::Location => Inline(3),
        AttributeKind::Builtin => Inline(2),
        AttributeKind::MustUse => Inline(1),
        AttributeKind::Const => Inline(0),
        AttributeKind::Conditional(ConditionalAttributeKind::If) => {
            Grouped(AttributeGroup::Conditional, 0)
        },
        AttributeKind::Conditional(ConditionalAttributeKind::Elif) => {
            Grouped(AttributeGroup::Conditional, 1)
        },
        AttributeKind::Conditional(ConditionalAttributeKind::Else) => {
            Grouped(AttributeGroup::Conditional, 2)
        },
        AttributeKind::Other => {
            let Some(name_token) = attribute.name() else {
                return Ungrouped(String::new());
            };
            Ungrouped(name_token.text().to_owned())
        },
    }
}

fn get_attribute_layout(attribute_list: &AttributeList) -> AttributeLayout {
    if let Some(parent) = attribute_list.syntax().parent()
        && matches!(
            parent.kind(),
            SyntaxKind::FunctionDeclaration | SyntaxKind::SwitchStatement | SyntaxKind::ReturnType
        )
    {
        AttributeLayout::Inline
    } else if let Some(sibling) = attribute_list.syntax().next_sibling()
        && matches!(
            sibling.kind(),
            SyntaxKind::CompoundStatement | SyntaxKind::GlobalCompoundDeclaration
        )
    {
        if let Some(last) = attribute_list.attributes().last()
            && last.is_conditional_compilation()
        {
            if TEMP_EXPERIMENTAL_CONDCOMP_MODE.condcomp_body_braces_on_same_line {
                AttributeLayout::Inline
            } else {
                AttributeLayout::Multiline
            }
        } else {
            AttributeLayout::Inline
        }
    } else {
        AttributeLayout::Multiline
    }
}

pub(crate) fn gen_attribute_list(
    attribute_list: &AttributeList
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = syntax_iter(attribute_list.syntax());

    let attributes = parse_many_nodes_with(&mut syntax, Succeeding(NoTrivia))
        .map(|mut node| {
            // We only preserve blankspaces if there is a comment as context.
            // otherwise we discard it.
            if node
                .preceding_trivia
                .iter()
                .all(|trivia| matches!(trivia, NodeTriviaItem::LineSpacing(_)))
            {
                node.preceding_trivia = vec![];
            }
            node
        })
        .filter(|node| !node.is_whitespace())
        .map(NodeWithTrivia::expect_ast_node_optional::<ast::Attribute>)
        .map(|item| {
            let item = item?;
            let content = item.content();
            let attribute = content
                .as_ref()
                .and_then(|node_or_token| node_or_token.clone().into_node())
                .and_then(ast::Attribute::cast)
                .ok_or(FormatDocumentError::UnexpectedNodeOrToken { received: content })?;
            Ok((item, attribute))
        })
        .collect::<Result<Vec<_>, _>>()?;

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

    for (attribute_item, attribute) in &attributes {
        match categorize_attribute(attribute) {
            AttributeCategorization::Ungrouped(order) => {
                ungrouped_attributes.push((order, attribute_item));
            },
            AttributeCategorization::Grouped(attribute_group, order) => grouped_attributes
                .entry(attribute_group)
                .or_default()
                .push((order, attribute_item)),
            AttributeCategorization::Inline(order) => {
                attribute_group_inlined_with_target.push((order, attribute_item));
            },
        }
    }

    let expect_space_or_linebreak = Request::expect(RequestItem::Space).or_newline();

    let group_separator = match get_attribute_layout(attribute_list) {
        AttributeLayout::Inline => expect_space_or_linebreak.clone(),
        AttributeLayout::Multiline => Request::expect(RequestItem::LineBreak),
    };

    let mut formatted = PrintItemBuffer::default();
    formatted.start_new_line_group_before_requests();

    // Ungrouped attributes go first
    if !ungrouped_attributes.is_empty() {
        formatted.extend(gen_attribute_group(ungrouped_attributes, &group_separator)?);
        formatted.request(group_separator.clone());
    }

    // The grouped attributes in-order.
    // They are ordered by the `AttributeGroup`'s discriminator because of the `BTreeMap`.
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

    formatted.finish_new_line_group_before_requests();

    Ok(formatted)
}

pub(crate) fn gen_attribute(attribute: &ast::Attribute) -> FormatDocumentResult<PrintItemBuffer> {
    if let AttributeKind::Conditional(_) = attribute.kind() {
        gen_attr_condcomp(attribute)
    } else {
        gen_attr_standard(attribute)
    }
}

pub(crate) fn gen_attribute_arguments(
    arguments: &ast::AttributeArguments
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = syntax_iter(arguments.syntax());
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::ParenthesisLeft)?;
    let item_arguments: Vec<_> = parse_many_nodes_with(
        &mut syntax,
        (
            Succeeding(StopAtNewline),
            DiscardBlankspace,
            DiscardComma,
            DiscardParenthesis,
        ),
    )
    .filter(|node| !node.is_whitespace())
    .map(|item| item.expect_ast_node_optional::<ast::Expression>())
    .try_collect()?;

    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    let mut multiline_group = MultilineGroup::new_before_requests(&mut formatted);
    multiline_group.push_sc(sc!("("));

    // If its blank we do not give the formatter the option to break within the ()
    if !item_arguments.is_empty() {
        multiline_group.start_indent_before_requests();
        multiline_group.grouped_possible_newline();
        multiline_group.request(Request::discourage(RequestItem::EmptyLine));
        multiline_group.request(Request::discourage(RequestItem::Space));

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
        multiline_group.finish_indent_before_requests();
    }

    multiline_group.push_sc(sc!(")"));
    multiline_group.end_before_requests();
    Ok(formatted)
}

/// Attributes of the form:
/// `'@' 'expected_token'`
/// and
/// `'@' 'expected_token' '(' expression (',' expression)* [','] ')'`.
pub(crate) fn gen_attr_standard(
    attribute: &ast::Attribute
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = syntax_iter(attribute.syntax());

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::AttributeOperator)?;
    let item_attribute_name = parse_node_with(&mut syntax, DiscardBlankspace)
        .expect_kind(syntax::SyntaxKind::Identifier)?;
    let item_arguments = parse_node_with(&mut syntax, DiscardBlankspace)
        .only_if_kind(SyntaxKind::AttributeArguments, &mut syntax);
    parse_end(&mut syntax)?;

    // ==== Format ====

    let mut formatted = PrintItemBuffer::default();
    formatted.push_sc(sc!("@"));
    formatted.extend(gen_node_with_trivia(&item_attribute_name)?);
    if let Some(item_arguments) = item_arguments {
        formatted.extend(gen_node_with_trivia(&item_arguments)?);
    }
    Ok(formatted)
}

fn gen_attr_condcomp(attribute: &ast::Attribute) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Context
    let merge_with_preceding_compound = if TEMP_EXPERIMENTAL_CONDCOMP_MODE
        .condcomp_body_braces_on_same_line
        && !matches!(
            attribute.kind(),
            AttributeKind::Conditional(ast::ConditionalAttributeKind::If)
        )
        && let Some(parent) = attribute.syntax().parent()
        && let Some(previous) = parent.prev_sibling()
        && matches!(
            previous.kind(),
            SyntaxKind::CompoundStatement | SyntaxKind::GlobalCompoundDeclaration
        ) {
        true
    } else {
        false
    };

    let dedent = if let Some(parent) = attribute.syntax().parent()
        && let Some(previous) = parent.next_sibling()
        && matches!(
            previous.kind(),
            SyntaxKind::CompoundStatement | SyntaxKind::GlobalCompoundDeclaration
        ) {
        TEMP_EXPERIMENTAL_CONDCOMP_MODE.dedent_condcomp_with_body
    } else {
        TEMP_EXPERIMENTAL_CONDCOMP_MODE.dedent_condcomp_without_body
    };

    // ==== Format

    let mut formatted = PrintItemBuffer::default();

    if merge_with_preceding_compound {
        formatted.request(Request::expect(RequestItem::Space));
        formatted.request(Request::discourage(RequestItem::LineBreak));
        formatted.request(Request::discourage(RequestItem::EmptyLine));
    }

    if dedent {
        formatted.start_ignoring_indent_before_requests();
    }
    formatted.extend(gen_attr_standard(attribute)?);

    if dedent {
        formatted.finish_ignoring_indent_before_requests();
    }

    Ok(formatted)
}
