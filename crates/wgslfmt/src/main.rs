#![expect(clippy::print_stderr, reason = "CLI program")]
#![expect(clippy::print_stdout, reason = "CLI program")]
#![allow(
    unfulfilled_lint_expectations,
    reason = "https://github.com/rust-lang/rust-clippy/issues/15107"
)]

use std::{io::Read as _, path::PathBuf};

use anyhow::Context as _;
use clap::Parser;
use wgsl_formatter::FormattingOptions;

/// Tool to find and fix WGSL/WESL formatting issues.
///
/// Reads from stdin when no files are given (or when a file is "-").
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

    /// Files to format. Reads from stdin if omitted or if a file is "-".
    files: Vec<PathBuf>,
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Args::parse();

    let files = if cli.files.is_empty() {
        vec![PathBuf::from("-")]
    } else {
        cli.files
    };

    for file in &files {
        let is_stdin = file.as_os_str() == "-";
        let input = if is_stdin {
            read_stdin()?
        } else {
            std::fs::read_to_string(file)?
        };

        let mut formatting_options = FormattingOptions::default();
        if cli.tabs {
            "\t".clone_into(&mut formatting_options.indent_symbol);
        }
        let output = wgsl_formatter::format_str(&input, &formatting_options);

        if cli.check {
            if output != input {
                let diff = prettydiff::diff_lines(&input, &output);
                println!("Diff in {}\n{diff}:", file.display());
            }
        } else if is_stdin {
            print!("{output}");
        } else {
            std::fs::write(file, output)
                .with_context(|| format!("failed to write to {}", file.display()))?;
        }
    }

    Ok(())
}

fn read_stdin() -> Result<String, std::io::Error> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}
