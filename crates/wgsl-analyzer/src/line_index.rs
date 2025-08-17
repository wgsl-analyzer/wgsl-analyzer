//! Enhances `ide::LineIndex` with additional info required to convert offsets
//! into lsp positions.
//!
//! We maintain invariant that all internal strings use `\n` as line separator.
//! This module does line ending conversion and detection (so that we can
//! convert back to `\r\n` on the way out).

use line_index::WideEncoding;
use memchr::memmem;
use triomphe::Arc;

#[derive(Clone, Copy)]
pub enum PositionEncoding {
    Utf8,
    Wide(WideEncoding),
}

pub(crate) struct LineIndex {
    pub(crate) index: Arc<ide::LineIndex>,
    pub(crate) endings: LineEndings,
    pub(crate) encoding: PositionEncoding,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum LineEndings {
    Unix,
    Dos,
}

impl LineEndings {
    /// Replaces `\r\n` with `\n` in-place in `source`.
    pub(crate) fn normalize(source: String) -> (String, Self) {
        // We replace `\r\n` with `\n` in-place, which does not break UTF-8 encoding.
        // While we *can* call `as_mut_vec` and do surgery on the live string
        // directly, let us rather steal the contents of `source`.
        // This makes the code safe even if a panic occurs.

        let mut buffer = source.into_bytes();
        let mut gap_length = 0;
        let mut tail = buffer.as_mut_slice();
        let mut crlf_seen = false;

        let finder = memmem::Finder::new(b"\r\n");

        loop {
            let index = match finder.find(&tail[gap_length..]) {
                None if crlf_seen => tail.len(),
                None => {
                    // SAFETY: buf is unchanged and therefore still contains utf8 data
                    return (unsafe { String::from_utf8_unchecked(buffer) }, Self::Unix);
                },
                Some(index) => {
                    crlf_seen = true;
                    index + gap_length
                },
            };
            tail.copy_within(gap_length..index, 0);
            tail = &mut tail[index - gap_length..];
            if tail.len() == gap_length {
                break;
            }
            gap_length += 1;
        }

        let new_len = buffer.len() - gap_length;
        // SAFETY: Shrinking the buffer to account for them removed `\r` is safe.
        unsafe {
            buffer.set_len(new_len);
        }
        // SAFETY: After `set_len`, `buf` is guaranteed to contain UTF-8 again.
        let source = unsafe { String::from_utf8_unchecked(buffer) };
        (source, Self::Dos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unix() {
        let source = "a\nb\nc\n\n\n\n";
        let (result, endings) = LineEndings::normalize(source.into());
        assert_eq!(endings, LineEndings::Unix);
        assert_eq!(result, source);
    }

    #[test]
    fn dos() {
        let source = "\r\na\r\n\r\nb\r\nc\r\n\r\n\r\n\r\n";
        let (result, endings) = LineEndings::normalize(source.into());
        assert_eq!(endings, LineEndings::Dos);
        assert_eq!(result, "\na\n\nb\nc\n\n\n\n");
    }

    #[test]
    fn mixed() {
        let source = "a\r\nb\r\nc\r\n\n\r\n\n";
        let (result, endings) = LineEndings::normalize(source.into());
        assert_eq!(endings, LineEndings::Dos);
        assert_eq!(result, "a\nb\nc\n\n\n\n");
    }

    #[test]
    fn none() {
        let source = "abc";
        let (result, endings) = LineEndings::normalize(source.into());
        assert_eq!(endings, LineEndings::Unix);
        assert_eq!(result, source);
    }
}
