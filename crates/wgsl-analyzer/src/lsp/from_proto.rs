use anyhow::format_err;
use base_db::{FilePosition, FileRange, TextRange, TextSize};
use line_index::{LineCol, WideLineCol};
use paths::Utf8PathBuf;
use vfs::{AbsPathBuf, FileId};

use crate::{
    Result,
    global_state::GlobalStateSnapshot,
    line_index::{LineIndex, OffsetEncoding, PositionEncoding},
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
    let line_col = match line_index.encoding {
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
    let line_range = line_index.index.line(line_col.line).ok_or_else(|| {
        format_err!(
            "Invalid offset {line_col:?} (line index length: {:?})",
            line_index.index.len()
        )
    })?;
    let col = TextSize::from(line_col.col);
    let clamped_length = col.min(line_range.len());
    if clamped_length < col {
        tracing::error!(
            "Position {line_col:?} column exceeds line length {}, clamping it",
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

pub(crate) fn file_id(
    snap: &GlobalStateSnapshot,
    url: &lsp_types::Url,
) -> Result<FileId> {
    snap.url_to_file_id(url)
}

pub(crate) fn file_position(
    snap: &GlobalStateSnapshot,
    tdpp: &lsp_types::TextDocumentPositionParams,
) -> Result<FilePosition> {
    let file_id = file_id(snap, &tdpp.text_document.uri)?;
    let line_index = snap.file_line_index(file_id)?;
    let offset = offset(&line_index, tdpp.position)?;
    Ok(FilePosition { file_id, offset })
}

pub(crate) fn file_range(
    snap: &GlobalStateSnapshot,
    text_document_identifier: &lsp_types::TextDocumentIdentifier,
    range: lsp_types::Range,
) -> Result<FileRange> {
    let file_id = file_id(snap, &text_document_identifier.uri)?;
    let line_index = snap.file_line_index(file_id)?;
    let range = text_range(&line_index, range)?;
    Ok(FileRange { file_id, range })
}
