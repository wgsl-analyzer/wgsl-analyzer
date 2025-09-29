use base_db::{FileLoader, FileLoaderDelegate, SourceDatabase, TextRange, change::Change};
use hir_def::database::DefDatabase;
use rustc_hash::FxHashMap;
use std::{fmt, panic, sync::Mutex};
use triomphe::Arc;
use vfs::{AnchoredPath, FileId, VfsPath};

#[salsa::database(
    base_db::SourceDatabaseStorage,
    hir_def::database::DefDatabaseStorage,
    hir_def::database::InternDatabaseStorage,
    crate::database::HirDatabaseStorage
)]
pub(crate) struct TestDB {
    storage: salsa::Storage<Self>,
}

impl Default for TestDB {
    fn default() -> Self {
        let database = Self {
            storage: salsa::Storage::default(),
        };
        database
    }
}
impl fmt::Debug for TestDB {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct("TestDB").finish()
    }
}

impl salsa::Database for TestDB {}

impl salsa::ParallelDatabase for TestDB {
    fn snapshot(&self) -> salsa::Snapshot<TestDB> {
        salsa::Snapshot::new(TestDB {
            storage: self.storage.snapshot(),
        })
    }
}

impl panic::RefUnwindSafe for TestDB {}

impl FileLoader for TestDB {
    fn resolve_path(
        &self,
        path: AnchoredPath<'_>,
    ) -> Option<base_db::FileId> {
        FileLoaderDelegate(self).resolve_path(path)
    }
}

impl TestDB {
    pub fn apply_change(
        &mut self,
        change: Change,
    ) {
        change.apply(self);
    }
}

pub(crate) fn single_file_db(source: &str) -> (TestDB, FileId) {
    let mut database = TestDB::default();
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
