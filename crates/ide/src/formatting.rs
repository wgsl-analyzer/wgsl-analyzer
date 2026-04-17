use base_db::{EditionedFileId, FileId, SourceDatabase as _, TextRange};
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
    let file_id = EditionedFileId::from_file(database, file_id);
    let file = file_id.parse(database).tree();

    let node = match range {
        None => file.syntax().clone_for_update(),
        Some(range) => match file.syntax().covering_element(range) {
            NodeOrToken::Node(node) => node.clone_for_update(),
            NodeOrToken::Token(_) => return None,
        },
    };

    wgsl_formatter::format_recursive(&node, &FormattingOptions::default());
    Some(node)
}
