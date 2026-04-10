// We re-enable a warn lint within the format.rs because it is very easy to parse an item within a gen_*-function and
// then forget to print it to the PrintItemBuffer.
// Also it is very easy to forget a "?" after a parse_*-function, that would be caught by
// "unused std::result::Result that must be used"
#![warn(unused)]

mod ast_parse;
mod helpers;
mod print_item_buffer;

pub mod gen_assignment_statement;
mod gen_attributes;
mod gen_comments;
mod gen_diagnostic;
mod gen_directive;
mod gen_expression;
mod gen_function;
mod gen_function_call;
mod gen_if_statement;
pub mod gen_import;
mod gen_node;
mod gen_path;
mod gen_source_file;
mod gen_statement;
pub mod gen_statement_compound;
mod gen_statement_import;
mod gen_struct;
pub mod gen_switch_statement;
mod gen_type_alias_declaration;
mod gen_types;
mod gen_var_let_const_override_statement;
pub mod multiline_group;
mod reporting;

use dprint_core::formatting::PrintOptions;
use parser::{Edition, SyntaxNode};
use rowan::{NodeOrToken, TextRange};
use syntax::{AstNode as _, Parse, ast};

use crate::{
    FormattingOptions, IndentStyle,
    format::{
        gen_node::{gen_node, gen_node_no_newlines},
        print_item_buffer::PrintItemBuffer,
        reporting::{FormatDocumentError, FormatDocumentResult},
    },
};

#[derive(Clone, Debug)]
pub struct FormattedRange {
    /// The actual range that the formatted text should replace.
    pub range: TextRange,

    /// The formatted text.
    pub formatted: String,
}

#[derive(Debug)]
pub enum FormatStringError {
    FormatDocumentError { error: FormatDocumentError },
    ParserErrors { parse: Parse },
}

pub fn format_range(
    file: &SyntaxNode,
    range: Option<TextRange>,
    config: &FormattingOptions,
) -> FormatDocumentResult<FormattedRange> {
    let node = match range {
        None => file.syntax().clone(),
        Some(range) => match file.syntax().covering_element(range) {
            NodeOrToken::Node(node) => node,
            NodeOrToken::Token(token) => token.parent().ok_or(FormatDocumentError::MissingNode)?,
        },
    };

    format_node(&node, config).map(|formatted| FormattedRange {
        range: node.text_range(),
        formatted,
    })
}

pub fn format_file(
    input: &str,
    options: &FormattingOptions,
) -> Result<String, FormatStringError> {
    let parse = syntax::parse(input, Edition::LATEST);
    //TODO Return error if the syntax could not parse.

    if !parse.errors().is_empty() {
        return Err(FormatStringError::ParserErrors { parse });
    }

    let file = parse.tree();
    format_tree(&file, options).map_err(|error| FormatStringError::FormatDocumentError { error })
}

pub fn format_tree(
    syntax: &ast::SourceFile,
    options: &FormattingOptions,
) -> FormatDocumentResult<String> {
    format(options, || gen_node(syntax.syntax()))
}

pub fn format_node(
    syntax: &SyntaxNode,
    options: &FormattingOptions,
) -> FormatDocumentResult<String> {
    format(options, || gen_node_no_newlines(syntax))
}

pub fn format<F>(
    options: &FormattingOptions,
    format: F,
) -> FormatDocumentResult<String>
where
    F: FnOnce() -> FormatDocumentResult<PrintItemBuffer>,
{
    let mut error = None;

    let formatted = dprint_core::formatting::format(
        || match format() {
            Ok(items) => items.finish(),
            Err(gen_error) => {
                //We seem to have to do it this weird way, because
                // a) We can't return the error from the closure because of dprint's api
                // b) We can't call gen_source_file outside of the closure because
                //    dprint requires the gen_items to be allocated using a thread local
                //    allocator that only exists within the closure.
                let _none = error.insert(gen_error);

                //TODO maybe we should instead output the whole source verbatim
                // so that if many things go wrong and this value does somehow
                // reach the user's file, it doesn't just delete it all.
                let mut items = PrintItemBuffer::new();
                items.push_string("ERROR".into());
                items.finish()
            },
        },
        PrintOptions {
            max_width: options.max_line_width,
            indent_width: options.indent_width,
            use_tabs: options.indent_style == IndentStyle::Tabs,
            new_line_text: options.line_break_style.text(),
        },
    );

    match error {
        Some(error) => Err(error),
        None => Ok(formatted),
    }
}
