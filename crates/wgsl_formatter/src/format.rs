#![expect(
    clippy::branches_sharing_code,
    reason = "Its helpful to explicitly state intent here."
)]
mod ast_parse;
mod helpers;
mod print_item_buffer;

mod reporting;

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
        helpers::{gen_spaced_lines, into_items},
        print_item_buffer::{PrintItemBuffer, PrintItemRequest, SeparationPolicy},
        reporting::{FormatDocumentError, FormatDocumentErrorKind, FormatDocumentResult, err_src},
    },
};

pub fn format_str(
    input: &str,
    options: &FormattingOptions,
) -> FormatDocumentResult<String> {
    let parse = syntax::parse(input);
    //TODO Return error if the syntax could not parse.
    let file = parse.tree();
    format_tree(&file, options)
}

pub fn format_tree(
    syntax: &ast::SourceFile,
    options: &FormattingOptions,
) -> FormatDocumentResult<String> {
    let mut error = None;

    let formatted = dprint_core::formatting::format(
        || match gen_source_file(syntax) {
            Ok(items) => items.finish(),
            Err(err) => {
                //We seem to have to do it this weird way, because
                // a) We can't return the error from the closure because of dprint's api
                // b) We can't call gen_source_file outside of the closure because
                //    dprint requires the gen_items to be allocated using a thread local
                //    allocator that only exists within the closure.
                error.insert(err);

                //TODO maybe we should instead output the whole source verbatim
                // so that if many things go wrong and this value does somehow
                // reach the user's file, it doesn't just delete it all.
                let mut items = PrintItemBuffer::new();
                items.push_string("ERROR".into());
                items.finish()
            },
        },
        PrintOptions {
            //TODO Populate these from options
            max_width: options.width,
            indent_width: 4,
            use_tabs: false,
            new_line_text: "\n",
        },
    );

    match error {
        Some(err) => Err(err),
        None => Ok(formatted),
    }
}

fn gen_item(node: &ast::Item) -> FormatDocumentResult<PrintItemBuffer> {
    match node {
        ast::Item::FunctionDeclaration(function_declaration) => {
            gen_function_declaration(function_declaration)
        },
        ast::Item::VariableDeclaration(variable_declaration) => todo!(),
        ast::Item::ConstantDeclaration(constant_declaration) => todo!(),
        ast::Item::OverrideDeclaration(override_declaration) => todo!(),
        ast::Item::TypeAliasDeclaration(type_alias_declaration) => todo!(),
        ast::Item::StructDeclaration(struct_declaration) => todo!(),
    }
}

fn gen_source_file(node: &ast::SourceFile) -> FormatDocumentResult<PrintItemBuffer> {
    gen_spaced_lines(node.syntax(), |child| {
        //TODO This clone is unnecessary if we had a cast that returned the passed in node
        // on a failure like std::any::Any (SyntaxNode -> Result<Item, Syntaxnode>)

        if let NodeOrToken::Node(child) = child
            && let Some(item) = ast::Item::cast(child.clone())
        {
            gen_item(&item)
        } else if let NodeOrToken::Token(child) = child
            && (child.kind() == SyntaxKind::BlockComment
                || child.kind() == SyntaxKind::LineEndingComment)
        {
            Ok(gen_comment(child))
        } else {
            Err(FormatDocumentErrorKind::UnexpectedModuleNode.at(child.text_range(), err_src!()))
        }
    })
}

fn gen_function_declaration(
    node: &ast::FunctionDeclaration
) -> FormatDocumentResult<PrintItemBuffer> {
    let mut syntax = put_back(node.syntax().children_with_tokens());

    let item_fn = parse_token(&mut syntax, SyntaxKind::Fn)?;
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
    formatted.push_sc(sc!("fn"));
    formatted.request_single_space();
    formatted.extend(gen_comments(item_comments_after_fn));

    // Name
    formatted.request_single_space();
    formatted.push_string(item_name.text().to_string());
    formatted.extend(gen_comments(item_comments_after_name));

    // Params
    formatted.extend(gen_fn_parameters(&item_params)?);
    formatted.extend(gen_comments(item_comments_after_params));

    // Return
    if let Some(item_return) = item_return {
        formatted.extend(gen_fn_return_type(&item_return)?);
    }
    formatted.extend(gen_comments(item_comments_after_return));

    // Body
    formatted.request_single_space();
    formatted.extend(gen_fn_body(&item_body)?);

    Ok(formatted)
}

fn gen_comments(comments: Vec<SyntaxToken>) -> PrintItemBuffer {
    let mut formatted = PrintItemBuffer::new();
    for item in comments {
        formatted.extend(gen_comment(&item));
    }
    formatted
}
fn gen_comment(item: &SyntaxToken) -> PrintItemBuffer {
    let mut formatted = PrintItemBuffer::new();
    if item.kind() == SyntaxKind::BlockComment {
        formatted.request_single_space();
        formatted.push_string(item.to_string());
        formatted.request_single_space();
    } else if item.kind() == SyntaxKind::LineEndingComment {
        formatted.request_single_space();
        formatted.push_string(item.to_string());
        //TODO This should be a request, but for now we have no way of encoding a "forced newline no matter what"
        formatted.request(PrintItemRequest {
            line_break: SeparationPolicy::Forced,
            ..Default::default()
        });
    } else {
        //TODO Make this unrepresentable
        unreachable!("Non comment entry found in comments Vec");
    }
    formatted
}

#[expect(clippy::too_many_lines, reason = "TODO")]
fn gen_fn_parameters(node: &ast::FunctionParameters) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====

    let mut syntax = put_back(node.syntax().children_with_tokens());

    parse_token(&mut syntax, SyntaxKind::ParenthesisLeft)?;
    let item_comments_start = parse_many_comments_and_blankspace(&mut syntax)?;

    let mut item_parameters = Vec::new();

    loop {
        let Some(item_param) = parse_node_optional::<ast::Parameter>(&mut syntax) else {
            break;
        };
        let item_comments_after_param = parse_many_comments_and_blankspace(&mut syntax)?;

        parse_token_optional(&mut syntax, SyntaxKind::Comma); //Optional
        let item_comments_after_comma = parse_many_comments_and_blankspace(&mut syntax)?;

        item_parameters.push((
            item_param,
            item_comments_after_param,
            item_comments_after_comma,
        ));
    }

    let item_comments_after_params = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::ParenthesisRight)?;
    parse_end(&mut syntax)?;

    // ==== Format ====

    let start_ln = LineNumber::new("start");
    let end_ln = LineNumber::new("end");
    let is_multiple_lines = create_is_multiple_lines_resolver(start_ln, end_ln);

    let mut formatted = PrintItemBuffer::new();

    formatted.push_info(start_ln);
    formatted.push_anchor(LineNumberAnchor::new(end_ln));

    formatted.push_sc(sc!("("));

    let mut start_nl_condition = conditions::if_true_or(
        "paramMultilineStartIndent",
        Rc::clone(&is_multiple_lines),
        {
            let mut pi = PrintItems::default();
            pi.push_signal(Signal::NewLine);
            pi.push_signal(Signal::StartIndent);
            pi
        },
        {
            let mut pi = PrintItems::default();
            pi.push_signal(Signal::PossibleNewLine);
            pi
        },
    );
    let start_reeval = start_nl_condition.create_reevaluation();
    formatted.push_condition(start_nl_condition);
    formatted.push_signal(Signal::StartNewLineGroup);

    // TODO This is a bit of a shortcoming of the PBI api, we would want to write this after the "(", but can't because of the conditions between
    formatted.request(PrintItemRequest::discouraged());

    formatted.extend(gen_comments(item_comments_start));

    for (pos, (item_parameter, item_comments_after_param, item_comments_after_comma)) in
        item_parameters.into_iter().with_position()
    {
        formatted.extend(gen_fn_parameter(&item_parameter)?);
        if pos == Position::Last || pos == Position::Only {
            formatted.push_condition(conditions::if_true(
                "paramTrailingComma",
                Rc::clone(&is_multiple_lines),
                {
                    let mut pi = PrintItems::default();
                    pi.push_sc(sc!(","));
                    pi
                },
            ));
        } else {
            formatted.push_sc(sc!(","));
        }

        //The comma should be immediately after the parameter, we move the comment back
        formatted.extend(gen_comments(item_comments_after_param));
        formatted.extend(gen_comments(item_comments_after_comma));

        formatted.request(PrintItemRequest {
            line_break: SeparationPolicy::ExpectedIf {
                on_branch: true,
                of_resolver: Rc::clone(&is_multiple_lines),
            },
            space: SeparationPolicy::ExpectedIf {
                on_branch: false,
                of_resolver: Rc::clone(&is_multiple_lines),
            },
            ..Default::default()
        });
    }
    formatted.extend(gen_comments(item_comments_after_params));

    // No trailing spaces
    formatted.request(PrintItemRequest {
        space: SeparationPolicy::Discouraged,
        ..Default::default()
    });

    formatted.push_condition(conditions::if_true(
        "paramMultilineEndIndent",
        is_multiple_lines,
        {
            let mut pi = PrintItems::default();
            pi.push_signal(Signal::FinishIndent);
            pi
        },
    ));

    formatted.push_sc(sc!(")"));

    formatted.push_signal(Signal::FinishNewLineGroup);
    formatted.push_info(end_ln);
    formatted.push_reevaluation(start_reeval);

    Ok(formatted)
}

fn gen_fn_parameter(syntax: &ast::Parameter) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(syntax.syntax().children_with_tokens());

    let item_name = parse_node::<ast::Name>(&mut syntax)?;
    let item_comments_after_name = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::Colon);
    let item_comments_after_colon = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_type_specifier = parse_node::<ast::TypeSpecifier>(&mut syntax)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    formatted.push_string(item_name.text().to_string());
    formatted.push_sc(sc!(":"));
    formatted.request_single_space();
    //The colon should immediately follow the name, we intentionally move the comment
    formatted.extend(gen_comments(item_comments_after_name));
    formatted.extend(gen_comments(item_comments_after_colon));
    formatted.extend(gen_type_specifier(&item_type_specifier)?);
    Ok(formatted)
}

fn gen_fn_return_type(syntax: &ast::ReturnType) -> FormatDocumentResult<PrintItemBuffer> {
    // ==== Parse ====
    let mut syntax = put_back(syntax.syntax().children_with_tokens());

    parse_token(&mut syntax, SyntaxKind::Arrow);
    let item_comments_after_arrow = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_type_specifier = parse_node::<ast::TypeSpecifier>(&mut syntax)?;
    let item_comments_after_type = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItemBuffer::default();

    formatted.request_single_space();
    formatted.push_sc(sc!("->"));
    formatted.request_single_space();
    formatted.extend(gen_comments(item_comments_after_arrow));
    formatted.extend(gen_type_specifier(&item_type_specifier)?);
    Ok(formatted)
}

fn gen_fn_body(syntax: &ast::CompoundStatement) -> FormatDocumentResult<PrintItemBuffer> {
    //TODO
    todo_verbatim(syntax.syntax())
}

fn gen_type_specifier(
    type_specifier: &ast::TypeSpecifier
) -> FormatDocumentResult<PrintItemBuffer> {
    //TODO
    todo_verbatim(type_specifier.syntax())
}

/// In cases where the formatter is not yet complete we simply output source verbatim.
#[deprecated]
fn todo_verbatim(source: &parser::SyntaxNode) -> FormatDocumentResult<PrintItemBuffer> {
    let mut items = PrintItemBuffer::default();
    items.push_string(source.to_string());
    Ok(items)
}

fn create_is_multiple_lines_resolver(
    start_ln: LineNumber,
    end_ln: LineNumber,
) -> ConditionResolver {
    Rc::new(
        move |condition_context: &mut ConditionResolverContext<'_, '_>| {
            // // no items, so format on the same line
            // if child_positions.is_empty() {
            //   return Some(false);
            // }
            // // first child is on a different line than the start of the parent
            // // so format all the children as multi-line
            // if parent_position.line_number < child_positions[0].line_number {
            //   return Some(true);
            // }

            // check if it spans multiple lines, and if it does then make it multi-line
            condition_helpers::is_multiple_lines(condition_context, start_ln, end_ln)
        },
    )
}
