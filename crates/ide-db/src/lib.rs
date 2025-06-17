use rustc_hash::FxHashMap;
use std::{fmt, mem::ManuallyDrop};
use triomphe::Arc;

use base_db::{FileId, FileLoader, FileLoaderDelegate, SourceDatabase, change::Change};
use vfs::AnchoredPath;

pub mod source_change;
pub mod text_edit;

#[salsa::database(
    base_db::SourceDatabaseStorage,
    hir_def::database::DefDatabaseStorage,
    hir_def::database::InternDatabaseStorage,
    hir_ty::database::HirDatabaseStorage
)]
pub struct RootDatabase {
    // FIXME: Revisit this commit now that we migrated to the new salsa, given we store arcs in this
    // db directly now
    // We use `ManuallyDrop` here because every codegen unit that contains a
    // `&RootDatabase -> &dyn OtherDatabase` cast will instantiate its drop glue in the vtable,
    // which duplicates `Weak::drop` and `Arc::drop` tens of thousands of times, which makes
    // compile times of all `ide_*` and downstream crates suffer greatly.
    storage: ManuallyDrop<salsa::Storage<Self>>,
    // files: Arc<Files>,
    // crates_map: Arc<CratesMap>,
}

impl std::panic::RefUnwindSafe for RootDatabase {}

impl Drop for RootDatabase {
    fn drop(&mut self) {
        unsafe { ManuallyDrop::drop(&mut self.storage) };
    }
}

impl fmt::Debug for RootDatabase {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct("RootDatabase").finish()
    }
}

impl salsa::Database for RootDatabase {}

impl salsa::ParallelDatabase for RootDatabase {
    fn snapshot(&self) -> salsa::Snapshot<RootDatabase> {
        salsa::Snapshot::new(RootDatabase {
            storage: ManuallyDrop::new(self.storage.snapshot()),
        })
    }
}

impl RootDatabase {
    pub fn new(lru_capacity: Option<u16>) -> RootDatabase {
        let mut database = RootDatabase {
            storage: ManuallyDrop::new(salsa::Storage::default()),
            // files: Default::default(),
            // crates_map: Default::default(),
        };
        database.set_custom_imports(Arc::new(Default::default()));
        database.set_shader_defs(Arc::new(Default::default()));

        // This needs to be here otherwise `CrateGraphBuilder` will panic.
        // database.set_all_crates(Arc::new(Box::new([])));
        // CrateGraphBuilder::default().set_in_db(&mut database);
        // database.set_proc_macros_with_durability(Default::default(), Durability::MEDIUM);
        // database.set_local_roots_with_durability(Default::default(), Durability::MEDIUM);
        // database.set_library_roots_with_durability(Default::default(), Durability::MEDIUM);
        // database.set_expand_proc_attr_macros_with_durability(false, Durability::HIGH);
        database.update_base_query_lru_capacities(lru_capacity);
        database
    }

    pub fn update_base_query_lru_capacities(
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

    pub fn update_lru_capacities(
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
        change.apply(self);
    }
}

impl FileLoader for RootDatabase {
    fn resolve_path(
        &self,
        path: AnchoredPath<'_>,
    ) -> Option<FileId> {
        FileLoaderDelegate(self).resolve_path(path)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SnippetCap {
    _private: (),
}

impl SnippetCap {
    pub const fn new(allow_snippets: bool) -> Option<SnippetCap> {
        if allow_snippets {
            Some(SnippetCap { _private: () })
        } else {
            None
        }
    }
}
