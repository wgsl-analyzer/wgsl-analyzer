use base_db::{SourceDatabase, TextRange};
use vfs::FileId;

pub fn syntax_tree(
    db: &dyn SourceDatabase,
    file_id: FileId,
    range: Option<TextRange>,
) -> Option<String> {
    let syntax_node = db.parse(file_id).syntax();
    if let Some(range) = range {
        let token_or_node = syntax_node.covering_element(range);
        match token_or_node {
            rowan::NodeOrToken::Node(node) => Some(format!("{:#?}", node)),
            rowan::NodeOrToken::Token(token) => Some(format!("{:#?}", token.parent()?)),
        }
    } else {
        Some(format!("{:#?}", syntax_node))
    }
}
