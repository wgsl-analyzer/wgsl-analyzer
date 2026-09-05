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

    let estimated_capacity = url.as_str().len();
    let mut path = String::with_capacity(estimated_capacity);
    for segment in segments {
        path.push('/');
        let decoded = percent_decode(segment.as_bytes()).decode_utf8()?;
        path.push_str(&decoded);
    }
    Ok(VirtualPath::new(path))
}

pub(crate) fn vfs_path(url: &Uri) -> Result<vfs::VfsPath> {
    match url.scheme() {
        "file" => Ok(vfs::VfsPath::from(url_to_absolute_path(url)?)),
        VirtualPath::SCHEME => Ok(vfs::VfsPath::from(url_to_virtual_path(url)?)),
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

/// Returns `None` if the file was excluded.
pub(crate) fn file_id(
    snap: &GlobalStateSnapshot,
    url: &Uri,
) -> anyhow::Result<Option<FileId>> {
    snap.url_to_file_id(url)
}

/// Returns `None` if the file was excluded.
pub(crate) fn file_position(
    snap: &GlobalStateSnapshot,
    tdpp: &TextDocumentPositionParams,
) -> anyhow::Result<Option<FilePosition>> {
    let file_id = try_default!(file_id(snap, &tdpp.text_document.uri)?);
    let line_index = snap.file_line_index(file_id)?;
    let offset = offset(&line_index, tdpp.position)?;
    Ok(Some(FilePosition { file_id, offset }))
}

/// Returns `None` if the file was excluded.
pub(crate) fn file_range(
    snap: &GlobalStateSnapshot,
    text_document_identifier: &TextDocumentIdentifier,
    range: Range,
) -> anyhow::Result<Option<FileRange>> {
    file_range_uri(snap, &text_document_identifier.uri, range)
}

/// Returns `None` if the file was excluded.
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
