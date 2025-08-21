mod format;
#[cfg(test)]
mod tests;

//This cannot be gated, as we depend on it in doctests and the doctests are
// run against the public api.
pub mod test_util;

//Include the Formatting documentation, so that code blocks are run as doctests.
#[doc = include_str!("../Formatting.md")]
#[cfg(doctest)]
pub struct FormattingMdDocTests;

use rowan::{GreenNode, GreenToken, NodeOrToken, WalkEvent};
use syntax::{AstNode, HasName, SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken, ast};

pub use format::{format_str, format_tree};

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
    pub width: usize,
}

impl Default for FormattingOptions {
    fn default() -> Self {
        Self {
            trailing_commas: Policy::Insert,
            indent_symbol: "    ".to_owned(),
            width: 80,
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
