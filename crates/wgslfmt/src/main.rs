#![expect(clippy::print_stderr, reason = "CLI program")]
#![expect(clippy::print_stdout, reason = "CLI program")]
#![allow(
    unfulfilled_lint_expectations,
    reason = "https://github.com/rust-lang/rust-clippy/issues/15107"
)]

mod cli;

use std::{io::Read as _, path::PathBuf, time::Instant};

use anyhow::{Context as _, bail};
use serde::Serialize;
use wgsl_formatter::FormattingOptions;

use crate::cli::{Args, OutputFormat};

#[derive(Serialize)]
struct JsonOutput {
    files: Vec<FileResult>,
    total_files: usize,
    files_changed: usize,
    total_duration_ms: u128,
}

#[derive(Serialize)]
struct FileResult {
    file: String,
    changed: bool,
    duration_ms: u128,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    parse_errors: Vec<ParseError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    diff: Option<String>,
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

    let mut formatting_options = FormattingOptions::default();
    if cli.use_tabs {
        "\t".clone_into(&mut formatting_options.indent_symbol);
    } else if let Some(width) = cli.indent_width {
        formatting_options.indent_symbol = " ".repeat(width);
    }

    let json_mode = matches!(cli.output_format, OutputFormat::Json);
    let total_start = Instant::now();
    let mut check_failed = false;
    let mut results: Vec<FileResult> = Vec::new();

    for file in &files {
        let result = format_file(file, &formatting_options, json_mode, cli.check)?;

        if !json_mode {
            emit_text_result(file, &result, cli.check);
        }

        if result.changed {
            check_failed = true;
        }

        results.push(result);
    }

    let total_elapsed = total_start.elapsed();

    if json_mode {
        let total_files = results.len();
        let files_changed = results.iter().filter(|result| result.changed).count();
        let json_output = JsonOutput {
            files: results,
            total_files,
            files_changed,
            total_duration_ms: total_elapsed.as_millis(),
        };
        println!("{}", serde_json::to_string_pretty(&json_output)?);
        if cli.check && check_failed {
            std::process::exit(1);
        }
    } else if cli.check {
        if check_failed {
            eprintln!("Code style issues found in the above file(s). Forgot to run wgslfmt?");
            std::process::exit(1);
        }
        eprintln!(
            "All matched files use wgslfmt code style! Checked {} file(s) in {}ms.",
            files.len(),
            total_elapsed.as_millis()
        );
    }

    Ok(())
}

/// Formats a single file (or stdin) and returns a [`FileResult`].
///
/// When `json_mode` is false, parse errors are printed to stderr.
fn format_file(
    file: &std::path::Path,
    options: &FormattingOptions,
    json_mode: bool,
    check_mode: bool,
) -> Result<FileResult, anyhow::Error> {
    let is_stdin = file.as_os_str() == "-";
    let input = if is_stdin {
        read_stdin()?
    } else {
        std::fs::read_to_string(file)?
    };

    let label = if is_stdin {
        "stdin".to_owned()
    } else {
        file.display().to_string()
    };

    // Detect edition from file extension for diagnostics.
    let edition = if !is_stdin
        && file
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("wesl"))
    {
        syntax::Edition::Wesl2025Unstable
    } else {
        syntax::Edition::Wgsl
    };

    let parse = syntax::parse(&input, edition);
    let errors = parse.errors();
    let parse_errors: Vec<ParseError> = if errors.is_empty() {
        Vec::new()
    } else {
        let line_index = line_index::LineIndex::new(&input);
        if !json_mode {
            eprintln!(
                "[warn] {label}: {count} parse error(s)",
                count = errors.len()
            );
        }
        errors
            .iter()
            .map(|diagnostic| {
                let start = line_index.line_col(diagnostic.range.start());
                if !json_mode {
                    eprintln!(
                        "  {label}:{}:{}: {}",
                        start.line + 1,
                        start.col + 1,
                        diagnostic.message
                    );
                }
                ParseError {
                    line: start.line + 1,
                    col: start.col + 1,
                    message: diagnostic.message.clone(),
                }
            })
            .collect()
    };

    let file_start = Instant::now();
    let output = wgsl_formatter::format_str(&input, options);
    let elapsed = file_start.elapsed();
    let changed = output != input;

    let diff = changed.then(|| {
        let raw = format!("{}", prettydiff::diff_lines(&input, &output));
        if json_mode {
            // Strip ANSI escape codes for machine-readable output.
            strip_ansi_codes(&raw)
        } else {
            raw
        }
    });

    // Write formatted output (skip in check mode; skip stdin in json mode).
    if !check_mode {
        if is_stdin {
            if !json_mode {
                print!("{output}");
            }
        } else {
            std::fs::write(file, &output)
                .with_context(|| format!("failed to write to {}", file.display()))?;
        }
    }

    Ok(FileResult {
        file: label,
        changed,
        duration_ms: elapsed.as_millis(),
        parse_errors,
        diff,
    })
}

/// Prints human-readable output for a single file result.
fn emit_text_result(
    file: &std::path::Path,
    result: &FileResult,
    check_mode: bool,
) {
    if check_mode {
        if let (true, Some(diff)) = (result.changed, &result.diff) {
            println!("{}\n{diff}", file.display());
        }
    } else if file.as_os_str() != "-" {
        let suffix = if result.changed { "" } else { " (unchanged)" };
        println!("{} {}ms{suffix}", file.display(), result.duration_ms);
    }
}

/// Resolves a list of patterns into concrete file paths.
///
/// Each pattern is interpreted as:
/// - `"-"` → stdin
/// - A directory path → recursively walk for `.wgsl` files
/// - A glob pattern (contains `*`, `?`, or `[`) → expand via glob
/// - Otherwise → a literal file path
fn resolve_patterns(patterns: &[String]) -> Result<Vec<PathBuf>, anyhow::Error> {
    let mut files = Vec::new();

    for pattern in patterns {
        if pattern == "-" {
            files.push(PathBuf::from("-"));
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
                    files.push(path);
                }
            }
        } else {
            files.push(PathBuf::from(pattern));
        }
    }

    Ok(files)
}

/// Recursively collects all `.wgsl` and `.wesl` files under `directory`.
fn collect_wgsl_files(
    directory: &PathBuf,
    out: &mut Vec<PathBuf>,
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
            out.push(path);
        }
    }
    Ok(())
}

fn read_stdin() -> Result<String, std::io::Error> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

/// Strips ANSI escape sequences (e.g. color codes) from a string.
fn strip_ansi_codes(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars();
    while let Some(character) = chars.next() {
        if character == '\x1b' {
            // Skip until we hit the terminating letter [A-Za-z].
            for character in chars.by_ref() {
                if character.is_ascii_alphabetic() {
                    break;
                }
            }
        } else {
            result.push(character);
        }
    }
    result
}
