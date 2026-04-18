use dprint_core_macros::sc;
use itertools::put_back;
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self, DiagnosticControl},
};

use crate::{
    ast_parse::{parse_end, parse_node, parse_node_optional, parse_token, parse_token_optional},
    generators::{
        gen_comments::{
            Comment, gen_comment, gen_comments, parse_comment_optional,
            parse_many_comments_and_blankspace,
        },
        gen_diagnostic::gen_diagnostic_control,
    },
    multiline_group::MultilineGroup,
    print_item_buffer::{
        PrintItemBuffer,
        request_folder::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};
pub fn gen_enable_extension_name(
    node: &ast::EnableExtensionName
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = put_back(node.syntax().children_with_tokens());
    let identifier = parse_token(&mut syntax, SyntaxKind::Identifier)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.push_string(identifier.text().to_owned());
    Ok(formatted)
}

pub fn gen_enable_directive(node: &ast::EnableDirective) -> FormatDocumentResult<PrintItemBuffer> {
    enum EnableDirectiveItem {
        EnableExtensionName(ast::EnableExtensionName),
        Comment(Comment),
    }

    let mut syntax = put_back(node.syntax().children_with_tokens());

    parse_token(&mut syntax, SyntaxKind::Enable)?;

    let mut items = Vec::new();
    let mut last_content_item_index = None;
    loop {
        if let Some(_bs) = parse_token_optional(&mut syntax, SyntaxKind::Blankspace) {
            // We throw away any information about blankspace
        } else if let Some(_node) = parse_token_optional(&mut syntax, SyntaxKind::Comma) {
            // We throw away any information about commas
        } else if let Some(node) = parse_node_optional::<ast::EnableExtensionName>(&mut syntax) {
            last_content_item_index = Some(items.len());
            items.push(EnableDirectiveItem::EnableExtensionName(node));
        } else if let Some(comment) = parse_comment_optional(&mut syntax) {
            items.push(EnableDirectiveItem::Comment(comment));
        } else {
            break;
        }
    }
    parse_token(&mut syntax, SyntaxKind::Semicolon)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("enable"));
    formatted.expect(RequestItem::Space);

    let mut multiline_group = MultilineGroup::new(&mut formatted);
    multiline_group.start_indent();

    for (index, item) in items.into_iter().enumerate() {
        match item {
            EnableDirectiveItem::EnableExtensionName(extension_name) => {
                multiline_group.grouped_newline_or_space();

                multiline_group.extend(gen_enable_extension_name(&extension_name)?);
                if Some(index) != last_content_item_index {
                    multiline_group.push_sc(sc!(","));
                }
            },
            EnableDirectiveItem::Comment(comment) => {
                multiline_group.extend(gen_comment(&comment));
            },
        }
    }

    multiline_group.request(Request::discourage(RequestItem::Space));
    multiline_group.push_sc(sc!(";"));

    multiline_group.finish_indent();
    multiline_group.end();

    Ok(formatted)
}

pub fn gen_language_extension_name(
    node: &ast::LanguageExtensionName
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = put_back(node.syntax().children_with_tokens());
    let identifier = parse_token(&mut syntax, SyntaxKind::Identifier)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.push_string(identifier.text().to_owned());
    Ok(formatted)
}

pub fn gen_requires_directive(
    node: &ast::RequiresDirective
) -> FormatDocumentResult<PrintItemBuffer> {
    enum RequiresDirectiveItem {
        LanguageExtensionName(ast::LanguageExtensionName),
        Comment(Comment),
    }

    let mut syntax = put_back(node.syntax().children_with_tokens());

    parse_token(&mut syntax, SyntaxKind::Requires)?;

    let mut items = Vec::new();
    let mut last_content_item_index = None;
    loop {
        if let Some(_bs) = parse_token_optional(&mut syntax, SyntaxKind::Blankspace) {
            // We throw away any information about blankspace
        } else if let Some(_node) = parse_token_optional(&mut syntax, SyntaxKind::Comma) {
            // We throw away any information about commas
        } else if let Some(node) = parse_node_optional::<ast::LanguageExtensionName>(&mut syntax) {
            last_content_item_index = Some(items.len());
            items.push(RequiresDirectiveItem::LanguageExtensionName(node));
        } else if let Some(comment) = parse_comment_optional(&mut syntax) {
            items.push(RequiresDirectiveItem::Comment(comment));
        } else {
            break;
        }
    }
    parse_token(&mut syntax, SyntaxKind::Semicolon)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("requires"));
    formatted.expect(RequestItem::Space);

    let mut multiline_group = MultilineGroup::new(&mut formatted);
    multiline_group.start_indent();

    for (index, item) in items.into_iter().enumerate() {
        match item {
            RequiresDirectiveItem::LanguageExtensionName(extension_name) => {
                multiline_group.grouped_newline_or_space();

                multiline_group.extend(gen_language_extension_name(&extension_name)?);
                if Some(index) != last_content_item_index {
                    multiline_group.push_sc(sc!(","));
                }
            },
            RequiresDirectiveItem::Comment(comment) => {
                multiline_group.extend(gen_comment(&comment));
            },
        }
    }

    multiline_group.request(Request::discourage(RequestItem::Space));
    multiline_group.push_sc(sc!(";"));

    multiline_group.finish_indent();
    multiline_group.end();

    Ok(formatted)
}

pub fn gen_diagnostic_directive(
    node: &ast::DiagnosticDirective
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = put_back(node.syntax().children_with_tokens());

    parse_token(&mut syntax, SyntaxKind::Diagnostic)?;
    let item_comments_after_identifier = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_control = parse_node::<DiagnosticControl>(&mut syntax)?;
    let item_comments_after_control = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::Semicolon)?;
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::new();
    formatted.push_sc(sc!("diagnostic"));
    formatted.extend(gen_comments(&item_comments_after_identifier));
    formatted.extend(gen_diagnostic_control(&item_control)?);
    formatted.extend(gen_comments(&item_comments_after_control));
    formatted.push_sc(sc!(";"));
    Ok(formatted)
}
