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

#[derive(Debug)]
pub struct FormattingOptions {
    pub trailing_commas: Policy,
    pub indent_symbol: String,
    pub width: u32,
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

#[derive(Debug, Copy, Clone)]
pub enum Policy {
    Ignore,
    Remove,
    Insert,
}
