#![expect(clippy::print_stdout, reason = "useful in tests")]
#![expect(clippy::use_debug, reason = "useful in tests")]
#![expect(
    clippy::missing_panics_doc,
    reason = "we want to be able to use assert!"
)]
use std::{borrow::ToOwned, fmt::Debug, panic};

use itertools::Itertools as _;
use parser::{Capabilities, Edition, ParseEntryPoint};
use rowan::{TextLen as _, TextRange};

use crate::{FormattingOptions, IndentStyle, format::format_tree, format_range};

//Taken from expect_test
// TODO(MonaMayrhofer,blocked) This can be removed once expect_test exposes the trimmed data
// https://github.com/rust-analyzer/expect-test/issues/60
mod strip_indent {
    pub(crate) fn trim_indent(mut text: &str) -> String {
        if text.starts_with('\n') {
            text = &text[1..];
        }
        let indent = text
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.len() - line.trim_start().len())
            .min()
            .unwrap_or(0);

        lines_with_ends(text)
            .map(|line| {
                if line.len() <= indent {
                    line.trim_start_matches(' ')
                } else {
                    &line[indent..]
                }
            })
            .collect()
    }

    const fn lines_with_ends(text: &str) -> LinesWithEnds<'_> {
        LinesWithEnds { text }
    }

    struct LinesWithEnds<'text> {
        text: &'text str,
    }

    impl<'text> Iterator for LinesWithEnds<'text> {
        type Item = &'text str;
        fn next(&mut self) -> Option<&'text str> {
            if self.text.is_empty() {
                return None;
            }
            let index = self
                .text
                .find('\n')
                .map_or(self.text.len(), |position| position + 1);
            let (result, next) = self.text.split_at(index);
            self.text = next;
            Some(result)
        }
    }
}

pub trait ExpectAssertEq: Debug {
    fn assert_eq(
        &self,
        other: &str,
    );

    fn trimmed_data(&self) -> String;
}

#[cfg(test)]
impl ExpectAssertEq for expect_test::Expect {
    fn assert_eq(
        &self,
        other: &str,
    ) {
        self.assert_eq(other);
    }

    fn trimmed_data(&self) -> String {
        strip_indent::trim_indent(self.data())
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

    fn trimmed_data(&self) -> String {
        self.data()
    }
}

impl ExpectAssertEq for &str {
    fn assert_eq(
        &self,
        other: &str,
    ) {
        assert_eq!(*self, other);
    }

    fn trimmed_data(&self) -> String {
        self.to_string()
    }
}

#[expect(clippy::needless_pass_by_value, reason = "intentional API")]
pub fn check<E>(
    before: &str,
    after: E,
) -> String
where
    E: ExpectAssertEq,
{
    check_with_options(
        before,
        &after,
        &FormattingOptions::default().into(),
        Edition::LATEST,
    )
}

/// Checks that the given source raises parsing diagnostics and is
/// thus out of scope of having to be formatted correctly.
///
/// Code that is out of scope would just be left untouched by the formatter.
///
/// Even tho these tests only test the behavior of the parser,
/// they are useful to be included in the formatter unit tests,
/// in order to keep track of the boundaries of what the
/// formatter is supposed to deal with and to "get notified"
/// as soon as the parser starts implementing certain new features.
///
/// These assertions also exist, to help document certain design decisions,
/// and prevent people from doing futile work, trying to "complete holes
/// in the functionality of the formatter", while not realizing that there a
/// reason certain things are intentionally not supported.
#[track_caller]
pub fn assert_out_of_scope(
    before: &str,
    reason: &str,
) {
    let parse = syntax::parse(before.trim_start(), Edition::LATEST);
    let _syntax = parse.tree();

    if parse.errors().is_empty() {
        println!(
            "Expected source to raise parsing diagnostics and as such be out of scope for formatting. Reason: {reason}\nHowever the given source parsed without error. \nSource: {before}"
        );
        // Use resume_unwind instead of panic!() to prevent a backtrace, which is unnecessary noise.
        panic::resume_unwind(Box::new(()));
    }
}

#[derive(Default)]
pub struct CheckOptions {
    pub assert_line_width: Option<usize>,
    pub formatting: FormattingOptions,
}
impl From<FormattingOptions> for CheckOptions {
    fn from(value: FormattingOptions) -> Self {
        Self {
            assert_line_width: Some(
                usize::try_from(value.max_line_width).expect("max_line_width must fit into usize"),
            ),
            formatting: value,
        }
    }
}

#[track_caller]
pub fn check_with_options<E>(
    before: &str,
    after: &E,
    options: &CheckOptions,
    edition: Edition,
) -> String
where
    E: ExpectAssertEq,
{
    let parse = syntax::parse(before.trim_start(), edition);
    let syntax = parse.tree();
    //dbg!(&syntax);

    assert!(
        parse.errors().is_empty(),
        "Parsing the source to be formatted failed with errors: {:#?} \n Source: {:#?}",
        parse.errors(),
        parse.syntax()
    );

    // dbg!(&parse.errors());
    let formatted = match format_tree(&syntax, &options.formatting) {
        Ok(formatted) => formatted,
        Err(format_error) => {
            println!("Formatting returned an unexpected error: {format_error:?}");
            panic::resume_unwind(Box::new(()));
        },
    };

    after.assert_eq(&formatted);

    if let Some(max_line_width) = options.assert_line_width {
        for line in formatted.lines() {
            assert!(
                line.chars().count() <= max_line_width,
                "Formatted line length must be <= {max_line_width}\nOffending line (at length {}):\n{line}",
                line.len(),
            );
        }
    }

    println!("==Idempodence check==");

    // Check for idempotence
    let syntax = syntax::parse(formatted.trim_start(), edition).tree();
    //dbg!(&syntax);

    let formatted_twice = format_tree(&syntax, &options.formatting)
        .expect("Formatting already formatted sources should never fail with an error");
    let position = panic::Location::caller();
    if formatted != formatted_twice {
        println!(
            "\n
\x1b[1m\x1b[91merror\x1b[97m: Formatting Idempotence check failed. Expected output to after two formatting passes to equal the output after just one.\x1b[0m
\x1b[1m\x1b[34m-->\x1b[0m {position}
\x1b[1mAfter one formatting pass\x1b[0m:
----
{formatted}
----

\x1b[1mAfter two formatting passes\x1b[0m:
----
{formatted_twice}
----
"
    );
        #[cfg(test)]
        {
            let diff = dissimilar::diff(&formatted, &formatted_twice);
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

    //Check tabs
    if options.formatting.indent_style != IndentStyle::Tabs {
        let tab_source = before.replace("    ", "\t");

        let parse = syntax::parse(tab_source.trim_start(), edition);
        let syntax = parse.tree();

        assert!(
            parse.errors().is_empty(),
            "Parsing the source after inserting tabs failed with errors: {:#?} \n Source: {:#?}",
            parse.errors(),
            parse.syntax()
        );

        // dbg!(&parse.errors());
        let formatted = match format_tree(
            &syntax,
            &FormattingOptions {
                indent_style: IndentStyle::Tabs,
                ..options.formatting
            },
        ) {
            Ok(formatted) => formatted,
            Err(format_error) => {
                println!("Formatting returned an unexpected error: {format_error:?}");
                panic::resume_unwind(Box::new(()));
            },
        };

        let after_trimmed = strip_indent::trim_indent(&after.trimmed_data()).replace("    ", "\t");
        assert_eq!(after_trimmed, formatted, "Formatting with tabs should work");
    }

    formatted
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
/// Replaces all occurrences of `##` in the `before` string with block and line comments.
///
/// THIS SHOULD *NOT* BE USED TO TEST POSITIONING OF COMMENTS.
/// This should only be used to test if all comments exist and are placed in the
/// correct order.
/// If exact positioning with regard to line-breaks and spaces is important,
/// write an explicit test instead.
///
/// The `before` string should be a one-liner, because for line-comments the newlines
/// will get inserted, and if its multiline already, then there might be cases
/// where there are two newlines after one another, which could lead to unexpected
/// empty lines.
///
/// For line comments, `##` gets replaced with line comments of an increasing number.
/// `## a ## b` would become:
/// ```compile_fail
/// // 0
/// a // 1
/// b
/// ```
///
/// For block comments, `##` gets replaced with block comments of an increasing number.
/// `## a ## b` would become:
/// ```compile_fail
/// /* 0 */ a /* 1 */ b
/// ```
pub fn check_comments<E>(
    before: &str,
    after_block: E,
    after_line: E,
) where
    E: ExpectAssertEq,
{
    let before = before.lines().join("");
    {
        let mut comment_index = 0;
        let commented: String = itertools::Itertools::intersperse_with(
            before.split("##").map(ToOwned::to_owned),
            || {
                let comment = format!("// {comment_index}\n");
                comment_index += 1;
                comment
            },
        )
        .join("");
        let formatted = check_with_options(
            &commented,
            &after_line,
            &CheckOptions {
                assert_line_width: None,
                formatting: FormattingOptions::default(),
            },
            Edition::LATEST,
        );

        //Check that all the comments are still present after formatting
        let mut remainder = formatted.as_str();
        for search_index in 0..comment_index {
            if let Some((_, after)) = remainder.split_once(&format!("// {search_index}")) {
                remainder = after;
            } else {
                panic!(
                    "Expected to find a comment with number {search_index} within the formatted string: {remainder}"
                )
            }
        }

        // Check that every line is indented by a multiple of 4 spaces
        assert!(
            formatted.lines().all(|line| {
                let indent_width = line
                    .split_once(|ch| ch != ' ')
                    .map_or(0, |(indent, _)| indent.len());
                indent_width % 4 == 0
            }),
            "Expected every line to be indented by a multiple of 4 spaces"
        );
    }
    {
        let mut comment_index = 0;
        let commented: String = itertools::Itertools::intersperse_with(
            before.split("##").map(ToOwned::to_owned),
            || {
                let comment = format!("/* {comment_index} */");
                comment_index += 1;
                comment
            },
        )
        .join("");
        let formatted = check_with_options(
            &commented,
            &after_block,
            &CheckOptions {
                assert_line_width: None,
                formatting: FormattingOptions::default(),
            },
            Edition::LATEST,
        );

        //Check that all the comments are still present after formatting
        let mut remainder = formatted.as_str();
        for search_index in 0..comment_index {
            if let Some((_, after)) = remainder.split_once(&format!("/* {search_index} */")) {
                remainder = after;
            } else {
                panic!(
                    "Expected to find a comment with number {search_index} within the formatted string: {remainder}"
                )
            }
        }

        // Check that every line is indented by a multiple of 4 spaces
        assert!(
            formatted.lines().all(|line| {
                let indent_width = line
                    .split_once(|ch| ch != ' ')
                    .map_or(0, |(indent, _)| indent.len());
                indent_width % 4 == 0
            }),
            "Expected every line to be indented by a multiple of 4 spaces"
        );
    }
}

/// Simulates wgsl-analyzer range formatting.
///
/// The source string should contain exactly two #|# markers, that represent the start and end of the selected
/// range to format. They will be removed from the string, and then that range will be formatted.
///
/// Note that the range formatting works on the level of syntax nodes, so the node that will be formatted might
/// be larger than the range specified by the markers.
#[expect(clippy::needless_pass_by_value, reason = "intentional API")]
pub fn check_range<E>(
    source: &str,
    expected: E,
) where
    E: ExpectAssertEq,
{
    let (raw_text, range_to_format) = {
        let mut parts = source.split("#|#");
        let pre = parts
            .next()
            .expect("Source must contain exactly two #|# markers");
        let to_format = parts
            .next()
            .expect("Source must contain exactly two #|# markers");
        let post = parts
            .next()
            .expect("Source must contain exactly two #|# markers");
        assert!(
            parts.next().is_none(),
            "Source must contain exactly two #|# markers"
        );
        let raw_text = format!("{pre}{to_format}{post}");
        let range_to_format = TextRange::at(pre.text_len(), to_format.text_len());
        (raw_text, range_to_format)
    };

    let parse = parser::parse_entrypoint_with_capabilities(
        &raw_text,
        ParseEntryPoint::File,
        Edition::LATEST,
        Capabilities::default(),
    );
    assert!(parse.errors().is_empty());

    let formatted = format_range(
        &parse.syntax(),
        Some(range_to_format),
        &FormattingOptions::default(),
    )
    .unwrap();
    let pre_formatted = &raw_text[TextRange::up_to(formatted.range.start())];
    let post_formatted = &raw_text[TextRange::new(formatted.range.end(), raw_text.text_len())];
    let formatted_text = format!("{pre_formatted}{}{post_formatted}", formatted.formatted);

    expected.assert_eq(&formatted_text);
}

#[must_use]
pub fn strip_leading_indentation(text: &str) -> String {
    let Some(first_line) = text.lines().find(|line| !line.is_empty()) else {
        return text.to_owned();
    };
    let indentation: String = first_line
        .chars()
        .take_while(|character| character.is_whitespace())
        .join("");

    text.lines()
        .skip_while(|line| line.is_empty())
        .map(|line| line.strip_prefix(&indentation).unwrap_or(line))
        .join("\n")
}
