use std::{error::Error, str::FromStr};

use clap::{Arg, ArgAction, ArgGroup, Command, ValueEnum, builder::PossibleValue, value_parser};

#[derive(Clone, Debug)]
pub struct Args {
    /// The mode (check or write) to run in.
    pub mode: WgslFmtMode,

    /// The format to use for stdout output.
    pub stdout_format: OutputFormat,

    /// Whether to include diffs in the stdout output - works for all `mode`s and all `stdout_format`s.
    pub print_diff: bool,

    /// Overrides for the formatting configuration.
    pub config_overrides: Vec<ConfigOverride>,

    /// Files, directories, or glob patterns to format.
    /// Pass "-" to read from stdin.
    pub patterns: Vec<String>,
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
pub enum WgslFmtMode {
    Check,
    #[default]
    Write,
}

#[derive(Clone, Copy, Debug, Default)]
pub enum OutputFormat {
    Json,
    #[default]
    Text,
}

impl Args {
    fn command() -> Command {
        Command::new("wgslfmt")
            .about("Tool to find and fix WGSL/WESL formatting issues")
            .version(env!("CARGO_PKG_VERSION"))

            // Mode setters
            .arg(
                Arg::new("check")
                    .long("check")
                    .help("Run in 'check' mode.")
                    .long_help(
                        "Run in 'check' mode.
Exits with 0 if input is formatted correctly.
Exits with 1 if formatting is required.",
                    )
                    .action(ArgAction::SetTrue),
            )

            // Output Format Setters
            .arg(
                Arg::new("json")
                    .long("json")
                    .help("Format stdio output as JSON")
                    .action(ArgAction::SetTrue),
            )

            .arg(
                Arg::new("print-diff")
                    .long("print-diff")
                    .help("Include diffs in the stdio output")
                    .action(ArgAction::SetTrue),
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
        let mode = if matches.remove_one::<bool>("check").unwrap_or_default() {
            WgslFmtMode::Check
        } else {
            WgslFmtMode::Write
        };

        let stdout_format = if matches.remove_one::<bool>("json").unwrap_or_default() {
            OutputFormat::Json
        } else {
            OutputFormat::Text
        };

        Self {
            mode,
            stdout_format,
            print_diff: matches
                .remove_one::<bool>("print-diff")
                .unwrap_or_default(),
            config_overrides: matches
                .remove_many::<ConfigOverride>("config")
                .map(std::iter::Iterator::collect::<Vec<_>>)
                .unwrap_or_default(),
            patterns: matches
                .remove_many::<String>("patterns")
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
