mod format;
#[cfg(test)]
mod tests;

use rowan::{GreenNode, GreenToken, NodeOrToken, WalkEvent};
use syntax::{AstNode, HasName, SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken, ast};

pub use format::{format_str, format_tree};

#[derive(Debug)]
pub struct FormattingOptions {
    pub trailing_commas: Policy,
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

#[derive(Debug, Copy, Clone)]
pub enum Policy {
    Ignore,
    Remove,
    Insert,
}
