use std::{io::Read, path::PathBuf};

use anyhow::Context;
use lexopt::prelude::*;
use wgsl_formatter::FormattingOptions;

const HELP_STR: &str = r#"wgslfmt [options] <file>...

Options:
    --check     Run in 'check' mode. Exists with 0 if input is formatted correctly.
                Exits with 1 and prints a diff if formatting is required.
    --tabs      Use tabs for indentation (instead of spaces)
"#;

struct Arguments {
    check: bool,
    tab_indent: bool,
    files: Vec<PathBuf>,
}

fn parse_arguments() -> Result<Arguments, lexopt::Error> {
    let mut parser = lexopt::Parser::from_env();
    let mut arguments = Arguments {
        check: false,
        tab_indent: false,
        files: Vec::new(),
    };

    while let Some(arg) = parser.next()? {
        match arg {
            Short('h') | Long("help") => {
                print!("{}", HELP_STR);
                std::process::exit(0);
            },
            Long("check") => arguments.check = true,
            Long("tabs") => arguments.tab_indent = true,
            Value(file) => arguments.files.push(PathBuf::from(file)),
            _ => return Err(arg.unexpected()),
        }
    }
    Ok(arguments)
}

fn main() -> Result<(), anyhow::Error> {
    let mut arguments = parse_arguments()?;

    if arguments.files.is_empty() {
        arguments.files.push(PathBuf::from("-"))
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
            formatting_options.indent_symbol = "\t".to_string();
        }
        let output = wgsl_formatter::format_str(&input, &formatting_options);

        if arguments.check {
            let same = output == input;
            if !same {
                let diff = prettydiff::diff_lines(&input, &output);

                println!("Diff in {}\n{}:", file.display(), diff);
                std::process::exit(1);
            }
        } else if is_stdin {
            print!("{}", output);
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
