use base_db::{EditionedFileId, FileId, RootQueryDb as _, SourceDatabase as _, TextRange};
use hir_def::database::DefDatabase as _;
use rowan::NodeOrToken;
use syntax::{AstNode as _, SyntaxNode, ast};
use wgsl_formatter::{FormattedRange, FormattingOptions};

use crate::RootDatabase;

/// Formats at least the given range of the file.
///
/// Note, that range that will actually be formatted is dependent on the syntax tree and may be larger than the given range.
/// The returned [`FormattedRange`] contains the actual range that the formatted text should replace.
pub(crate) fn format(
    database: &RootDatabase,
    config: &FormattingOptions,
    file_id: FileId,
    range: Option<TextRange>,
) -> Option<FormattedRange> {
    let file_id = EditionedFileId::from_file(database, file_id);
    let parsed = database.parse(file_id);

    // Refuse to format documents with syntax errors
    if !parsed.errors().is_empty() {
        tracing::warn!("Skipped formatting, file has syntax errors");
        return None;
    }

    match wgsl_formatter::format_range(&parsed.syntax(), range, config) {
        Ok(formatted) => Some(formatted),
        Err(error) => {
            // TODO: Properly display this error
            tracing::warn!("Failed to format: {error:?}");
            None
        },
    }
}
