use base_db::input::{PackageName, PackageOrigin};
use edition::Edition;
use paths::AbsPathBuf;

use crate::{PackageKey, PackageRoot, manifest_path::ManifestPath};

/// Information associated with a wesl package.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WeslPackage {
    /// Path to the `wesl.toml`.
    pub manifest: ManifestPath,
    /// Name generated from the folder name.
    pub display_name: Option<String>,
    /// Path to the main folder of the package.
    pub root: AbsPathBuf,
    /// Does this package come from the local filesystem (and is editable)?
    pub origin: PackageOrigin,
    /// List of packages this package depends on.
    pub dependencies: Vec<PackageDependency>,
    /// WESL edition for this package.
    pub edition: Edition,
    // TODO: Support include and excludes https://github.com/wgsl-analyzer/wgsl-analyzer/issues/993
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum WeslPackageRoot {
    File(AbsPathBuf),
    Folder(AbsPathBuf),
}

impl WeslPackage {
    #[must_use]
    pub fn to_root(&self) -> PackageRoot {
        // TODO: For maximal correctness, we'd opportunistically include every wesl.toml between the `self.manifest.parent()` folder and the `root_folder`
        PackageRoot {
            origin: self.origin,
            manifest: self.manifest.clone(),
            include_files: [AbsPathBuf::from(self.manifest.clone())].to_vec(),
            include: [self.root.clone()].to_vec(),
            exclude: Vec::new(),
        }
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
            Self::Path { path, name: _ } => PackageKey::from_manifest_path(path.clone()),
            Self::Library { name, package } => {
                todo!("Library dependencies are still unsupported")
            },
        }
    }

    #[must_use]
    pub const fn name(&self) -> &PackageName {
        match self {
            Self::Path { name, path } => name,
            Self::Library { name, package } => name,
        }
    }
}
