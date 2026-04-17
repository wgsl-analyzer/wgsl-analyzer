use base_db::{EditionedFileId, FilePosition};
use hir::HirDatabase;

pub(crate) fn debug_command(
    database: &dyn HirDatabase,
    file_position: FilePosition,
) -> Option<()> {
    let file_id = EditionedFileId::from_file(database, file_position.file_id);
    let _file = file_id.parse(database).tree();

    None
}
