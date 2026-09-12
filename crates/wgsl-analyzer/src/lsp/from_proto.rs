use std::str::FromStr as _;

use anyhow::format_err;
use base_db::{FilePosition, FileRange, TextRange, TextSize};
use line_index::{LineCol, WideLineCol};
use lsp_types::{Position, Range, TextDocumentIdentifier, TextDocumentPositionParams, Uri};
use paths::Utf8PathBuf;
use percent_encoding::percent_decode;
use vfs::{AbsPathBuf, FileId, VirtualPath};

use crate::{
    Result,
    global_state::GlobalStateSnapshot,
    line_index::{LineIndex, PositionEncoding},
    lsp::to_proto,
    try_default,
};

pub(crate) fn url_to_absolute_path(url: &Uri) -> anyhow::Result<AbsPathBuf> {
    let path = url
        .to_file_path()
        .map_err(|()| anyhow::format_err!("url is not a file"))?;
    Ok(AbsPathBuf::try_from(Utf8PathBuf::from_path_buf(path).unwrap()).unwrap())
}

pub(crate) fn url_to_virtual_path(url: &Uri) -> anyhow::Result<VirtualPath> {
    let segments = url
        .path_segments()
        .ok_or_else(|| format_err!("url is not a file"))?;

    if !matches!(url.host_str(), None | Some("localhost")) {
        return Err(format_err!("url for virtual path cannot have a host"));
    }

    // The path segments are empty if the URL only has the root path (e.g., "wgsl://localhost/")
    // We assume non hostile clients and do not perform additional validation on the path segments.
    let estimated_capacity = url.as_str().len();
    let mut path = String::with_capacity(estimated_capacity);
    for segment in segments {
        path.push('/');
        let decoded = percent_decode(segment.as_bytes()).decode_utf8()?;
        path.push_str(&decoded);
    }
    // Special case the root URL
    if path == "/" {
        path.clear();
    }
    Ok(VirtualPath::new(path))
}

pub(crate) fn vfs_path(url: &Uri) -> Result<vfs::VfsPath> {
    match url.scheme() {
        to_proto::PATH_SCHEME => Ok(vfs::VfsPath::from(url_to_absolute_path(url)?)),
        to_proto::VIRTUAL_PATH_SCHEME => Ok(vfs::VfsPath::from(url_to_virtual_path(url)?)),
        _ => Err(format_err!("url has unsupported scheme: {}", url.scheme())),
    }
}

pub(crate) fn offset(
    line_index: &LineIndex,
    position: Position,
) -> anyhow::Result<TextSize> {
    let line_column = match line_index.encoding {
        PositionEncoding::Utf8 => LineCol {
            line: position.line,
            col: position.character,
        },
        PositionEncoding::Wide(enc) => {
            let line_col = WideLineCol {
                line: position.line,
                col: position.character,
            };
            line_index
                .index
                .to_utf8(enc, line_col)
                .ok_or_else(|| format_err!("Invalid wide col offset"))?
        },
    };
    let line_range = line_index.index.line(line_column.line).ok_or_else(|| {
        format_err!(
            "Invalid offset {line_column:?} (line index length: {:?})",
            line_index.index.len()
        )
    })?;
    let column = TextSize::from(line_column.col);
    let clamped_length = column.min(line_range.len());
    if clamped_length < column {
        tracing::error!(
            "Position {line_column:?} column exceeds line length {}, clamping it",
            u32::from(line_range.len()),
        );
    }
    Ok(line_range.start() + clamped_length)
}

pub(crate) fn text_range(
    line_index: &LineIndex,
    range: Range,
) -> Result<TextRange> {
    let start = offset(line_index, range.start)?;
    let end = offset(line_index, range.end)?;
    let text_range = TextRange::new(start, end);
    Ok(text_range)
}

/// Returns [`None`] if the file was excluded.
pub(crate) fn file_id(
    snap: &GlobalStateSnapshot,
    url: &Uri,
) -> anyhow::Result<Option<FileId>> {
    snap.url_to_file_id(url)
}

/// Returns [`None`] if the file was excluded.
pub(crate) fn file_position(
    snap: &GlobalStateSnapshot,
    tdpp: &TextDocumentPositionParams,
) -> anyhow::Result<Option<FilePosition>> {
    let file_id = try_default!(file_id(snap, &tdpp.text_document.uri)?);
    let line_index = snap.file_line_index(file_id)?;
    let offset = offset(&line_index, tdpp.position)?;
    Ok(Some(FilePosition { file_id, offset }))
}

/// Returns [`None`] if the file was excluded.
pub(crate) fn file_range(
    snap: &GlobalStateSnapshot,
    text_document_identifier: &TextDocumentIdentifier,
    range: Range,
) -> anyhow::Result<Option<FileRange>> {
    file_range_uri(snap, &text_document_identifier.uri, range)
}

/// Returns [`None`] if the file was excluded.
pub(crate) fn file_range_uri(
    snap: &GlobalStateSnapshot,
    document: &Uri,
    range: Range,
) -> anyhow::Result<Option<FileRange>> {
    let file_id = try_default!(file_id(snap, document)?);
    let line_index = snap.file_line_index(file_id)?;
    let range = text_range(&line_index, range)?;
    Ok(Some(FileRange { file_id, range }))
}

#[cfg(test)]
mod tests {
    use crate::line_index::LineEndings;

    use super::*;
    use line_index::WideEncoding;
    use lsp_types::{Position, Range};
    use triomphe::Arc;
    use vfs::VfsPath;

    fn line_index_utf8(text: &str) -> LineIndex {
        LineIndex {
            index: Arc::new(ide::LineIndex::new(text)),
            endings: LineEndings::Unix,
            encoding: PositionEncoding::Utf8,
        }
    }

    fn line_index_wide(text: &str) -> LineIndex {
        LineIndex {
            index: Arc::new(ide::LineIndex::new(text)),
            endings: LineEndings::Unix,
            encoding: PositionEncoding::Wide(WideEncoding::Utf16),
        }
    }

    #[test]
    fn url_to_absolute_path_converts_file_url() {
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("url_to_absolute_path_converts_file_url.wgsl");
        let result = Uri::from_file_path(&temp_file);
        let url = result.expect("valid file path");
        let result = url_to_absolute_path(&url);
        let abs_path_buf = result.expect("should convert to AbsPathBuf");
        assert_eq!(abs_path_buf, Utf8PathBuf::from_path_buf(temp_file).unwrap());
    }

    #[test]
    fn url_to_absolute_path_rejects_non_file_url() {
        let parse = Uri::parse("http://example.com");
        let url = parse.unwrap();
        let result = url_to_absolute_path(&url);
        let error = result.unwrap_err().to_string();
        assert_eq!(error, "url is not a file");
    }

    #[test]
    fn url_to_virtual_path_root_path_is_empty() {
        let path = format!("{}://localhost/", to_proto::VIRTUAL_PATH_SCHEME);
        let parse = Uri::parse(&path);
        let url = parse.unwrap();
        let result = url_to_virtual_path(&url);
        let virtual_path = result.unwrap();
        assert_eq!(virtual_path, VirtualPath::empty());
    }

    #[test]
    fn url_to_virtual_path_decodes_percent_encoded_segments() {
        let path = format!("{}://localhost/a/b%20c", to_proto::VIRTUAL_PATH_SCHEME);
        let parse = Uri::parse(&path);
        let url = parse.unwrap();
        let result = url_to_virtual_path(&url);
        let virtual_path = result.unwrap();
        assert_eq!(virtual_path, VirtualPath::new("/a/b c".to_owned()));
    }

    #[test]
    fn url_to_virtual_path_allows_no_host() {
        let path = format!("{}:/a/b", to_proto::VIRTUAL_PATH_SCHEME);
        let parse = Uri::parse(&path);
        let url = parse.unwrap();
        let result = url_to_virtual_path(&url);
        let virtual_path = result.unwrap();
        assert_eq!(virtual_path, VirtualPath::new("/a/b".to_owned()));
    }

    #[test]
    fn url_to_virtual_path_rejects_non_localhost_host() {
        let path = format!("{}://remote/", to_proto::VIRTUAL_PATH_SCHEME);
        let result = Uri::parse(&path);
        let url = result.unwrap();
        let result = url_to_virtual_path(&url);
        let error = result.unwrap_err();
        let message = error.to_string();
        assert_eq!(message, "url for virtual path cannot have a host");
    }

    #[test]
    fn url_to_virtual_path_rejects_cannot_be_a_base_url() {
        let path = format!("{}:opaque", to_proto::VIRTUAL_PATH_SCHEME);
        let parse = Uri::parse(&path);
        let url = parse.unwrap();
        let result = url_to_virtual_path(&url);
        let error = result.unwrap_err();
        let message = error.to_string();
        assert_eq!(message, "url is not a file");
    }

    #[test]
    fn url_to_virtual_path_rejects_invalid_percent_encoding() {
        let path = format!("{}://localhost/foo%FF", to_proto::VIRTUAL_PATH_SCHEME);
        let parse = Uri::parse(&path);
        let url = parse.unwrap();
        let result = url_to_virtual_path(&url);
        let error = result.unwrap_err();
        let message = error.to_string();
        assert_eq!(message, "invalid utf-8 sequence of 1 bytes from index 3");
    }

    #[test]
    fn vfs_path_dispatches_to_absolute_path() {
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("vfs_path_dispatches_to_absolute_path.wgsl");
        let parse = Uri::from_file_path(&temp_file);
        let url = parse.expect("valid file path");
        assert_eq!(url.scheme(), to_proto::PATH_SCHEME);
        let result = vfs_path(&url);
        let vfs_path = result.unwrap();
        assert_eq!(
            vfs_path,
            VfsPath::new_real_path(temp_file.to_string_lossy().to_string())
        );
    }

    #[test]
    fn vfs_path_dispatches_to_virtual_path() {
        let path = format!("{}://localhost/foo/bar", to_proto::VIRTUAL_PATH_SCHEME);
        let parse = Uri::parse(&path);
        let url = parse.unwrap();
        let result = vfs_path(&url);
        let vfs_path = result.unwrap();
        assert_eq!(vfs_path, VfsPath::new_virtual_path("/foo/bar".to_owned()));
    }

    #[test]
    fn vfs_path_rejects_unsupported_scheme() {
        let parse = Uri::parse("ftp://localhost/foo");
        let url = parse.unwrap();
        let result = vfs_path(&url);
        let error = result.unwrap_err();
        let message = error.to_string();
        assert_eq!(message, "url has unsupported scheme: ftp");
    }

    #[test]
    fn offset_utf8_encoding_success() {
        let index = line_index_utf8("one\ntwo\n");
        let result = offset(&index, Position::new(1, 1));
        let offset: u32 = result.unwrap().into();
        assert_eq!(offset, 5);
    }

    #[test]
    fn offset_wide_encoding_success() {
        let index = line_index_wide("one\ntwo\n");
        let result = offset(&index, Position::new(1, 1));
        let offset: u32 = result.unwrap().into();
        assert_eq!(offset, 5);
    }

    #[test]
    fn offset_wide_encoding_invalid_col_errors() {
        let index = line_index_wide("\u{1f600}\n");
        let result = offset(&index, Position::new(0, u32::MAX));
        let error = result.unwrap_err();
        let message = error.to_string();
        assert_eq!(message, "Invalid wide col offset");
    }

    #[test]
    fn offset_invalid_line_errors() {
        let index = line_index_utf8("one\n");
        let result = offset(&index, Position::new(99, 0));
        let error = result.unwrap_err();
        let message = error.to_string();
        assert_eq!(
            message,
            "Invalid offset LineCol { line: 99, col: 0 } (line index length: 4)",
        );
    }

    #[test]
    fn offset_clamps_column_past_end_of_line() {
        let index = line_index_utf8("one\n");
        let result = offset(&index, Position::new(0, 10));
        let offset: u32 = result.unwrap().into();
        assert_eq!(offset, 4);
    }

    #[test]
    fn text_range_success() {
        let index = line_index_utf8("one\ntwo\n");
        let range = Range::new(Position::new(0, 0), Position::new(1, 2));
        let result = text_range(&index, range);
        let text_range = result.unwrap();
        let start: u32 = text_range.start().into();
        let end: u32 = text_range.end().into();
        assert_eq!((start, end), (0, 6));
    }

    #[test]
    fn text_range_propagates_start_error() {
        let index = line_index_utf8("one\n");
        let range = Range::new(Position::new(99, 0), Position::new(0, 0));
        let result = text_range(&index, range);
        let text_range = result.unwrap_err();
        let message = text_range.to_string();
        assert_eq!(
            message,
            "Invalid offset LineCol { line: 99, col: 0 } (line index length: 4)",
        );
    }

    #[test]
    fn text_range_propagates_end_error() {
        let index = line_index_utf8("one\n");
        let range = Range::new(Position::new(0, 0), Position::new(99, 0));
        let result = text_range(&index, range);
        let error = result.unwrap_err();
        let message = error.to_string();
        assert_eq!(
            message,
            "Invalid offset LineCol { line: 99, col: 0 } (line index length: 4)",
        );
    }
}
