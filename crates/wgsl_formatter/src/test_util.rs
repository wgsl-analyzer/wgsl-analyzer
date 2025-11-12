#![expect(clippy::print_stdout, reason = "useful in tests")]
#![expect(clippy::use_debug, reason = "useful in tests")]
#![expect(
    clippy::missing_panics_doc,
    reason = "we want to be able to use assert!"
)]
use std::{ffi::OsString, fmt::Debug, panic, path::Path};

use crate::{FormattingOptions, format_tree};

pub trait ExpectAssertEq: Debug {
    fn assert_eq(
        &self,
        other: &str,
    );
}

#[cfg(test)]
impl ExpectAssertEq for expect_test::Expect {
    fn assert_eq(
        &self,
        other: &str,
    ) {
        self.assert_eq(other);
    }
}

#[cfg(test)]
impl ExpectAssertEq for expect_test::ExpectFile {
    fn assert_eq(
        &self,
        other: &str,
    ) {
        self.assert_eq(other);
    }
}

impl ExpectAssertEq for &str {
    fn assert_eq(
        &self,
        other: &str,
    ) {
        assert_eq!(*self, other);
    }
}

pub fn assert_is_formatted(source: &str) {
    check(source, source);
}

#[expect(clippy::needless_pass_by_value, reason = "intentional API")]
pub fn check<E: ExpectAssertEq>(
    before: &str,
    after: E,
) {
    check_with_options(before, &after, &FormattingOptions::default());
}

#[expect(clippy::needless_pass_by_value, reason = "intentional API")]
pub fn check_tabs<E: ExpectAssertEq>(
    before: &str,
    after: E,
) {
    let options = FormattingOptions {
        indent_symbol: "\t".to_owned(),
        ..Default::default()
    };
    check_with_options(before, &after, &options);
}

/// Checks that the given source raises parsing diagnostics and is
/// thus out of scope of having to be formatted correctly.
///
/// Code that is out of scope would just be left untouched by the formatter
///
/// Even tho these tests only test the behavior of the parser,
/// they are useful to be included in the formatter unit tests,
/// in order to keep track of the boundaries of what the
/// formatter is supposed to deal with and to "get notified"
/// as soon as the parser starts implementing certain new features.
///
/// These assertions also exist, to help document certain design decisions,
/// and prevent people from doing futile work, trying to "complete holes
/// in the functionality of the formatter", while not realising that there a
/// reason certain things are intentionally not supported.
#[track_caller]
pub fn assert_out_of_scope(
    before: &str,
    reason: &str,
) {
    let parse = syntax::parse(before.trim_start());
    let syntax = parse.tree();

    if parse.errors().is_empty() {
        println!(
            "Expected source to raise parsing diagnostics and as such be out of scope for formatting. Reason: {reason}\nHowever the given source parsed without error. \nSource: {before}"
        );
        // Use resume_unwind instead of panic!() to prevent a backtrace, which is unnecessary noise.
        panic::resume_unwind(Box::new(()));
    }
}

#[track_caller]
pub fn check_with_options<E: ExpectAssertEq>(
    before: &str,
    after: &E,
    options: &FormattingOptions,
) {
    let parse = syntax::parse(before.trim_start());
    let syntax = parse.tree();

    if !parse.errors().is_empty() {
        panic!(
            "Parsing the source to be formatted failed with errors: {:#?}",
            parse.errors()
        );
    }

    dbg!(&parse.errors());
    dbg!(&syntax);
    let formatted = match format_tree(&syntax, options) {
        Ok(formatted) => formatted,
        Err(format_error) => {
            println!("Formatting returned an unexpected error: {format_error:?}");
            panic::resume_unwind(Box::new(()));
        },
    };

    after.assert_eq(&formatted);

    // Check for idempotence
    let syntax = syntax::parse(formatted.trim_start()).tree();
    let new_second = format_tree(&syntax, options)
        .expect("Formatting already formatted sources should never fail with an error");
    let position = panic::Location::caller();
    if formatted == new_second {
        return;
    }

    println!(
        "\n
\x1b[1m\x1b[91merror\x1b[97m: Formatting Idempotence check failed\x1b[0m
\x1b[1m\x1b[34m-->\x1b[0m {position}
\x1b[1mExpect\x1b[0m:
----
{formatted}
----

\x1b[1mActual\x1b[0m:
----
{new_second}
----

"
    );
    #[cfg(test)]
    {
        let diff = dissimilar::diff(&formatted, &new_second);
        println!(
            "
\x1b[1mDiff\x1b[0m:
----
{}
----
",
            format_chunks(diff)
        );
    }
    // Use resume_unwind instead of panic!() to prevent a backtrace, which is unnecessary noise.
    panic::resume_unwind(Box::new(()));
}

#[cfg(test)]
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
