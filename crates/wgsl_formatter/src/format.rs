//! The entry points to the formatter.

use std::{error::Error, fmt::Display};

use dprint_core::formatting::{PrintItems, PrintOptions};
use parser::{Edition, SyntaxNode};
use rowan::{NodeOrToken, TextRange};
use syntax::{AstNode as _, Parse, ast};

use crate::{
    FormattingOptions, IndentStyle,
    generators::node::{gen_node_with_trivia, gen_node_with_trivia_no_newlines},
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

#[derive(Debug)]
pub enum FormatStringError {
    FormatDocumentError { error: FormatDocumentError },
    ParserErrors { parse: Parse },
}

impl Error for FormatStringError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::FormatDocumentError { error } => Some(error),
            Self::ParserErrors { .. } => None,
        }
    }
}

impl Display for FormatStringError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::FormatDocumentError { error } => {
                write!(
                    f,
                    "Could not format source. This is a bug in the formatter - feel free to open an issue.\nOriginal Error:\n{error}"
                )?;
            },
            Self::ParserErrors { parse } => {
                writeln!(f, "Could not parse source:")?;
                for error in parse.errors() {
                    writeln!(f, "{error}")?;
                }
            },
        }
        Ok(())
    }
}

/// Format the whole string, as if it were a complete source file.
pub fn format_file(
    input: &str,
    options: &FormattingOptions,
) -> Result<String, FormatStringError> {
    let parse = syntax::parse(input, Edition::LATEST);

    if !parse.errors().is_empty() {
        return Err(FormatStringError::ParserErrors { parse });
    }

    let file = parse.tree();
    format_tree(&file, options).map_err(|error| FormatStringError::FormatDocumentError { error })
}

/// Format the whole given `SourceFile`.
pub fn format_tree(
    syntax: &ast::SourceFile,
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

    format(options, || gen_node_with_trivia(&trivia))
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

    // This will contain the actual formatted, but only if output if error is None
    let formatted_if_ok = dprint_core::formatting::format(
        || match format() {
            Ok(items) => items.finish(),
            Err(gen_error) => {
                //We seem to have to do it this weird way, because
                // a) We can't return the error from the closure because of dprint's api
                // b) We can't call gen_source_file outside of the closure because
                //    dprint requires the gen_items to be allocated using a thread local
                //    allocator that only exists within the closure.
                error = Some(gen_error);
                PrintItems::new()
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
        None => Ok(formatted_if_ok),
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use expect_test::expect;
    use syntax::parse;

    use crate::{FormatStringError, reporting::FormatDocumentError};

    #[test]
    pub fn format_string_error_display_on_parser_error() {
        let source = "
            notValidWgsl::String
            notValidWgsl = 'What??'
            ";
        let parse = parse(source, parser::Edition::Wgsl);
        let error = FormatStringError::ParserErrors { parse };

        expect![[r#"
            Could not parse source:
            error at 61..62: unexpected tokens
            error at 66..67: unexpected tokens
            error at 67..68: unexpected tokens
            error at 68..69: unexpected tokens
            error at 13..25: invalid syntax, expected one of: 'alias', '@', '{', 'const', 'const_assert', 'diagnostic', <end of file>, 'enable', 'fn', 'import', 'let', 'override', 'requires', ';', 'struct', 'var'
        "#]].assert_eq(&format!("{error}"));
    }

    #[test]
    pub fn format_string_error_display_on_formatter_error() {
        let error = FormatStringError::FormatDocumentError {
            error: FormatDocumentError::MissingNode,
        };

        expect![[r#"
            Could not format source. This is a bug in the formatter - feel free to open an issue.
            Original Error:
            Expected to find a node but found none"#]]
        .assert_eq(&format!("{error}"));
    }

    #[test]
    pub fn format_string_error_source_format_document_error() {
        let source = FormatDocumentError::MissingNode;
        let error = FormatStringError::FormatDocumentError {
            error: source.clone(),
        };

        assert_eq!(
            error
                .source()
                .and_then(|error| error.downcast_ref::<FormatDocumentError>()),
            Some(source).as_ref()
        );
    }

    #[test]
    pub fn format_string_error_source_parser_error() {
        let source = "
            notValidWgsl::String
            notValidWgsl = 'What??'
            ";
        let parse = parse(source, parser::Edition::Wgsl);
        let error = FormatStringError::ParserErrors { parse };

        assert!(error.source().is_none());
    }
}
