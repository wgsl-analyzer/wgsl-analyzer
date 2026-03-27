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

use std::str::FromStr;

use dprint_core::configuration::{NewLineKind, ParseConfigurationError};
use rowan::{GreenNode, GreenToken, NodeOrToken, WalkEvent};
use syntax::{AstNode, HasName, SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken, ast};

pub use format::{FormatStringError, FormattedRange, format_file, format_range, format_tree};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FormattingOptions {
    // TODO(MonaMayrhofer) Reintroduce that setting if needed
    // #[cfg_attr(feature = "serde", serde(alias = "trailingCommas"))]
    // pub trailing_commas: Policy,
    #[cfg_attr(feature = "serde", serde(alias = "maxLineWidth"))]
    pub max_line_width: u32,
    #[cfg_attr(feature = "serde", serde(alias = "indentWidth"))]
    pub indent_width: u8,
    #[cfg_attr(feature = "serde", serde(alias = "indentStyle"))]
    pub indent_style: IndentStyle,

    // We could use `[dprint_core::configuration::NewLineKind]` here, but that has
    // support to guess the line break style from the input, which
    // a) I don't like (opinion)
    // b) Would mean that for range formatting, we must call syntax().to_string() to obtain
    //    the unformatted source code, and scan it for line breaks which feels very unnecessarily inefficient.
    #[cfg_attr(feature = "serde", serde(alias = "lineBreakStyle"))]
    pub line_break_style: LineBreakStyle,
}

/// Style to be used when indenting code.
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IndentStyle {
    /// Indent using spaces. The amount of spaces is determined by the `[FormattingOptions.indent_width]` option.
    Spaces,
    /// Indent using tabs. The amount of space a tab is assumed to take is determined by the `[FormattingOptions.indent_width]` option.
    Tabs,
}

/// Style to be used for line breaks.
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[expect(clippy::enum_variant_names, reason = "That's simply their names.")]
pub enum LineBreakStyle {
    /// Unix style `\n`.
    LineFeed,
    /// Windows style `\r\n`.
    CarriageReturnLineFeed,
}

impl LineBreakStyle {
    #[must_use]
    pub const fn text(self) -> &'static str {
        match self {
            Self::LineFeed => "\n",
            Self::CarriageReturnLineFeed => "\r\n",
        }
    }
}

impl FromStr for LineBreakStyle {
    type Err = ParseConfigurationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lf" => Ok(Self::LineFeed),
            "crlf" => Ok(Self::CarriageReturnLineFeed),
            _ => Err(ParseConfigurationError(String::from(s))),
        }
    }
}

impl Default for FormattingOptions {
    fn default() -> Self {
        Self {
            max_line_width: 80,
            indent_width: 4,
            indent_style: IndentStyle::Spaces,
            line_break_style: LineBreakStyle::LineFeed,
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
