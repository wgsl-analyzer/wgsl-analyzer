use base_db::{
    line_index::{LineCol, LineIndex},
    FilePosition, TextRange, TextSize,
};
use std::convert::TryFrom;
use vfs::{AbsPathBuf, FileId};

use crate::{global_state::GlobalStateSnapshot, Result};

pub(crate) fn abs_path(url: &lsp_types::Url) -> Result<AbsPathBuf> {
    let path = url
        .to_file_path()
        .map_err(|()| anyhow::anyhow!("url is not a file: {}", url.as_str()))?;
    Ok(AbsPathBuf::try_from(path).unwrap())
}

pub(crate) fn vfs_path(url: &lsp_types::Url) -> Result<vfs::VfsPath> {
    abs_path(url).map(vfs::VfsPath::from)
}

pub(crate) fn offset(line_index: &LineIndex, position: lsp_types::Position) -> TextSize {
    let line_col = LineCol {
        line: position.line as u32,
        col: position.character as u32,
    };
    line_index.offset(line_col)
}

pub(crate) fn text_range(line_index: &LineIndex, range: lsp_types::Range) -> TextRange {
    let start = offset(line_index, range.start);
    let end = offset(line_index, range.end);
    TextRange::new(start, end)
}

pub(crate) fn file_id(snap: &GlobalStateSnapshot, url: &lsp_types::Url) -> Result<FileId> {
    snap.url_to_file_id(url)
}

pub(crate) fn file_position(
    snap: &GlobalStateSnapshot,
    tdpp: lsp_types::TextDocumentPositionParams,
) -> Result<FilePosition> {
    let file_id = file_id(snap, &tdpp.text_document.uri)?;
    let line_index = snap.analysis.line_index(file_id)?;
    let offset = offset(&line_index, tdpp.position);
    Ok(FilePosition { file_id, offset })
}
