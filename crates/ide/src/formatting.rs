use base_db::{FileId, SourceDatabase as _, TextRange};
use hir_def::database::DefDatabase as _;
use rowan::NodeOrToken;
use syntax::{AstNode as _, SyntaxNode, ast};
use wgsl_formatter::FormattingOptions;

use crate::RootDatabase;

pub(crate) fn format(
    database: &RootDatabase,
    file_id: FileId,
    range: Option<TextRange>,
) -> Option<SyntaxNode> {
    let file_id = database.editioned_file_id(file_id);
    let file = database.parse(file_id).tree();

    let node = match range {
        None => file.syntax().clone_for_update(),
        Some(range) => match file.syntax().covering_element(range) {
            NodeOrToken::Node(node) => node.clone_for_update(),
            NodeOrToken::Token(_) => return None,
        },
    };

    // TODO: Re-enable the formatter
    // wgsl_formatter::format_recursive(&node, &FormattingOptions::default());
    Some(node)
}
