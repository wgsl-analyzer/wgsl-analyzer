use std::sync::Arc;

use salsa::Durability;
use vfs::{FileId, VfsPath};

use crate::{
    SourceDatabase,
    input::{SourceRoot, SourceRootId},
};

#[derive(Default)]
pub struct Change {
    pub roots: Option<Vec<SourceRoot>>,
    pub files_changed: Vec<(FileId, Option<Arc<String>>, VfsPath)>,
}

impl Change {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_roots(
        &mut self,
        roots: Vec<SourceRoot>,
    ) {
        self.roots = Some(roots);
    }

    pub fn change_file(
        &mut self,
        file_id: FileId,
        new_text: Option<Arc<String>>,
        new_path: VfsPath,
    ) {
        self.files_changed.push((file_id, new_text, new_path));
    }

    pub fn apply(
        self,
        db: &mut dyn SourceDatabase,
    ) {
        if let Some(roots) = self.roots {
            for (root_id, root) in roots.into_iter().enumerate() {
                let root_id = SourceRootId(root_id as u32);
                for file_id in root.iter() {
                    db.set_file_source_root_with_durability(file_id, root_id, Durability::LOW);
                }
                db.set_source_root_with_durability(root_id, Arc::new(root), Durability::LOW);
            }
        }

        for (file_id, text, path) in self.files_changed {
            db.set_file_text(file_id, text.unwrap_or_default());
            db.set_file_path(file_id, path.clone());
            db.set_file_id(path, file_id);
        }
    }
}
