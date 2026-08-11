use std::{fmt, ops};

use salsa::{Durability, Setter as _};
use syntax::{Edition, ExtensionsConfig};

use crate::{
    SourceDatabase, SourceRootId,
    input::{PackageData, PackageId, PackageName, PackageOrigin},
    package,
};

#[salsa::input(debug)]
pub struct Package {
    #[returns(ref)]
    pub data: PackageData,
    // TODO: separate display name and version into extra_data
    // https://github.com/wgsl-analyzer/wgsl-analyzer/issues/999
    // /// Package data that is not needed for analysis.
    // ///
    // /// This is split into a separate field to increase incrementality.
    // #[returns(ref)]
    // pub extra_data: ExtraPackageData,
    #[returns(copy)]
    pub package_id: PackageId,
}

/// Package data unrelated to analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtraPackageData {
    pub version: Option<String>,
    /// A name used in the package's project declaration: for Cargo projects,
    /// its `[package].name` can be different for other project types or even
    /// absent (a dummy package for the code snippet, for example).
    ///
    /// For purposes of analysis, packages are anonymous (only names in
    /// [`crate::input::Dependency`] matters). This name should only be used for UI.
    pub display_name: Option<PackageDisplayName>,
}

#[expect(clippy::struct_field_names, reason = "no better idea")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PackageDisplayName {
    // The name we use to display various paths (with `_`).
    package_name: PackageName,
    // The name as specified in, for example, wesl.toml (with `-`).
    canonical_name: String,
}

impl PackageDisplayName {
    #[must_use]
    pub const fn canonical_name(&self) -> &String {
        &self.canonical_name
    }

    #[must_use]
    pub const fn package_name(&self) -> &PackageName {
        &self.package_name
    }
}

impl From<PackageName> for PackageDisplayName {
    fn from(package_name: PackageName) -> Self {
        let canonical_name = package_name.to_string();
        Self {
            package_name,
            canonical_name,
        }
    }
}

impl fmt::Display for PackageDisplayName {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        self.package_name.fmt(f)
    }
}

impl ops::Deref for PackageDisplayName {
    type Target = String;

    fn deref(&self) -> &String {
        &self.package_name
    }
}

impl PackageDisplayName {
    #[must_use]
    pub fn from_canonical_name(canonical_name: &str) -> Self {
        let package_name = PackageName::normalize_dashes(canonical_name);
        Self {
            package_name,
            canonical_name: canonical_name.to_owned(),
        }
    }
}

#[salsa::input(singleton, debug)]
struct AllPackages {
    #[returns(clone)]
    packages: std::sync::Arc<[Package]>,
}

pub fn set_all_packages_with_durability<Packages>(
    db: &mut dyn salsa::Database,
    packages: Packages,
    durability: Durability,
) where
    Packages: IntoIterator<Item = Package>,
{
    AllPackages::try_get(db)
        .unwrap_or_else(|| AllPackages::new(db, std::sync::Arc::default()))
        .set_packages(db)
        .with_durability(durability)
        .to(packages.into_iter().collect());
}

/// Returns the packages in topological order.
///
/// **Warning**: do not use this query in `hir-*` crates! It kills incrementality across crate metadata modifications.
pub fn all_packages(db: &dyn salsa::Database) -> std::sync::Arc<[Package]> {
    AllPackages::try_get(db).map_or_else(std::sync::Arc::default, |all_packages| {
        all_packages.packages(db)
    })
}

/// Finds the builtin package.
///
/// Different editions and extensions are all modeled as the same built-in package.
/// They just have attributes on their definitions that specify which edition/extension is required.
///
/// # Panics
/// Panics if there are multiple builtin packages.
#[salsa::tracked(returns(clone))]
pub fn builtin_package(db: &dyn SourceDatabase) -> Option<Package> {
    let packages = all_packages(db);
    let mut builtins = packages
        .iter()
        .filter(|package| package.data(db).origin == PackageOrigin::Language)
        .copied();

    let builtin = builtins.next();
    assert!(builtins.next().is_none(), "Multiple builtin packages found");
    builtin
}

/// Returns the package for a given file, if the file is a part of one.
pub fn file_package(
    db: &dyn SourceDatabase,
    file_id: vfs::FileId,
) -> Option<Package> {
    /// I believe this exists because each file has a different `FileSourceRootInput`.
    /// So Salsa cannot reuse computations that are driven by a `FileSourceRootInput`.
    /// TODO: Rust-Analyzer will remove this when the vfs gets rewritten.
    #[salsa::interned]
    struct InternedSourceRootId {
        #[returns(copy)]
        pub id: SourceRootId,
    }

    #[salsa::tracked(returns(clone))]
    fn file_package<'db>(
        db: &'db dyn SourceDatabase,
        id: InternedSourceRootId<'db>,
    ) -> Option<Package> {
        let packages = AllPackages::get(db).packages(db);
        let id = id.id(db);

        packages.iter().copied().find(|package| {
            let manifest_file = package.data(db).manifest_file_id;
            db.file_source_root(manifest_file).source_root_id(db) == id
        })
    }

    let _p = tracing::info_span!("file_package").entered();
    let source_root = db.file_source_root(file_id);
    file_package(
        db,
        InternedSourceRootId::new(db, source_root.source_root_id(db)),
    )
}

pub(crate) fn package_by_id(
    db: &dyn SourceDatabase,
    id: PackageId,
) -> Package {
    #[salsa::interned]
    struct InternedPackageId {
        #[returns(copy)]
        pub id: PackageId,
    }

    #[salsa::tracked(returns(clone))]
    fn package_by_id<'db>(
        db: &'db dyn SourceDatabase,
        id: InternedPackageId<'db>,
    ) -> Package {
        let packages = AllPackages::get(db).packages(db);
        let id = id.id(db);

        *packages
            .iter()
            .find(|package| package.package_id(db) == id)
            .unwrap()
    }

    package_by_id(db, InternedPackageId::new(db, id))
}
