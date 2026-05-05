#![expect(clippy::print_stderr, reason = "CLI program")]
#![expect(clippy::print_stdout, reason = "CLI program")]

mod cli;
pub mod options;

use std::{
    io::Read as _,
    path::PathBuf,
    process::exit,
    task::Context,
    time::{Duration, Instant},
};

use anyhow::{Context as _, bail};
use prettydiff::text::ContextConfig;
use serde::Serialize;
use wgsl_formatter::{FormatStringError, FormattingOptions, IndentStyle};

use crate::{
    cli::{Args, ConfigOverride, OutputMode},
    options::WgslFmtOptions,
};

struct JsonOutput {
    files: Vec<FileResult>,
    total_files: usize,
    files_changed: usize,
    total_duration_ms: u128,
}

// #[derive(Serialize)]
// struct FileResult {
//     file: String,
//     changed: bool,
//     duration_ms: u128,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     diff: Option<String>,
// }

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
    Changed {},
}

#[derive(Serialize)]
struct ParseError {
    line: u32,
    col: u32,
    message: String,
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Args::parse();

    let files = resolve_patterns(&cli.patterns)?;

    if files.is_empty() {
        bail!("no .wgsl/.wesl files found matching the given patterns");
    }

    let wgslfmt_options = collect_options(cli.config_overrides)?;

    let mut results = Vec::new();

    for file in &files {
        let start = Instant::now();
        let source = {
            let mut source = String::new();
            file.read()?.read_to_string(&mut source)?;
            source
        };

        let options = wgslfmt_options.to_formatting_options();

        let result = wgsl_formatter::format_file(&source, &options);

        let Ok(formatted) = result else {
            results.push(FileResult {
                file: file.clone(),
                duration: Instant::now().duration_since(start),
                status: FileStatus::Errors,
            });
            continue;
        };

        if formatted == source {
            results.push(FileResult {
                file: file.clone(),
                duration: Instant::now().duration_since(start),
                status: FileStatus::Unchanged,
            });
        } else {
            if cli.overwrite {
                file.write()?.write_all(formatted.as_bytes())?;
            }

            results.push(FileResult {
                file: file.clone(),
                duration: Instant::now().duration_since(start),
                status: FileStatus::Changed {},
            });
        }
    }

    dbg!(results);
    exit(0);
}

fn collect_options(config_overrides: Vec<ConfigOverride>) -> anyhow::Result<WgslFmtOptions> {
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

/// Resolves a list of patterns into concrete file paths.
///
/// Each pattern is interpreted as:
/// - `"-"` → stdin
/// - A directory path → recursively walk for `.wgsl` files
/// - A glob pattern (contains `*`, `?`, or `[`) → expand via glob
/// - Otherwise → a literal file path
fn resolve_patterns(patterns: &[String]) -> Result<Vec<FormattingSource>, anyhow::Error> {
    let mut files = Vec::new();
    for pattern in patterns {
        if pattern == "-" {
            files.push(FormattingSource::Stdin);
        } else if PathBuf::from(pattern).is_dir() {
            collect_wgsl_files(&PathBuf::from(pattern), &mut files)?;
        } else if pattern.contains('*') || pattern.contains('?') || pattern.contains('[') {
            let paths =
                glob::glob(pattern).with_context(|| format!("invalid glob pattern: {pattern}"))?;
            for entry in paths {
                let path =
                    entry.with_context(|| format!("error reading glob match for: {pattern}"))?;
                if path.is_dir() {
                    collect_wgsl_files(&path, &mut files)?;
                } else {
                    files.push(FormattingSource::File(path));
                }
            }
        } else {
            files.push(FormattingSource::File(PathBuf::from(pattern)));
        }
    }
    Ok(files)
}

/// Recursively collects all `.wgsl` and `.wesl` files under `directory`.
fn collect_wgsl_files(
    directory: &PathBuf,
    out: &mut Vec<FormattingSource>,
) -> Result<(), anyhow::Error> {
    for entry in std::fs::read_dir(directory)
        .with_context(|| format!("failed to read directory: {}", directory.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_wgsl_files(&path, out)?;
        } else if path
            .extension()
            .is_some_and(|ext| ext == "wgsl" || ext == "wesl")
        {
            out.push(FormattingSource::File(path));
        }
    }
    Ok(())
}
