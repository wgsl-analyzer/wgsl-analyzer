use anyhow::Context;
use lexopt::prelude::*;
use std::{io::Read, path::PathBuf};

use wgsl_formatter::FormattingOptions;

const HELP_STR: &str = r#"wgslfmt [options] <file>...

Options:
    --check     Run in 'check' mode. Exists with 0 if input is formatted correctly.
                Exits with 1 and prints a diff if formatting is requried.
    --tabs      Use tabs for indentation (instead of spaces)
"#;

struct Args {
    check: bool,
    tab_indent: bool,
    files: Vec<PathBuf>,
}

fn parse_args() -> Result<Args, lexopt::Error> {
    let mut parser = lexopt::Parser::from_env();
    let mut args = Args {
        check: false,
        tab_indent: false,
        files: Vec::new(),
    };

    while let Some(arg) = parser.next()? {
        match arg {
            Short('h') | Long("help") => {
                print!("{}", HELP_STR);
                std::process::exit(0);
            }
            Long("check") => args.check = true,
            Long("tabs") => args.tab_indent = true,
            Value(file) => args.files.push(PathBuf::from(file)),
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(args)
}

fn main() -> Result<(), anyhow::Error> {
    let mut args = parse_args()?;

    if args.files.is_empty() {
        args.files.push(PathBuf::from("-"))
    }

    for file in args.files {
        let is_stdin = file.as_os_str() == "-";
        let input = if is_stdin {
            read_stdin()?
        } else {
            std::fs::read_to_string(&file)?
        };

        let mut formatting_options = FormattingOptions::default();
        if args.tab_indent {
            formatting_options.indent_symbol = "\t".to_string();
        }
        let output = wgsl_formatter::format_str(&input, &formatting_options);

        if args.check {
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
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;
    Ok(buf)
}
