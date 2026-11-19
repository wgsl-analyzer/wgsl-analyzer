use dprint_core::formatting::{Arena, PrintItems};
use dprint_core_macros::sc;
use itertools::Itertools as _;
use syntax::{AstNode as _, SyntaxKind, ast};

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
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

pub(crate) fn gen_function_declaration<'buffer>(
    arena: &'buffer Arena,
    node: &ast::FunctionDeclaration,
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    let mut syntax = syntax_iter(node.syntax());

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Fn)?;
    let item_name =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_kind(SyntaxKind::Name)?;
    let item_params = parse_node_with(&mut syntax, DiscardBlankspace)
        .expect_kind_optional(SyntaxKind::FunctionParameters)?;

    let item_return = parse_node_with(&mut syntax, DiscardBlankspace)
        .only_if_kind(SyntaxKind::ReturnType, &mut syntax);
    let item_body = parse_node_with(&mut syntax, DiscardBlankspace)
        .expect_kind_optional(SyntaxKind::CompoundStatement)?;

    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::new(arena);

    // Fn
    formatted.push_sc(arena, sc!("fn"));
    formatted.request(Request::expect(RequestItem::Space));

    // Name
    formatted.extend(arena, gen_node_with_trivia(arena, &item_name)?);

    // Params
    formatted.extend(arena, gen_node_with_trivia(arena, &item_params)?);

    // Return
    if let Some(item_return) = item_return {
        // We would much rather break the params than the return type
        formatted.start_new_line_group_before_requests(arena);
        formatted.start_new_line_group_before_requests(arena);
        formatted.extend(arena, gen_node_with_trivia(arena, &item_return)?);
        formatted.finish_new_line_group_after_requests(arena);
        formatted.finish_new_line_group_after_requests(arena);
    }

    // Body
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(arena, gen_node_with_trivia(arena, &item_body)?);

    Ok(formatted)
}

pub(crate) fn gen_fn_parameters<'buffer>(
    arena: &'buffer Arena,
    node: &ast::FunctionParameters,
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    // ==== Parse ====

    let mut syntax = syntax_iter(node.syntax());

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::ParenthesisLeft)?;

    // This
    let items = parse_many_nodes_with(
        &mut syntax,
        (
            Succeeding(StopAtNewline),
            DiscardBlankspace,
            DiscardComma,
            DiscardParenthesis,
        ),
    )
    .filter(|node| !node.is_whitespace())
    .collect::<Vec<_>>();

    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new(arena);

    let mut multiline_group = MultilineGroup::new_before_requests(arena, &mut formatted);

    multiline_group.push_sc(arena, sc!("("));

    multiline_group.start_indent_before_requests(arena);
    multiline_group.grouped_possible_newline();
    multiline_group.request(Request::discourage(RequestItem::EmptyLine));
    multiline_group.request(Request::discourage(RequestItem::Space));

    for (position, item) in items.into_iter().with_position() {
        // If the parameters are multiple lines long, every parameter should be on a new line
        // If the parameters is a single line long, every parameter should be prepended with a space,
        // with a chance for breaking into multiple lines
        multiline_group.grouped_newline_or_space();

        multiline_group.request(Request::discourage(RequestItem::EmptyLine));
        multiline_group.extend(arena, gen_node_preceding_trivia(arena, &item)?);
        multiline_group.extend(arena, gen_node_content(arena, &item)?);
        if item.has_content() {
            if position.is_last || position.is_exactly_one() {
                multiline_group.extend_if_multi_line(arena, {
                    let mut pi = PrintItems::new(arena);
                    pi.push_sc(sc!(","));
                    pi
                });
            } else {
                multiline_group.push_sc(arena, sc!(","));
            }
        }
        multiline_group.extend(arena, gen_node_succeeding_trivia(arena, &item)?);
    }

    multiline_group.request(Request::discourage(RequestItem::Space));
    multiline_group.grouped_possible_newline();
    multiline_group.finish_indent_before_requests(arena);

    multiline_group.push_sc(arena, sc!(")"));

    multiline_group.end_before_requests(arena);

    Ok(formatted)
}

pub(crate) fn gen_fn_parameter<'buffer>(
    arena: &'buffer Arena,
    syntax: &ast::Parameter,
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    // ==== Parse ====
    let mut syntax = syntax_iter(syntax.syntax());

    let item_name =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_kind(SyntaxKind::Name)?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Colon)?;
    let item_type_specifier =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_kind(SyntaxKind::TypeSpecifier)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new(arena);

    formatted.extend(arena, gen_node_with_trivia(arena, &item_name)?);
    formatted.push_sc(arena, sc!(":"));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(arena, gen_node_with_trivia(arena, &item_type_specifier)?);
    Ok(formatted)
}

pub(crate) fn gen_fn_return_type<'buffer>(
    arena: &'buffer Arena,
    syntax: &ast::ReturnType,
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    // ==== Parse ====
    let mut syntax = syntax_iter(syntax.syntax());

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Arrow)?;
    let item_type_specifier =
        parse_node_with(&mut syntax, DiscardBlankspace).expect_kind(SyntaxKind::TypeSpecifier)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new(arena);

    formatted.request(Request::expect(RequestItem::Space));
    formatted.push_sc(arena, sc!("->"));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(arena, gen_node_with_trivia(arena, &item_type_specifier)?);
    Ok(formatted)
}
