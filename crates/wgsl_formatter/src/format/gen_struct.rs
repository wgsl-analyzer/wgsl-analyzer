use std::{alloc::alloc, iter::repeat_with, rc::Rc};

use dprint_core::formatting::{
    ConditionResolver, ConditionResolverContext, LineNumber, LineNumberAnchor, PrintItem,
    PrintItems, PrintOptions, Signal, actions, condition_helpers, condition_resolvers, conditions,
    ir_helpers,
};
use dprint_core_macros::sc;
use itertools::{Itertools as _, Position, PutBack, put_back};
use parser::{SyntaxKind, SyntaxNode, SyntaxToken, WeslLanguage};
use rowan::{NodeOrToken, SyntaxElementChildren};
use syntax::{
    AstNode as _, HasName as _,
    ast::{self},
    match_ast,
};

use crate::{
    FormattingOptions,
    format::{
        self,
        ast_parse::{
            parse_end, parse_end_optional, parse_many_comments_and_blankspace, parse_node,
            parse_node_optional, parse_token, parse_token_optional,
        },
        gen_comments::gen_comments,
        gen_types::gen_type_specifier,
        helpers::{create_is_multiple_lines_resolver, gen_spaced_lines, into_items, todo_verbatim},
        print_item_buffer::{PrintItemBuffer, SeparationPolicy, SeparationRequest},
        reporting::{FormatDocumentError, FormatDocumentErrorKind, FormatDocumentResult, err_src},
    },
};

pub fn gen_struct_declaration(
    node: &ast::StructDeclaration
) -> FormatDocumentResult<PrintItemBuffer> {
    // === Parse ===
    let mut syntax = put_back(node.syntax().children_with_tokens());

    let item_struct = parse_token(&mut syntax, SyntaxKind::Struct)?;
    let item_comments_after_struct = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_name = parse_node::<ast::Name>(&mut syntax)?;
    let item_comments_after_name = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_body = parse_node::<ast::StructBody>(&mut syntax)?;
    parse_end(&mut syntax)?;

    // === Format ===
    let mut formatted = PrintItemBuffer::new();

    // Struct
    formatted.push_sc(sc!("struct"));
    formatted.request_single_space();
    formatted.extend(gen_comments(item_comments_after_struct));

    // Name
    formatted.request_single_space();
    formatted.push_string(item_name.text().to_string());
    formatted.extend(gen_comments(item_comments_after_name));

    // Body
    formatted.request_single_space();
    formatted.extend(gen_struct_body(&item_body)?);

    Ok(formatted)
}

fn gen_struct_body(body: &ast::StructBody) -> FormatDocumentResult<PrintItemBuffer> {
    // === Parse ===
    let mut syntax = put_back(body.syntax().children_with_tokens());

    parse_token(&mut syntax, SyntaxKind::BraceLeft)?;
    let item_comments_after_open_paren = parse_many_comments_and_blankspace(&mut syntax)?;

    let mut item_members = Vec::new();

    loop {
        let Some(item_member) = parse_node_optional::<ast::StructMember>(&mut syntax) else {
            break;
        };
        let item_comments_after_member = parse_many_comments_and_blankspace(&mut syntax)?;

        parse_token_optional(&mut syntax, SyntaxKind::Comma); //Optional
        let item_comments_after_comma = parse_many_comments_and_blankspace(&mut syntax)?;

        item_members.push((
            item_member,
            item_comments_after_member,
            item_comments_after_comma,
        ));
    }

    parse_token(&mut syntax, SyntaxKind::BraceRight)?;
    parse_end(&mut syntax);

    // === Format ===
    let mut formatted = PrintItemBuffer::new();

    formatted.push_sc(sc!("{"));
    formatted.push_signal(Signal::StartIndent);

    if !item_members.is_empty() {
        formatted.request_line_break();
        for (member, comments_after_member, comments_after_comma) in item_members {
            formatted.extend(gen_struct_member(&member)?);
            formatted.push_sc(sc!(","));
            formatted.request_line_break();

            // Intentionally reorder comments to move them after the comma
            formatted.extend(gen_comments(comments_after_member));
            formatted.extend(gen_comments(comments_after_comma));
        }
    }

    formatted.push_signal(Signal::FinishIndent);
    formatted.push_sc(sc!("}"));

    Ok(formatted)
}

fn gen_struct_member(member: &ast::StructMember) -> FormatDocumentResult<PrintItemBuffer> {
    // === Parse ===
    let mut syntax = put_back(member.syntax().children_with_tokens());

    let item_name = parse_node::<ast::Name>(&mut syntax)?;
    let item_comments_after_name = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::Colon);
    let item_comments_after_colon = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_type_specifier = parse_node::<ast::TypeSpecifier>(&mut syntax)?;
    parse_end(&mut syntax)?;

    // === Format ===
    let mut formatted = PrintItemBuffer::new();

    formatted.push_string(item_name.text().to_string());
    formatted.push_sc(sc!(":"));
    formatted.request_single_space();
    //The colon should immediately follow the name, we intentionally move the comment
    formatted.extend(gen_comments(item_comments_after_name));
    formatted.extend(gen_comments(item_comments_after_colon));
    formatted.extend(gen_type_specifier(&item_type_specifier)?);

    Ok(formatted)
}
