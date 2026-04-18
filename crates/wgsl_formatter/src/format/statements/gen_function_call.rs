use dprint_core::formatting::PrintItems;
use dprint_core_macros::sc;
use itertools::{Itertools as _, Position, put_back};
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self, FunctionCall},
};

use crate::format::{
    ast_parse::{
        SyntaxIter, parse_end, parse_node, parse_node_optional, parse_token, parse_token_optional,
    },
    expressions::{gen_expression::gen_expression, gen_ident::gen_ident_expression},
    gen_comments::{gen_comments, parse_many_comments_and_blankspace},
    multiline_group::MultilineGroup,
    print_item_buffer::{
        PrintItemBuffer,
        request_folder::{Request, RequestItem},
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
    let mut formatted = PrintItemBuffer::new();
    // TODO(MonaMayrhofer) Are we guaranteed to have an ident_expression here? i dont think soo....
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
    let formatted = gen_function_call_like_comma_separated_values(&mut syntax)?;
    parse_end(&mut syntax)?;
    Ok(formatted)
}

pub fn gen_function_call_like_comma_separated_values(
    syntax: &mut SyntaxIter
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    // TODO(MonaMayrhofer) Mehhh this logic is misgeneralized i need to do it properly...
    if parse_token_optional(syntax, SyntaxKind::ParenthesisLeft).is_none() {
        return Ok(PrintItemBuffer::new());
    }
    let item_comments_after_open_paren = parse_many_comments_and_blankspace(syntax)?;

    let mut item_parameters = Vec::new();
    loop {
        let Some(item_param) = parse_node_optional::<ast::Expression>(syntax) else {
            break;
        };
        let item_comments_after_param = parse_many_comments_and_blankspace(syntax)?;

        parse_token_optional(syntax, SyntaxKind::Comma);
        let item_comments_after_comma = parse_many_comments_and_blankspace(syntax)?;

        item_parameters.push((
            item_param,
            item_comments_after_param,
            item_comments_after_comma,
        ));
    }

    parse_token(syntax, SyntaxKind::ParenthesisRight)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();

    let mut multiline_group = MultilineGroup::new(&mut formatted);

    multiline_group.push_sc(sc!("("));

    // TODO(MonaMayrhofer) Maybe this (and type-templates) should have a similar architecture to the function signature
    // where comments are items
    if !item_parameters.is_empty() || !item_comments_after_open_paren.is_empty() {
        multiline_group.start_indent();

        multiline_group.extend(gen_comments(&item_comments_after_open_paren));

        for (pos, (item_expression, item_comments_after_param, item_comments_after_comma)) in
            item_parameters.into_iter().with_position()
        {
            multiline_group.extend(gen_expression(&item_expression, false)?);
            if pos == Position::Last || pos == Position::Only {
                multiline_group.extend_if_multi_line({
                    let mut pi = PrintItems::default();
                    pi.push_sc(sc!(","));
                    pi
                });
            } else {
                multiline_group.push_sc(sc!(","));
            }

            //The comma should be immediately after the parameter, we move the comment back
            multiline_group.extend(gen_comments(&item_comments_after_param));
            multiline_group.extend(gen_comments(&item_comments_after_comma));

            multiline_group.grouped_newline_or_space();
        }

        multiline_group.request(Request::discourage(RequestItem::Space));

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
    let mut formatted = PrintItemBuffer::new();
    formatted.extend(gen_function_call(&function_call)?);
    formatted.extend(gen_comments(&comments_after_function_call));
    if include_semicolon {
        formatted.push_sc(sc!(";"));
    }
    Ok(formatted)
}
