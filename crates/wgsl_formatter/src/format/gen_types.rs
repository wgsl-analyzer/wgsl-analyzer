use dprint_core::formatting::PrintItems;
use dprint_core_macros::sc;
use itertools::{Itertools as _, Position, put_back};
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self, Path, TemplateList},
};

use crate::format::{
    ast_parse::{parse_end, parse_node, parse_node_optional, parse_token, parse_token_optional},
    expressions::gen_expression,
    gen_comments::{gen_comments, parse_many_comments_and_blankspace},
    gen_path::gen_path,
    multiline_group::MultilineGroup,
    print_item_buffer::{
        PrintItemBuffer,
        request_folder::{Request, RequestItem},
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
    let mut formatted = PrintItemBuffer::new();
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
    let item_comments_after_start = parse_many_comments_and_blankspace(&mut syntax)?;

    let mut item_args = Vec::new();
    loop {
        let Some(item_arg) = parse_node_optional::<ast::Expression>(&mut syntax) else {
            break;
        };
        let item_comments_after_arg = parse_many_comments_and_blankspace(&mut syntax)?;

        parse_token_optional(&mut syntax, SyntaxKind::Comma);
        let item_comments_after_comma = parse_many_comments_and_blankspace(&mut syntax)?;

        item_args.push((item_arg, item_comments_after_arg, item_comments_after_comma));
    }

    parse_token(&mut syntax, parser::SyntaxKind::TemplateEnd)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();

    let mut multiline_group = MultilineGroup::new(&mut formatted);

    multiline_group.push_sc(sc!("<"));

    multiline_group.start_indent();

    multiline_group.extend(gen_comments(&item_comments_after_start));

    for (pos, (item_expression, item_comments_after_arg, item_comments_after_comma)) in
        item_args.into_iter().with_position()
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
        multiline_group.extend(gen_comments(&item_comments_after_arg));
        multiline_group.extend(gen_comments(&item_comments_after_comma));

        multiline_group.grouped_newline_or_space();
    }

    multiline_group.request(Request::discourage(RequestItem::Space));

    multiline_group.finish_indent();

    multiline_group.push_sc(sc!(">"));

    multiline_group.end();

    Ok(formatted)
}
