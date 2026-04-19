//! In `wgsl-analyzer`, we maintain a strict separation between pure abstract
//! semantic project model and a concrete model of a particular build system.
//!
//! Pure model is represented by the [`PackageGraph`] from another package.
//!
//! In this crate, we are concerned with "real world" project models.
//!
//! Specifically, here we have a representation for a `wesl-rs` project
//! ([`WeslToml`]) and for manually specified layout ([`ProjectManifest::ProjectJson`]).
//!
//! Roughly, the things we do here are:
//!
//! * Project discovery (where is the relevant `wesl.toml` for the current directory?)
//! * Lowering of concrete model to a [`PackageGraph`]

mod manifest_path;
mod package_graph;
mod package_interner;
mod wesl_package;
mod wesl_toml;
use std::{fmt, fs};

use anyhow::{bail, format_err};
use base_db::input::PackageOrigin;
pub use manifest_path::ManifestPath;
pub use package_graph::{PackageChange, PackageGraph, PackageKey};
use paths::{AbsPath, AbsPathBuf};
pub use wesl_package::{PackageDependency, WeslPackage, WeslPackageRoot};
pub use wesl_toml::{WeslDependency, WeslToml};

use crate::package_interner::PackageInterner;

/// Points at a relevant manifest file on disk.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum ProjectManifest {
    ProjectJson(ManifestPath),
    WeslToml(ManifestPath),
}

impl ProjectManifest {
    pub fn from_manifest_file(path: AbsPathBuf) -> anyhow::Result<Self> {
        let path = ManifestPath::try_from(path)
            .map_err(|path| format_err!("bad manifest path: {path}"))?;
        if path.file_name().unwrap_or_default() == "wesl-project.json" {
            return Ok(Self::ProjectJson(path));
        }
        if path.file_name().unwrap_or_default() == ".wesl-project.json" {
            return Ok(Self::ProjectJson(path));
        }
        if path.file_name().unwrap_or_default() == "wesl.toml" {
            return Ok(Self::WeslToml(path));
        }
        bail!("project root must point to a wesl.toml or wesl-project.json file: {path}");
    }

    #[must_use]
    pub fn discover(
        path: &AbsPath,
        search_parents: bool,
    ) -> Option<Self> {
        if let Some(project_json) =
            find_in_parent_directories(path, "wesl-project.json", search_parents)
        {
            return Some(Self::ProjectJson(project_json));
        }
        if let Some(wesl_toml) = find_in_parent_directories(path, "wesl.toml", search_parents) {
            return Some(Self::WeslToml(wesl_toml));
        }
        return None;
        fn find_in_parent_directories(
            path: &AbsPath,
            target_file_name: &str,
            search_parents: bool,
        ) -> Option<ManifestPath> {
            if path.file_name().unwrap_or_default() == target_file_name
                && let Ok(manifest) = ManifestPath::try_from(path.to_path_buf())
            {
                return Some(manifest);
            }

            if !search_parents {
                return None;
            }

            let mut curr = Some(path);
            while let Some(path) = curr {
                let candidate = path.join(target_file_name);
                if fs::metadata(&candidate).is_ok()
                    && let Ok(manifest) = ManifestPath::try_from(candidate)
                {
                    return Some(manifest);
                }
                curr = path.parent();
            }

            None
        }
    }

    #[must_use]
    pub const fn manifest_path(&self) -> &ManifestPath {
        match self {
            Self::ProjectJson(manifest) | Self::WeslToml(manifest) => manifest,
        }
    }
}

impl fmt::Display for ProjectManifest {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        fmt::Display::fmt(self.manifest_path(), formatter)
    }
}

/// `PackageRoot` describes a package root folder.
/// Which may be an external dependency, or a member of
/// the current workspace.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct PackageRoot {
    pub origin: PackageOrigin,
    pub manifest: ManifestPath,
    /// Files to include.
    pub include_files: Vec<AbsPathBuf>,
    /// Directories to include.
    pub include: Vec<AbsPathBuf>,
    /// Directories to exclude.
    pub exclude: Vec<AbsPathBuf>,
}
