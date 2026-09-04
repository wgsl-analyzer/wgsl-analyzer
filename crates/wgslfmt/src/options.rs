//! The interface to a wgslfmt.toml file.
use anyhow::Context as _;
use serde::{Deserialize, Serialize};

use crate::cli::ConfigOverride;

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

#[expect(
    clippy::inline_modules,
    reason = "These should be colocated with the WgslFmtOptions struct"
)]
mod defaults {
    use wgsl_formatter::FormattingOptions;

    use crate::options::{IndentStyle, LineBreakStyle};

    pub const fn indent_style() -> IndentStyle {
        match FormattingOptions::const_default().indent_style {
            wgsl_formatter::IndentStyle::Spaces => IndentStyle::Spaces,
            wgsl_formatter::IndentStyle::Tabs => IndentStyle::Tabs,
        }
    }

    pub const fn indent_width() -> u8 {
        FormattingOptions::const_default().indent_width
    }

    pub const fn max_line_width() -> u32 {
        FormattingOptions::const_default().max_line_width
    }

    pub const fn line_break_style() -> LineBreakStyle {
        match FormattingOptions::const_default().line_break_style {
            wgsl_formatter::LineBreakStyle::LineFeed => LineBreakStyle::LineFeed,
            wgsl_formatter::LineBreakStyle::CarriageReturnLineFeed => {
                LineBreakStyle::CarriageReturnLineFeed
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum IndentStyle {
    #[serde(rename = "spaces")]
    Spaces,
    #[serde(rename = "tabs")]
    Tabs,
}

#[expect(
    clippy::enum_variant_names,
    reason = "it is a coincidence that the line-break styles' names both end in lf"
)]
#[derive(Serialize, Deserialize, Debug)]
pub enum LineBreakStyle {
    #[serde(rename = "crlf")]
    CarriageReturnLineFeed,
    #[serde(rename = "lf")]
    LineFeed,
}

impl WgslFmtOptions {
    #[must_use]
    pub const fn to_formatting_options(&self) -> wgsl_formatter::FormattingOptions {
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

pub fn collect_options(config_overrides: Vec<ConfigOverride>) -> anyhow::Result<WgslFmtOptions> {
    // Here we would instead parse a wgslfmt.toml into a serde_json::Value
    let mut formatting_options = serde_json::Map::default();

    // Patch the formatting options with the CLI options
    for config_override in config_overrides {
        let value = serde_json::from_str::<serde_json::Value>(&config_override.value)
            .unwrap_or(serde_json::Value::String(config_override.value));
        formatting_options.insert(config_override.key, value);
    }

    // Parse the merged options
    let formatting_options =
        serde_json::from_value::<WgslFmtOptions>(serde_json::Value::Object(formatting_options))
            .context("Could not parse the merged wgslfmt options")?;

    Ok(formatting_options)
}
