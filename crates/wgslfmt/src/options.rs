//! The interface to a wgslfmt.toml file.
use serde::{Deserialize, Serialize};

// We do not expose the wgsl_formatter::FormattingOptions directly, because we will want
// to provide stronger stability guarantees for the wgslfmt.toml, than
// for the FormattingOptions struct itself.
// Also the wgsl_formatter crate should not need to concern itself with the details of wgslfmt.toml etc.
/// The struct representing the contents of a wgslfmt.toml.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct WgslFmtOptions {
    /// The indentation style to use.
    #[serde(default = "defaults::indent_style")]
    pub indent_style: IndentStyle,
    /// The number of spaces to indent by.
    #[serde(default = "defaults::indent_width")]
    pub indent_width: u8,
    /// The target width that lines should not exceed.
    #[serde(default = "defaults::max_line_width")]
    pub max_line_width: u32,
    /// The line break style to use.
    #[serde(default = "defaults::line_break_style")]
    pub line_break_style: LineBreakStyle,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum IndentStyle {
    #[serde(rename = "spaces")]
    Spaces,
    #[serde(rename = "tabs")]
    Tabs,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LineBreakStyle {
    #[serde(rename = "crlf")]
    CarriageReturnLineFeed,
    #[serde(rename = "lf")]
    LineFeed,
}

mod defaults {
    use crate::options::{IndentStyle, LineBreakStyle};

    pub fn indent_style() -> IndentStyle {
        IndentStyle::Spaces
    }

    pub fn indent_width() -> u8 {
        4
    }

    pub fn max_line_width() -> u32 {
        80
    }

    pub fn line_break_style() -> LineBreakStyle {
        LineBreakStyle::LineFeed
    }
}

impl WgslFmtOptions {
    pub fn to_formatting_options(&self) -> wgsl_formatter::FormattingOptions {
        wgsl_formatter::FormattingOptions {
            indent_style: match self.indent_style {
                IndentStyle::Spaces => wgsl_formatter::IndentStyle::Spaces,
                IndentStyle::Tabs => wgsl_formatter::IndentStyle::Tabs,
            },
            indent_width: self.indent_width,
            max_line_width: self.max_line_width,
            line_break_style: match self.line_break_style {
                LineBreakStyle::CarriageReturnLineFeed => {
                    wgsl_formatter::LineBreakStyle::CarriageReturnLineFeed
                },
                LineBreakStyle::LineFeed => wgsl_formatter::LineBreakStyle::LineFeed,
            },
        }
    }
}
