use base_db::{
    VfsPath,
    input::{PackageName, PackageOrigin},
};
use edition::Edition;
use paths::AbsPathBuf;

use crate::{PackageKey, PackageRoot, manifest_path::ManifestPath};

/// Information associated with a wesl package.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WeslPackage {
    /// Path to the `wesl.toml`.
    pub manifest: VfsPath,
    /// Name generated from the folder name.
    pub display_name: Option<String>,
    /// Path to the main folder of the package.
    pub root: VfsPath,
    /// Origin of the package.
    pub origin: PackageOrigin,
    /// List of packages this package depends on.
    pub dependencies: Vec<PackageDependency>,
    /// WESL edition for this package.
    pub edition: Edition,
    // TODO: Support include and excludes https://github.com/wgsl-analyzer/wgsl-analyzer/issues/993
}

impl WeslPackage {
    #[must_use]
    pub fn to_root(&self) -> Option<PackageRoot> {
        let root = self.root.as_path()?;
        let manifest = self.manifest.as_path()?;
        // TODO: For maximal correctness, we'd opportunistically include every wesl.toml between the `self.manifest.parent()` folder and the `root_folder`
        Some(PackageRoot {
            origin: self.origin,
            directory: manifest.parent()?.to_owned(),
            include_files: [manifest.to_owned()].to_vec(),
            include: [root.to_owned()].to_vec(),
            exclude: Vec::new(),
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PackageDependency {
    Path {
        name: PackageName,
        path: ManifestPath,
    },
    Library {
        name: PackageName,
        package: String,
    },
}

impl PackageDependency {
    #[must_use]
    pub fn package_key(&self) -> PackageKey {
        #[expect(
            clippy::todo,
            reason = "See https://github.com/wgsl-analyzer/wgsl-analyzer/issues/976"
        )]
        match self {
            Self::Path { path, name: _ } => PackageKey::Manifest(path.clone()),
            Self::Library { name, package } => {
                todo!("Library dependencies are still unsupported")
            },
        }
    }

    #[must_use]
    pub const fn name(&self) -> &PackageName {
        match self {
            Self::Path { name, path: _ } | Self::Library { name, package: _ } => name,
        }
    }
}
