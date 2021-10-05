use std::mem::ManuallyDrop;

use base_db::{change::Change, SourceDatabase, Upcast};
use hir_def::db::DefDatabase;

#[salsa::database(
    base_db::SourceDatabaseStorage,
    hir_def::db::DefDatabaseStorage,
    hir_def::db::InternDatabaseStorage,
    hir_ty::HirDatabaseStorage
)]
pub struct RootDatabase {
    // We use `ManuallyDrop` here because every codegen unit that contains a
    // `&RootDatabase -> &dyn OtherDatabase` cast will instantiate its drop glue in the vtable,
    // which duplicates `Weak::drop` and `Arc::drop` tens of thousands of times, which makes
    // compile times of all `ide_*` and downstream crates suffer greatly.
    storage: ManuallyDrop<salsa::Storage<RootDatabase>>,
}

impl std::fmt::Debug for RootDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl Drop for RootDatabase {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.storage);
        }
    }
}

impl RootDatabase {
    pub fn new() -> Self {
        Self {
            storage: ManuallyDrop::new(salsa::Storage::default()),
        }
    }

    pub fn apply_change(&mut self, change: Change) {
        change.apply(self);
    }
}

impl Upcast<dyn DefDatabase> for RootDatabase {
    fn upcast(&self) -> &(dyn DefDatabase + 'static) {
        &*self
    }
}

impl Upcast<dyn SourceDatabase> for RootDatabase {
    fn upcast(&self) -> &(dyn SourceDatabase + 'static) {
        &*self
    }
}
