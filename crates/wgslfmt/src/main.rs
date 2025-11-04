#![expect(clippy::print_stderr, reason = "CLI program")]
#![expect(clippy::print_stdout, reason = "CLI program")]
#![allow(
    unfulfilled_lint_expectations,
    reason = "https://github.com/rust-lang/rust-clippy/issues/15107"
)]

use std::{io::Read as _, path::PathBuf};

use anyhow::Context as _;
use lexopt::prelude::*;
use wgsl_formatter::FormattingOptions;

const HELP_STR: &str = "wgslfmt [options] <file>...

Options:
    --check     Run in 'check' mode. Exists with 0 if input is formatted correctly.
                Exits with 1 and prints a diff if formatting is required.
    --tabs      Use tabs for indentation (instead of spaces)
";

struct Arguments {
    check: bool,
    tab_indent: bool,
    files: Vec<PathBuf>,
}

#[expect(clippy::exit, reason = "TODO: use clap")]
fn parse_arguments() -> Result<Arguments, lexopt::Error> {
    let mut parser = lexopt::Parser::from_env();
    let mut arguments = Arguments {
        check: false,
        tab_indent: false,
        files: Vec::new(),
    };

    while let Some(argument) = parser.next()? {
        match argument {
            Short('h') | Long("help") => {
                print!("{HELP_STR}");
                std::process::exit(0);
            },
            Long("check") => arguments.check = true,
            Long("tabs") => arguments.tab_indent = true,
            Value(file) => arguments.files.push(PathBuf::from(file)),
            Short(_) | Long(_) => return Err(argument.unexpected()),
        }
    }
    Ok(arguments)
}

#[expect(clippy::exit, reason = "TODO: use clap")]
fn main() -> Result<(), anyhow::Error> {
    let mut arguments = parse_arguments()?;

    if arguments.files.is_empty() {
        arguments.files.push(PathBuf::from("-"));
    }

    for file in arguments.files {
        let is_stdin = file.as_os_str() == "-";
        let input = if is_stdin {
            read_stdin()?
        } else {
            std::fs::read_to_string(&file)?
        };

        let mut formatting_options = FormattingOptions::default();
        if arguments.tab_indent {
            formatting_options
                .indent_symbol
                .clone_from(&"\t".to_owned());
        }
        let output = wgsl_formatter::format_str(&input, &formatting_options);

        if arguments.check {
            let same = output == input;
            if !same {
                let diff = prettydiff::diff_lines(&input, &output);

                println!("Diff in {}\n{diff}:", file.display());
            }
        } else if is_stdin {
            print!("{output}");
        } else {
            std::fs::write(&file, output)
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
