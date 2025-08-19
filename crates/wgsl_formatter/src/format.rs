mod helpers;

use std::{alloc::alloc, iter::repeat_with};

use parser::SyntaxNode;
use pretty::{BoxAllocator, DocAllocator, DocBuilder};
use syntax::{
    AstNode as _, HasName as _,
    ast::{self},
};

use crate::{FormattingOptions, format::helpers::pretty_spaced_lines};

#[must_use]
pub fn format_str(
    input: &str,
    options: &FormattingOptions,
) -> String {
    let parse = syntax::parse(input);
    let file = parse.tree();
    format_tree(&file, options)
}

#[must_use]
pub fn format_tree(
    syntax: &ast::SourceFile,
    options: &FormattingOptions,
) -> String {
    let allocator = BoxAllocator;
    let builder: DocBuilder<'_, _, ()> = pretty_source_file(syntax, &allocator);

    //TODO: I'm sure that there are better ways to stringify the doc,
    // ways that a) can't panic and b) are more efficient.
    // Investigate if render_fmt can actually panic in this circumstance
    // Investigate if render_fmt is at least as performant as a custom Render struct using StringBuilder
    let mut str = String::new();
    builder.render_fmt(options.width, &mut str);
    str
}

fn pretty_item<'ann, D, TAnnotation>(
    node: &ast::Item,
    allocator: &'ann D,
) -> Option<DocBuilder<'ann, D, TAnnotation>>
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
) -> DocBuilder<'ann, D, TAnnotation>
where
    D: DocAllocator<'ann, TAnnotation>,
{
    pretty_spaced_lines(node.syntax(), allocator, |child| {
        //TODO This clone is unnecessary if we had a cast that returned the passed in node
        // on a failure like std::any::Any (SyntaxNode -> Result<Item, Syntaxnode>)
        if let Some(item) = ast::Item::cast(child.clone()) {
            pretty_item(&item, allocator)
                .unwrap_or_else(|| unformatable_source(item.syntax(), allocator))
        } else {
            //TODO There is a case to be made about formatting nonsensical top-level items
            // for now we just leave them alone as to not to annoy the user with misguided formats.
            allocator.text(child.to_string())
        }
    })
}

fn pretty_function_declaration<'ann, D, TAnnotation>(
    node: &ast::FunctionDeclaration,
    allocator: &'ann D,
) -> Option<DocBuilder<'ann, D, TAnnotation>>
where
    D: DocAllocator<'ann, TAnnotation>,
{
    //TODO Don't unwrap but instead:
    //TODO Check if the function declaration is complete - else reemit verbatim syntax
    //Both TODOs can be solved by instead of using name(), parsing throug the node syntax, and casting the respective expected elements.
    let name = node.name()?;
    let name = name.text();
    let node_params = node.parameter_list()?.parameters();

    let formatted_params = node_params
        .map(|param| {
            let p_name = param.name()?;
            let p_type = param.ty()?;
            Some((p_name, p_type))
        })
        .collect::<Option<Vec<_>>>()?;

    let return_type = node.return_type()?.ty()?;

    //TODO Don't to_owned here, but instead specify smarter lifetimes
    let built_fn = allocator
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
        )
        .append(allocator.text(" -> "))
        .append(pretty_type_specifier(&return_type, allocator))
        .append(allocator.text(" "))
        .append(allocator.text("{}"));
    Some(built_fn)
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

/// In cases where there seems to be malformed or incomplete source
/// we simply output it verbatim.
fn unformatable_source<'ann, D, TAnnotation>(
    source: &parser::SyntaxNode,
    allocator: &'ann D,
) -> DocBuilder<'ann, D, TAnnotation>
where
    D: DocAllocator<'ann, TAnnotation>,
{
    allocator.text(source.to_string())
}
