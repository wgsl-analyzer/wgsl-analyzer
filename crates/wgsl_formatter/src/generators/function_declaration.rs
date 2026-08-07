use dprint_core::formatting::PrintItems;
use dprint_core_macros::sc;
use itertools::{Itertools, Position, put_back};
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::{
    ast_parse::{
        FilterAction, IgnoreBlankspace, NoTrivia, parse_end, parse_node, parse_node_optional,
        parse_node_with, parse_node_with_trivia_filter, parse_token, parse_token_optional,
        syntax_iter,
    },
    generators::{
        attributes::{gen_attributes, parse_many_attributes},
        comments::{
            Comment, gen_comment, gen_comments, parse_comment_optional,
            parse_many_comments_and_blankspace,
        },
        node::{
            gen_node_content, gen_node_preceding_trivia, gen_node_succeeding_trivia,
            gen_node_with_trivia,
        },
        statements::compound_statement::gen_compound_statement,
        types::gen_type_specifier,
    },
    helpers::{LineSpacing, gen_line_spacing, parse_line_spacing},
    multiline_group::MultilineGroup,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
    trivia::NodeWithTriviaContent,
};

use super::attributes::AttributeLayout;

pub fn gen_function_declaration(
    node: &ast::FunctionDeclaration
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = syntax_iter(node.syntax());

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Fn)?;
    let item_name = parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(SyntaxKind::Name)?;
    let item_params = parse_node_with(&mut syntax, IgnoreBlankspace)
        .expect_kind(SyntaxKind::FunctionParameters)?;
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
    enum GenFnParameterItem {
        Parameter(ast::Parameter),
        LineSpacing(LineSpacing),
        Comment(Comment),
    }
    // ==== Parse ====

    let mut syntax = syntax_iter(node.syntax());

    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::ParenthesisLeft)?;
    let item_comments_start = parse_many_comments_and_blankspace(&mut syntax)?;

    let mut items = Vec::new();

    loop {
        let mut item = parse_node_with_trivia_filter(&mut syntax, |node| match node.kind() {
            SyntaxKind::Comma => Some(FilterAction::Ignored),
            _ => None,
        });

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

    multiline_group.extend(gen_comments(&item_comments_start));

    for (pos, item) in items.into_iter().with_position() {
        if item.has_content() {
            // If the parameters are multiple lines long, every parameter should be on a new line
            // If the parameters is a single line long, every parameter should be prepended with a space,
            // with a chance for breaking into multiple lines
            multiline_group.grouped_newline_or_space();

            multiline_group.request(Request::discourage(RequestItem::EmptyLine));
            multiline_group.extend(gen_node_preceding_trivia(&item)?);
            multiline_group.extend(gen_node_content(&item)?);
            if pos == Position::Last || pos == Position::Only {
                multiline_group.extend_if_multi_line({
                    let mut pi = PrintItems::default();
                    pi.push_sc(sc!(","));
                    pi
                });
            } else {
                multiline_group.push_sc(sc!(","));
            }
            multiline_group.extend(gen_node_succeeding_trivia(&item)?);
        }

        // match item {
        //     GenFnParameterItem::Parameter(parameter) => {
        //         // If the parameters are multiple lines long, every parameter should be on a new line
        //         // If the parameters is a single line long, every parameter should be prepended with a space,
        //         // with a chance for breaking into multiple lines
        //         multiline_group.grouped_newline_or_space();

        //         multiline_group.extend(gen_fn_parameter(&parameter)?);
        //         if index == last_parameter_index {
        //             multiline_group.extend_if_multi_line({
        //                 let mut pi = PrintItems::default();
        //                 pi.push_sc(sc!(","));
        //                 pi
        //             });
        //         } else {
        //             multiline_group.push_sc(sc!(","));
        //         }
        //     },
        //     GenFnParameterItem::LineSpacing(line_spacing) => {
        //         multiline_group.extend(gen_line_spacing(&line_spacing)?);
        //     },
        //     GenFnParameterItem::Comment(comment) => {
        //         multiline_group.extend(gen_comment(&comment));
        //     },
        // }
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

    let item_attributes = parse_many_attributes(&mut syntax)?;
    let item_name = parse_node::<ast::Name>(&mut syntax)?;
    let item_comments_after_name = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind(SyntaxKind::Colon)?;
    let item_comments_after_colon = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_type_specifier = parse_node::<ast::TypeSpecifier>(&mut syntax)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    formatted.extend(gen_attributes(
        &item_attributes,
        AttributeLayout::Multiline,
    )?);
    formatted.push_string(item_name.text().to_string());
    formatted.push_sc(sc!(":"));
    formatted.request(Request::expect(RequestItem::Space));
    //The colon should immediately follow the name, we intentionally move the comment
    formatted.extend(gen_comments(&item_comments_after_name));
    formatted.extend(gen_comments(&item_comments_after_colon));
    formatted.extend(gen_type_specifier(&item_type_specifier)?);
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

fn gen_fn_body(syntax: &ast::CompoundStatement) -> FormatDocumentResult<PrintItemBuffer> {
    gen_compound_statement(syntax)
}
