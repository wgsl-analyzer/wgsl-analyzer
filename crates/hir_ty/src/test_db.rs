use std::{fmt, panic};

use base_db::{EditionedFileId, FileLoader, FileLoaderDelegate, change::Change, input::SourceRoot};
use hir_def::database::{DefDatabase, ExtensionsConfig};
use salsa::Durability;
use syntax::Edition;
use triomphe::Arc;
use vfs::{AnchoredPath, FileId, VfsPath, file_set::FileSet};

#[salsa::database(
    base_db::SourceDatabaseStorage,
    hir_def::database::DefDatabaseStorage,
    hir_def::database::InternDatabaseStorage,
    crate::database::HirDatabaseStorage
)]
pub(crate) struct TestDatabase {
    storage: salsa::Storage<Self>,
}

impl Default for TestDatabase {
    fn default() -> Self {
        let mut value = Self {
            storage: Default::default(),
        };
        value.set_extensions_with_durability(
            ExtensionsConfig {
                shader_int64: false,
            },
            Durability::MEDIUM,
        );
        value
    }
}

impl fmt::Debug for TestDatabase {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct("TestDB").finish()
    }
}

impl salsa::Database for TestDatabase {}

impl salsa::ParallelDatabase for TestDatabase {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(Self {
            storage: self.storage.snapshot(),
        })
    }
}

impl panic::RefUnwindSafe for TestDatabase {}

impl FileLoader for TestDatabase {
    fn resolve_path(
        &self,
        path: AnchoredPath<'_>,
    ) -> Option<base_db::FileId> {
        FileLoaderDelegate(self).resolve_path(path)
    }
}
