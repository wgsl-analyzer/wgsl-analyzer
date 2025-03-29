use base_db::FilePosition;
use hir::HirDatabase;
use syntax::{AstNode, ast};

pub(crate) fn debug_command(
    db: &dyn HirDatabase,
    file_position: FilePosition,
) -> Option<()> {
    let file_id = file_position.file_id;
    let file = db.parse(file_id).tree();

    let import = file
        .syntax()
        .token_at_offset(file_position.offset)
        .left_biased()?
        .parent()?
        .ancestors()
        .find_map(ast::Import::cast)?;

    dbg!(&import);

    None
}
