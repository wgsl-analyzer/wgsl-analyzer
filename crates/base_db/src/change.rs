//! Defines a unit of change that can applied to the database to get the next
//! state. Changes are transactional.

use salsa::Durability;
use std::fmt;
use triomphe::Arc;
use vfs::{FileId, VfsPath};

use crate::{
    PackageGraph, SourceDatabase,
    input::{SourceRoot, SourceRootId},
};

/// Encapsulate a bunch of raw `.set` calls on the database.
#[derive(Default)]
pub struct Change {
    pub roots: Option<Vec<SourceRoot>>,
    pub files_changed: Vec<(FileId, Option<Arc<String>>, VfsPath)>,
    pub package_graph: Option<PackageGraph>,
}

impl fmt::Debug for Change {
    fn fmt(
        &self,
        fmt: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let mut d = fmt.debug_struct("Change");
        if let Some(roots) = &self.roots {
            d.field("roots", roots);
        }
        if !self.files_changed.is_empty() {
            d.field("files_changed", &self.files_changed.len());
        }
        if self.package_graph.is_some() {
            d.field("package_graph", &self.package_graph);
        }
        d.finish()
    }
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
    pub fn set_package_graph(
        &mut self,
        package_graph: PackageGraph,
    ) {
        self.package_graph = Some(package_graph);
    }

    pub fn change_file(
        &mut self,
        file_id: FileId,
        new_text: Option<Arc<String>>,
        new_path: VfsPath,
    ) {
        self.files_changed.push((file_id, new_text, new_path));
    }

    /// Applies a change to the database.
    ///
    /// # Panics
    ///
    /// Panics if the number of source roots exceeds `u32::MAX`, as `SourceRootId` holds a `u32`.
    pub fn apply(
        self,
        database: &mut dyn SourceDatabase,
    ) {
        if let Some(roots) = self.roots {
            for (root_id, root) in roots.into_iter().enumerate() {
                let root_id = SourceRootId(u32::try_from(root_id).unwrap());
                for file_id in root.iter() {
                    database.set_file_source_root_with_durability(
                        file_id,
                        root_id,
                        Durability::LOW,
                    );
                }
                database.set_source_root_with_durability(root_id, Arc::new(root), Durability::LOW);
            }
        }

        for (file_id, text, path) in self.files_changed {
            database.set_file_text(file_id, text.unwrap_or_default());
            database.set_file_path(file_id, path.clone());
            database.set_file_id(path, file_id);
        }

        if let Some(package_graph) = self.package_graph {
            database.set_package_graph_with_durability(Arc::new(package_graph), Durability::HIGH);
        }
    }
}
