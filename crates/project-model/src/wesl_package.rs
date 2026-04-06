use crate::{PackageKey, PackageRoot, manifest_path::ManifestPath};
use base_db::input::PackageOrigin;
use edition::Edition;
use paths::AbsPathBuf;

/// Information associated with a wesl package.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WeslPackage {
    /// Path to the `wesl.toml`.
    pub manifest: ManifestPath,
    /// Name generated from the folder name.
    pub display_name: Option<String>,
    /// Path to the main source file of the target.
    pub root: WeslPackageRoot,
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
        // We purposefully do not support the case where the user replaces a `shaders/main.wesl` file with a folder named `shaders/main.wesl/`.
        // If the user does that, then it is on them to restart the language server.
        let root_folder = match &self.root {
            #[expect(
                clippy::missing_panics_doc,
                reason = "This panic should not be possible"
            )]
            WeslPackageRoot::File(path) => path
                .parent()
                .expect("Files are always contained in a parent folder")
                .to_path_buf(),
            WeslPackageRoot::Folder(path) => path.clone(),
        };
        // TODO: For maximal correctness, we'd opportunistically include every wesl.toml between the `self.manifest.parent()` folder and the `root_folder`
        PackageRoot {
            origin: self.origin,
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
