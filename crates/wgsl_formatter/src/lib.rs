//! # WGSL-Formatter
//!
//! A library designed to format (pretty-print) wesl/wgsl source code.
//!
//! ## Entrypoints
//!
//! The entry points that actually format code are [`format_file`], [`format_range`] [`format_node`] and [`format_tree`].
#![cfg_attr(doc, doc = include_str!("../Architecture.md"))]
// We re-enable a warn lint within the formatter because it is very easy to parse an item within a gen_*-function and
// then forget to print it to the PrintItemBuffer.
// Also it is very easy to forget a "?" after a parse_*-function, that would be caught by
// "unused std::result::Result that must be used"
#![warn(unused)]

pub mod generators;
#[cfg(test)]
mod tests;

//This cannot be gated, as we depend on it in doctests and the doctests are
// run against the public api.
pub mod ast_parse;
pub mod context_policies;
pub mod format;
pub mod helpers;
pub mod ignore;
pub mod multiline_group;
pub mod options;
pub mod print_item_buffer;
pub mod reporting;
pub mod test_util;
pub mod trivia;

use std::str::FromStr;

use dprint_core::configuration::ParseConfigurationError;

pub use format::{
    FormatStringError, FormattedRange, format_file, format_node, format_range, format_tree,
};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FormattingOptions {
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

impl FormattingOptions {
    #[must_use]
    pub const fn const_default() -> Self {
        Self {
            max_line_width: 100,
            indent_width: 4,
            indent_style: IndentStyle::Spaces,
            line_break_style: LineBreakStyle::LineFeed,
        }
    }
}

impl Default for FormattingOptions {
    fn default() -> Self {
        Self::const_default()
    }
}
