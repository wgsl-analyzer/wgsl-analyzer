//! Enhances `ide::LineIndex` with additional info required to convert offsets
//! into lsp positions.
//!
//! We maintain invariant that all internal strings use `\n` as line separator.
//! This module does line ending conversion and detection (so that we can
//! convert back to `\r\n` on the way out).

use line_index::WideEncoding;
use std::sync::Arc;

#[derive(Clone, Copy)]
pub(crate) enum PositionEncoding {
    Utf8,
    Wide(WideEncoding),
}

pub(crate) enum OffsetEncoding {
    Utf8,
    Utf16,
}

pub(crate) struct LineIndex {
    pub(crate) index: Arc<line_index::LineIndex>,
    pub(crate) endings: LineEndings,
    pub(crate) encoding: PositionEncoding,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum LineEndings {
    Unix,
    Dos,
}

impl LineEndings {
    /// Replaces `\r\n` with `\n` in-place in `src`.
    pub(crate) fn normalize(source: String) -> (String, Self) {
        if !source.as_bytes().contains(&b'\r') {
            return (source, Self::Unix);
        }

        // We replace `\r\n` with `\n` in-place, which does not break utf-8 encoding.
        // While we *can* call `as_mut_vec` and do surgery on the live string
        // directly, prefer to steal the contents of `src`. This makes the code
        // safe even if a panic occurs.

        let mut buffer = source.into_bytes();
        let mut gap_length = 0;
        let mut tail = buffer.as_mut_slice();
        loop {
            let index =
                find_crlf(&tail[gap_length..]).map_or(tail.len(), |index| index + gap_length);
            tail.copy_within(gap_length..index, 0);
            tail = &mut tail[index - gap_length..];
            if tail.len() == gap_length {
                break;
            }
            gap_length += 1;
        }

        // Account for removed `\r`.
        // After `set_length`, `buf` is guaranteed to contain utf-8 again.
        let new_length = buffer.len() - gap_length;
        let source = unsafe {
            buffer.set_len(new_length);
            String::from_utf8_unchecked(buffer)
        };
        return (source, Self::Dos);

        fn find_crlf(source: &[u8]) -> Option<usize> {
            source.windows(2).position(|it| it == b"\r\n")
        }
    }
}
