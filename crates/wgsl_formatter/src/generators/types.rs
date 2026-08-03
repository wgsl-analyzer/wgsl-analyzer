use dprint_core_macros::sc;
use itertools::put_back;
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self, Path, TemplateList},
};

use crate::{
    ast_parse::{parse_end, parse_node, parse_node_optional, parse_token, parse_token_optional},
    generators::{
        comments::{gen_comments, parse_many_comments_and_blankspace},
        expressions::gen_expression,
        path::gen_path,
    },
    helpers::separated_items::{format_separated_items, parse_separated_items},
    multiline_group::MultilineGroup,
    print_item_buffer::{
        PrintItemBuffer,
        spacing_request::{Request, RequestItem},
    },
    reporting::FormatDocumentResult,
};

pub fn gen_type_specifier(
    type_specifier: &ast::TypeSpecifier
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(type_specifier.syntax().children_with_tokens());

    let item_path = parse_node::<Path>(&mut syntax)?;
    let comments_after_ident = parse_many_comments_and_blankspace(&mut syntax)?;

    let item_template = parse_node_optional::<TemplateList>(&mut syntax);

    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();
    formatted.extend(gen_path(&item_path)?);
    formatted.extend(gen_comments(&comments_after_ident));
    if let Some(template) = item_template {
        formatted.extend(gen_template_list(&template)?);
    }
    Ok(formatted)
}

pub fn gen_template_list(
    template_list: &ast::TemplateList
) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(template_list.syntax().children_with_tokens());
    parse_token(&mut syntax, SyntaxKind::TemplateStart)?;

    let items = parse_separated_items(
        &mut syntax,
        parse_node_optional::<ast::Expression>,
        |syntax| parse_token_optional(syntax, SyntaxKind::Comma),
    );
    parse_token(&mut syntax, parser::SyntaxKind::TemplateEnd)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    let mut multiline_group = MultilineGroup::new(&mut formatted);
    multiline_group.push_sc(sc!("<"));

    // If its blank we do not give the formatter the option to break within the <>
    if !items.is_blank {
        multiline_group.start_indent();
        format_separated_items(&mut multiline_group, items, gen_expression, sc!(","))?;
        multiline_group.request(Request::discourage(RequestItem::Space));
        multiline_group.finish_indent();
        multiline_group.grouped_possible_newline();
    }
    multiline_group.push_sc(sc!(">"));
    multiline_group.end();

    Ok(formatted)
}
