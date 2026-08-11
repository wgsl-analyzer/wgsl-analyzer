use dprint_core::formatting::PrintItems;
use dprint_core_macros::sc;
use itertools::{Itertools as _, Position};
use parser::{
    SyntaxKind::{self},
    SyntaxNode,
};
use syntax::{
    AstNode as _,
    ast::{self, FunctionCall},
};

use crate::{
    ast_parse::{
        Chain, IgnoreBlankspace, IgnoreComma, NoTrivia, SyntaxIter, UntilSucceedingNewline,
        parse_end, parse_node_with, syntax_iter,
    },
    context_policies::statement_needs_semicolon_policy,
    generators::node::{
        gen_node_content, gen_node_preceding_trivia, gen_node_succeeding_trivia,
        gen_node_with_trivia,
    },
    multiline_group::MultilineGroup,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::{FormatDocumentError, FormatDocumentResult},
    trivia::{NodeWithTrivia, NodeWithTriviaContent},
};

pub fn gen_function_call(
    function_call: &ast::FunctionCall
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = syntax_iter(function_call.syntax());
    let item_identifier =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(SyntaxKind::IdentExpression)?;
    let item_arguments =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_kind(SyntaxKind::Arguments)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_node_with_trivia(&item_identifier)?);
    formatted.extend(gen_node_with_trivia(&item_arguments)?);
    Ok(formatted)
}

pub fn determine_function_call_argument_style(
    function_call: Option<SyntaxNode>
) -> FunctionCallArgumentStyle {
    let Some(path) = function_call
        .and_then(FunctionCall::cast)
        .and_then(|node| node.ident_expression())
        .and_then(|node| node.path())
    else {
        return FunctionCallArgumentStyle::Standard;
    };
    let mut segments = path.segments();
    let only_segment = segments.next();

    if let Some(segment) = only_segment
        && segments.next().is_none()
    {
        match segment.text() {
            "mat2x2" => FunctionCallArgumentStyle::Tabular {
                width: 2,
                height: 2,
            },
            "mat2x3" => FunctionCallArgumentStyle::Tabular {
                width: 3,
                height: 2,
            },
            "mat2x4" => FunctionCallArgumentStyle::Tabular {
                width: 4,
                height: 2,
            },
            "mat3x2" => FunctionCallArgumentStyle::Tabular {
                width: 2,
                height: 3,
            },
            "mat3x3" => FunctionCallArgumentStyle::Tabular {
                width: 3,
                height: 3,
            },
            "mat3x4" => FunctionCallArgumentStyle::Tabular {
                width: 4,
                height: 3,
            },
            "mat4x2" => FunctionCallArgumentStyle::Tabular {
                width: 2,
                height: 4,
            },
            "mat4x3" => FunctionCallArgumentStyle::Tabular {
                width: 3,
                height: 4,
            },
            "mat4x4" => FunctionCallArgumentStyle::Tabular {
                width: 4,
                height: 4,
            },
            _ => FunctionCallArgumentStyle::Standard,
        }
    } else {
        FunctionCallArgumentStyle::Standard
    }
}

pub enum FunctionCallArgumentStyle {
    Standard,
    Tabular { width: usize, height: usize },
}

pub fn parse_function_call_arguments(
    syntax: &mut SyntaxIter
) -> FormatDocumentResult<Vec<NodeWithTrivia>> {
    parse_node_with(syntax, NoTrivia).expect_kind(SyntaxKind::ParenthesisLeft)?;
    let mut item_arguments = Vec::new();
    loop {
        let mut item = parse_node_with(
            syntax,
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
    parse_node_with(syntax, NoTrivia).expect_kind(SyntaxKind::ParenthesisRight)?;
    Ok(item_arguments)
}

pub fn gen_function_call_arguments(
    arguments: &ast::Arguments
) -> FormatDocumentResult<PrintItemBuffer> {
    let style = determine_function_call_argument_style(arguments.syntax().parent());
    match style {
        FunctionCallArgumentStyle::Standard => gen_function_call_arguments_standard(arguments),
        FunctionCallArgumentStyle::Tabular { width, height } => {
            gen_function_call_arguments_tabular(arguments, width, height)
        },
    }
}

pub fn gen_function_call_arguments_standard(
    arguments: &ast::Arguments
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = syntax_iter(arguments.syntax());
    let item_arguments = parse_function_call_arguments(&mut syntax)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    let mut multiline_group = MultilineGroup::new_before_requests(&mut formatted);

    multiline_group.push_sc(sc!("("));

    // If its blank we do not give the formatter the option to break within the ()
    if !item_arguments.is_empty() {
        multiline_group.start_indent_before_requests();
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
        multiline_group.finish_indent();
    }

    multiline_group.push_sc(sc!(")"));

    multiline_group.end_before_requests();

    Ok(formatted)
}

pub fn gen_function_call_arguments_tabular(
    arguments: &ast::Arguments,
    table_columns: usize,
    table_rows: usize,
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = syntax_iter(arguments.syntax());
    let item_arguments = parse_function_call_arguments(&mut syntax)?;
    parse_end(&mut syntax)?;

    let item_count = item_arguments.len();

    if item_count != table_columns * table_rows {
        return gen_function_call_arguments_standard(arguments);
    }

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    formatted.push_sc(sc!("("));
    formatted.start_indent_before_requests();

    // If its blank we do not give the formatter the option to break within the ()
    if !item_arguments.is_empty() {
        formatted.request(Request::expect(RequestItem::LineBreak));
        for row in &item_arguments.into_iter().chunks(table_columns) {
            formatted.apply_end_request();
            formatted.request(Request::discourage(RequestItem::Space));
            let mut multiline_group = MultilineGroup::new_before_requests(&mut formatted);
            for column in row {
                multiline_group.extend(gen_node_preceding_trivia(&column)?);
                multiline_group.extend(gen_node_content(&column)?);
                multiline_group.request(Request::discourage(RequestItem::Space));
                // We know that there is always a separator - even on the final item
                multiline_group.push_sc(sc!(","));
                multiline_group.extend(gen_node_succeeding_trivia(&column)?);

                multiline_group.grouped_newline_or_space();
            }
            multiline_group.end_before_requests();

            // A row always ends on a new line
            formatted.request(Request::expect(RequestItem::LineBreak));
        }
        formatted.request(Request::expect(RequestItem::LineBreak));
    }

    formatted.finish_indent_before_requests();
    formatted.push_sc(sc!(")"));

    Ok(formatted)
}

pub fn gen_function_call_statement(
    function_call_statement: &ast::FunctionCallStatement
) -> Result<PrintItemBuffer, FormatDocumentError> {
    // ==== Parse ====
    let mut syntax = syntax_iter(function_call_statement.syntax());
    let function_call =
        parse_node_with(&mut syntax, IgnoreBlankspace).expect_castable_kind::<FunctionCall>()?;
    parse_node_with(&mut syntax, NoTrivia).expect_kind_optional(SyntaxKind::Semicolon)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_node_with_trivia(&function_call)?);
    if statement_needs_semicolon_policy(function_call_statement.syntax()) {
        formatted.push_sc(sc!(";"));
    }
    Ok(formatted)
}
