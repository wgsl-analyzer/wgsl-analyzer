//! Missing batteries for standard libraries.

#![warn(unused)]

use std::process::Command;
use std::{cmp::Ordering, ops};
use std::{hash, io as sio};

mod macros;

pub mod anymap;
pub mod assert;
pub mod non_empty_vec;
pub mod panic_context;
pub mod process;
pub mod rand;
pub mod tempfile;
pub mod thread;
pub mod variance;

pub use itertools;

#[must_use]
pub const fn is_ci() -> bool {
    option_env!("CI").is_some()
}

#[expect(
    clippy::impl_trait_in_params,
    reason = "API - generic parameter is not bound to parameters so it must be written"
)]
pub fn hash_once<Hasher>(thing: impl std::hash::Hash) -> u64
where
    Hasher: std::hash::Hasher + Default,
{
    hash::BuildHasher::hash_one(&hash::BuildHasherDefault::<Hasher>::default(), thing)
}

#[cfg(test)]
#[must_use]
#[expect(clippy::print_stderr, reason = "only visible to developers")]
pub fn timeit(label: &'static str) -> impl Drop {
    use std::time::Instant;
    let start = Instant::now();
    defer(move || eprintln!("{label}: {:.2}", start.elapsed().as_nanos()))
}

pub trait TupleExt {
    type Head;
    type Tail;
    fn head(self) -> Self::Head;
    fn tail(self) -> Self::Tail;
}

impl<T, U> TupleExt for (T, U) {
    type Head = T;
    type Tail = U;

    fn head(self) -> Self::Head {
        self.0
    }

    fn tail(self) -> Self::Tail {
        self.1
    }
}

impl<T, U, V> TupleExt for (T, U, V) {
    type Head = T;
    type Tail = V;

    fn head(self) -> Self::Head {
        self.0
    }

    fn tail(self) -> Self::Tail {
        self.2
    }
}

impl<T> TupleExt for &T
where
    T: TupleExt + Copy,
{
    type Head = T::Head;
    type Tail = T::Tail;
    fn head(self) -> Self::Head {
        (*self).head()
    }
    fn tail(self) -> Self::Tail {
        (*self).tail()
    }
}

pub fn to_lower_snake_case(string: &str) -> String {
    to_snake_case(string, char::to_lowercase)
}

pub fn to_upper_snake_case(string: &str) -> String {
    to_snake_case(string, char::to_uppercase)
}

// Code partially taken from rust/compiler/rustc_lint/src/nonstandard_style.rs
// commit: 9626f2b
fn to_snake_case<F, I>(
    mut string: &str,
    change_case: F,
) -> String
where
    F: Fn(char) -> I,
    I: Iterator<Item = char>,
{
    let mut words = vec![];

    // Preserve leading underscores
    string = string.trim_start_matches(|character: char| {
        if character == '_' {
            words.push(String::new());
            true
        } else {
            false
        }
    });

    for string in string.split('_') {
        let mut last_upper = false;
        let mut buffer = String::new();

        if string.is_empty() {
            continue;
        }

        for character in string.chars() {
            if !buffer.is_empty() && buffer != "'" && character.is_uppercase() && !last_upper {
                words.push(buffer);
                buffer = String::new();
            }

            last_upper = character.is_uppercase();
            buffer.extend(change_case(character));
        }

        words.push(buffer);
    }

    words.join("_")
}

// Taken from rustc.
#[must_use]
pub fn to_camel_case(identifier: &str) -> String {
    identifier
        .trim_matches('_')
        .split('_')
        .filter(|component| !component.is_empty())
        .map(|component| {
            let mut camel_cased_component = String::with_capacity(component.len());

            let mut new_word = true;
            let mut prev_is_lower_case = true;

            for character in component.chars() {
                // Preserve the case if an uppercase letter follows a lowercase letter, so that
                // `camelCase` is converted to `CamelCase`.
                if prev_is_lower_case && character.is_uppercase() {
                    new_word = true;
                }

                if new_word {
                    camel_cased_component.extend(character.to_uppercase());
                } else {
                    camel_cased_component.extend(character.to_lowercase());
                }

                prev_is_lower_case = character.is_lowercase();
                new_word = false;
            }

            camel_cased_component
        })
        .fold(
            (String::new(), None),
            |(mut accumulator, previous): (_, Option<String>), next| {
                // separate two components with an underscore if their boundary cannot
                // be distinguished using an uppercase/lowercase case distinction
                let join = previous
                    .and_then(|previous| {
                        let first = next.chars().next()?;
                        let last = previous.chars().last()?;
                        Some(!char_has_case(last) && !char_has_case(first))
                    })
                    .unwrap_or(false);
                accumulator.push_str(if join { "_" } else { "" });
                accumulator.push_str(&next);
                (accumulator, Some(next))
            },
        )
        .0
}

// Taken from rustc.
#[must_use]
pub const fn char_has_case(character: char) -> bool {
    character.is_lowercase() || character.is_uppercase()
}

pub fn replace(
    buffer: &mut String,
    from: char,
    to: &str,
) {
    let replace_count = buffer.chars().filter(|&ch| ch == from).count();
    if replace_count == 0 {
        return;
    }
    let from_len = from.len_utf8();
    let additional = to.len().saturating_sub(from_len);
    buffer.reserve(additional * replace_count);

    let mut end = buffer.len();
    while let Some(index) = buffer[..end].rfind(from) {
        buffer.replace_range(index..index + from_len, to);
        end = index;
    }
}

#[must_use]
pub fn trim_indent(mut text: &str) -> String {
    if text.starts_with('\n') {
        text = &text[1..];
    }
    let indent = indent_of(text);
    text.split_inclusive('\n')
        .map(|line| {
            if line.len() <= indent {
                line.trim_start_matches(' ')
            } else {
                &line[indent..]
            }
        })
        .collect()
}

#[must_use]
fn indent_of(text: &str) -> usize {
    text.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|trimmed_line| trimmed_line.len() - trimmed_line.trim_start().len())
        .min()
        .unwrap_or(0)
}

#[must_use]
pub fn dedent_by(
    spaces: usize,
    text: &str,
) -> String {
    text.split_inclusive('\n')
        .map(|line| {
            let trimmed = line.trim_start_matches(' ');
            if line.len() - trimmed.len() <= spaces {
                trimmed
            } else {
                &line[spaces..]
            }
        })
        .collect()
}

pub fn equal_range_by<T, F>(
    slice: &[T],
    mut key: F,
) -> ops::Range<usize>
where
    F: FnMut(&T) -> Ordering,
{
    let start = slice.partition_point(|item| key(item) == Ordering::Less);
    let length = slice[start..].partition_point(|item| key(item) == Ordering::Equal);
    start..start + length
}

/// Constructs a type that, when dropped, calls `function()`.
#[must_use]
pub fn defer<Function>(function: Function) -> impl Drop
where
    Function: FnOnce(),
{
    struct Droppable<F: FnOnce()>(Option<F>);

    impl<F: FnOnce()> Drop for Droppable<F> {
        fn drop(&mut self) {
            if let Some(function) = self.0.take() {
                function();
            }
        }
    }
    Droppable(Some(function))
}

/// A [`std::process::Child`] wrapper that will kill the child on drop.
#[cfg_attr(not(target_arch = "wasm32"), repr(transparent))]
#[derive(Debug)]
pub struct JodChild(pub std::process::Child);

impl ops::Deref for JodChild {
    type Target = std::process::Child;

    fn deref(&self) -> &std::process::Child {
        &self.0
    }
}

impl ops::DerefMut for JodChild {
    fn deref_mut(&mut self) -> &mut std::process::Child {
        &mut self.0
    }
}

impl Drop for JodChild {
    fn drop(&mut self) {
        let _unused1 = self.0.kill();
        let _unused2 = self.0.wait();
    }
}

impl JodChild {
    pub fn spawn(mut command: Command) -> sio::Result<Self> {
        command.spawn().map(Self)
    }

    #[must_use]
    #[cfg(not(target_arch = "wasm32"))]
    pub fn into_inner(self) -> std::process::Child {
        // SAFETY: repr transparent, except on WASM
        unsafe { std::mem::transmute::<Self, std::process::Child>(self) }
    }
}

// feature: iter_order_by
// Iterator::eq_by
// https://github.com/rust-lang/rust/issues/64295
pub fn iter_eq_by<I, I2, F>(
    this: I2,
    other: I,
    mut eq: F,
) -> bool
where
    I: IntoIterator,
    I2: IntoIterator,
    F: FnMut(I2::Item, I::Item) -> bool,
{
    let mut other = other.into_iter();
    let mut this = this.into_iter();

    loop {
        let Some(an_item) = this.next() else {
            return other.next().is_none();
        };

        let Some(another_item) = other.next() else {
            return false;
        };

        if !eq(an_item, another_item) {
            return false;
        }
    }
}

/// Returns all final segments of the argument, longest first.
pub fn slice_tails<T>(this: &[T]) -> impl Iterator<Item = &[T]> {
    (0..this.len()).map(|index| &this[index..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_camel_case_works() {
        assert_eq!(to_camel_case("___"), "");
        assert_eq!(to_camel_case("hello_world"), "HelloWorld");
        assert_eq!(to_camel_case("camelCase"), "CamelCase");
        assert_eq!(to_camel_case("XML_http"), "XmlHttp");
        assert_eq!(to_camel_case("123_456"), "123_456");
        assert_eq!(to_camel_case("__foo___bar__"), "FooBar");
    }

    #[test]
    fn to_snake_case_works() {
        assert_eq!(to_lower_snake_case("_"), "");
        assert_eq!(to_lower_snake_case("HelloWorld"), "hello_world");
        assert_eq!(to_lower_snake_case("_FooBar"), "_foo_bar");
        assert_eq!(to_upper_snake_case("_"), "");
        assert_eq!(to_upper_snake_case("HelloWorld"), "HELLO_WORLD");
        assert_eq!(to_upper_snake_case("_FooBar"), "_FOO_BAR");
    }

    #[test]
    fn trim_indent_works() {
        assert_eq!(trim_indent(""), "");
        assert_eq!(
            trim_indent(
                "
            hello
            world
"
            ),
            "hello\nworld\n"
        );
        assert_eq!(
            trim_indent(
                "
            hello
            world"
            ),
            "hello\nworld"
        );
        assert_eq!(trim_indent("    hello\n    world\n"), "hello\nworld\n");
        assert_eq!(
            trim_indent(
                "
            fn main() {
                return 92;
            }
        "
            ),
            "fn main() {\n    return 92;\n}\n"
        );
    }

    #[test]
    fn dedent_works() {
        assert_eq!(dedent_by(0, ""), "");
        assert_eq!(dedent_by(1, ""), "");
        assert_eq!(dedent_by(2, ""), "");
        assert_eq!(dedent_by(0, "foo"), "foo");
        assert_eq!(dedent_by(2, "foo"), "foo");
        assert_eq!(dedent_by(2, "  foo"), "foo");
        assert_eq!(dedent_by(2, "    foo"), "  foo");
        assert_eq!(dedent_by(2, "    foo\nbar"), "  foo\nbar");
        assert_eq!(dedent_by(2, "foo\n    bar"), "foo\n  bar");
        assert_eq!(dedent_by(2, "foo\n\n    bar"), "foo\n\n  bar");
        assert_eq!(dedent_by(2, "foo\n.\n    bar"), "foo\n.\n  bar");
        assert_eq!(dedent_by(2, "foo\n .\n    bar"), "foo\n.\n  bar");
        assert_eq!(dedent_by(2, "foo\n   .\n    bar"), "foo\n .\n  bar");
    }

    #[test]
    fn indent_of_works() {
        assert_eq!(indent_of(""), 0);
        assert_eq!(indent_of(" "), 0);
        assert_eq!(indent_of(" x"), 1);
        assert_eq!(indent_of(" x\n"), 1);
        assert_eq!(indent_of(" x\ny"), 0);
        assert_eq!(indent_of(" x\n y"), 1);
        assert_eq!(indent_of(" x\n  y"), 1);
        assert_eq!(indent_of("  x\n  y"), 2);
        assert_eq!(indent_of("  x\n  y\n"), 2);
        assert_eq!(indent_of("  x\n\n  y\n"), 2);
    }

    #[expect(clippy::non_ascii_literal, reason = "the point of the test")]
    #[test]
    fn replace_works() {
        #[track_caller]
        fn test_replace(
            src: &str,
            from: char,
            to: &str,
            expected: &str,
        ) {
            let mut source = src.to_owned();
            replace(&mut source, from, to);
            assert_eq!(source, expected, "from: {from:?}, to: {to:?}");
        }

        test_replace("", 'a', "b", "");
        test_replace("", 'a', "😀", "");
        test_replace("", '😀', "a", "");
        test_replace("a", 'a', "b", "b");
        test_replace("aa", 'a', "b", "bb");
        test_replace("ada", 'a', "b", "bdb");
        test_replace("a", 'a', "😀", "😀");
        test_replace("😀", '😀', "a", "a");
        test_replace("😀x", '😀', "a", "ax");
        test_replace("y😀x", '😀', "a", "yax");
        test_replace("a,b,c", ',', ".", "a.b.c");
        test_replace("a,b,c", ',', "..", "a..b..c");
        test_replace("a.b.c", '.', "..", "a..b..c");
        test_replace("a.b.c", '.', "..", "a..b..c");
        test_replace("a😀b😀c", '😀', ".", "a.b.c");
        test_replace("a.b.c", '.', "😀", "a😀b😀c");
        test_replace("a.b.c", '.', "😀😀", "a😀😀b😀😀c");
        test_replace(".a.b.c.", '.', "()", "()a()b()c()");
        test_replace(".a.b.c.", '.', "", "abc");
    }

    #[test]
    fn equal_range_by_edge_cases() {
        assert_eq!(equal_range_by(&[], |_: &i32| Ordering::Equal), 0..0);
        assert_eq!(equal_range_by(&[2, 2, 3], |index| index.cmp(&2)), 0..2);
        assert_eq!(
            equal_range_by(&[1, 2, 2, 2, 3], |index| index.cmp(&2)),
            1..4
        );
        assert_eq!(equal_range_by(&[1, 2, 2], |index| index.cmp(&2)), 1..3);
        assert_eq!(equal_range_by(&[1, 3], |index| index.cmp(&2)), 1..1);
        assert_eq!(equal_range_by(&[2, 3], |index| index.cmp(&1)), 0..0);
        assert_eq!(equal_range_by(&[1, 2], |index| index.cmp(&3)), 2..2);
        assert_eq!(equal_range_by(&[2, 2, 2], |index| index.cmp(&2)), 0..3);
    }

    #[test]
    fn defer_works() {
        let data = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
        {
            let _to_drop = defer(|| data.borrow_mut().push('a'));
            data.borrow_mut().push('b');
        }
        data.borrow_mut().push('c');
        assert_eq!(data.take(), "bac");
    }
}
