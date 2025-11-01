use base_db::FilePosition;
use hir::HirDatabase;
use syntax::{AstNode as _, ast};

pub(crate) fn debug_command(
    database: &dyn HirDatabase,
    file_position: FilePosition,
) -> Option<()> {
    let file_id = file_position.file_id;
    let _file = database.parse(file_id).tree();

    None
}
