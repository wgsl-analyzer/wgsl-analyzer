mod helpers;
mod reporting;

use std::{alloc::alloc, iter::repeat_with};

use parser::{SyntaxKind, SyntaxNode};
use pretty::{BoxAllocator, DocAllocator, DocBuilder};
use syntax::{
    AstNode as _, HasName as _,
    ast::{self},
    match_ast,
};

use crate::{
    FormattingOptions,
    format::{
        self,
        helpers::pretty_spaced_lines,
        reporting::{FormatDocumentError, FormatDocumentErrorKind, FormatDocumentResult},
    },
};

pub fn format_str(
    input: &str,
    options: &FormattingOptions,
) -> FormatDocumentResult<String> {
    let parse = syntax::parse(input);
    let file = parse.tree();
    format_tree(&file, options)
}

pub fn format_tree(
    syntax: &ast::SourceFile,
    options: &FormattingOptions,
) -> FormatDocumentResult<String> {
    let allocator = BoxAllocator;
    let builder: DocBuilder<'_, _, ()> = pretty_source_file(syntax, &allocator)?;

    //TODO: I'm sure that there are better ways to stringify the doc,
    // ways that a) can't panic and b) are more efficient.
    // Investigate if render_fmt can actually panic in this circumstance
    // Investigate if render_fmt is at least as performant as a custom Render struct using StringBuilder
    let mut str = String::new();
    builder.render_fmt(options.width, &mut str);
    Ok(str)
}

fn pretty_item<'ann, D, TAnnotation>(
    node: &ast::Item,
    allocator: &'ann D,
) -> FormatDocumentResult<DocBuilder<'ann, D, TAnnotation>>
where
    D: DocAllocator<'ann, TAnnotation>,
{
    match node {
        ast::Item::FunctionDeclaration(function_declaration) => {
            pretty_function_declaration(function_declaration, allocator)
        },
        ast::Item::VariableDeclaration(variable_declaration) => todo!(),
        ast::Item::ConstantDeclaration(constant_declaration) => todo!(),
        ast::Item::OverrideDeclaration(override_declaration) => todo!(),
        ast::Item::TypeAliasDeclaration(type_alias_declaration) => todo!(),
        ast::Item::StructDeclaration(struct_declaration) => todo!(),
    }
}

fn pretty_source_file<'ann, D, TAnnotation>(
    node: &ast::SourceFile,
    allocator: &'ann D,
) -> FormatDocumentResult<DocBuilder<'ann, D, TAnnotation>>
where
    D: DocAllocator<'ann, TAnnotation>,
{
    pretty_spaced_lines(node.syntax(), allocator, |child| {
        //TODO This clone is unnecessary if we had a cast that returned the passed in node
        // on a failure like std::any::Any (SyntaxNode -> Result<Item, Syntaxnode>)
        if let Some(item) = ast::Item::cast(child.clone()) {
            pretty_item(&item, allocator)
        } else {
            Err(FormatDocumentErrorKind::UnexpectedModuleNode.at(child.text_range()))
        }
    })
}

fn pretty_function_declaration<'ann, D, TAnnotation>(
    node: &ast::FunctionDeclaration,
    allocator: &'ann D,
) -> FormatDocumentResult<DocBuilder<'ann, D, TAnnotation>>
where
    D: DocAllocator<'ann, TAnnotation>,
{
    enum FunctionDeclarationState {
        Init,
        HasFn,
        HasName,
        HasParams,
        HasReturnType,
        HasBody,
    }

    let mut state = FunctionDeclarationState::Init;
    let mut formatted = allocator.nil();

    for child in node.syntax().children_with_tokens() {
        match state {
            FunctionDeclarationState::Init => {
                if child.kind() == SyntaxKind::Blankspace {
                    // Is allowed, we ignore it
                } else if child.kind() == SyntaxKind::Fn {
                    formatted = formatted
                        .append(allocator.text("fn"))
                        .append(allocator.space());
                    state = FunctionDeclarationState::HasFn;
                } else {
                    return Err(FormatDocumentErrorKind::UnexpectedToken.at(child.text_range()));
                }
            },
            FunctionDeclarationState::HasFn => {
                if child.kind() == SyntaxKind::Blankspace {
                    // Is allowed, we ignore it
                } else if let Some(name) = child.as_node().cloned().and_then(ast::Name::cast) {
                    formatted = formatted.append(allocator.text(name.text().to_string()));
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
                    formatted = formatted.append(pretty_fn_parameters(&params, allocator)?);
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
                    formatted = formatted
                        .append(allocator.space()) //There is no case where there wouldn't be a space here.
                        .append(pretty_fn_return_type(&return_type, allocator)?);
                    state = FunctionDeclarationState::HasReturnType;
                } else if let Some(body) = child
                    .as_node()
                    .cloned()
                    .and_then(ast::CompoundStatement::cast)
                {
                    formatted = formatted
                        .append(allocator.space()) //There is no case where there wouldn't be a space here.
                        .append(pretty_fn_body(&body, allocator)?);
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
                    formatted = formatted
                        .append(allocator.space()) //There is no case where there wouldn't be a space here.
                        .append(pretty_fn_body(&body, allocator)?);
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

    if matches!(FunctionDeclarationState::HasBody, state) {
        Ok(formatted)
    } else {
        Err(FormatDocumentErrorKind::MissingTokens.at(node.syntax().text_range()))
    }
}

fn pretty_fn_parameters<'ann, D, TAnnotation>(
    syntax: &ast::FunctionParameters,
    allocator: &'ann D,
) -> FormatDocumentResult<DocBuilder<'ann, D, TAnnotation>>
where
    D: DocAllocator<'ann, TAnnotation>,
{
    let mut formatted = allocator.line_();

    let mut queued_comma = None;

    for child in syntax.syntax().children_with_tokens() {
        if child.kind() == SyntaxKind::Blankspace
            || child.kind() == SyntaxKind::ParenthesisLeft
            || child.kind() == SyntaxKind::ParenthesisRight
        {
            // Is allowed, we ignore it
        } else if child.kind() == SyntaxKind::Comma {
            queued_comma = Some(allocator.text(",").append(allocator.line()));
        } else if let Some(parameter) = child.as_node().cloned().and_then(ast::Parameter::cast) {
            formatted = formatted
                .append(std::mem::take(&mut queued_comma))
                .append(pretty_fn_parameter(&parameter, allocator)?);
        } else {
            return Err(FormatDocumentErrorKind::UnexpectedToken.at(child.text_range()));
        }
    }

    formatted = formatted
        .append(allocator.text(",").flat_alt(allocator.nil()))
        .append(allocator.line_());

    //Both states are fine
    Ok(formatted.group().nest(4).parens())
}

fn pretty_fn_parameter<'ann, D, TAnnotation>(
    syntax: &ast::Parameter,
    allocator: &'ann D,
) -> FormatDocumentResult<DocBuilder<'ann, D, TAnnotation>>
where
    D: DocAllocator<'ann, TAnnotation>,
{
    enum ParameterState {
        Init,
        HasName,
        HasType,
    }

    let mut state = ParameterState::Init;
    let mut formatted = allocator.nil();

    for child in syntax.syntax().children_with_tokens() {
        match state {
            ParameterState::Init => {
                if child.kind() == SyntaxKind::Blankspace
                    || child.kind() == SyntaxKind::ParenthesisLeft
                {
                    // Is allowed, we ignore it
                } else if let Some(name) = child.as_node().cloned().and_then(ast::Name::cast) {
                    formatted = formatted
                        .append(allocator.text(name.text().to_string()))
                        .append(allocator.text(": "));
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
                    formatted =
                        formatted.append(pretty_type_specifier(&type_specifier, allocator)?);
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
                    formatted = formatted.append(allocator.text(name.text().to_string()));
                    state = ParameterState::HasName;
                } else {
                    return Err(FormatDocumentErrorKind::UnexpectedToken.at(child.text_range()));
                }
            },
        }
    }

    if matches!(ParameterState::HasType, state) {
        Ok(formatted)
    } else {
        Err(FormatDocumentErrorKind::MissingTokens.at(syntax.syntax().text_range()))
    }
}

fn pretty_fn_return_type<'ann, D, TAnnotation>(
    syntax: &ast::ReturnType,
    allocator: &'ann D,
) -> FormatDocumentResult<DocBuilder<'ann, D, TAnnotation>>
where
    D: DocAllocator<'ann, TAnnotation>,
{
    //TODO
    todo_verbatim(syntax.syntax(), allocator)
}

fn pretty_fn_body<'ann, D, TAnnotation>(
    syntax: &ast::CompoundStatement,
    allocator: &'ann D,
) -> FormatDocumentResult<DocBuilder<'ann, D, TAnnotation>>
where
    D: DocAllocator<'ann, TAnnotation>,
{
    //TODO
    todo_verbatim(syntax.syntax(), allocator)
}

fn pretty_type_specifier<'ann, D, TAnnotation>(
    type_specifier: &ast::TypeSpecifier,
    allocator: &'ann D,
) -> FormatDocumentResult<DocBuilder<'ann, D, TAnnotation>>
where
    D: DocAllocator<'ann, TAnnotation>,
{
    //TODO
    todo_verbatim(type_specifier.syntax(), allocator)
}

/// In cases where the formatter is not yet complete we simply output source verbatim.
#[deprecated]
fn todo_verbatim<'ann, D, TAnnotation>(
    source: &parser::SyntaxNode,
    allocator: &'ann D,
) -> FormatDocumentResult<DocBuilder<'ann, D, TAnnotation>>
where
    D: DocAllocator<'ann, TAnnotation>,
{
    Ok(allocator.text(source.to_string()))
}
