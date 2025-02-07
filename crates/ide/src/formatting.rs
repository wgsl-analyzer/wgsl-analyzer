use base_db::FileId;
use base_db::SourceDatabase;
use base_db::TextRange;
use rowan::NodeOrToken;
use syntax::{
	AstNode,
	SyntaxNode,
	ast,
};
use wgsl_formatter::FormattingOptions;

use crate::RootDatabase;

pub fn format(
	db: &RootDatabase,
	file_id: FileId,
	range: Option<TextRange>,
) -> Option<SyntaxNode> {
	let file: ast::SourceFile = db.parse_no_preprocessor(file_id).tree();

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
