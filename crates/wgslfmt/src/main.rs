#![expect(clippy::print_stderr, reason = "CLI program")]
#![expect(clippy::print_stdout, reason = "CLI program")]
#![allow(
    unfulfilled_lint_expectations,
    reason = "https://github.com/rust-lang/rust-clippy/issues/15107"
)]

use std::{io::Read as _, path::PathBuf, time::Instant};

use anyhow::{Context as _, bail};
use clap::Parser;
use wgsl_formatter::FormattingOptions;

/// Tool to find and fix WGSL/WESL formatting issues.
///
/// Accepts file paths, directories (recursively finds .wgsl files), and
/// glob patterns (e.g. "src/**/*.wgsl"). Pass "-" to read from stdin.
#[derive(Parser)]
#[command(name = "wgslfmt", version)]
struct Args {
    /// Run in 'check' mode. Exits with 0 if input is formatted correctly.
    /// Exits with 1 and prints a diff if formatting is required.
    #[arg(long)]
    check: bool,

    /// Use tabs for indentation (instead of spaces).
    #[arg(long)]
    tabs: bool,

    /// Files, directories, or glob patterns to format.
    /// Pass "-" to read from stdin.
    #[arg(required = true)]
    patterns: Vec<String>,
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Args::parse();

    let files = resolve_patterns(&cli.patterns)?;

    if files.is_empty() {
        bail!("no .wgsl files found matching the given patterns");
    }

    let mut formatting_options = FormattingOptions::default();
    if cli.tabs {
        "\t".clone_into(&mut formatting_options.indent_symbol);
    }

    let total_start = Instant::now();
    let mut check_failed = false;

    for file in &files {
        let is_stdin = file.as_os_str() == "-";
        let input = if is_stdin {
            read_stdin()?
        } else {
            std::fs::read_to_string(file)?
        };

        // Check for parse errors and warn (but still format).
        let parse = parser::parse_file(&input);
        let errors = parse.errors();
        if !errors.is_empty() {
            let label = if is_stdin {
                "stdin".to_owned()
            } else {
                file.display().to_string()
            };
            let line_index = line_index::LineIndex::new(&input);
            eprintln!(
                "[warn] {label}: {count} parse error(s)",
                count = errors.len()
            );
            for diagnostic in errors {
                let start = line_index.line_col(diagnostic.range.start());
                eprintln!(
                    "  {label}:{}:{}: {}",
                    start.line + 1,
                    start.col + 1,
                    diagnostic.message
                );
            }
        }

        let file_start = Instant::now();
        let output = wgsl_formatter::format_str(&input, &formatting_options);
        let elapsed = file_start.elapsed();

        if cli.check {
            if output != input {
                check_failed = true;
                let diff = prettydiff::diff_lines(&input, &output);
                println!("{}\n{diff}", file.display());
            }
        } else if is_stdin {
            print!("{output}");
        } else {
            std::fs::write(file, &output)
                .with_context(|| format!("failed to write to {}", file.display()))?;
            let suffix = if output == input { " (unchanged)" } else { "" };
            println!("{} {}ms{suffix}", file.display(), elapsed.as_millis());
        }
    }

    let total_elapsed = total_start.elapsed();

    if cli.check {
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

/// Recursively collects all `.wgsl` files under `directory`.
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
        } else if path.extension().is_some_and(|ext| ext == "wgsl") {
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
