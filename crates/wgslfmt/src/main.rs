
mod cli;
pub mod options;
mod patterns;
mod summary;

use std::{
    fmt::Display, io::Read as _, path::PathBuf, process::exit, task::Context, time::{Duration, Instant}
};

use anyhow::{Context as _, bail};
use prettydiff::text::ContextConfig;
use serde::Serialize;
use summary::WriteSummary;
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

#[derive(Serialize)]
struct ParseError {
    line: u32,
    col: u32,
    message: String,
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


    match cli.mode {
        WgslFmtMode::Check => check_file_results(formatted_files),
        WgslFmtMode::Write => write_file_results(formatted_files, cli.stdout_format, cli.print_diff),
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

fn check_file_results(results: impl Iterator<Item = FileResult>) {
    let mut has_errors = false;
    for result in results {
        if matches!(result.status, FileStatus::Errors) {
            has_errors = true;
        }
    }
    if has_errors {
        exit(1);
    }
}

fn write_file_results(results: impl Iterator<Item = FileResult>, output: OutputFormat, print_diff: bool) {
    let mut summary = WriteSummary::begin(output, print_diff);
    for result in results {
        if let FileStatus::Changed { source, formatted } = &result.status {
            let mut writer = result.file.write().unwrap();
            writer.write_all(formatted.as_bytes()).unwrap();
        }
        summary.submit(&result);
    }
    summary.finish();
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormattingSource::File(path_buf) => write!(f, "{}", path_buf.display()),
            FormattingSource::Stdin => write!(f, "Stdin"),
        }
    }
}
