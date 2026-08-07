use dprint_core::formatting::PrintItems;
use dprint_core_macros::sc;
use parser::{
    SyntaxKind::{self},
    SyntaxNode,
};
use syntax::{
    AstNode as _,
    ast::{self, Expression, FunctionCall},
};

use crate::{
    ast_parse::{
        IgnoreBlankspace, NoTrivia, SyntaxIter, parse_end, parse_node, parse_node_optional,
        parse_node_with, parse_token, parse_token_optional, syntax_iter,
    },
    context_policies::statement_needs_semicolon_policy,
    generators::{
        comments::{gen_comment, gen_comments, parse_many_comments_and_blankspace},
        expressions::gen_expression,
        node::gen_node_with_trivia,
    },
    helpers::separated_items::{
        SeparatedItem, SeparatedItems, format_separated_items, parse_separated_items,
    },
    multiline_group::MultilineGroup,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::{FormatDocumentError, FormatDocumentResult},
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
) -> FormatDocumentResult<SeparatedItems<Expression>> {
    parse_token(syntax, SyntaxKind::ParenthesisLeft)?;
    let item_parameters =
        parse_separated_items(syntax, parse_node_optional::<ast::Expression>, |syntax| {
            parse_token_optional(syntax, SyntaxKind::Comma)
        });
    parse_token(syntax, SyntaxKind::ParenthesisRight)?;
    Ok(item_parameters)
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

    let mut multiline_group = MultilineGroup::new(&mut formatted);

    multiline_group.push_sc(sc!("("));

    // If its blank we do not give the formatter the option to break within the ()
    if !item_arguments.is_blank {
        multiline_group.start_indent();

        format_separated_items(
            &mut multiline_group,
            item_arguments,
            gen_expression,
            sc!(","),
        )?;

        multiline_group.request(Request::discourage(RequestItem::Space));
        multiline_group.grouped_possible_newline();
        multiline_group.finish_indent();
    }

    multiline_group.push_sc(sc!(")"));

    multiline_group.end();

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

    let item_count = item_arguments
        .items
        .iter()
        .filter(|item| matches!(item, SeparatedItem::Item(_)))
        .count();

    if item_count != table_columns * table_rows {
        return gen_function_call_arguments_standard(arguments);
    }

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    let mut multiline_group = MultilineGroup::new(&mut formatted);

    multiline_group.push_sc(sc!("("));

    // If its blank we do not give the formatter the option to break within the ()
    if !item_arguments.is_blank {
        multiline_group.start_indent();

        let mut item_index = 0;
        for (index, item) in item_arguments.items.into_iter().enumerate() {
            match item {
                SeparatedItem::Item(item) => {
                    // Separated Items only start on a new line if they are the first of a table
                    if item_index % table_columns == 0 {
                        multiline_group.request(Request::expect(RequestItem::LineBreak));
                    } else {
                        multiline_group.request(Request::expect(RequestItem::Space));
                    }
                    multiline_group.extend(gen_expression(&item)?);

                    // The separator is always immediately after the item
                    if index == item_arguments.last_item_index {
                        multiline_group.extend_if_multi_line({
                            let mut pi = PrintItems::default();
                            pi.push_sc(sc!(","));
                            pi
                        });
                    } else {
                        multiline_group.push_sc(sc!(","));
                    }
                    item_index += 1;
                },
                SeparatedItem::Separator => {
                    // The separator is always immediately after the item
                },
                SeparatedItem::Comment(comment) => {
                    multiline_group.extend(gen_comment(&comment));
                },
                SeparatedItem::LineSpacing(_line_spacing) => {
                    // We discard empty lines
                },
            }
        }
    }

    multiline_group.request(Request::discourage(RequestItem::Space));
    multiline_group.grouped_possible_newline();
    multiline_group.finish_indent();

    multiline_group.push_sc(sc!(")"));

    multiline_group.end();

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
