use dprint_core::formatting::PrintItems;
use dprint_core_macros::sc;
use itertools::{Itertools as _, Position};
use parser::SyntaxKind;
use syntax::{AstNode as _, ast};

use crate::{
    ast_parse::{
        IgnoreBlankspace, IgnoreComma, IgnoreTemplateDelimiters, NoTrivia, Succeeding,
        UntilNewline, parse_end, parse_many_nodes_with, parse_node_with, syntax_iter,
    },
    generators::node::{
        gen_node_content, gen_node_preceding_trivia, gen_node_succeeding_trivia,
        gen_node_with_trivia,
    },
    multiline_group::MultilineGroup,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

pub fn gen_type_specifier(
    type_specifier: &ast::TypeSpecifier
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = syntax_iter(type_specifier.syntax());

    let item_path = parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(SyntaxKind::Path)?;

    let item_template = parse_node_with(&mut syntax, IgnoreBlankspace)
        .only_if_kind(SyntaxKind::TemplateList, &mut syntax);

    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_node_with_trivia(&item_path)?);
    if let Some(template) = item_template {
        formatted.extend(gen_node_with_trivia(&template)?);
    }
    Ok(formatted)
}

pub fn gen_template_list(
    template_list: &ast::TemplateList
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = syntax_iter(template_list.syntax());
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::TemplateStart)?;

    let item_arguments = parse_many_nodes_with(
        &mut syntax,
        (
            Succeeding(UntilNewline),
            IgnoreBlankspace,
            IgnoreComma,
            IgnoreTemplateDelimiters,
        ),
    )
    .filter(|node| !node.is_whitespace())
    .map(|node| node.expect_ast_node_optional::<ast::Expression>())
    .collect::<Result<Vec<_>, _>>()?;

    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    let mut multiline_group = MultilineGroup::new_before_requests(&mut formatted);
    multiline_group.push_sc(sc!("<"));
    multiline_group.request(Request::discourage(RequestItem::EmptyLine));
    multiline_group.request(Request::discourage(RequestItem::Space));

    // If its blank we do not give the formatter the option to break within the <>
    if !item_arguments.is_empty() {
        multiline_group.start_indent_before_requests();

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
        multiline_group.finish_indent();
        multiline_group.grouped_possible_newline();
    }
    multiline_group.push_sc(sc!(">"));
    multiline_group.end_before_requests();

    Ok(formatted)
}
