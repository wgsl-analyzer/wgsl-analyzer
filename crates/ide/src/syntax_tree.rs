use base_db::{
    line_index::{LineCol, LineColUtf16, LineIndex},
    SourceDatabase, TextRange,
};
use lsp_types::Range;
use vfs::FileId;

pub fn syntax_tree(
    db: &dyn SourceDatabase,
    file_id: FileId,
    range: Option<Range>,
) -> Option<String> {
    let syntax_node = db.parse(file_id).syntax();
    if let Some(range) = range {
        let Range { start, end } = range;
        let line_index = db.line_index(file_id);
        let start = line_index.offset(LineCol {
            line: start.line,
            col: start.character,
        })?;
        let end = line_index.offset(LineCol {
            line: end.line,
            col: end.character,
        })?;
        let token_or_node = syntax_node.covering_element(TextRange::new(start, end));
        match token_or_node {
            rowan::NodeOrToken::Node(node) => Some(format!("{:#?}", node)),
            rowan::NodeOrToken::Token(token) => Some(format!("{:#?}", token.parent()?)),
        }
    } else {
        Some(format!("{:#?}", syntax_node))
    }
}