//! Missing batteries for standard libraries.

use std::{
    cmp::Ordering,
    hash::{BuildHasher, BuildHasherDefault},
    ops,
    time::Instant,
};
use std::{hash::Hash, io as sio};
use std::{hash::Hasher, process::Command};
pub mod anymap;
pub mod assert;
mod macros;
pub mod non_empty_vec;
pub mod panic_context;
pub mod process;
pub mod thin_vec;
pub mod thread;

pub use itertools;

#[expect(clippy::inline_always, reason = "copy pasted from r-a")]
#[inline(always)]
#[must_use]
pub const fn is_ci() -> bool {
    option_env!("CI").is_some()
}

#[inline]
pub fn hash_once<AHasher: Hasher + Default, Hashable: Hash>(thing: Hashable) -> u64 {
    BuildHasher::hash_one(&BuildHasherDefault::<AHasher>::default(), thing)
}

#[must_use]
#[inline]
#[expect(clippy::print_stderr, reason = "copy pasted from r-a")]
pub fn timeit(label: &'static str) -> impl Drop {
    let start = Instant::now();
    defer(move || eprintln!("{}: {:.2}", label, start.elapsed().as_nanos()))
}

/// Prints backtrace to stderr, useful for debugging.
#[expect(clippy::print_stderr, reason = "copy pasted from r-a")]
#[inline]
pub fn print_backtrace() {
    #[cfg(feature = "backtrace")]
    #[expect(clippy::use_debug, reason = "Backtrace does not implement Display")]
    {
        eprintln!("{:?}", backtrace::Backtrace::new());
    }

    #[cfg(not(feature = "backtrace"))]
    eprintln!(
        r#"Enable the backtrace feature.
Uncomment `default = [ "backtrace" ]` in `crates/stdx/Cargo.toml`.
"#
    );
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

    #[inline]
    fn head(self) -> Self::Head {
        self.0
    }

    #[inline]
    fn tail(self) -> Self::Tail {
        self.1
    }
}

impl<T, U, V> TupleExt for (T, U, V) {
    type Head = T;
    type Tail = V;

    #[inline]
    fn head(self) -> Self::Head {
        self.0
    }

    #[inline]
    fn tail(self) -> Self::Tail {
        self.2
    }
}

#[inline]
pub fn to_lower_snake_case(string: &str) -> String {
    to_snake_case(string, char::to_lowercase)
}

#[inline]
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

    for split in string.split('_') {
        let mut last_upper = false;
        let mut buffer = String::new();

        if split.is_empty() {
            continue;
        }

        for ch in split.chars() {
            if !buffer.is_empty() && buffer != "'" && ch.is_uppercase() && !last_upper {
                words.push(buffer);
                buffer = String::new();
            }

            last_upper = ch.is_uppercase();
            buffer.extend(change_case(ch));
        }

        words.push(buffer);
    }

    words.join("_")
}

// Taken from rustc.
#[inline]
#[must_use]
pub fn to_camel_case(ident: &str) -> String {
    ident
        .trim_matches('_')
        .split('_')
        .filter(|component| !component.is_empty())
        .map(|component| {
            let mut camel_cased_component = String::with_capacity(component.len());

            let mut new_word = true;
            let mut previous_is_lower_case = true;

            for character in component.chars() {
                // Preserve the case if an uppercase letter follows a lowercase letter, so that
                // `camelCase` is converted to `CamelCase`.
                if previous_is_lower_case && character.is_uppercase() {
                    new_word = true;
                }

                if new_word {
                    camel_cased_component.extend(character.to_uppercase());
                } else {
                    camel_cased_component.extend(character.to_lowercase());
                }

                previous_is_lower_case = character.is_lowercase();
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
#[inline]
#[must_use]
pub const fn char_has_case(character: char) -> bool {
    character.is_lowercase() || character.is_uppercase()
}

#[inline]
#[must_use]
pub fn is_upper_snake_case(string: &str) -> bool {
    string
        .chars()
        .all(|character| character.is_uppercase() || character == '_' || character.is_numeric())
}

#[inline]
pub fn replace(
    buffer: &mut String,
    from: char,
    to: &str,
) {
    if !buffer.contains(from) {
        return;
    }
    // FIXME: do this in place.
    *buffer = buffer.replace(from, to);
}

#[inline]
#[must_use]
pub fn trim_indent(mut text: &str) -> String {
    if text.starts_with('\n') {
        text = &text[1..];
    }
    let indent = text
        .lines()
        .filter(|it| !it.trim().is_empty())
        .map(|it| it.len() - it.trim_start().len())
        .min()
        .unwrap_or(0);
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

#[inline]
pub fn equal_range_by<T, F>(
    slice: &[T],
    mut key: F,
) -> ops::Range<usize>
where
    F: FnMut(&T) -> Ordering,
{
    let start = slice.partition_point(|it| key(it) == Ordering::Less);
    let length = slice[start..].partition_point(|it| key(it) == Ordering::Equal);
    start..start + length
}

#[must_use]
#[inline]
pub fn defer<Function: FnOnce()>(function: Function) -> impl Drop {
    struct Droppable<Function: FnOnce()>(Option<Function>);

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
    #[inline]
    fn deref(&self) -> &std::process::Child {
        &self.0
    }
}

impl ops::DerefMut for JodChild {
    #[inline]
    fn deref_mut(&mut self) -> &mut std::process::Child {
        &mut self.0
    }
}

impl Drop for JodChild {
    #[inline]
    fn drop(&mut self) {
        self.0.kill();
        self.0.wait();
    }
}

impl JodChild {
    #[inline]
    pub fn spawn(mut command: Command) -> sio::Result<Self> {
        command.spawn().map(Self)
    }

    #[must_use]
    #[inline]
    #[cfg(target_arch = "wasm32")]
    pub fn into_inner(self) -> std::process::Child {
        // SAFETY: repr transparent, except on WASM
        unsafe { std::mem::transmute::<Self, std::process::Child>(self) }
    }
}

// feature: iter_order_by
// Iterator::eq_by
#[inline]
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
        let x = match this.next() {
            None => return other.next().is_none(),
            Some(value) => value,
        };

        let y = match other.next() {
            None => return false,
            Some(value) => value,
        };

        if !eq(x, y) {
            return false;
        }
    }
}

/// Returns all final segments of the argument, longest first.
#[inline]
pub fn slice_tails<T>(this: &[T]) -> impl Iterator<Item = &[T]> {
    (0..this.len()).map(|i| &this[i..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_indent() {
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
}
