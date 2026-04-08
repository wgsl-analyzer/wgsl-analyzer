//! WGSL source code formatter.
//!
//! Provides opinionated formatting for WGSL shader source code, normalizing
//! whitespace, indentation, and punctuation while preserving semantic meaning.
//!
//! # Usage
//!
//! ```ignore
//! use wgsl_formatter::{format_str, FormattingOptions};
//!
//! let formatted = format_str("fn  main( ) {  }", &FormattingOptions::default());
//! assert_eq!(formatted, "fn main() {}\n");
//! ```

mod format;
mod util;

use rowan::WalkEvent;
use syntax::{AstNode as _, SyntaxKind, SyntaxNode, ast};

/// Formats a WGSL/WESL source string and returns the formatted result.
///
/// Parses with `Edition::LATEST` so that all syntax (including WESL
/// extensions like imports and qualified paths) is recognized and
/// formatted correctly regardless of file extension.
#[must_use]
pub fn format_str(
    input: &str,
    options: &FormattingOptions,
) -> String {
    let parse = syntax::parse(input, syntax::Edition::LATEST);
    let node = parse.syntax().clone_for_update();
    format_recursive(&node, options);
    let mut result = node.to_string();

    // File-level normalization: strip leading blank lines.
    let trimmed = result.trim_start_matches(['\n', '\r']);
    if trimmed.len() != result.len() {
        result = trimmed.to_owned();
    }

    // Ensure exactly one trailing newline.
    let trimmed_end = result.trim_end_matches(['\n', '\r']);
    result.truncate(trimmed_end.len());
    result.push('\n');

    result
}

/// Configuration options for the WGSL formatter.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FormattingOptions {
    /// How to handle trailing commas in parameter and argument lists.
    #[cfg_attr(feature = "serde", serde(alias = "trailingCommas"))]
    pub trailing_commas: Policy,
    /// The string used for one level of indentation (e.g. `"    "` or `"\t"`).
    #[cfg_attr(feature = "serde", serde(alias = "indentSymbol"))]
    pub indent_symbol: String,
}

impl Default for FormattingOptions {
    fn default() -> Self {
        Self {
            trailing_commas: Policy::Ignore,
            indent_symbol: "    ".to_owned(),
        }
    }
}

/// Controls whether the formatter should insert, remove, or leave a
/// particular syntactic element (e.g. trailing commas) unchanged.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Policy {
    /// Leave existing usage as-is.
    Ignore,
    /// Remove the element if present.
    Remove,
    /// Insert the element if absent.
    Insert,
}

impl std::str::FromStr for Policy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ignore" => Ok(Self::Ignore),
            "insert" => Ok(Self::Insert),
            "remove" => Ok(Self::Remove),
            _ => Err(format!("invalid policy: {s}")),
        }
    }
}

/// Walks the syntax tree in pre-order and applies formatting to each node.
///
/// This is the core recursive driver. It tracks indentation depth as it
/// enters and leaves block-like nodes, delegating per-node formatting to
/// [`format::format_syntax_node`].
pub fn format_recursive(
    syntax: &SyntaxNode,
    options: &FormattingOptions,
) {
    let preorder = syntax.preorder();

    let mut indentation: usize = 0;

    for event in preorder {
        match event {
            WalkEvent::Enter(node) => {
                if is_indent_kind(&node) {
                    indentation += 1;
                }
                format::format_syntax_node(&node, indentation, options);
            },
            WalkEvent::Leave(node) => {
                if is_indent_kind(&node) {
                    indentation = indentation.saturating_sub(1);
                }
            },
        }
    }
}

/// Returns `true` if entering this node should increase the indentation level.
///
/// Compound statements and switch bodies are indented. Multi-line parameter
/// and argument lists are also treated as indent scopes.
pub(crate) fn is_indent_kind(node: &SyntaxNode) -> bool {
    // NOTE: LoopStatement is intentionally excluded here. Its body is a
    // CompoundStatement which already increments indentation; including
    // LoopStatement would double-indent the loop contents.
    if matches!(
        node.kind(),
        SyntaxKind::CompoundStatement | SyntaxKind::SwitchBody
    ) {
        return true;
    }

    let param_list_left_paren = ast::FunctionParameters::cast(node.clone())
        .and_then(|list| list.left_parenthesis_token())
        .or_else(|| {
            let list = ast::Arguments::cast(node.clone())?;
            list.left_parenthesis_token()
        });

    if param_list_left_paren
        .and_then(|token| token.next_token())
        .as_ref()
        .is_some_and(util::is_whitespace_with_newline)
    {
        return true;
    }

    false
}

#[cfg(test)]
mod policy_tests {
    use std::str::FromStr as _;

    use super::*;
    #[test]
    fn policy_from_str_valid_values() {
        assert!(matches!(Policy::from_str("ignore"), Ok(Policy::Ignore)));
        assert!(matches!(Policy::from_str("insert"), Ok(Policy::Insert)));
        assert!(matches!(Policy::from_str("remove"), Ok(Policy::Remove)));
    }
    #[test]
    fn policy_from_str_invalid_value() {
        let result = Policy::from_str("invalid_value");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "invalid policy: invalid_value");
    }
}
