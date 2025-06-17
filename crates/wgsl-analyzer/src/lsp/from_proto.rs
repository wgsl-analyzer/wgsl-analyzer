use anyhow::format_err;
use base_db::{FilePosition, FileRange, TextRange, TextSize};
use line_index::{LineCol, WideLineCol};
use paths::Utf8PathBuf;
use vfs::{AbsPathBuf, FileId};

use crate::{
    Result,
    global_state::GlobalStateSnapshot,
    line_index::{LineIndex, PositionEncoding},
    try_default,
};

pub(crate) fn absolute_path(url: &lsp_types::Url) -> anyhow::Result<AbsPathBuf> {
    let path = url
        .to_file_path()
        .map_err(|()| anyhow::format_err!("url is not a file"))?;
    Ok(AbsPathBuf::try_from(Utf8PathBuf::from_path_buf(path).unwrap()).unwrap())
}

pub(crate) fn vfs_path(url: &lsp_types::Url) -> Result<vfs::VfsPath> {
    absolute_path(url).map(vfs::VfsPath::from)
}

pub(crate) fn offset(
    line_index: &LineIndex,
    position: lsp_types::Position,
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
    range: lsp_types::Range,
) -> Result<TextRange> {
    let start = offset(line_index, range.start)?;
    let end = offset(line_index, range.end)?;
    let text_range = TextRange::new(start, end);
    Ok(text_range)
}

/// Returns `None` if the file was excluded.
pub(crate) fn file_id(
    snap: &GlobalStateSnapshot,
    url: &lsp_types::Url,
) -> anyhow::Result<Option<FileId>> {
    snap.url_to_file_id(url)
}

/// Returns `None` if the file was excluded.
pub(crate) fn file_position(
    snap: &GlobalStateSnapshot,
    tdpp: &lsp_types::TextDocumentPositionParams,
) -> anyhow::Result<Option<FilePosition>> {
    let file_id = try_default!(file_id(snap, &tdpp.text_document.uri)?);
    let line_index = snap.file_line_index(file_id)?;
    let offset = offset(&line_index, tdpp.position)?;
    Ok(Some(FilePosition { file_id, offset }))
}

/// Returns `None` if the file was excluded.
pub(crate) fn file_range(
    snap: &GlobalStateSnapshot,
    text_document_identifier: &lsp_types::TextDocumentIdentifier,
    range: lsp_types::Range,
) -> anyhow::Result<Option<FileRange>> {
    file_range_uri(snap, &text_document_identifier.uri, range)
}

/// Returns `None` if the file was excluded.
pub(crate) fn file_range_uri(
    snap: &GlobalStateSnapshot,
    document: &lsp_types::Url,
    range: lsp_types::Range,
) -> anyhow::Result<Option<FileRange>> {
    let file_id = try_default!(file_id(snap, document)?);
    let line_index = snap.file_line_index(file_id)?;
    let range = text_range(&line_index, range)?;
    Ok(Some(FileRange { file_id, range }))
}
