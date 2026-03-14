use edition::Edition;
use paths::{AbsPath, AbsPathBuf};
use triomphe::Arc;

use crate::{PackageKey, manifest_path::ManifestPath};

/// Information associated with a wesl package.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WeslPackage {
    /// Path to the `wesl.toml`.
    pub manifest: ManifestPath,
    /// Name generated from the folder name.
    pub display_name: Option<String>,
    /// Path to the main source file of the target.
    pub root: AbsPathBuf,
    /// Does this package come from the local filesystem (and is editable)?
    pub is_local: bool,
    /// List of packages this package depends on.
    pub dependencies: Vec<PackageDependency>,
    /// WESL edition for this package.
    pub edition: Edition,
    // TODO: Add "include" and "exclude" here
}

impl WeslPackage {
    #[must_use]
    pub fn to_root(&self) -> PackageRoot {
        let root_folder = if self.root.extension().is_some() {
            self.root.parent().map_or_else(
                || self.manifest.parent().to_path_buf(),
                AbsPath::to_path_buf,
            )
        } else {
            self.root.clone()
        };
        // TODO: For maximal correctness, we'd opportunistically include every wesl.toml between the `self.manifest.parent()` folder and the `root_folder`
        PackageRoot {
            is_local: self.is_local,
            manifest: self.manifest.clone(),
            include_files: [AbsPathBuf::from(self.manifest.clone())].to_vec(),
            include: [root_folder].to_vec(),
            exclude: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PackageDependency {
    pub pkg: PackageKey,
    pub name: String,
}

/// `PackageRoot` describes a package root folder.
/// Which may be an external dependency, or a member of
/// the current workspace.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct PackageRoot {
    /// Is from the local filesystem and may be edited.
    pub is_local: bool,
    pub manifest: ManifestPath,
    /// Files to include.
    pub include_files: Vec<AbsPathBuf>,
    /// Directories to include.
    pub include: Vec<AbsPathBuf>,
    /// Directories to exclude.
    pub exclude: Vec<AbsPathBuf>,
}
