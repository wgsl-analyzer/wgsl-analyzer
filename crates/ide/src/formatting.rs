use base_db::{FileId, SourceDatabase as _, TextRange};
use rowan::NodeOrToken;
use syntax::{AstNode as _, SyntaxNode, ast};
use wgsl_formatter::FormattingOptions;

use crate::RootDatabase;

pub(crate) fn format(
    database: &RootDatabase,
    file_id: FileId,
    range: Option<TextRange>,
) -> Option<SyntaxNode> {
    let file = database.parse_no_preprocessor(file_id).tree();

    let node = match range {
        None => file.syntax().clone_for_update(),
        Some(range) => match file.syntax().covering_element(range) {
            NodeOrToken::Node(node) => node.clone_for_update(),
            NodeOrToken::Token(_) => return None,
        },
    };

    wgsl_formatter::format_recursive(node.clone(), &FormattingOptions::default());
    Some(node)
}
