//! Assorted testing utilities.
//!
//! Most notable things are:
//!
//! * Rich text comparison, which outputs a diff.
//! * Extracting markup (mainly, `$0` markers) out of fixture strings.
//! * marks (see the eponymous module).

#![expect(clippy::min_ident_chars, reason = "pure math")]
#![expect(clippy::print_stderr, reason = "stderr is useful in testing")]

mod assert_linear;

use std::{
    collections::BTreeMap,
    env, fs,
    path::{Path, PathBuf},
};

use paths::Utf8PathBuf;
use profile::StopWatch;
use stdx::is_ci;
use text_size::{TextRange, TextSize};

pub use dissimilar::diff as __diff;
pub use rustc_hash::FxHashMap;

pub use crate::assert_linear::AssertLinear;

pub const CURSOR_MARKER: &str = "$0";
pub const ESCAPED_CURSOR_MARKER: &str = "\\$0";

/// Asserts that two strings are equal, otherwise displays a rich diff between them.
///
/// The diff shows changes from the "original" left string to the "actual" right string.
///
/// All arguments starting from and including the 3rd one are passed to
/// `eprintln!()` macro in case of text inequality.
#[macro_export]
macro_rules! assert_eq_text {
    ($left:expr, $right:expr) => {
        assert_eq_text!($left, $right,)
    };
    ($left:expr, $right:expr, $($tt:tt)*) => {{
        let left = $left;
        let right = $right;
        if left != right {
            if left.trim() == right.trim() {
                std::eprintln!("Left:\n{:?}\n\nRight:\n{:?}\n\nWhitespace difference\n", left, right);
            } else {
                let diff = $crate::__diff(left, right);
                std::eprintln!("Left:\n{}\n\nRight:\n{}\n\nDiff:\n{}\n", left, right, $crate::format_diff(diff));
            }
            std::eprintln!($($tt)*);
            panic!("text differs");
        }
    }};
}

/// Infallible version of `try_extract_offset()`.
///
/// # Panics
///
/// Panics if the text does not contain a cursor marker.
#[must_use]
pub fn extract_offset(text: &str) -> (TextSize, String) {
    match try_extract_offset(text) {
        None => panic!("text should contain cursor marker"),
        Some(result) => result,
    }
}

/// Returns the offset of the first occurrence of `$0` marker and the copy of `text`
/// without the marker.
fn try_extract_offset(text: &str) -> Option<(TextSize, String)> {
    let cursor_pos = text.find(CURSOR_MARKER)?;
    let mut new_text = String::with_capacity(text.len() - CURSOR_MARKER.len());
    new_text.push_str(&text[..cursor_pos]);
    new_text.push_str(&text[cursor_pos + CURSOR_MARKER.len()..]);
    let cursor_pos = TextSize::from(
        <usize as std::convert::TryInto<u32>>::try_into(cursor_pos)
            .expect("usize should be larger than u32"),
    );
    Some((cursor_pos, new_text))
}

/// Infallible version of `try_extract_range()`.
///
/// # Panics
///
/// Panics if the text does not contain a cursor marker.
#[must_use]
pub fn extract_range(text: &str) -> (TextRange, String) {
    match try_extract_range(text) {
        None => panic!("text should contain cursor marker"),
        Some(result) => result,
    }
}

/// Returns `TextRange` between the first two markers `$0...$0` and the copy
/// of `text` without both of these markers.
fn try_extract_range(text: &str) -> Option<(TextRange, String)> {
    let (start, text) = try_extract_offset(text)?;
    let (end, text) = try_extract_offset(&text)?;
    Some((TextRange::new(start, end), text))
}

#[derive(Clone, Copy, Debug)]
pub enum RangeOrOffset {
    Range(TextRange),
    Offset(TextSize),
}

impl RangeOrOffset {
    /// Expect that the value is a `RangeOrOffset::Offset`.
    ///
    /// # Panics
    ///
    /// Panics if the value is a `RangeOrOffset::Range`.
    #[must_use]
    pub fn expect_offset(self) -> TextSize {
        match self {
            Self::Offset(item) => item,
            Self::Range(_) => panic!("expected an offset but got a range instead"),
        }
    }

    /// Expect that the value is a `RangeOrOffset::Range`.
    ///
    /// # Panics
    ///
    /// Panics if the value is a `RangeOrOffset::Offset`.
    #[must_use]
    pub fn expect_range(self) -> TextRange {
        match self {
            Self::Range(item) => item,
            Self::Offset(_) => panic!("expected a range but got an offset"),
        }
    }

    /// Expect that the value is a `RangeOrOffset::Range` or if it is an empty `RangeOrOffset::Offset`.
    ///
    /// # Panics
    ///
    /// Panics if the value is a non-empty `RangeOrOffset::Offset`.
    #[must_use]
    pub const fn range_or_empty(self) -> TextRange {
        match self {
            Self::Range(range) => range,
            Self::Offset(offset) => TextRange::empty(offset),
        }
    }
}

impl From<RangeOrOffset> for TextRange {
    fn from(selection: RangeOrOffset) -> Self {
        match selection {
            RangeOrOffset::Range(item) => item,
            RangeOrOffset::Offset(item) => Self::empty(item),
        }
    }
}

/// Extracts `TextRange` or `TextSize` depending on the amount of `$0` markers
/// found in `text`.
///
/// # Panics
/// Panics if no `$0` marker is present in the `text`.
#[must_use]
pub fn extract_range_or_offset(text: &str) -> (RangeOrOffset, String) {
    if let Some((range, text)) = try_extract_range(text) {
        return (RangeOrOffset::Range(range), text);
    }
    let (offset, text) = extract_offset(text);
    (RangeOrOffset::Offset(offset), text)
}

/// Extracts ranges, marked with `<tag> </tag>` pairs from the `text`.
///
/// # Panics
///
/// Panics if tags are unmatches.
#[must_use]
pub fn extract_tags(
    mut text: &str,
    tag: &str,
) -> (Vec<(TextRange, Option<String>)>, String) {
    let open = format!("<{tag}");
    let close = format!("</{tag}>");
    let mut ranges = Vec::new();
    let mut result = String::new();
    let mut stack = Vec::new();
    loop {
        match text.find('<') {
            None => {
                result.push_str(text);
                break;
            },
            Some(i) => {
                result.push_str(&text[..i]);
                text = &text[i..];
                if text.starts_with(&open) {
                    let close_open = text.find('>').unwrap();
                    let attribute = text[open.len()..close_open].trim();
                    let attribute = if attribute.is_empty() {
                        None
                    } else {
                        Some(attribute.to_owned())
                    };
                    text = &text[close_open + '>'.len_utf8()..];
                    let from = TextSize::of(&result);
                    stack.push((from, attribute));
                } else if text.starts_with(&close) {
                    text = &text[close.len()..];
                    let (from, attribute) =
                        stack.pop().unwrap_or_else(|| panic!("unmatched </{tag}>"));
                    let to = TextSize::of(&result);
                    ranges.push((TextRange::new(from, to), attribute));
                } else {
                    result.push('<');
                    text = &text['<'.len_utf8()..];
                }
            },
        }
    }
    assert!(stack.is_empty(), "unmatched <{tag}>");
    ranges.sort_by_key(|r| (r.0.start(), r.0.end()));
    (ranges, result)
}

#[cfg(test)]
mod extract_tags_tests {
    use crate::extract_tags;

    #[test]
    fn can_extract_tags() {
        let (tags, text) = extract_tags("<tag fn>fn <tag>main</tag>() {}</tag>", "tag");
        let actual = tags
            .into_iter()
            .map(|(range, attr)| (&text[range], attr))
            .collect::<Vec<_>>();
        assert_eq!(
            actual,
            vec![("fn main() {}", Some("fn".into())), ("main", None),]
        );
    }
}

/// Inserts `$0` marker into the `text` at `offset`.
#[must_use]
pub fn add_cursor(
    text: &str,
    offset: TextSize,
) -> String {
    let offset: usize = offset.into();
    let mut result = String::new();
    result.push_str(&text[..offset]);
    result.push_str("$0");
    result.push_str(&text[offset..]);
    result
}

/// Extracts `//^^^ some text` annotations.
///
/// A run of `^^^` can be arbitrary long and points to the corresponding range
/// in the line above.
///
/// The `// ^file text` syntax can be used to attach `text` to the entirety of
/// the file.
///
/// Multiline string values are supported:
///
/// // ^^^ first line
/// //   | second line
///
/// Trailing whitespace is sometimes desired but usually stripped by the editor
/// if at the end of a line, or incorrectly sized if followed by another
/// annotation. In those cases the annotation can be explicitly ended with the
/// `$` character.
///
/// // ^^^ trailing-ws-wanted  $
///
/// Annotations point to the last line that actually was long enough for the
/// range, not counting annotations themselves. So overlapping annotations are
/// possible:
/// ```text
/// // stuff        other stuff
/// // ^^ 'st'
/// // ^^^^^ 'stuff'
/// //              ^^^^^^^^^^^ 'other stuff'
/// ```
///
/// # Panics
///
/// Panics if the internal algorithm is incorrect.
#[must_use]
pub fn extract_annotations(text: &str) -> Vec<(TextRange, String)> {
    let mut result = Vec::new();
    // map from line length to beginning of last line that had that length
    let mut line_start_map = BTreeMap::new();
    let mut line_start: TextSize = 0.into();
    let mut previous_line_annotations: Vec<(TextSize, usize)> = Vec::new();
    for line in text.split_inclusive('\n') {
        let mut this_line_annotations = Vec::new();
        let line_length = if let Some((prefix, suffix)) = line.split_once("//") {
            let ss_len = TextSize::of("//");
            let annotation_offset = TextSize::of(prefix) + ss_len;
            for annotation in extract_line_annotations(suffix.trim_end_matches('\n')) {
                match annotation {
                    LineAnnotation::Annotation {
                        mut range,
                        content,
                        file,
                    } => {
                        range += annotation_offset;
                        this_line_annotations.push((range.end(), result.len()));
                        let range = if file {
                            TextRange::up_to(TextSize::of(text))
                        } else {
                            let line_start = line_start_map.range(range.end()..).next().unwrap();

                            range + line_start.1
                        };
                        result.push((range, content));
                    },
                    LineAnnotation::Continuation {
                        mut offset,
                        content,
                    } => {
                        offset += annotation_offset;
                        let &(_, index) = previous_line_annotations
                            .iter()
                            .find(|&&(off, _idx)| off == offset)
                            .unwrap();
                        result[index].1.push('\n');
                        result[index].1.push_str(&content);
                        result[index].1.push('\n');
                    },
                }
            }
            annotation_offset
        } else {
            TextSize::of(line)
        };

        line_start_map = line_start_map.split_off(&line_length);
        line_start_map.insert(line_length, line_start);

        line_start += TextSize::of(line);

        previous_line_annotations = this_line_annotations;
    }
    result
}

enum LineAnnotation {
    Annotation {
        range: TextRange,
        content: String,
        file: bool,
    },
    Continuation {
        offset: TextSize,
        content: String,
    },
}

fn extract_line_annotations(mut line: &str) -> Vec<LineAnnotation> {
    let mut result = Vec::new();
    let mut offset: TextSize = 0.into();
    let marker: fn(char) -> bool = if line.contains('^') {
        |c| c == '^'
    } else {
        |c| c == '|'
    };
    while let Some(index) = line.find(marker) {
        offset += TextSize::try_from(index).unwrap();
        line = &line[index..];

        let mut length = line.chars().take_while(|&item| item == '^').count();
        let mut continuation = false;
        if length == 0 {
            assert!(line.starts_with('|'));
            continuation = true;
            length = 1;
        }
        let range = TextRange::at(offset, length.try_into().unwrap());
        let line_no_caret = &line[length..];
        let end_marker = line_no_caret.find('$');
        let next = line_no_caret
            .find(marker)
            .map_or(line.len(), |item| item + length);

        let cond = |end_marker| {
            end_marker < next
                && (line_no_caret[end_marker + 1..].is_empty()
                    || line_no_caret[end_marker + 1..]
                        .strip_prefix(|c: char| c.is_whitespace() || c == '^')
                        .is_some())
        };
        let mut content = match end_marker {
            Some(end_marker) if cond(end_marker) => &line_no_caret[..end_marker],
            _ => line_no_caret[..next - length].trim_end(),
        };

        let mut file = false;
        if !continuation && content.starts_with("file") {
            file = true;
            content = &content["file".len()..];
        }

        let content = content.trim_start().to_owned();

        let annotation = if continuation {
            LineAnnotation::Continuation {
                offset: range.end(),
                content,
            }
        } else {
            LineAnnotation::Annotation {
                range,
                content,
                file,
            }
        };
        result.push(annotation);
        line = &line[next..];
        offset += TextSize::try_from(next).unwrap();
    }
    result
}

#[cfg(test)]
mod extract_annotations_tests {
    use crate::extract_annotations;

    #[test]
    fn can_extract_annotations_1() {
        let text = stdx::trim_indent(
            "
fn main() {
    let (x,     y) = (9, 2);
       //^ def  ^ def
    zoo + 1
} //^^^ type:
  //  | i32

// ^file
    ",
        );
        let result = extract_annotations(&text)
            .into_iter()
            .map(|(range, ann)| (&text[range], ann))
            .collect::<Vec<_>>();

        assert_eq!(
            result[..3],
            [
                ("x", "def".into()),
                ("y", "def".into()),
                ("zoo", "type:\ni32\n".into())
            ]
        );
        assert_eq!(result[3].0.len(), 115);
    }

    #[test]
    fn can_extract_annotations_2() {
        let text = stdx::trim_indent(
            "
fn main() {
    (x,   y);
   //^ a
      //  ^ b
  //^^^^^^^^ c
}",
        );
        let result = extract_annotations(&text)
            .into_iter()
            .map(|(range, ann)| (&text[range], ann))
            .collect::<Vec<_>>();

        assert_eq!(
            result,
            [
                ("x", "a".into()),
                ("y", "b".into()),
                ("(x,   y)", "c".into())
            ]
        );
    }
}

/// Returns `false` if slow tests should not run, otherwise returns `true` and
/// also creates a file at `./target/.slow_tests_cookie` which serves as a flag
/// that slow tests did run.
///
/// # Panics
///
/// Panics if the path could not be written to.
#[must_use]
pub fn skip_slow_tests() -> bool {
    let should_skip = (std::env::var("CI").is_err() && std::env::var("RUN_SLOW_TESTS").is_err())
        || std::env::var("SKIP_SLOW_TESTS").is_ok();
    if should_skip {
        eprintln!("ignoring slow test");
    } else {
        let path = target_dir().join(".slow_tests_cookie");
        fs::write(path, ".").unwrap();
    }
    should_skip
}

#[must_use]
pub fn target_dir() -> Utf8PathBuf {
    match std::env::var("CARGO_TARGET_DIR") {
        Ok(target) => Utf8PathBuf::from(target),
        Err(_) => project_root().join("target"),
    }
}

/// Returns the path to the root directory of `wgsl-analyzer` project.
///
/// # Panics
///
/// Panics if the value of environment variable `CARGO_MANIFEST_DIR` is not a valid path.
#[must_use]
pub fn project_root() -> Utf8PathBuf {
    let directory = env!("CARGO_MANIFEST_DIR");
    Utf8PathBuf::from_path_buf(
        PathBuf::from(directory)
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_owned(),
    )
    .unwrap()
}

#[must_use]
pub fn format_diff(chunks: Vec<dissimilar::Chunk<'_>>) -> String {
    let mut buffer = String::new();
    for chunk in chunks {
        let formatted = match chunk {
            dissimilar::Chunk::Equal(text) => text.into(),
            dissimilar::Chunk::Delete(text) => format!("\x1b[41m{text}\x1b[0m\x1b[K"),
            dissimilar::Chunk::Insert(text) => format!("\x1b[42m{text}\x1b[0m\x1b[K"),
        };
        buffer.push_str(&formatted);
    }
    buffer
}

/// Utility for writing benchmark tests.
///
/// A benchmark test looks like this:
///
/// ```ignore
/// #[test]
/// fn benchmark_foo() {
///     if skip_slow_tests() { return; }
///
///     let data = bench_fixture::some_fixture();
///     let analysis = some_setup();
///
///     let hash = {
///         let _b = bench("foo");
///         actual_work(analysis)
///     };
///     assert_eq!(hash, 92);
/// }
/// ```
///
/// * We skip benchmarks by default, to save time.
///   Ideal benchmark time is 800 -- 1500 ms in debug.
/// * We don't count preparation as part of the benchmark
/// * The benchmark itself returns some kind of numeric hash.
///   The hash is used as a sanity check that some code is actually run.
///   Otherwise, it's too easy to win the benchmark by just doing nothing.
#[must_use]
pub fn bench(label: &'static str) -> impl Drop {
    struct Bencher {
        sw: StopWatch,
        label: &'static str,
    }

    impl Drop for Bencher {
        fn drop(&mut self) {
            eprintln!("{}: {}", self.label, self.sw.elapsed());
        }
    }

    Bencher {
        sw: StopWatch::start(),
        label,
    }
}

/// Checks that the `file` has the specified `contents`. If that is not the
/// case, updates the file and then fails the test.
///
/// # Panics
///
/// Panics if files are not up to date.
#[track_caller]
pub fn ensure_file_contents(
    file: &Path,
    contents: &str,
) {
    assert!(
        try_ensure_file_contents(file, contents) != Err(()),
        "Some files were not up-to-date"
    );
}

/// Checks that the `file` has the specified `contents`. If that is not the
/// case, updates the file and return an Error.
///
/// # Panics
///
/// Panics if the file could not be written.
#[expect(clippy::result_unit_err, reason = "Simple enough")]
pub fn try_ensure_file_contents(
    file: &Path,
    contents: &str,
) -> Result<(), ()> {
    match std::fs::read_to_string(file) {
        Ok(old_contents) if normalize_newlines(&old_contents) == normalize_newlines(contents) => {
            return Ok(());
        },
        _ => (),
    }
    let display_path = file.strip_prefix(project_root()).unwrap_or(file);
    eprintln!(
        "\n\x1b[31;1merror\x1b[0m: {} was not up-to-date, updating\n",
        display_path.display()
    );
    if is_ci() {
        eprintln!("    NOTE: run `cargo test` locally and commit the updated files\n");
    }
    if let Some(parent) = file.parent() {
        std::fs::create_dir_all(parent);
    }
    std::fs::write(file, contents).unwrap();
    Err(())
}

fn normalize_newlines(s: &str) -> String {
    s.replace("\r\n", "\n")
}
