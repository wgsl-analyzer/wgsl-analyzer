use std::{fmt, panic};

use base_db::{FileLoader, FileLoaderDelegate, change::Change};
use triomphe::Arc;
use vfs::{AnchoredPath, FileId, VfsPath};

#[salsa::database(
    base_db::SourceDatabaseStorage,
    hir_def::database::DefDatabaseStorage,
    hir_def::database::InternDatabaseStorage,
    crate::database::HirDatabaseStorage
)]
#[derive(Default)]
pub(crate) struct TestDatabase {
    storage: salsa::Storage<Self>,
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

impl TestDatabase {
    pub fn apply_change(
        &mut self,
        change: Change,
    ) {
        change.apply(self);
    }
}

pub(crate) fn single_file_db(source: &str) -> (TestDatabase, FileId) {
    let mut database = TestDatabase::default();
    let mut change = Change::new();
    let file_id = FileId::from_raw(0);
    change.change_file(
        file_id,
        Some(Arc::new(source.to_owned())),
        VfsPath::new_virtual_path("/".into()),
    );
    database.apply_change(change);

    (database, file_id)
}
