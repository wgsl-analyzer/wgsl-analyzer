use std::hash::BuildHasherDefault;

use base_db::input::PackageId;
use edition::Edition;
use indexmap::IndexMap;
use paths::AbsPathBuf;
use rustc_hash::{FxHashMap, FxHashSet, FxHasher};
use triomphe::Arc;

use crate::{
    manifest_path::ManifestPath, package_interner::PackageInterner, wesl_package::WeslPackage,
};

/// An "opaque" and stable identifier for a package.
///
/// We request the dependencies for each package with cargo/npm.
/// From there, we get something that uniquely identifies the package.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PackageKey(ManifestPath);

impl PackageKey {
    #[must_use]
    pub fn from_package(package: &WeslPackage) -> Self {
        Self(package.manifest.clone())
    }
    #[must_use]
    pub const fn from_manifest_path(path: ManifestPath) -> Self {
        Self(path)
    }
}

/// Keeps track of the packages and their changes.
///
/// Packages inside of the editor are local packages.
///
/// Is separate from the workspace roots, because LSP clients can open
/// nested folders as separate workspaces. So a project can dynamically
/// be a part of multiple workspaces.
#[derive(Default)]
pub struct PackageGraph {
    interner: PackageInterner,
    packages: FxHashMap<PackageId, WeslPackage>,
    changes: IndexMap<PackageId, PackageChange, BuildHasherDefault<FxHasher>>,
}

#[derive(Eq, PartialEq, Debug)]
pub enum PackageChange {
    Set,
    Delete,
}

impl PackageGraph {
    /// Id of the given package if it exists and is not deleted.
    #[must_use]
    pub fn package_id(
        &self,
        key: &PackageKey,
    ) -> Option<PackageId> {
        let package_id = self.interner.get(key)?;
        self.contains(package_id).then_some(package_id)
    }

    /// Id of the given package if it exists and is not deleted.
    #[must_use]
    pub fn package_key(
        &self,
        id: PackageId,
    ) -> &PackageKey {
        self.interner.lookup(id)
    }

    #[must_use]
    pub fn contains(
        &self,
        id: PackageId,
    ) -> bool {
        self.packages.contains_key(&id)
    }

    pub fn set(
        &mut self,
        key: PackageKey,
        data: WeslPackage,
    ) {
        let package_id = self.interner.intern(key);

        self.changes.insert(package_id, PackageChange::Set);
        self.packages.insert(package_id, data);
    }

    pub fn remove(
        &mut self,
        id: PackageId,
    ) -> Result<(), ()> {
        self.changes.insert(id, PackageChange::Delete);
        match self.packages.remove(&id) {
            Some(_) => Ok(()),
            None => Err(()),
        }
    }

    #[must_use]
    pub fn get(
        &self,
        id: PackageId,
    ) -> Option<&WeslPackage> {
        self.packages.get(&id)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.packages.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.packages.is_empty()
    }

    pub fn take_changes(
        &mut self
    ) -> IndexMap<PackageId, PackageChange, BuildHasherDefault<FxHasher>> {
        std::mem::take(&mut self.changes)
    }

    /// Cleans up the set of discovered projects.
    pub fn retain<F>(
        &mut self,
        f: F,
    ) where
        F: Fn(&WeslPackage) -> bool,
    {
        self.packages.retain(|id, package| {
            let retain = f(package);
            if !retain {
                self.changes.insert(*id, PackageChange::Delete);
            }
            retain
        });
    }

    pub fn retain_referenced(&mut self) {
        let roots: FxHashSet<PackageId> = self
            .packages
            .iter()
            .filter(|(_, package)| package.is_local)
            .map(|(id, _)| *id)
            .collect();
        // TODO: finish this, it should clear all packages that are not somehow depended upon by a root package.
    }

    /// Returns an iterator over the stored ids and their corresponding data.
    ///
    /// This will skip deleted packages.
    pub fn iter(&self) -> impl Iterator<Item = (PackageId, &WeslPackage)> + '_ {
        self.packages.iter().map(|(id, package)| (*id, package))
    }
}
