mod helpers;
mod reporting;

use std::{alloc::alloc, iter::repeat_with};

use parser::SyntaxNode;
use pretty::{BoxAllocator, DocAllocator, DocBuilder};
use syntax::{
    AstNode as _, HasName as _,
    ast::{self},
};

use crate::{
    FormattingOptions,
    format::{
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
            Err(FormatDocumentError {
                error_kind: reporting::FormatDocumentErrorKind::UnexpectedModuleNode,
                syntax_node: child,
            })
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
    let name = node
        .name()
        .ok_or_else(|| FormatDocumentErrorKind::MissingFnName.at(node.syntax().clone()))?;
    let name = name.text();
    let node_params = node
        .parameter_list()
        .ok_or_else(|| FormatDocumentErrorKind::MissingFnParams.at(node.syntax().clone()))?
        .parameters();

    let formatted_params = node_params
        .map(|param| {
            let p_name = param.name().ok_or_else(|| {
                FormatDocumentErrorKind::MissingFnParamName.at(param.syntax().clone())
            })?;
            let p_type = param.ty().ok_or_else(|| {
                FormatDocumentErrorKind::MissingFnParamType.at(param.syntax().clone())
            })?;
            Ok((p_name, p_type))
        })
        .collect::<FormatDocumentResult<Vec<_>>>()?;

    let return_type = node.return_type().and_then(|return_type| return_type.ty());

    //TODO Don't to_owned here, but instead specify smarter lifetimes
    let mut built_fn = allocator
        .text("fn ")
        .append(allocator.text(name.as_str().to_owned()))
        .append(
            allocator
                .intersperse(
                    formatted_params.iter().map(|(param_name, param_type)| {
                        allocator
                            .text(param_name.text().as_str().to_owned())
                            .append(allocator.text(": "))
                            .append(pretty_type_specifier(param_type, allocator))
                    }),
                    ", ",
                )
                .parens(),
        );
    if let Some(return_type) = return_type {
        built_fn = built_fn
            .append(allocator.text(" -> "))
            .append(pretty_type_specifier(&return_type, allocator));
    }
    built_fn = built_fn
        .append(allocator.text(" "))
        .append(allocator.text("{}"));
    Ok(built_fn)
}

fn pretty_fn_parameters<'ann, D, TAnnotation>(
    type_specifier: &ast::TypeSpecifier,
    allocator: &'ann D,
) -> DocBuilder<'ann, D, TAnnotation>
where
    D: DocAllocator<'ann, TAnnotation>,
{
    //TODO
    allocator.text(type_specifier.syntax().to_string())
}

fn pretty_type_specifier<'ann, D, TAnnotation>(
    type_specifier: &ast::TypeSpecifier,
    allocator: &'ann D,
) -> DocBuilder<'ann, D, TAnnotation>
where
    D: DocAllocator<'ann, TAnnotation>,
{
    //TODO
    allocator.text(type_specifier.syntax().to_string())
}

/// In cases where the formatter is not yet complete we simply output source verbatim.
#[deprecated]
fn todo_verbatim<'ann, D, TAnnotation>(
    source: &parser::SyntaxNode,
    allocator: &'ann D,
) -> DocBuilder<'ann, D, TAnnotation>
where
    D: DocAllocator<'ann, TAnnotation>,
{
    allocator.text(source.to_string())
}
