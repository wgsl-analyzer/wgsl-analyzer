mod cli;
pub mod options;
mod patterns;
mod summary;

use std::{
    fmt::Display,
    io::Read as _,
    path::PathBuf,
    process::exit,
    task::Context,
    time::{Duration, Instant},
};

use anyhow::{Context as _, bail};
use prettydiff::text::ContextConfig;
use serde::Serialize;
use summary::{JsonSummary, SilentSummary, Summary, TextSummary};
use wgsl_formatter::{FormatStringError, FormattingOptions, IndentStyle};

use crate::{
    cli::{Args, ConfigOverride, OutputFormat, WgslFmtMode},
    options::{WgslFmtOptions, collect_options},
    patterns::resolve_patterns,
};

#[derive(Debug)]
struct FileResult {
    file: FormattingSource,
    duration: Duration,
    status: FileStatus,
}

#[derive(Debug)]
enum FileStatus {
    Unchanged,
    Errors,
    Changed { source: String, formatted: String },
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Args::parse();

    let wgslfmt_options = collect_options(cli.config_overrides)?;

    let files = resolve_patterns(&cli.patterns)?;

    if files.is_empty() {
        bail!("no .wgsl/.wesl files found matching the given patterns");
    }

    let formatted_files = files
        .into_iter()
        .map(read_file)
        .map(|(file, source)| format_file(file, source, &wgslfmt_options));

    match &cli.stdout_format {
        OutputFormat::Json => {
            process(cli.mode, formatted_files, JsonSummary::default());
        },
        OutputFormat::Text => {
            process(
                cli.mode,
                formatted_files,
                TextSummary {
                    print_diff: cli.print_diff,
                },
            );
        },
        OutputFormat::Silent => process(cli.mode, formatted_files, SilentSummary),
    }

    exit(0);
}

fn read_file(file: FormattingSource) -> (FormattingSource, String) {
    let text = {
        let mut text = String::new();
        file.read().unwrap().read_to_string(&mut text).unwrap();
        text
    };
    (file, text)
}

fn format_file(
    file: FormattingSource,
    source: String,
    wgslfmt_options: &WgslFmtOptions,
) -> FileResult {
    let start = Instant::now();
    let options = wgslfmt_options.to_formatting_options();

    let result = wgsl_formatter::format_file(&source, &options);

    match result {
        Ok(formatted) => {
            if formatted == source {
                FileResult {
                    file,
                    duration: Instant::now().duration_since(start),
                    status: FileStatus::Unchanged,
                }
            } else {
                FileResult {
                    file,
                    duration: Instant::now().duration_since(start),
                    status: FileStatus::Changed { formatted, source },
                }
            }
        },
        Err(error) => FileResult {
            file,
            duration: Instant::now().duration_since(start),
            status: FileStatus::Errors,
        },
    }
}

fn process<S>(
    mode: WgslFmtMode,
    formatted_files: impl Iterator<Item = FileResult>,
    summary: S,
) where
    S: Summary,
{
    match mode {
        WgslFmtMode::Check => check_file_results(formatted_files, summary),
        WgslFmtMode::Write => write_file_results(formatted_files, summary),
    }
}

#[expect(
    clippy::exit,
    reason = "the task of this fn is to exit the process if there are errors"
)]
fn check_file_results<S>(
    results: impl Iterator<Item = FileResult>,
    mut summary: S,
) where
    S: Summary,
{
    summary.begin();
    let mut passed_paths = Vec::new();
    let mut failed_paths = Vec::new();
    let mut errored_paths = Vec::new();

    summary.start_files();
    for result in results {
        match &result.status {
            FileStatus::Unchanged => {
                passed_paths.push(result.file.clone());
            },
            FileStatus::Errors => {
                errored_paths.push(result.file.clone());
            },
            FileStatus::Changed { source, formatted } => {
                failed_paths.push(result.file.clone());
            },
        }
        summary.file_result_checked(&result);
    }
    summary.end_files();

    summary.check_summary(&failed_paths, &passed_paths, &errored_paths);
    summary.end();

    if !failed_paths.is_empty() | !errored_paths.is_empty() {
        exit(1);
    }
}

fn write_file_results<S>(
    results: impl Iterator<Item = FileResult>,
    mut summary: S,
) where
    S: Summary,
{
    summary.begin();

    let mut formatted_count = Vec::new();
    let mut unchanged_count = Vec::new();
    let mut errored_count = Vec::new();

    summary.start_files();
    for result in results {
        match &result.status {
            FileStatus::Unchanged => {
                unchanged_count.push(result.file.clone());
            },
            FileStatus::Errors => {
                errored_count.push(result.file.clone());
            },
            FileStatus::Changed { source, formatted } => {
                formatted_count.push(result.file.clone());
                let mut writer = result.file.write().unwrap();
                writer.write_all(formatted.as_bytes()).unwrap();
            },
        }
        summary.file_result_written(&result);
    }
    summary.end_files();

    summary.write_summary(&formatted_count, &unchanged_count, &errored_count);
    summary.end();
}

#[derive(Debug, Clone)]
enum FormattingSource {
    File(PathBuf),
    Stdin,
}

impl FormattingSource {
    pub fn read(&self) -> Result<Box<dyn std::io::Read>, anyhow::Error> {
        match self {
            Self::File(path) => Ok(Box::new(std::fs::File::open(path)?)),
            Self::Stdin => Ok(Box::new(std::io::stdin())),
        }
    }
    pub fn write(&self) -> Result<Box<dyn std::io::Write>, anyhow::Error> {
        match self {
            Self::File(path) => Ok(Box::new(std::fs::File::create(path)?)),
            Self::Stdin => Ok(Box::new(std::io::stdout())),
        }
    }
}

impl Display for FormattingSource {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::File(path_buf) => write!(f, "{}", path_buf.display()),
            Self::Stdin => write!(f, "Stdin"),
        }
    }
}
