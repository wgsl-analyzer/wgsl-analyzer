#![expect(clippy::print_stderr, reason = "CLI program")]
#![expect(clippy::print_stdout, reason = "CLI program")]

use crate::{FileResult, FormattingSource, cli::OutputFormat};

pub struct WriteSummary {
    pub format: OutputFormat,
    pub print_diff: bool,

    pub formatted_files: Vec<FormattingSource>,
    pub unchanged_files: Vec<FormattingSource>,
    pub errored_files: Vec<FormattingSource>,
}

impl WriteSummary {
    pub fn begin(format: OutputFormat, print_diff: bool) -> Self {
        if matches!(format, OutputFormat::Json) {
            println!("{{");
            println!("  \"files\": [");
        }

        Self {
            format,
            print_diff,

            unchanged_files: Vec::new(),
            formatted_files: Vec::new(),
            errored_files: Vec::new(),
        }
    }

    pub fn submit(&mut self, result: &FileResult) {

        match self.format {
            OutputFormat::Json => {
                if !(self.unchanged_files.is_empty() && self.errored_files.is_empty() && self.formatted_files.is_empty()) {
                    println!(",");
                }
                println!("{{");
                print!("  \"file\": \"{}\",", result.file);
                match &result.status {
                    crate::FileStatus::Unchanged => {
                        print!("\n  \"status\": \"unchanged\"");
                    },
                    crate::FileStatus::Errors => {
                        print!("\n  \"status\": \"errors\"");
                        //TODO Print actual error
                    },
                    crate::FileStatus::Changed { source, formatted } => {
                        print!("\n  \"status\": \"formatted\"");
                        if self.print_diff {
                            let diff = prettydiff::diff_lines(source, formatted);
                            print!(",\n  \"diff\": \"{}\"", diff);
                        }
                    },
                }
                print!("\n}}");
            },
            OutputFormat::Text => {
                match &result.status {
                    crate::FileStatus::Unchanged => {
                        println!("{}: Unchanged", result.file);
                    },
                    crate::FileStatus::Errors => {
                        eprintln!("{}: Error while formatting", result.file);
                        //TODO Print actual error
                    },
                    crate::FileStatus::Changed { source, formatted } => {
                        println!("{}: Formatted", result.file);
                        if self.print_diff {
                            let diff = prettydiff::diff_lines(source, formatted);
                            println!("{}", diff.format());
                        }
                    },
                }
            },
        }

        match &result.status {
            crate::FileStatus::Unchanged => {
                self.unchanged_files.push(result.file.clone());
            },
            crate::FileStatus::Errors => {
                self.errored_files.push(result.file.clone());
            },
            crate::FileStatus::Changed { source, formatted } => {
                self.formatted_files.push(result.file.clone());
            },
        }
    }

    pub fn finish(&self) {

        match self.format {
            OutputFormat::Json => {
                println!("");
                println!("  ],");
                println!("  \"summary\": {{");
                println!("      \"formatted\": {},", self.formatted_files.len());
                println!("      \"unchanged\": {},", self.unchanged_files.len());
                println!("      \"errors\": {}", self.errored_files.len());
                println!("  }}");
                println!("}}");
            },
            OutputFormat::Text => {
                println!("Formatted: {}, Unchanged: {}, Errors: {}", self.formatted_files.len(), self.unchanged_files.len(), self.errored_files.len());
            },
        }
    }
}
