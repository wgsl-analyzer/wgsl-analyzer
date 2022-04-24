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
    // let file_id = file_id(snap, &text_document_identifier.uri)?;
    // let line_index = snap.file_line_index(file_id)?;
    // let range = text_range(&line_index, range)?;
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
        let token_or_node = syntax_node.child_or_token_at_range(TextRange::new(start, end))?;
        match token_or_node {
            rowan::NodeOrToken::Node(node) => Some(format!("{:#?}", node)),
            rowan::NodeOrToken::Token(token) => Some(format!("{:#?}", token.parent())),
        }
    } else {
        Some(format!("{:#?}", syntax_node))
    }
}
// pub fn offset(line_index: &LineIndex, position: lsp_types::Position) -> Result<TextSize> {
//     let line_col = match line_index.encoding {
//         OffsetEncoding::Utf8 => LineCol {
//             line: position.line as u32,
//             col: position.character as u32,
//         },
//         OffsetEncoding::Utf16 => {
//             let line_col = LineColUtf16 {
//                 line: position.line as u32,
//                 col: position.character as u32,
//             };
//             line_index.index.to_utf8(line_col)
//         }
//     };
//     Ok(text_size)
// }
