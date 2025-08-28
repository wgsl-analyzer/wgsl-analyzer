#![expect(
    clippy::branches_sharing_code,
    reason = "Its helpful to explicitly state intent here."
)]
mod ast_parse;
mod helpers;

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
            Ok(items) => items,
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
                let mut items = PrintItems::new();
                items.push_string("ERROR".into());
                items
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

fn gen_item(node: &ast::Item) -> FormatDocumentResult<PrintItems> {
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

fn gen_source_file(node: &ast::SourceFile) -> FormatDocumentResult<PrintItems> {
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
            Ok(gen_comment(child, &mut true))
        } else {
            Err(FormatDocumentErrorKind::UnexpectedModuleNode.at(child.text_range(), err_src!()))
        }
    })
}

fn gen_function_declaration(node: &ast::FunctionDeclaration) -> FormatDocumentResult<PrintItems> {
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

    //TODO This is very bad spaghetti, unmaintainable, brittle code, remove this asap
    let mut last_item_was_space_or_newline = false;

    let mut formatted = PrintItems::new();

    // Fn
    formatted.push_sc(sc!("fn "));
    last_item_was_space_or_newline = true;
    formatted.extend(gen_comments(
        item_comments_after_fn,
        &mut last_item_was_space_or_newline,
    ));

    // Name
    if !last_item_was_space_or_newline {
        formatted.push_space();
    }
    formatted.push_string(item_name.text().to_string());
    last_item_was_space_or_newline = false;
    formatted.extend(gen_comments(
        item_comments_after_name,
        &mut last_item_was_space_or_newline,
    ));

    // Params
    formatted.extend(gen_fn_parameters(
        &item_params,
        &mut last_item_was_space_or_newline,
    )?);
    last_item_was_space_or_newline = false;
    formatted.extend(gen_comments(
        item_comments_after_params,
        &mut last_item_was_space_or_newline,
    ));

    // Return
    if let Some(item_return) = item_return {
        formatted.extend(gen_fn_return_type(
            &item_return,
            &mut last_item_was_space_or_newline,
        )?);
        last_item_was_space_or_newline = false;
    }
    formatted.extend(gen_comments(
        item_comments_after_return,
        &mut last_item_was_space_or_newline,
    ));
    if !last_item_was_space_or_newline {
        formatted.push_space();
        last_item_was_space_or_newline = true;
    }

    // Body
    formatted.extend(gen_fn_body(&item_body)?);
    last_item_was_space_or_newline = false;

    Ok(formatted)
}

fn gen_comments(
    comments: Vec<SyntaxToken>,
    last_item_was_space_or_newline: &mut bool,
) -> PrintItems {
    let mut formatted = PrintItems::new();
    for item in comments {
        formatted.extend(gen_comment(&item, last_item_was_space_or_newline));
    }
    formatted
}
fn gen_comment(
    item: &SyntaxToken,
    last_item_was_space_or_newline: &mut bool,
) -> PrintItems {
    let mut formatted = PrintItems::new();
    if item.kind() == SyntaxKind::BlockComment {
        if !*last_item_was_space_or_newline {
            formatted.push_space();
        }
        formatted.push_string(item.to_string());
        *last_item_was_space_or_newline = false;
    } else if item.kind() == SyntaxKind::LineEndingComment {
        if !*last_item_was_space_or_newline {
            formatted.push_space();
        }
        formatted.push_string(item.to_string());
        formatted.push_signal(Signal::ExpectNewLine);
        *last_item_was_space_or_newline = true;
    } else {
        //TODO Make this unrepresentable
        unreachable!("Non comment entry found in comments Vec");
    }
    formatted
}

#[expect(clippy::too_many_lines, reason = "TODO")]
fn gen_fn_parameters(
    node: &ast::FunctionParameters,
    forbid_space: &mut bool,
) -> FormatDocumentResult<PrintItems> {
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

    let mut formatted = PrintItems::new();

    formatted.push_info(start_ln);
    formatted.push_anchor(LineNumberAnchor::new(end_ln));

    formatted.push_sc(sc!("("));
    *forbid_space = true;

    let mut start_nl_condition = conditions::if_true(
        "paramMultilineStartIndent",
        Rc::clone(&is_multiple_lines),
        {
            let mut pi = PrintItems::default();
            pi.push_signal(Signal::NewLine);
            pi.push_signal(Signal::StartIndent);
            pi
        },
    );
    let start_reeval = start_nl_condition.create_reevaluation();
    formatted.push_condition(start_nl_condition);
    formatted.push_signal(Signal::StartNewLineGroup);

    formatted.extend(gen_comments(item_comments_start, forbid_space));

    for (pos, (item_parameter, item_comments_after_param, item_comments_after_comma)) in
        item_parameters.into_iter().with_position()
    {
        if !*forbid_space {
            formatted.push_condition(conditions::if_true_or(
                "paramTrailingComma",
                Rc::clone(&is_multiple_lines),
                {
                    let mut pi = PrintItems::default();
                    pi.push_signal(Signal::NewLine);
                    pi
                },
                {
                    let mut pi = PrintItems::default();
                    pi.push_signal(Signal::SpaceOrNewLine);
                    pi
                },
            ));
            *forbid_space = true;
        }

        formatted.extend(gen_fn_parameter(&item_parameter, forbid_space)?);
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
            *forbid_space = false; //This is a lie, because due to the conditional we don't know... but currently we can't do anything about this.
        } else {
            formatted.push_sc(sc!(","));
            *forbid_space = false;
        }

        //The comma should be immediately after the parameter, we move the comment back
        formatted.extend(gen_comments(item_comments_after_param, forbid_space));
        formatted.extend(gen_comments(item_comments_after_comma, forbid_space));
    }
    formatted.extend(gen_comments(item_comments_after_params, forbid_space));

    if !*forbid_space {
        formatted.push_condition(conditions::if_true(
            "paramMultilineLastNewline",
            Rc::clone(&is_multiple_lines),
            {
                let mut pi = PrintItems::default();
                pi.push_signal(Signal::NewLine);
                pi
            },
        ));
        *forbid_space = true; //TODO This is a lie, but because of the conditional we can't do better currently
    }

    formatted.push_condition(conditions::if_true(
        "paramMultilineEndIndent",
        is_multiple_lines,
        {
            let mut pi = PrintItems::default();
            pi.push_signal(Signal::FinishIndent);
            pi
        },
    ));

    formatted.push_signal(Signal::FinishNewLineGroup);
    formatted.push_sc(sc!(")"));
    formatted.push_info(end_ln);
    formatted.push_reevaluation(start_reeval);
    *forbid_space = false;

    Ok(formatted)
}

fn gen_fn_parameter(
    syntax: &ast::Parameter,
    ended_with_space_or_newline: &mut bool,
) -> FormatDocumentResult<PrintItems> {
    // ==== Parse ====
    let mut syntax = put_back(syntax.syntax().children_with_tokens());

    let item_name = parse_node::<ast::Name>(&mut syntax)?;
    let item_comments_after_name = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_token(&mut syntax, SyntaxKind::Colon);
    let item_comments_after_colon = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_type_specifier = parse_node::<ast::TypeSpecifier>(&mut syntax)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItems::default();

    formatted.push_string(item_name.text().to_string());
    formatted.push_sc(sc!(": "));
    *ended_with_space_or_newline = true;
    //The colon should immediately follow the name, we intentionally move the comment
    formatted.extend(gen_comments(
        item_comments_after_name,
        ended_with_space_or_newline,
    ));
    formatted.extend(gen_comments(
        item_comments_after_colon,
        ended_with_space_or_newline,
    ));
    formatted.extend(gen_type_specifier(&item_type_specifier)?);
    *ended_with_space_or_newline = false;
    Ok(formatted)
}

fn gen_fn_return_type(
    syntax: &ast::ReturnType,
    ended_with_space_or_newline: &mut bool,
) -> FormatDocumentResult<PrintItems> {
    // ==== Parse ====
    let mut syntax = put_back(syntax.syntax().children_with_tokens());

    parse_token(&mut syntax, SyntaxKind::Arrow);
    let item_comments_after_arrow = parse_many_comments_and_blankspace(&mut syntax)?;
    let item_type_specifier = parse_node::<ast::TypeSpecifier>(&mut syntax)?;
    let item_comments_after_type = parse_many_comments_and_blankspace(&mut syntax)?;
    parse_end(&mut syntax)?;

    // ==== Format ====
    let mut formatted = PrintItems::default();

    if !*ended_with_space_or_newline {
        formatted.push_space();
    }
    formatted.push_sc(sc!("-> "));
    *ended_with_space_or_newline = true;
    formatted.extend(gen_comments(
        item_comments_after_arrow,
        ended_with_space_or_newline,
    ));
    formatted.extend(gen_type_specifier(&item_type_specifier)?);
    *ended_with_space_or_newline = false;
    Ok(formatted)
}

fn gen_fn_body(syntax: &ast::CompoundStatement) -> FormatDocumentResult<PrintItems> {
    //TODO
    todo_verbatim(syntax.syntax())
}

fn gen_type_specifier(type_specifier: &ast::TypeSpecifier) -> FormatDocumentResult<PrintItems> {
    //TODO
    todo_verbatim(type_specifier.syntax())
}

/// In cases where the formatter is not yet complete we simply output source verbatim.
#[deprecated]
fn todo_verbatim(source: &parser::SyntaxNode) -> FormatDocumentResult<PrintItems> {
    let mut items = PrintItems::default();
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
