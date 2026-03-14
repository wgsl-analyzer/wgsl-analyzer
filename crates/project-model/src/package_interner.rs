//! Maps packages to compact integer ids. We don't care about clearings paths which
//! no longer exist -- the assumption is total size of packages we ever look at is
//! not too big.
use std::hash::BuildHasherDefault;

use base_db::input::PackageId;
use indexmap::IndexSet;
use rustc_hash::FxHasher;

use crate::PackageKey;

/// Structure to map between [`PackageKey`] and [`PackageId`].
#[derive(Default)]
pub(crate) struct PackageInterner {
    map: IndexSet<PackageKey, BuildHasherDefault<FxHasher>>,
}

impl PackageInterner {
    /// Get the id corresponding to `path`.
    ///
    /// If `path` does not exists in `self`, returns [`None`].
    pub(crate) fn get(
        &self,
        path: &PackageKey,
    ) -> Option<PackageId> {
        self.map.get_index_of(path).map(PackageId::from_raw_usize)
    }

    /// Insert `path` in `self`.
    ///
    /// - If `path` already exists in `self`, returns its associated id;
    /// - Else, returns a newly allocated id.
    pub(crate) fn intern(
        &mut self,
        path: PackageKey,
    ) -> PackageId {
        let (id, _added) = self.map.insert_full(path);
        PackageId::from_raw_usize(id)
    }

    /// Returns the path corresponding to `id`.
    ///
    /// # Panics
    ///
    /// Panics if `id` does not exists in `self`.
    pub(crate) fn lookup(
        &self,
        id: PackageId,
    ) -> &PackageKey {
        self.map.get_index(id.to_raw_usize()).unwrap()
    }
}
