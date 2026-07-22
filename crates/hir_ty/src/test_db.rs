use std::{fmt, panic, sync::Mutex};

use base_db::{
    EditionedFileId, FileSourceRootInput, FileText, Nonce, SourceDatabase, SourceRootId,
    SourceRootInput, change::Change, input::SourceRoot, set_all_packages_with_durability,
};
use hir_def::database::DefDatabase as _;
use salsa::{Database as _, Durability};
use syntax::{Edition, ExtensionsConfig};
use triomphe::Arc;
use vfs::{AnchoredPath, FileId, VfsPath, file_set::FileSet};

#[salsa_macros::db]
pub(crate) struct TestDatabase {
    storage: salsa::Storage<Self>,
    files: Arc<base_db::Files>,
    events: Arc<Mutex<Option<Vec<salsa::Event>>>>,
    nonce: Nonce,
}
impl Default for TestDatabase {
    fn default() -> Self {
        let events = Arc::<Mutex<Option<Vec<salsa::Event>>>>::default();
        let mut value = Self {
            storage: salsa::Storage::new(Some(Box::new({
                let events = events.clone();
                move |event| {
                    let mut events = events.lock().unwrap();
                    if let Some(events) = &mut *events {
                        events.push(event);
                    }
                }
            }))),
            files: Arc::default(),
            events,
            nonce: Nonce::new(),
        };
        value.set_extensions_with_durability(ExtensionsConfig::none(), Durability::MEDIUM);
        // This needs to be here otherwise the first `Change` will panic.
        set_all_packages_with_durability(&mut value, [], Durability::LOW);
        value
    }
}

impl Clone for TestDatabase {
    fn clone(&self) -> Self {
        Self {
            storage: self.storage.clone(),
            files: self.files.clone(),
            events: self.events.clone(),
            nonce: Nonce::new(),
        }
    }
}

impl fmt::Debug for TestDatabase {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct("TestDatabase").finish()
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

impl TestDatabase {
    pub(crate) fn log<Callback>(
        &self,
        callback: Callback,
    ) -> Vec<salsa::Event>
    where
        Callback: FnOnce(),
    {
        *self.events.lock().unwrap() = Some(Vec::new());
        callback();
        self.events.lock().unwrap().take().unwrap()
    }

    pub(crate) fn log_executed<Callback>(
        &self,
        callback: Callback,
    ) -> (Vec<String>, Vec<salsa::Event>)
    where
        Callback: FnOnce(),
    {
        let events = self.log(callback);
        let executed = events
            .iter()
            .filter_map(|event| match event.kind {
                // This is pretty horrible, but `Debug` is the only way to inspect
                // QueryDescriptor at the moment.
                salsa::EventKind::WillExecute { database_key } => {
                    let ingredient = self.ingredient_debug_name(database_key.ingredient_index());
                    Some(ingredient.to_string())
                },
                salsa::EventKind::DidValidateMemoizedValue { .. }
                | salsa::EventKind::WillBlockOn { .. }
                | salsa::EventKind::WillIterateCycle { .. }
                | salsa::EventKind::WillCheckCancellation
                | salsa::EventKind::DidSetCancellationFlag
                | salsa::EventKind::WillDiscardStaleOutput { .. }
                | salsa::EventKind::DidDiscard { .. }
                | salsa::EventKind::DidDiscardAccumulated { .. }
                | salsa::EventKind::DidInternValue { .. }
                | salsa::EventKind::DidReuseInternedValue { .. }
                | salsa::EventKind::DidValidateInternedValue { .. } => None,
            })
            .collect();
        (executed, events)
    }
}
