use dprint_core_macros::sc;
use itertools::put_back;
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self, FunctionCall},
};

use crate::{
    ast_parse::{parse_end, parse_node, parse_node_optional, parse_token, parse_token_optional},
    generators::{
        comments::{gen_comments, parse_many_comments_and_blankspace},
        expressions::{gen_expression, ident_expression::gen_ident_expression},
    },
    helpers::separated_items::{format_separated_items, parse_separated_items},
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
    let mut syntax = put_back(function_call.syntax().children_with_tokens());
    let item_identifier = parse_node::<ast::IdentExpression>(&mut syntax)?;
    let item_comments_after_identifier = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_arguments = parse_node::<ast::Arguments>(&mut syntax)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    // Function call "name" is always an ident_expression
    formatted.extend(gen_ident_expression(&item_identifier)?);
    formatted.extend(gen_comments(&item_comments_after_identifier));
    formatted.extend(gen_function_call_arguments(&item_arguments)?);
    Ok(formatted)
}

pub fn gen_function_call_arguments(
    arguments: &ast::Arguments
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(arguments.syntax().children_with_tokens());
    parse_token(&mut syntax, SyntaxKind::ParenthesisLeft)?;
    let item_parameters = parse_separated_items(
        &mut syntax,
        parse_node_optional::<ast::Expression>,
        |syntax| parse_token_optional(syntax, SyntaxKind::Comma),
    );
    parse_token(&mut syntax, SyntaxKind::ParenthesisRight)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    let mut multiline_group = MultilineGroup::new(&mut formatted);

    multiline_group.push_sc(sc!("("));

    // If its blank we do not give the formatter the option to break within the ()
    if !item_parameters.is_blank {
        multiline_group.start_indent();

        format_separated_items(
            &mut multiline_group,
            item_parameters,
            |item| gen_expression(item, false),
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

pub fn gen_function_call_statement(
    function_call_statement: &ast::FunctionCallStatement,
    include_semicolon: bool,
) -> Result<PrintItemBuffer, FormatDocumentError> {
    // ==== Parse ====
    let mut syntax = put_back(function_call_statement.syntax().children_with_tokens());
    let function_call = parse_node::<FunctionCall>(&mut syntax)?;
    let comments_after_function_call = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token_optional(&mut syntax, SyntaxKind::Semicolon);
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_function_call(&function_call)?);
    formatted.extend(gen_comments(&comments_after_function_call));
    if include_semicolon {
        formatted.push_sc(sc!(";"));
    }
    Ok(formatted)
}
