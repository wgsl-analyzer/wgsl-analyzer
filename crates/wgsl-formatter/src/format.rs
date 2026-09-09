//! The entry points to the formatter.

use dprint_core::formatting::{PrintItems, PrintOptions};
use rowan::{NodeOrToken, TextRange};
use syntax::{AstNode as _, SyntaxNode};

use crate::{
    FormattingOptions, IndentStyle,
    generators::node::gen_node_with_trivia_no_newlines,
    ignore::is_ignored_from_within,
    print_item_buffer::PrintItemBuffer,
    reporting::{FormatDocumentError, FormatDocumentResult},
    trivia::{NodeWithTrivia, NodeWithTriviaContent},
};

/// A piece of formatted code, together with info about its covering range.
///
/// See [`format_range`] for details.
#[derive(Clone, Debug)]
pub struct FormattedRange {
    /// The actual range that the formatted text should replace.
    pub range: TextRange,

    /// The formatted text.
    pub formatted: String,
}

/// Format only the given `range` within the `file`.
///
/// This may conservatively also format a little bit of context around the
/// provided range, as the formatter can only format whole `SyntaxNode`s.
pub fn format_range(
    file: &SyntaxNode,
    range: TextRange,
    config: &FormattingOptions,
) -> FormatDocumentResult<FormattedRange> {
    let node = match file.syntax().covering_element(range) {
        NodeOrToken::Node(node) => node,
        NodeOrToken::Token(token) => token.parent().ok_or(FormatDocumentError::MissingNode)?,
    };

    format_node(&node, config).map(|formatted| FormattedRange {
        range: node.text_range(),
        formatted,
    })
}

/// Format the given `SyntaxNode`.
///
/// This strips any surrounding newlines out of the formatted result.
pub fn format_node(
    syntax: &SyntaxNode,
    options: &FormattingOptions,
) -> FormatDocumentResult<String> {
    let mut content = NodeWithTriviaContent::Content(NodeOrToken::Node(syntax.syntax().clone()));
    if is_ignored_from_within(syntax.syntax()) {
        content = NodeWithTriviaContent::IgnoredContent {
            ignore_pragma: None,
            ignored_preceding_trivia: vec![],
            ignored_content: Box::new(content),
        }
    }

    let trivia = NodeWithTrivia {
        preceding_trivia: Vec::new(),
        content,
        succeeding_trivia: Vec::new(),
    };

    format(options, || gen_node_with_trivia_no_newlines(&trivia))
}

fn format<F>(
    options: &FormattingOptions,
    format: F,
) -> FormatDocumentResult<String>
where
    F: FnOnce() -> FormatDocumentResult<PrintItemBuffer>,
{
    let mut error = None;

    let get_print_items = || match format() {
        Ok(items) => items.finish(),
        Err(format_error) => {
            // We have to do it this unintuitive way because:
            // - `dprint`'s API does not expose the error.
            // - Calling `gen_source_file` outside of the closure is invalid.
            //   `dprint` requires that `gen_items` be allocated using a thread-local allocator,
            //    but the allocator only exists within the closure.
            error = Some(format_error);
            PrintItems::new()
        },
    };
    // This will contain the formatted string, but only if error is `None`.
    let formatted_or_empty = dprint_core::formatting::format(
        get_print_items,
        PrintOptions {
            max_width: options.max_line_width,
            indent_width: options.indent_width,
            use_tabs: options.indent_style == IndentStyle::Tabs,
            new_line_text: options.line_break_style.text(),
        },
    );

    match error {
        Some(error) => Err(error),
        None => Ok(formatted_or_empty),
    }
}
