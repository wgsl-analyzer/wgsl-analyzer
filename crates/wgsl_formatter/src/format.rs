mod helpers;
mod reporting;

use std::{alloc::alloc, iter::repeat_with, rc::Rc};

use dprint_core::formatting::{
    ConditionResolver, ConditionResolverContext, LineNumber, LineNumberAnchor, PrintItem,
    PrintItems, PrintOptions, Signal, actions, condition_helpers, condition_resolvers, conditions,
    ir_helpers,
};
use dprint_core_macros::sc;
use parser::{SyntaxKind, SyntaxNode};
use syntax::{
    AstNode as _, HasName as _,
    ast::{self},
    match_ast,
};

use crate::{
    FormattingOptions,
    format::{
        self,
        helpers::{gen_spaced_lines, into_items},
        reporting::{FormatDocumentError, FormatDocumentErrorKind, FormatDocumentResult},
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
        || match gen_source_file(&syntax) {
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
        if let Some(item) = ast::Item::cast(child.clone()) {
            gen_item(&item)
        } else {
            Err(FormatDocumentErrorKind::UnexpectedModuleNode.at(child.text_range()))
        }
    })
}

#[expect(clippy::too_many_lines, reason = "TODO: Shorten function")]
fn gen_function_declaration(node: &ast::FunctionDeclaration) -> FormatDocumentResult<PrintItems> {
    enum FunctionDeclarationState {
        Init,
        HasFn,
        HasName,
        HasParams,
        HasReturnType,
        HasBody,
    }

    let mut state = FunctionDeclarationState::Init;
    let mut formatted = PrintItems::new();

    //TODO This is a *BAD* temporary bandaid, unmaintainable, spaghetti code solution.
    let mut ended_with_space_or_newline = false;

    for child in node.syntax().children_with_tokens() {
        // Comments are valid everywhere, we don't care about the state for them
        if let Some(items) = handle_comments(&child, &mut ended_with_space_or_newline) {
            formatted.extend(items?);
        } else {
            match state {
                FunctionDeclarationState::Init => {
                    if child.kind() == SyntaxKind::Blankspace {
                        // Is allowed, we ignore it
                    } else if child.kind() == SyntaxKind::Fn {
                        formatted.push_sc(sc!("fn"));
                        formatted.push_space();
                        ended_with_space_or_newline = true;
                        state = FunctionDeclarationState::HasFn;
                    } else {
                        return Err(FormatDocumentErrorKind::UnexpectedToken.at(child.text_range()));
                    }
                },
                FunctionDeclarationState::HasFn => {
                    if child.kind() == SyntaxKind::Blankspace {
                        // Is allowed, we ignore it
                    } else if let Some(name) = child.as_node().cloned().and_then(ast::Name::cast) {
                        if (!ended_with_space_or_newline) {
                            formatted.push_space();
                            ended_with_space_or_newline = true;
                        }
                        formatted.push_string(name.text().to_string());
                        ended_with_space_or_newline = false;
                        state = FunctionDeclarationState::HasName;
                    } else {
                        return Err(FormatDocumentErrorKind::UnexpectedToken.at(child.text_range()));
                    }
                },
                FunctionDeclarationState::HasName => {
                    if child.kind() == SyntaxKind::Blankspace {
                        // Is allowed, we ignore it
                    } else if let Some(params) = child
                        .as_node()
                        .cloned()
                        .and_then(ast::FunctionParameters::cast)
                    {
                        formatted.extend(gen_fn_parameters(
                            &params,
                            &mut ended_with_space_or_newline,
                        )?);
                        state = FunctionDeclarationState::HasParams;
                    } else {
                        return Err(FormatDocumentErrorKind::UnexpectedToken.at(child.text_range()));
                    }
                },
                FunctionDeclarationState::HasParams => {
                    if child.kind() == SyntaxKind::Blankspace {
                        // Is allowed, we ignore it
                    } else if let Some(return_type) =
                        child.as_node().cloned().and_then(ast::ReturnType::cast)
                    {
                        if (!ended_with_space_or_newline) {
                            formatted.push_space(); //There is no case where there wouldn't be a space here.
                            ended_with_space_or_newline = true;
                        }
                        formatted.extend(gen_fn_return_type(
                            &return_type,
                            &mut ended_with_space_or_newline,
                        )?);
                        ended_with_space_or_newline = false;
                        state = FunctionDeclarationState::HasReturnType;
                    } else if let Some(body) = child
                        .as_node()
                        .cloned()
                        .and_then(ast::CompoundStatement::cast)
                    {
                        if (!ended_with_space_or_newline) {
                            formatted.push_space(); //There is no case where there wouldn't be a space here.
                            ended_with_space_or_newline = true;
                        }
                        formatted.extend(gen_fn_body(&body)?);
                        ended_with_space_or_newline = false;
                        state = FunctionDeclarationState::HasBody;
                    } else {
                        return Err(FormatDocumentErrorKind::UnexpectedToken.at(child.text_range()));
                    }
                },
                FunctionDeclarationState::HasReturnType => {
                    //TODO This is duplication of the second HasParams clause, because the return type is optional
                    if child.kind() == SyntaxKind::Blankspace {
                        // Is allowed, we ignore it
                    } else if let Some(body) = child
                        .as_node()
                        .cloned()
                        .and_then(ast::CompoundStatement::cast)
                    {
                        if (!ended_with_space_or_newline) {
                            formatted.push_space(); //There is no case where there wouldn't be a space here.
                            ended_with_space_or_newline = true;
                        }
                        formatted.extend(gen_fn_body(&body)?);
                        ended_with_space_or_newline = false;
                        state = FunctionDeclarationState::HasBody;
                    } else {
                        return Err(FormatDocumentErrorKind::UnexpectedToken.at(child.text_range()));
                    }
                },
                FunctionDeclarationState::HasBody => {
                    return Err(FormatDocumentErrorKind::UnexpectedToken.at(child.text_range()));
                },
            }
        }
    }

    if matches!(FunctionDeclarationState::HasBody, state) {
        Ok(formatted)
    } else {
        Err(FormatDocumentErrorKind::MissingTokens.at(node.syntax().text_range()))
    }
}

fn gen_fn_parameters(
    syntax: &ast::FunctionParameters,
    ended_with_space_or_newline: &mut bool,
) -> FormatDocumentResult<PrintItems> {
    let start_ln = LineNumber::new("start");
    let end_ln = LineNumber::new("end");
    let is_multiple_lines = create_is_multiple_lines_resolver(start_ln, end_ln);

    let mut formatted = PrintItems::new();

    // formatted.extend(actions::if_column_number_changes(move |context| {
    //     context.clear_info(end_ln);
    // }));

    formatted.push_info(start_ln);
    formatted.push_anchor(LineNumberAnchor::new(end_ln));

    formatted.push_sc(sc!("("));
    let mut start_nl_condition =
        conditions::if_true("paramStartNewLine", Rc::clone(&is_multiple_lines), {
            let mut pi = PrintItems::default();
            pi.push_signal(Signal::NewLine);
            pi.push_signal(Signal::StartIndent);
            pi
        });
    let start_reeval = start_nl_condition.create_reevaluation();
    formatted.push_condition(start_nl_condition);
    formatted.push_signal(Signal::StartNewLineGroup);

    let mut queued_comma = false;

    for child in syntax.syntax().children_with_tokens() {
        if child.kind() == SyntaxKind::Blankspace
            || child.kind() == SyntaxKind::ParenthesisLeft
            || child.kind() == SyntaxKind::ParenthesisRight
        {
            // Is allowed, we ignore it
        } else if let Some(items) = handle_comments(&child, ended_with_space_or_newline) {
            //TODO ended with space or newline is out of date when its passed to hadle_comments
            if queued_comma {
                queued_comma = false;
                formatted.push_sc(sc!(", "));
                *ended_with_space_or_newline = true;
            }
            formatted.extend(items?);
        } else if child.kind() == SyntaxKind::Comma {
            //TODO queued_comma = Some(allocator.text(",").append(allocator.line()));
        } else if let Some(parameter) = child.as_node().cloned().and_then(ast::Parameter::cast) {
            if queued_comma {
                queued_comma = false;
                formatted.push_sc(sc!(","));
                formatted.push_condition(conditions::if_true_or(
                    "afterCommaSeparator",
                    Rc::clone(&is_multiple_lines),
                    Signal::NewLine.into(),
                    Signal::SpaceOrNewLine.into(),
                ));
            }
            formatted.extend(gen_fn_parameter(&parameter, ended_with_space_or_newline)?);
            queued_comma = true;
        } else {
            return Err(FormatDocumentErrorKind::UnexpectedToken.at(child.text_range()));
        }
    }

    if queued_comma {
        queued_comma = false;
        formatted.push_condition(conditions::if_true(
            "paramTrailingComma",
            is_multiple_lines.clone(),
            {
                let mut pi = PrintItems::default();
                pi.push_sc(sc!(","));
                pi.push_signal(Signal::NewLine);
                pi
            },
        ));
    }

    formatted.push_condition(conditions::if_true(
        "paramTrailingComma",
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
    *ended_with_space_or_newline = false;

    Ok(formatted)
}

fn gen_fn_parameter(
    syntax: &ast::Parameter,
    ended_with_space_or_newline: &mut bool,
) -> FormatDocumentResult<PrintItems> {
    enum ParameterState {
        Init,
        HasName,
        HasType,
    }

    let mut state = ParameterState::Init;
    let mut formatted = PrintItems::default();

    for child in syntax.syntax().children_with_tokens() {
        if let Some(items) = handle_comments(&child, ended_with_space_or_newline) {
            formatted.extend(items?);
        } else {
            match state {
                ParameterState::Init => {
                    if child.kind() == SyntaxKind::Blankspace
                        || child.kind() == SyntaxKind::ParenthesisLeft
                    {
                        // Is allowed, we ignore it
                    } else if let Some(name) = child.as_node().cloned().and_then(ast::Name::cast) {
                        formatted.push_string(name.text().to_string());
                        formatted.push_sc(sc!(": "));
                        *ended_with_space_or_newline = true;
                        state = ParameterState::HasName;
                    } else {
                        return Err(FormatDocumentErrorKind::UnexpectedToken.at(child.text_range()));
                    }
                },
                ParameterState::HasName => {
                    if child.kind() == SyntaxKind::Blankspace || child.kind() == SyntaxKind::Colon {
                        // Is allowed, we ignore it
                    } else if let Some(type_specifier) =
                        child.as_node().cloned().and_then(ast::TypeSpecifier::cast)
                    {
                        formatted.extend(gen_type_specifier(&type_specifier)?);
                        state = ParameterState::HasType;
                    } else {
                        return Err(FormatDocumentErrorKind::UnexpectedToken.at(child.text_range()));
                    }
                },
                ParameterState::HasType => {
                    //TODO This duplicates INIT state
                    if child.kind() == SyntaxKind::Blankspace || child.kind() == SyntaxKind::Comma {
                        // Is allowed, we ignore it
                    } else if let Some(name) = child.as_node().cloned().and_then(ast::Name::cast) {
                        formatted.push_string(name.text().to_string());
                        formatted.push_sc(sc!(": "));
                        *ended_with_space_or_newline = true;
                        state = ParameterState::HasName;
                    } else {
                        return Err(FormatDocumentErrorKind::UnexpectedToken.at(child.text_range()));
                    }
                },
            }
        }
    }

    if matches!(ParameterState::HasType, state) {
        Ok(formatted)
    } else {
        Err(FormatDocumentErrorKind::MissingTokens.at(syntax.syntax().text_range()))
    }
}

fn gen_fn_return_type(
    syntax: &ast::ReturnType,
    ended_with_space_or_newline: &mut bool,
) -> FormatDocumentResult<PrintItems> {
    //TODO
    let mut formatted = PrintItems::default();
    for child in syntax.syntax().children_with_tokens() {
        if let Some(items) = handle_comments(&child, ended_with_space_or_newline) {
            formatted.extend(items?);
        } else if child.kind() == SyntaxKind::Blankspace {
            //Allowed, ignored
        } else if child.kind() == SyntaxKind::Arrow {
            if !*ended_with_space_or_newline {
                formatted.push_space();
            }
            formatted.push_sc(sc!("->"));
            formatted.push_space();
            *ended_with_space_or_newline = true
        } else if let Some(type_specifier) =
            child.as_node().cloned().and_then(ast::TypeSpecifier::cast)
        {
            formatted.extend(todo_verbatim(type_specifier.syntax())?);
        } else {
            return Err(FormatDocumentErrorKind::UnexpectedToken.at(child.text_range()));
        }
    }
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

fn handle_comments(
    source: &rowan::NodeOrToken<parser::SyntaxNode, parser::SyntaxToken>,
    ended_with_space_or_newline: &mut bool,
) -> Option<FormatDocumentResult<PrintItems>> {
    if (source.kind() == SyntaxKind::BlockComment) {
        Some(gen_block_comment(source, ended_with_space_or_newline))
    } else if (source.kind() == SyntaxKind::LineEndingComment) {
        Some(gen_line_ending_comment(source, ended_with_space_or_newline))
    } else {
        None
    }
}

fn gen_block_comment(
    source: &rowan::NodeOrToken<parser::SyntaxNode, parser::SyntaxToken>,
    ended_with_space_or_newline: &mut bool,
) -> FormatDocumentResult<PrintItems> {
    let mut items = PrintItems::default();
    if (!*ended_with_space_or_newline) {
        items.push_space();
        *ended_with_space_or_newline = true;
    }
    items.push_string(source.to_string());
    items.push_space();
    *ended_with_space_or_newline = true;
    Ok(items)
}

fn gen_line_ending_comment(
    source: &rowan::NodeOrToken<parser::SyntaxNode, parser::SyntaxToken>,
    ended_with_space_or_newline: &mut bool,
) -> FormatDocumentResult<PrintItems> {
    let mut items = PrintItems::default();
    // dbg!(source.to_string());
    // items.push_condition(conditions::if_false(
    //     "space_if_not_start_of_line",
    //     condition_resolvers::is_start_of_line_or_is_start_of_line_indented(),
    //     {
    //         let mut pi = PrintItems::new();
    //         pi.push_space();
    //         pi
    //     },
    // ));

    if (!*ended_with_space_or_newline) {
        items.push_space();
        *ended_with_space_or_newline = true;
    }
    items.push_string(source.to_string());
    items.push_signal(Signal::ExpectNewLine);
    *ended_with_space_or_newline = true;
    Ok(items)
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
