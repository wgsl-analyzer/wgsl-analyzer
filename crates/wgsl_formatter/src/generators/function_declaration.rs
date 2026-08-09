use dprint_core::formatting::PrintItems;
use dprint_core_macros::sc;
use itertools::{Itertools as _, Position};
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self},
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
    trivia::NodeWithTriviaContent,
};

pub fn gen_function_declaration(
    node: &ast::FunctionDeclaration
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = syntax_iter(node.syntax());

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Fn)?;
    let item_name = parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(SyntaxKind::Name)?;
    let item_params = parse_node_with(&mut syntax, IgnoreBlankspace)
        .expect_kind(SyntaxKind::FunctionParameters)?;
    // TODO Use new only_if api here
    let (item_return, item_body) = {
        let item = parse_node_with(&mut syntax, IgnoreBlankspace);
        if item
            .kind()
            .is_some_and(|kind| kind == SyntaxKind::ReturnType)
        {
            (
                Some(item),
                parse_node_with(&mut syntax, IgnoreBlankspace)
                    .expect_kind(SyntaxKind::CompoundStatement)?,
            )
        } else {
            (None, item.expect_kind(SyntaxKind::CompoundStatement)?)
        }
    };
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::default();

    // Fn
    formatted.push_sc(sc!("fn"));
    formatted.request(Request::expect(RequestItem::Space));

    // Name
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_node_with_trivia(&item_name)?);

    // Params
    formatted.extend(gen_node_with_trivia(&item_params)?);

    // Return
    if let Some(item_return) = item_return {
        formatted.extend(gen_node_with_trivia(&item_return)?);
    }

    // Body
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_node_with_trivia(&item_body)?);

    Ok(formatted)
}

pub fn gen_fn_parameters(node: &ast::FunctionParameters) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====

    let mut syntax = syntax_iter(node.syntax());

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::ParenthesisLeft)?;

    let mut items = Vec::new();

    // TODO Recreate something akin to parse_separated_items
    // However i think it would be better to have a central space where
    // we define "strategies" - one for each application, and then look for those that are
    // the same, and type alias between them.
    loop {
        let mut item = parse_node_with(
            &mut syntax,
            Chain(UntilSucceedingNewline, Chain(IgnoreBlankspace, IgnoreComma)),
        );

        // TODO Do I want to move this logicto trivia_filter too?
        if matches!(item.kind(), Some(SyntaxKind::ParenthesisRight)) {
            let old_node = std::mem::replace(&mut item.node, NodeWithTriviaContent::End);
            syntax.put_back(old_node.into_option().unwrap()); //TODO
        }

        // TODO Move any code that looks like this to trivia_filter
        // if matches!(item.kind(), Some(SyntaxKind::Comma)) {
        //     // We throw away any information about commas
        //     item.node = NodeWithTriviaContent::NoContent;
        // }

        let is_end = item.is_end();
        if !item.is_whitespace() {
            items.push(item);
        }
        if is_end {
            break;
        }
    }

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::ParenthesisRight)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    let mut multiline_group = MultilineGroup::new(&mut formatted);

    multiline_group.push_sc(sc!("("));

    multiline_group.start_indent();

    for (pos, item) in items.into_iter().with_position() {
        // If the parameters are multiple lines long, every parameter should be on a new line
        // If the parameters is a single line long, every parameter should be prepended with a space,
        // with a chance for breaking into multiple lines
        multiline_group.grouped_newline_or_space();

        multiline_group.request(Request::discourage(RequestItem::EmptyLine));
        multiline_group.extend(gen_node_preceding_trivia(&item)?);
        multiline_group.extend(gen_node_content(&item)?);
        if item.has_content() {
            if pos == Position::Last || pos == Position::Only {
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

    multiline_group.push_sc(sc!(")"));

    multiline_group.end();

    Ok(formatted)
}

pub fn gen_fn_parameter(syntax: &ast::Parameter) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = syntax_iter(syntax.syntax());

    let item_name = parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(SyntaxKind::Name)?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Colon)?;
    let item_type_specifier =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(SyntaxKind::TypeSpecifier)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    formatted.extend(gen_node_with_trivia(&item_name)?);
    formatted.push_sc(sc!(":"));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_node_with_trivia(&item_type_specifier)?);
    Ok(formatted)
}

pub fn gen_fn_return_type(syntax: &ast::ReturnType) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = syntax_iter(syntax.syntax());

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Arrow)?;
    let item_type_specifier =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(SyntaxKind::TypeSpecifier)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    formatted.request(Request::expect(RequestItem::Space));
    formatted.push_sc(sc!("->"));
    formatted.request(Request::expect(RequestItem::Space));
    formatted.extend(gen_node_with_trivia(&item_type_specifier)?);
    Ok(formatted)
}
