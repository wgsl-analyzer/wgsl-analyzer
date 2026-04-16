use std::{fmt, panic};

use base_db::{
    EditionedFileId, FileSourceRootInput, FileText, Nonce, RootQueryDb as _, SourceDatabase,
    SourceRootId, SourceRootInput, change::Change, input::SourceRoot,
    set_all_packages_with_durability,
};
use hir_def::database::{DefDatabase as _, ExtensionsConfig};
use salsa::Durability;
use syntax::Edition;
use triomphe::Arc;
use vfs::{AnchoredPath, FileId, VfsPath, file_set::FileSet};

#[salsa_macros::db]
#[derive(Clone)]
pub(crate) struct TestDatabase {
    storage: salsa::Storage<Self>,
    files: Arc<base_db::Files>,
    nonce: Nonce,
}
impl Default for TestDatabase {
    fn default() -> Self {
        let mut value = Self {
            storage: salsa::Storage::default(),
            files: Arc::default(),
            nonce: Nonce::new(),
        };
        value.set_extensions_with_durability(
            ExtensionsConfig {
                shader_int64: false,
            },
            Durability::MEDIUM,
        );
        // This needs to be here otherwise the first `Change` will panic.
        set_all_packages_with_durability(&mut value, [], Durability::LOW);
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

impl panic::RefUnwindSafe for TestDatabase {}

#[salsa_macros::db]
impl SourceDatabase for TestDatabase {
    fn file_text(
        &self,
        file_id: base_db::FileId,
    ) -> FileText {
        self.files.file_text(file_id)
    }

    fn set_file_text(
        &mut self,
        file_id: base_db::FileId,
        text: &str,
    ) {
        let files = Arc::clone(&self.files);
        files.set_file_text(self, file_id, text);
    }

    fn set_file_text_with_durability(
        &mut self,
        file_id: base_db::FileId,
        text: &str,
        durability: Durability,
    ) {
        let files = Arc::clone(&self.files);
        files.set_file_text_with_durability(self, file_id, text, durability);
    }

    /// Source root of the file.
    fn source_root(
        &self,
        id: SourceRootId,
    ) -> SourceRootInput {
        self.files.source_root(id)
    }

    fn set_source_root_with_durability(
        &mut self,
        source_root_id: SourceRootId,
        source_root: Arc<SourceRoot>,
        durability: Durability,
    ) {
        let files = Arc::clone(&self.files);
        files.set_source_root_with_durability(self, source_root_id, source_root, durability);
    }

    fn file_source_root(
        &self,
        id: base_db::FileId,
    ) -> FileSourceRootInput {
        self.files.file_source_root(id)
    }

    fn set_file_source_root_with_durability(
        &mut self,
        id: base_db::FileId,
        source_root_id: SourceRootId,
        durability: Durability,
    ) {
        let files = Arc::clone(&self.files);
        files.set_file_source_root_with_durability(self, id, source_root_id, durability);
    }

    fn nonce_and_revision(&self) -> (Nonce, salsa::Revision) {
        (
            self.nonce,
            salsa::plumbing::ZalsaDatabase::zalsa(self).current_revision(),
        )
    }
}
