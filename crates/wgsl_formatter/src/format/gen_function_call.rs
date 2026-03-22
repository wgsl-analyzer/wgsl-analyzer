use dprint_core::formatting::PrintItems;
use dprint_core_macros::sc;
use itertools::{Itertools as _, Position, put_back};
use parser::SyntaxKind;
use syntax::{AstNode as _, ast};

use crate::format::{
    ast_parse::{
        parse_end, parse_many_comments_and_blankspace, parse_node, parse_node_optional,
        parse_token, parse_token_optional,
    },
    gen_comments::gen_comments,
    gen_expression::{gen_expression, gen_ident_expression},
    multiline_group::MultilineGroup,
    print_item_buffer::{
        PrintItemBuffer,
        request_folder::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
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
    let item_comments_after_open_paren = parse_many_comments_and_blankspace(&mut syntax)?;

    let mut item_parameters = Vec::new();
    loop {
        let Some(item_param) = parse_node_optional::<ast::Expression>(&mut syntax) else {
            break;
        };
        let item_comments_after_param = parse_many_comments_and_blankspace(&mut syntax)?;

        parse_token_optional(&mut syntax, SyntaxKind::Comma);
        let item_comments_after_comma = parse_many_comments_and_blankspace(&mut syntax)?;

        item_parameters.push((
            item_param,
            item_comments_after_param,
            item_comments_after_comma,
        ));
    }

    parse_token(&mut syntax, SyntaxKind::ParenthesisRight)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();

    let mut multiline_group = MultilineGroup::new(&mut formatted);

    multiline_group.push_sc(sc!("("));

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

    multiline_group.push_sc(sc!(")"));

    multiline_group.end();

    Ok(formatted)
}
