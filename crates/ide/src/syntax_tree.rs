use base_db::SourceDatabase;
use vfs::FileId;

pub fn syntax_tree(db: &dyn SourceDatabase, file_id: FileId) -> String {
    let syntax_node = db.parse(file_id).syntax();

    format!("{:#?}", syntax_node)
}
