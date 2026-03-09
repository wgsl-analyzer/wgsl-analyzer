#![expect(
    clippy::trailing_empty_array,
    reason = "Clippy has a false positive for the query_group macro, see: https://github.com/rust-lang/rust-clippy/issues/16754"
)]

use std::{fmt, panic};

use base_db::{
    FileId, FileSourceRootInput, FileText, Files, Nonce, RootQueryDb, SourceDatabase, SourceRoot,
    SourceRootId, SourceRootInput, change::Change,
};
use hir_def::database::{DefDatabase as _, ExtensionsConfig};
use line_index::LineIndex;
use rustc_hash::FxHashMap;
use salsa::{Database as _, Durability};
use triomphe::Arc;

pub mod source_change;
pub mod text_edit;

#[salsa_macros::db]
pub struct RootDatabase {
    // FIXME: Revisit this commit now that we migrated to the new salsa, given we store arcs in this
    // database directly now
    storage: salsa::Storage<Self>,
    files: Arc<Files>,
    // crates_map: Arc<CratesMap>,
    nonce: Nonce,
}

impl panic::RefUnwindSafe for RootDatabase {}

impl salsa::Database for RootDatabase {}

impl Clone for RootDatabase {
    fn clone(&self) -> Self {
        Self {
            storage: self.storage.clone(),
            files: self.files.clone(),
            nonce: Nonce::new(),
        }
    }
}

impl fmt::Debug for RootDatabase {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.debug_struct("RootDatabase").finish()
    }
}

#[salsa_macros::db]
impl SourceDatabase for RootDatabase {
    fn file_text(
        &self,
        file_id: vfs::FileId,
    ) -> FileText {
        self.files.file_text(file_id)
    }

    fn set_file_text(
        &mut self,
        file_id: vfs::FileId,
        text: &str,
    ) {
        let files = Arc::clone(&self.files);
        files.set_file_text(self, file_id, text);
    }

    fn set_file_text_with_durability(
        &mut self,
        file_id: vfs::FileId,
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
        id: vfs::FileId,
    ) -> FileSourceRootInput {
        self.files.file_source_root(id)
    }

    fn set_file_source_root_with_durability(
        &mut self,
        id: vfs::FileId,
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

impl RootDatabase {
    #[must_use]
    pub fn new(lru_capacity: Option<u16>) -> Self {
        let mut database = Self {
            storage: salsa::Storage::default(),
            files: Arc::default(),
            // crates_map: Default::default(),
            nonce: Nonce::new(),
        };
        // This needs to be here otherwise the first `Change` will panic.
        database.set_all_packages(Arc::new(Box::new([])));
        // CrateGraphBuilder::default().set_in_db(&mut database);
        // database.set_proc_macros_with_durability(Default::default(), Durability::MEDIUM);
        // database.set_local_roots_with_durability(Default::default(), Durability::MEDIUM);
        // database.set_library_roots_with_durability(Default::default(), Durability::MEDIUM);
        database.set_extensions_with_durability(ExtensionsConfig::default(), Durability::MEDIUM);
        database.update_base_query_lru_capacities(lru_capacity);
        database
    }

    #[expect(
        clippy::unused_self,
        clippy::needless_pass_by_ref_mut,
        reason = "TODO impl"
    )]
    pub const fn update_base_query_lru_capacities(
        &mut self,
        _lru_capacity: Option<u16>,
    ) {
        // let lru_capacity = lru_capacity.unwrap_or(base_db::DEFAULT_PARSE_LRU_CAP);
        // base_db::FileTextQuery.in_db_mut(self).set_lru_capacity(DEFAULT_FILE_TEXT_LRU_CAP);
        // base_db::ParseQuery.in_db_mut(self).set_lru_capacity(lru_capacity);
        // // macro expansions are usually rather small, so we can afford to keep more of them alive
        // hir::database::ParseMacroExpansionQuery.in_db_mut(self).set_lru_capacity(4 * lru_capacity);
        // hir::database::BorrowckQuery.in_db_mut(self).set_lru_capacity(base_db::DEFAULT_BORROWCK_LRU_CAP);
        // hir::database::BodyWithSourceMapQuery.in_db_mut(self).set_lru_capacity(2048);
    }

    #[expect(
        clippy::unused_self,
        clippy::needless_pass_by_ref_mut,
        reason = "TODO impl"
    )]
    pub const fn update_lru_capacities(
        &mut self,
        _lru_capacities: &FxHashMap<Box<str>, u16>,
    ) {
        // FIXME(salsa-transition): bring this back; allow changing LRU settings at runtime.
        // use hir::database as hir_db;

        // base_db::FileTextQuery.in_db_mut(self).set_lru_capacity(DEFAULT_FILE_TEXT_LRU_CAP);
        // base_db::ParseQuery.in_db_mut(self).set_lru_capacity(
        //     lru_capacities
        //         .get(stringify!(ParseQuery))
        //         .copied()
        //         .unwrap_or(base_db::DEFAULT_PARSE_LRU_CAP),
        // );
        // hir_db::ParseMacroExpansionQuery.in_db_mut(self).set_lru_capacity(
        //     lru_capacities
        //         .get(stringify!(ParseMacroExpansionQuery))
        //         .copied()
        //         .unwrap_or(4 * base_db::DEFAULT_PARSE_LRU_CAP),
        // );
        // hir_db::BorrowckQuery.in_db_mut(self).set_lru_capacity(
        //     lru_capacities
        //         .get(stringify!(BorrowckQuery))
        //         .copied()
        //         .unwrap_or(base_db::DEFAULT_BORROWCK_LRU_CAP),
        // );
        // hir::database::BodyWithSourceMapQuery.in_db_mut(self).set_lru_capacity(2048);
    }

    pub fn apply_change(
        &mut self,
        change: Change,
    ) {
        self.trigger_cancellation();
        change.apply(self);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SnippetCapability {
    _private: (),
}

impl SnippetCapability {
    #[must_use]
    pub const fn new(allow_snippets: bool) -> Option<Self> {
        if allow_snippets {
            Some(Self { _private: () })
        } else {
            None
        }
    }
}

#[query_group::query_group]
pub trait LineIndexDatabase: base_db::RootQueryDb {
    #[salsa::invoke_interned(line_index)]
    fn line_index(
        &self,
        file_id: FileId,
    ) -> Arc<LineIndex>;
}

fn line_index(
    database: &dyn LineIndexDatabase,
    file_id: FileId,
) -> Arc<LineIndex> {
    let text = database.file_text(file_id).text(database);
    Arc::new(LineIndex::new(text))
}
