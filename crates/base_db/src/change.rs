//! Defines a unit of change that can be applied to the database to get the next
//! state. Changes are transactional.

use salsa::Durability;
use triomphe::Arc;
use vfs::{FileId, VfsPath};

use crate::{
    RootQueryDb,
    input::{SourceRoot, SourceRootId},
};

#[derive(Default)]
pub struct Change {
    pub roots: Option<Vec<SourceRoot>>,
    pub files_changed: Vec<(FileId, Option<String>)>,
}

impl Change {
    #[must_use]
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
        new_text: Option<String>,
    ) {
        self.files_changed.push((file_id, new_text));
    }

    /// Applies a change to the database.
    ///
    /// # Panics
    ///
    /// Panics if the number of source roots exceeds `u32::MAX`, as `SourceRootId` holds a `u32`.
    pub fn apply(
        self,
        database: &mut dyn RootQueryDb,
    ) {
        if let Some(roots) = self.roots {
            for (root_id, root) in roots.into_iter().enumerate() {
                let root_id = SourceRootId(u32::try_from(root_id).unwrap());
                let durability = source_root_durability(&root);
                for file_id in root.iter() {
                    database.set_file_source_root_with_durability(file_id, root_id, durability);
                }
                database.set_source_root_with_durability(root_id, Arc::new(root), durability);
            }
        }

        for (file_id, text) in self.files_changed {
            let source_root_id = database.file_source_root(file_id);
            let source_root = database.source_root(source_root_id.source_root_id(database));

            let durability = file_text_durability(&source_root.source_root(database));
            // XXX: can't actually remove the file, just reset the text
            let text = text.unwrap_or_default();
            database.set_file_text_with_durability(file_id, &text, durability);
        }
    }
}

#[must_use]
const fn source_root_durability(source_root: &SourceRoot) -> Durability {
    if source_root.is_library() {
        Durability::MEDIUM
    } else {
        Durability::LOW
    }
}

#[must_use]
const fn file_text_durability(source_root: &SourceRoot) -> Durability {
    if source_root.is_library() {
        Durability::HIGH
    } else {
        Durability::LOW
    }
}
