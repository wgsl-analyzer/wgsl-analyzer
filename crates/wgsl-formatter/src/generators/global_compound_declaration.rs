use dprint_core::formatting::Arena;
use itertools::Itertools as _;
use syntax::{AstNode as _, ast::GlobalCompoundDeclaration};

use crate::{
    ast_parse::{
        DiscardBraces, StopAtNewline, Succeeding, parse_end, parse_many_nodes_with,
        parse_node_with, syntax_iter,
    },
    generators::{node::gen_node_with_trivia, source_file::source_file_item_policy},
    options::TEMP_EXPERIMENTAL_CONDCOMP_MODE,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
    trivia::{NodeTriviaItem, NodeWithTrivia},
};

pub(crate) fn gen_global_compound_declaration<'buffer>(
    arena: &'buffer Arena,
    with_trivia: &NodeWithTrivia,
    node: &GlobalCompoundDeclaration,
) -> FormatDocumentResult<PrintItemBuffer<'buffer>> {
    // ==== Context ====

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

    let dedent_braces = is_condcomp && TEMP_EXPERIMENTAL_CONDCOMP_MODE.dedent_condcomp_with_body;
    let creates_indentation =
        !is_condcomp || TEMP_EXPERIMENTAL_CONDCOMP_MODE.unscope_compound_statements_create_indent;

    // ==== Parse ====

    let mut syntax = syntax_iter(node.syntax());

    let item_open_brace = parse_node_with(&mut syntax, Succeeding(StopAtNewline))
        .expect_kind(syntax::SyntaxKind::BraceLeft)?;

    let items = parse_many_nodes_with(&mut syntax, (source_file_item_policy(), DiscardBraces))
        .filter(|item| !item.is_whitespace())
        .collect_vec();

    parse_end(&mut syntax)?;

    // ==== Format ====

    let mut formatted = PrintItemBuffer::new(arena);

    if dedent_braces {
        formatted.start_ignoring_indent_before_requests(arena);
    }
    formatted.extend(arena, gen_node_with_trivia(arena, &item_open_brace)?);
    if dedent_braces {
        formatted.finish_ignoring_indent_before_requests(arena);
    }

    formatted.request(Request::discourage(RequestItem::EmptyLine));
    formatted.request(Request::discourage(RequestItem::Space));

    if creates_indentation {
        formatted.start_indent_before_requests(arena);
    }

    for item in items {
        formatted.request(Request::expect(RequestItem::LineBreak));
        formatted.extend(arena, gen_node_with_trivia(arena, &item)?);
    }

    if creates_indentation {
        formatted.finish_indent_before_requests(arena);
    }

    formatted.request(Request::expect(RequestItem::LineBreak));
    formatted.request(Request::discourage(RequestItem::EmptyLine));
    formatted.request(Request::discourage(RequestItem::Space));

    if dedent_braces {
        formatted.start_ignoring_indent_before_requests(arena);
    }
    formatted.push_sc(arena, dprint_core_macros::sc!("}"));
    if dedent_braces {
        formatted.finish_ignoring_indent_before_requests(arena);
    }

    Ok(formatted)
}
