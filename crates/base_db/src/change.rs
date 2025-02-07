use std::sync::Arc;

use vfs::FileId;

use crate::SourceDatabase;

#[derive(Default)]
pub struct Change {
    pub files_changed: Vec<(FileId, Option<Arc<String>>)>,
}

impl Change {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn change_file(
        &mut self,
        file_id: FileId,
        new_text: Option<Arc<String>>,
    ) {
        self.files_changed.push((file_id, new_text));
    }

    pub fn apply(
        self,
        db: &mut dyn SourceDatabase,
    ) {
        for (file_id, text) in self.files_changed {
            db.set_file_text(file_id, text.unwrap_or_default());
        }
    }
}
