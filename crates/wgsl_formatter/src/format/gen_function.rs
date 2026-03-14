use std::{collections::BTreeSet, rc::Rc};

use dprint_core::formatting::{PrintItems, conditions};
use dprint_core_macros::sc;
use itertools::put_back;
use parser::SyntaxKind;
use syntax::{
    AstNode as _,
    ast::{self},
};

use crate::format::{
    ast_parse::{
        parse_end, parse_many_comments_and_blankspace, parse_node, parse_node_optional,
        parse_token, parse_token_optional,
    },
    gen_attributes::{gen_attributes, parse_many_attributes},
    gen_comments::{Comment, gen_comment, gen_comments, parse_comment_optional},
    gen_statement_compound::gen_compound_statement,
    gen_types::gen_type_specifier,
    helpers::{LineSpacing, gen_line_spacing, parse_line_spacing},
    multiline_group::MultilineGroup,
    print_item_buffer::{
        PrintItemBuffer,
        request_folder::{Request, RequestFolder, RequestItem},
    },
    reporting::FormatDocumentResult,
};

pub fn gen_function_declaration(
    node: &ast::FunctionDeclaration
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = put_back(node.syntax().children_with_tokens());

    let item_attributes = parse_many_attributes(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::Fn)?;
    let item_comments_after_fn = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_name = parse_node::<ast::Name>(&mut syntax)?;
    let item_comments_after_name = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_params = parse_node::<ast::FunctionParameters>(&mut syntax)?;
    let item_comments_after_params = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_return = parse_node_optional::<ast::ReturnType>(&mut syntax);
    let item_comments_after_return = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_body = parse_node::<ast::CompoundStatement>(&mut syntax)?;
    parse_end(&mut syntax)?;

    let mut formatted = PrintItemBuffer::new();

    // Fn
    formatted.extend(gen_attributes(&item_attributes)?);
    formatted.push_sc(sc!("fn"));
    formatted.expect(RequestItem::Space);
    formatted.extend(gen_comments(&item_comments_after_fn));

    // Name
    formatted.expect(RequestItem::Space);
    formatted.push_string(item_name.text().to_string());
    formatted.extend(gen_comments(&item_comments_after_name));

    // Params
    formatted.extend(gen_fn_parameters(&item_params)?);
    formatted.extend(gen_comments(&item_comments_after_params));

    // Return
    if let Some(item_return) = item_return {
        formatted.extend(gen_fn_return_type(&item_return)?);
    }
    formatted.extend(gen_comments(&item_comments_after_return));

    // Body
    formatted.expect(RequestItem::Space);
    formatted.extend(gen_fn_body(&item_body)?);

    Ok(formatted)
}

pub fn gen_fn_parameters(node: &ast::FunctionParameters) -> FormatDocumentResult<PrintItemBuffer> {
    enum GenFnParameterItem {
        Parameter(ast::Parameter),
        LineSpacing(LineSpacing),
        Comment(Comment),
    }
    // ==== Parse ====

    let mut syntax = put_back(node.syntax().children_with_tokens());

    parse_token(&mut syntax, SyntaxKind::ParenthesisLeft)?;
    let item_comments_start = parse_many_comments_and_blankspace(&mut syntax)?;

    let mut items = Vec::new();

    let mut possible_spacing_before_comments = None;
    let mut last_parameter_index = 0;

    loop {
        let current_pending_space = possible_spacing_before_comments.take();

        if let Some(spacing) = parse_line_spacing(&mut syntax) {
            // Currently we only respect line_spacings if they occur directly before a comment
            possible_spacing_before_comments = Some(spacing);
        } else if let Some(_statement) = parse_token_optional(&mut syntax, SyntaxKind::Blankspace) {
            // If its not a line_spacing blankspace, then we simply discard it
        } else if let Some(parameter) = parse_node_optional::<ast::Parameter>(&mut syntax) {
            last_parameter_index = items.len();
            items.push(GenFnParameterItem::Parameter(parameter));
        } else if let Some(comment) = parse_comment_optional(&mut syntax) {
            if let Some(spacing) = current_pending_space {
                items.push(GenFnParameterItem::LineSpacing(spacing));
            }
            items.push(GenFnParameterItem::Comment(comment));
        } else {
            break;
        }
        // We throw away any information about commas
        parse_token_optional(&mut syntax, SyntaxKind::Comma);
    }

    parse_token(&mut syntax, SyntaxKind::ParenthesisRight)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::new();

    let mut multiline_group = MultilineGroup::new(&mut formatted);

    multiline_group.push_sc(sc!("("));

    multiline_group.start_indent();

    multiline_group.extend(gen_comments(&item_comments_start));

    for (index, item) in items.into_iter().enumerate() {
        match item {
            GenFnParameterItem::Parameter(parameter) => {
                // TODO Polish Newline api to make this prettier

                // If the parameters are multiple lines long, every parameter should be on a new line
                // If the parameters is a single line long, every parameter should be prepended with a space,
                // with a chance for breaking into multiple lines
                let is_multiline = Rc::clone(&multiline_group.is_multiple_lines);
                multiline_group.request_request(Request::Conditional {
                    condition: is_multiline,
                    on_true: Box::new(RequestFolder {
                        folded_request: Some(Request::Unconditional {
                            expected: BTreeSet::from_iter([RequestItem::LineBreak]),
                            discouraged: BTreeSet::new(),
                            forced: BTreeSet::new(),
                        }),
                    }),
                    on_false: Box::new(RequestFolder::default()),
                });

                if index != 0 {
                    multiline_group.request_request(Request::Unconditional {
                        expected: BTreeSet::from([RequestItem::Space]),
                        discouraged: BTreeSet::new(),
                        forced: BTreeSet::new(),
                    });
                }

                multiline_group.extend(gen_fn_parameter(&parameter)?);
                if index == last_parameter_index {
                    let is_multiline = Rc::clone(&multiline_group.is_multiple_lines);
                    multiline_group.push_condition(conditions::if_true(
                        "paramTrailingComma",
                        is_multiline,
                        {
                            let mut pi = PrintItems::default();
                            pi.push_sc(sc!(","));
                            pi
                        },
                    ));
                } else {
                    multiline_group.push_sc(sc!(","));
                }
            },
            GenFnParameterItem::LineSpacing(line_spacing) => {
                multiline_group.extend(gen_line_spacing(&line_spacing)?);
            },
            GenFnParameterItem::Comment(comment) => {
                multiline_group.extend(gen_comment(&comment));
            },
        }
    }

    multiline_group.grouped_newline_or_space();
    multiline_group.finish_indent();

    multiline_group.push_sc(sc!(")"));

    multiline_group.end();

    Ok(formatted)
}

pub fn gen_fn_parameter(syntax: &ast::Parameter) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(syntax.syntax().children_with_tokens());

    let item_attributes = parse_many_attributes(&mut syntax)?;
    let item_name = parse_node::<ast::Name>(&mut syntax)?;
    let item_comments_after_name = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::Colon)?;
    let item_comments_after_colon = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_type_specifier = parse_node::<ast::TypeSpecifier>(&mut syntax)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    formatted.extend(gen_attributes(&item_attributes)?);
    formatted.push_string(item_name.text().to_string());
    formatted.push_sc(sc!(":"));
    formatted.expect(RequestItem::Space);
    //The colon should immediately follow the name, we intentionally move the comment
    formatted.extend(gen_comments(&item_comments_after_name));
    formatted.extend(gen_comments(&item_comments_after_colon));
    formatted.extend(gen_type_specifier(&item_type_specifier)?);
    Ok(formatted)
}

pub fn gen_fn_return_type(syntax: &ast::ReturnType) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(syntax.syntax().children_with_tokens());

    parse_token(&mut syntax, SyntaxKind::Arrow)?;
    let item_comments_after_arrow = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_attributes = parse_many_attributes(&mut syntax)?;
    let item_type_specifier = parse_node::<ast::TypeSpecifier>(&mut syntax)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    formatted.expect(RequestItem::Space);
    formatted.push_sc(sc!("->"));
    formatted.expect(RequestItem::Space);
    formatted.extend(gen_comments(&item_comments_after_arrow));
    formatted.extend(gen_attributes(&item_attributes)?);
    formatted.extend(gen_type_specifier(&item_type_specifier)?);
    Ok(formatted)
}

fn gen_fn_body(syntax: &ast::CompoundStatement) -> FormatDocumentResult<PrintItemBuffer> {
    gen_compound_statement(syntax)
}
