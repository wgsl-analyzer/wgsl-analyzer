use std::str::FromStr;

use clap::{Arg, ArgAction, Command, ValueEnum, arg, builder::PossibleValue, value_parser};

/// Tool to find and fix WGSL/WESL formatting issues.
///
/// Accepts file paths, directories (recursively finds .wgsl files), and
/// glob patterns (e.g. "src/**/*.wgsl"). Pass "-" to read from stdin.
pub struct Args {
    /// Run in 'check' mode. Exits with 0 if input is formatted correctly.
    /// Exits with 1 and prints a diff if formatting is required.
    pub check: bool,
    pub use_tabs: bool,
    pub indent_width: Option<u8>,
    pub output_format: OutputFormat,
    /// Files, directories, or glob patterns to format.
    /// Pass "-" to read from stdin.
    pub patterns: Vec<String>,
}

#[derive(Clone, Copy, Default)]
pub enum OutputFormat {
    #[default]
    Text,
    Json,
}
impl ValueEnum for OutputFormat {
    fn value_variants<'value>() -> &'value [Self] {
        &[Self::Text, Self::Json]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Text => PossibleValue::new("text").help("Print human-readable output"),
            Self::Json => PossibleValue::new("json").help("JSON object with all results"),
        })
    }
}

impl std::fmt::Display for OutputFormat {
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

impl FromStr for OutputFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "text" => Ok(Self::Text),
            "json" => Ok(Self::Json),
            _ => Err(()),
        }
    }
}

impl Args {
    #[expect(
        clippy::cognitive_complexity,
        reason = "Argument parsing should be in one place"
    )]
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
                Arg::new("tabs")
                    .long("tabs")
                    .help("Use tabs for indentation (instead of spaces)")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("indent-width")
                    .long("indent-width")
                    .value_name("WIDTH")
                    .help("Number of spaces per indentation level (default: 4)")
                    .value_parser(value_parser!(usize)),
            )
            .arg(
                Arg::new("output-format")
                    .long("output-format")
                    .value_name("FORMAT")
                    .value_parser(value_parser!(OutputFormat)),
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
            use_tabs: matches.remove_one::<bool>("tabs").unwrap_or_default(),
            indent_width: matches.remove_one::<u8>("indent-width"),
            output_format: matches
                .remove_one::<OutputFormat>("output-format")
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
