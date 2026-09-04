use itertools::Itertools as _;
use syntax::{AstNode as _, ast::GlobalCompoundDeclaration};

use crate::{
    ast_parse::{
        DiscardBraces, StopAtNewline, Succeeding, parse_end, parse_many_nodes_with,
        parse_node_with, syntax_iter,
    },
    generators::{node::gen_node_with_trivia, source_file::source_file_item_policy},
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
    trivia::{NodeTriviaItem, NodeWithTrivia},
};

pub fn gen_global_compound_declaration(
    with_trivia: &NodeWithTrivia,
    node: &GlobalCompoundDeclaration,
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let is_condcomp = with_trivia
        .preceding_trivia
        .iter()
        .any(|preceding| match preceding {
            NodeTriviaItem::AttributeList(attribute_list) => attribute_list
                .attributes()
                .any(|attribute| attribute.is_conditional_compilation()),
            NodeTriviaItem::Discarded(_)
            | NodeTriviaItem::LineSpacing(_)
            | NodeTriviaItem::Comment(_)
            | NodeTriviaItem::NewlinedComment(_) => false,
        });

    let mut syntax = syntax_iter(node.syntax());

    let item_open_brace = parse_node_with(&mut syntax, Succeeding(StopAtNewline))
        .expect_kind(parser::SyntaxKind::BraceLeft)?;

    let items = parse_many_nodes_with(&mut syntax, (source_file_item_policy(), DiscardBraces))
        .filter(|item| !item.is_whitespace())
        .collect_vec();

    parse_end(&mut syntax)?;

    // ==== Format ====

    let mut formatted = PrintItemBuffer::default();

    if is_condcomp {
        formatted.start_ignoring_indent_before_requests();
    }
    formatted.extend(gen_node_with_trivia(&item_open_brace)?);
    if is_condcomp {
        formatted.finish_ignoring_indent_before_requests();
    }

    formatted.request(Request::discourage(RequestItem::EmptyLine));
    formatted.request(Request::discourage(RequestItem::Space));

    for item in items {
        formatted.request(Request::expect(RequestItem::LineBreak));
        formatted.extend(gen_node_with_trivia(&item)?);
    }

    formatted.request(Request::expect(RequestItem::LineBreak));
    formatted.request(Request::discourage(RequestItem::EmptyLine));
    formatted.request(Request::discourage(RequestItem::Space));

    if is_condcomp {
        formatted.start_ignoring_indent_before_requests();
    }
    formatted.push_sc(dprint_core_macros::sc!("}"));
    if is_condcomp {
        formatted.finish_ignoring_indent_before_requests();
    }

    Ok(formatted)
}
