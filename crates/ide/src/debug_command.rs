use base_db::FilePosition;
use hir::HirDatabase;
use syntax::{AstNode as _, ast};

pub(crate) fn debug_command(
    database: &dyn HirDatabase,
    file_position: FilePosition,
) -> Option<()> {
    let file_id = file_position.file_id;
    let file = database.parse(file_id).tree();

    let import = file
        .syntax()
        .token_at_offset(file_position.offset)
        .left_biased()?
        .parent()?
        .ancestors()
        .find_map(ast::Import::cast)?;

    #[expect(clippy::disallowed_names, clippy::dbg_macro, reason = "debugging")]
    {
        dbg!(&import);
    }

    None
}
