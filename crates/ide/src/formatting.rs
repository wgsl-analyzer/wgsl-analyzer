use base_db::{EditionedFileId, FileId, SourceDatabase as _, TextRange};
use rowan::NodeOrToken;
use syntax::{AstNode as _, SyntaxNode, ast};
use wgsl_formatter::{FormattedRange, FormattingOptions};

use crate::RootDatabase;

/// Formats at least the given range of the file.
///
/// Note, that range that will actually be formatted is dependent on the syntax tree and may be larger than the given range.
/// The returned [`FormattedRange`] contains the actual range that the formatted text should replace.
pub(crate) fn format(
    db: &RootDatabase,
    config: &FormattingOptions,
    file_id: FileId,
    range: Option<TextRange>,
) -> Option<FormattedRange> {
    let file_id = EditionedFileId::from_file(db, file_id);
    let parsed = file_id.parse(db);

    // Refuse to format documents with syntax errors
    if !parsed.errors().is_empty() {
        tracing::warn!("Skipped formatting, file has syntax errors");
        return None;
    }

    let result = match range {
        Some(range) => wgsl_formatter::format_range(&parsed.syntax(), range, config),
        None => {
            wgsl_formatter::format_node(&parsed.syntax(), config).map(|formatted| FormattedRange {
                range: parsed.syntax().text_range(),
                formatted,
            })
        },
    };

    match result {
        Ok(formatted) => Some(formatted),
        Err(error) => {
            // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1505
            tracing::warn!("Failed to format: {error:?}");
            None
        },
    }
}
