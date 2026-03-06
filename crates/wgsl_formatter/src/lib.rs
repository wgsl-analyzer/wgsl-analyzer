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

pub use format::{format_node, format_str, format_tree};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FormattingOptions {
    #[cfg_attr(feature = "serde", serde(alias = "trailingCommas"))]
    pub trailing_commas: Policy,
    #[cfg_attr(feature = "serde", serde(alias = "indentSymbol"))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Policy {
    Ignore,
    Remove,
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

#[cfg(test)]
mod policy_tests {
    use super::*;
    use std::str::FromStr as _;
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
