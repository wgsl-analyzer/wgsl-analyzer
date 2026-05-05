use std::{error::Error, str::FromStr};

use clap::{Arg, ArgAction, Command, ValueEnum, builder::PossibleValue, value_parser};

/// Tool to find and fix WGSL/WESL formatting issues.
///
/// Accepts file paths, directories (recursively finds .wgsl files), and
/// glob patterns (e.g. "src/**/*.wgsl"). Pass "-" to read from stdin.
#[derive(Clone, Debug)]
pub struct Args {
    /// Run in 'check' mode. Exits with 0 if input is formatted correctly.
    /// Exits with 1 and prints a diff if formatting is required.
    pub check: bool,
    /// Whether to overwrite the files with the formatted output
    pub overwrite: bool,
    pub stdout_format: OutputMode,
    /// Files, directories, or glob patterns to format.
    /// Pass "-" to read from stdin.
    pub patterns: Vec<String>,

    /// Overrides for the formatting configuration.
    pub config_overrides: Vec<ConfigOverride>,
}

#[derive(Clone, Debug)]
pub struct ConfigOverride {
    pub key: String,
    pub value: String,
}

fn parse_config_override(
    argument: &str
) -> Result<ConfigOverride, Box<dyn Error + Send + Sync + 'static>> {
    let (key, value) = argument
        .split_once('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{argument}`"))?;
    Ok(ConfigOverride {
        key: key.to_owned(),
        value: value.to_owned(),
    })
}

#[derive(Clone, Copy, Debug, Default)]
pub enum OutputMode {
    Json,
    #[default]
    Text,
}
impl ValueEnum for OutputMode {
    fn value_variants<'value>() -> &'value [Self] {
        &[Self::Json, Self::Text]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Json => PossibleValue::new("json").help("Format all stdout output as JSON"),
            Self::Text => {
                PossibleValue::new("text").help("Format all stdout output as human-readable text")
            },
        })
    }
}

impl std::fmt::Display for OutputMode {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl FromStr for OutputMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(Self::Json),
            "text" => Ok(Self::Text),
            _ => Err(()),
        }
    }
}

impl Args {
    fn command() -> Command {
        Command::new("wgslfmt")
            .about("Tool to find and fix WGSL/WESL formatting issues")
            .version(env!("CARGO_PKG_VERSION"))
            .arg(
                Arg::new("check")
                    .long("check")
                    .help("Run in 'check' mode.")
                    .long_help(
                        "Run in 'check' mode.
Exits with 0 if input is formatted correctly.
Exits with 1 and prints a diff if formatting is required.",
                    )
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("output-mode")
                    .long("output-mode")
                    .value_name("MODE")
                    .value_parser(value_parser!(OutputMode)),
            )
            .arg(
                Arg::new("overwrite")
                    .long("overwrite")
                    .default_value("true")
                    .value_name("true|false")
                    .value_parser(value_parser!(bool)),
            )
            .arg(
                Arg::new("config")
                    .long("config")
                    .value_name("KEY=VALUE")
                    .help("Set formatting options from the command line.")
                    // Once a wgslfmt.toml exists, the documentation for these keys should be moved
                    // there - they should be analogous to what can be configured in that file.
                    // Also once that exists we should replace the "Supported keys:" section with
                    // a mention that this takes precedence over the wgslfmt.toml
                    .long_help(
                        "Set formatting options from the command line. Supported options:
indent_style=tabs|spaces (default: spaces)
indent_width=number of spaces per indentation level (default: 4)",
                    )
                    .value_parser(parse_config_override)
                    .action(ArgAction::Append),
            )
            .arg(
                Arg::new("patterns")
                    .value_name("file/dir/glob")
                    .default_value(".")
                    .value_parser(value_parser!(String))
                    .action(ArgAction::Append)
                    .help(
                        "Files, directories, or glob patterns to format. \
Pass \"-\" to read from stdin",
                    ),
            )
    }

    fn from_arg_matches(mut matches: clap::ArgMatches) -> Self {
        Self {
            check: matches.remove_one::<bool>("check").unwrap_or_default(),
            overwrite: matches.remove_one::<bool>("overwrite").unwrap_or_default(),
            stdout_format: matches
                .remove_one::<OutputMode>("output-mode")
                .unwrap_or_default(),
            patterns: matches
                .remove_many::<String>("patterns")
                .map(std::iter::Iterator::collect::<Vec<_>>)
                .unwrap_or_default(),
            config_overrides: matches
                .remove_many::<ConfigOverride>("config")
                .map(std::iter::Iterator::collect::<Vec<_>>)
                .unwrap_or_default(),
        }
    }

    pub fn parse() -> Self {
        let matches = Self::command().get_matches();
        Self::from_arg_matches(matches)
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::Args;

    #[test]
    pub fn verify_command() {
        Args::command().debug_assert();
    }
}
