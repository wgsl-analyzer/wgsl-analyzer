#![expect(clippy::print_stdout, reason = "useful in tests")]

use std::{ffi::OsString, panic, path::Path};

use crate::{FormattingOptions, format_tree};
use expect_test::{Expect, ExpectFile, expect, expect_file};

pub trait ExpectAssertEq {
    fn assert_eq(
        &self,
        other: &str,
    );
}

impl ExpectAssertEq for Expect {
    fn assert_eq(
        &self,
        other: &str,
    ) {
        self.assert_eq(other);
    }
}

impl ExpectAssertEq for ExpectFile {
    fn assert_eq(
        &self,
        other: &str,
    ) {
        self.assert_eq(other);
    }
}

#[expect(clippy::needless_pass_by_value, reason = "intentional API")]
pub fn check(
    before: &str,
    after: impl ExpectAssertEq,
) {
    check_with_options(before, &after, &FormattingOptions::default());
}

#[expect(clippy::needless_pass_by_value, reason = "intentional API")]
pub fn check_tabs(
    before: &str,
    after: impl ExpectAssertEq,
) {
    let options = FormattingOptions {
        indent_symbol: "\t".into(),
        ..Default::default()
    };
    check_with_options(before, &after, &options);
}

#[track_caller]
pub fn check_with_options(
    before: &str,
    after: &impl ExpectAssertEq,
    options: &FormattingOptions,
) {
    let syntax = syntax::parse(before.trim_start())
        .syntax()
        .clone_for_update();
    let new = format_tree(&syntax, options);

    after.assert_eq(&new);

    // Check for idempotence
    let syntax = syntax::parse(new.trim_start()).syntax().clone_for_update();
    let new_second = format_tree(&syntax, options);
    let diff = dissimilar::diff(&new, &new_second);
    let position = panic::Location::caller();
    if new == new_second {
        return;
    }
    println!(
        "\n
\x1b[1m\x1b[91merror\x1b[97m: Formatting Idempotence check failed\x1b[0m
\x1b[1m\x1b[34m-->\x1b[0m {position}
\x1b[1mExpect\x1b[0m:
----
{new}
----

\x1b[1mActual\x1b[0m:
----
{new_second}
----

\x1b[1mDiff\x1b[0m:
----
{}
----
",
        format_chunks(diff)
    );
    // Use resume_unwind instead of panic!() to prevent a backtrace, which is unnecessary noise.
    panic::resume_unwind(Box::new(()));
}

fn format_chunks(chunks: Vec<dissimilar::Chunk<'_>>) -> String {
    let mut buf = String::new();
    for chunk in chunks {
        let formatted = match chunk {
            dissimilar::Chunk::Equal(text) => text.into(),
            dissimilar::Chunk::Delete(text) => format!("\x1b[41m{text}\x1b[0m"),
            dissimilar::Chunk::Insert(text) => format!("\x1b[42m{text}\x1b[0m"),
        };
        buf.push_str(&formatted);
    }
    buf
}
