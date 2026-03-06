use base_db::{FileId, SourceDatabase as _, TextRange};
use hir_def::database::DefDatabase as _;
use rowan::NodeOrToken;
use syntax::{AstNode as _, SyntaxNode, ast};
use wgsl_formatter::FormattingOptions;

use crate::RootDatabase;

pub(crate) fn format(
    database: &RootDatabase,
    config: &FormattingOptions,
    file_id: FileId,
    range: Option<TextRange>,
) -> Option<String> {
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

    // TODO: Re-enable the formatter
    // wgsl_formatter::format_recursive(&node, &FormattingOptions::default());
    match wgsl_formatter::format_node(&node, config) {
        Ok(formatted) => Some(formatted),
        Err(error) => {
            // TODO: Properly display this error
            tracing::warn!("Failed to format: {error:?}");
            None
        },
    }
}
