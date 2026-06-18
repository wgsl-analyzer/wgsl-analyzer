#![expect(clippy::print_stderr, reason = "CLI program")]
#![expect(clippy::print_stdout, reason = "CLI program")]

use crate::{FileResult, FileStatus, FormattingSource, cli::OutputFormat};

pub trait Summary {
    fn begin(&mut self);
    fn end(&mut self);
    fn start_files(&mut self);
    fn file_result_written(
        &mut self,
        file_result: &FileResult,
    );
    fn file_result_checked(
        &mut self,
        file_result: &FileResult,
    );
    fn end_files(&mut self);
    fn write_summary(
        &mut self,
        formatted_files: usize,
        unchanged_files: usize,
        errored_files: usize,
    );
    fn check_summary(
        &mut self,
        failed_files: usize,
        passed_files: usize,
        errored_files: usize,
    );
}

pub struct SilentSummary;
impl Summary for SilentSummary {
    fn begin(&mut self) {}

    fn end(&mut self) {}

    fn start_files(&mut self) {}

    fn file_result_written(
        &mut self,
        file_result: &FileResult,
    ) {
    }

    fn file_result_checked(
        &mut self,
        file_result: &FileResult,
    ) {
    }

    fn end_files(&mut self) {}

    fn write_summary(
        &mut self,
        formatted_files: usize,
        unchanged_files: usize,
        errored_files: usize,
    ) {
    }

    fn check_summary(
        &mut self,
        failed_files: usize,
        passed_files: usize,
        errored_files: usize,
    ) {
    }
}

pub struct TextSummary {
    pub print_diff: bool,
}

impl TextSummary {
    fn print_diff(
        &self,
        result: &FileResult,
    ) {
        if let FileStatus::Changed { source, formatted } = &result.status {
            // We re-output the path of the file, to avoid confusion
            // about whether diff comes before or after the filename when the
            // output looks like
            // ...
            // [Name]: Formatted
            // [Diff]
            // [Name]: Formatted
            // [Diff]
            // [Name]: Formatted
            // ...
            println!("Diff of {}:", result.file);

            let diff = prettydiff::diff_lines(source, formatted);
            println!(
                "{}",
                diff.format_with_context(
                    Some(prettydiff::text::ContextConfig {
                        context_size: 5,
                        ..Default::default()
                    }),
                    true
                )
            );
        }
    }
}

impl Summary for TextSummary {
    fn begin(&mut self) {}

    fn end(&mut self) {}

    fn start_files(&mut self) {}

    fn file_result_written(
        &mut self,
        file_result: &FileResult,
    ) {
        print!("[{}]: ", file_result.file);
        match &file_result.status {
            crate::FileStatus::Unchanged => println!("Unchanged"),
            crate::FileStatus::Errors => println!("Errors"),
            crate::FileStatus::Changed { source, formatted } => {
                println!("Formatted");
            },
        }
        if self.print_diff {
            self.print_diff(file_result);
        }
    }

    fn file_result_checked(
        &mut self,
        file_result: &FileResult,
    ) {
        print!("[{}]: ", file_result.file);
        match &file_result.status {
            crate::FileStatus::Unchanged => println!("Pass"),
            crate::FileStatus::Errors => println!("Errors"),
            crate::FileStatus::Changed { source, formatted } => {
                println!("Failed");
            },
        }
        if self.print_diff {
            self.print_diff(file_result);
        }
    }

    fn end_files(&mut self) {}

    fn write_summary(
        &mut self,
        formatted_files: usize,
        unchanged_files: usize,
        errored_files: usize,
    ) {
        println!(
            "Formatted: {formatted_files}, Errors: {errored_files}, Unchanged: {unchanged_files}"
        );
    }

    fn check_summary(
        &mut self,
        failed_files: usize,
        passed_files: usize,
        errored_files: usize,
    ) {
        println!("Failed: {failed_files}, Errors: {errored_files}, Passed: {passed_files}");
    }
}

#[derive(Default)]
pub struct JsonSummary {
    need_semicolon: bool,
}

impl JsonSummary {
    fn begin_field(
        &mut self,
        name: &str,
    ) {
        if self.need_semicolon {
            print!(",");
        }
        print!("\"{}\":", name);
        self.need_semicolon = false;
    }
    fn string_literal(
        &mut self,
        value: &str,
    ) {
        print!("\"{}\"", value);
        self.need_semicolon = true;
    }
    fn usize_literal(
        &mut self,
        value: usize,
    ) {
        print!("\"{}\"", value);
        self.need_semicolon = true;
    }
    fn end_field(
        &mut self,
        name: &str,
    ) {
        print!("\"{}\"", name);
        self.need_semicolon = true;
    }
    fn begin_struct(&mut self) {
        if self.need_semicolon {
            print!(",");
        }
        print!("{{");
        self.need_semicolon = false;
    }
    fn end_struct(&mut self) {
        print!("}}");
        self.need_semicolon = true;
    }
    fn begin_array(&mut self) {
        if self.need_semicolon {
            print!(",");
        }
        print!("[");
        self.need_semicolon = false;
    }
    fn end_array(&mut self) {
        print!("]");
        self.need_semicolon = true;
    }
}

impl Summary for JsonSummary {
    fn begin(&mut self) {
        self.begin_struct();
    }

    fn file_result_written(
        &mut self,
        result: &FileResult,
    ) {
        self.begin_struct();
        self.begin_field("file");
        self.string_literal(&result.file.to_string());
        match &result.status {
            crate::FileStatus::Unchanged => {
                self.begin_field("status");
                self.string_literal("unchanged");
            },
            crate::FileStatus::Errors => {
                self.begin_field("status");
                self.string_literal("errors");
            },
            crate::FileStatus::Changed { source, formatted } => {
                self.begin_field("status");
                self.string_literal("formatted");
            },
        }
        self.end_struct();
        println!("");
    }

    fn file_result_checked(
        &mut self,
        result: &FileResult,
    ) {
        self.begin_struct();
        self.begin_field("file");
        self.string_literal(&result.file.to_string());
        match &result.status {
            crate::FileStatus::Unchanged => {
                self.begin_field("status");
                self.string_literal("pass");
            },
            crate::FileStatus::Errors => {
                self.begin_field("status");
                self.string_literal("errors");
            },
            crate::FileStatus::Changed { source, formatted } => {
                self.begin_field("status");
                self.string_literal("fail");
            },
        }
        self.end_struct();
        println!("");
    }

    fn write_summary(
        &mut self,
        formatted_files: usize,
        unchanged_fles: usize,
        errored_files: usize,
    ) {
        self.begin_field("summary");
        self.begin_struct();
        self.begin_field("formatted");
        self.usize_literal(formatted_files);
        self.begin_field("unchanged");
        self.usize_literal(unchanged_fles);
        self.begin_field("errors");
        self.usize_literal(errored_files);
        self.end_struct();
    }

    fn end(&mut self) {
        self.end_struct();
    }

    fn start_files(&mut self) {
        self.begin_field("files");
        self.begin_array();
    }

    fn end_files(&mut self) {
        self.end_array();
    }

    fn check_summary(
        &mut self,
        failed_files: usize,
        passed_files: usize,
        errored_files: usize,
    ) {
        self.begin_field("summary");
        self.begin_struct();
        self.begin_field("failed");
        self.usize_literal(failed_files);
        self.begin_field("passed");
        self.usize_literal(passed_files);
        self.begin_field("errors");
        self.usize_literal(errored_files);
        self.end_struct();
    }
}
