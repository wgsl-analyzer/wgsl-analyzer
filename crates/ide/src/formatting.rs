use base_db::{FileId, SourceDatabase as _, TextRange};
use hir_def::database::DefDatabase as _;
use rowan::NodeOrToken;
use syntax::{AstNode as _, SyntaxNode, ast};
use wgsl_formatter::FormattingOptions;

use crate::RootDatabase;

#[derive(Clone, Debug)]
pub struct FormattedRange {
    /// The actual range that the formatted text should replace.
    pub range: TextRange,

    /// The formatted text.
    pub formatted: String,
}

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
    let file_id = database.editioned_file_id(file_id);
    let parsed = database.parse(file_id);
    let file = parsed.tree();

    let node = match range {
        None => file.syntax().clone(),
        Some(range) => match file.syntax().covering_element(range) {
            NodeOrToken::Node(node) => node,
            NodeOrToken::Token(_) => return None,
        },
    };

    // Refuse to format documents with syntax errors
    if !parsed.errors().is_empty() {
        tracing::warn!("Skipped formatting, file has syntax errors");
        return None;
    }

    match wgsl_formatter::format_node(&node, config) {
        Ok(formatted) => Some(FormattedRange {
            range: node.text_range(),
            formatted,
        }),
        Err(error) => {
            // TODO: Properly display this error
            tracing::warn!("Failed to format: {error:?}");
            None
        },
    }
}
