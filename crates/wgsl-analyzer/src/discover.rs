//! Infrastructure for lazy project discovery and loading. Currently only support wesl.toml discovery.
use std::{path::Path, str::FromStr as _};

use anyhow::bail;
use base_db::input::PackageOrigin;
use crossbeam_channel::Sender;
use edition::Edition;
use paths::{AbsPathBuf, Utf8Path, Utf8PathBuf};
use project_model::{
    ManifestPath, PackageKey, ProjectManifest, WeslPackage, WeslPackageRoot, WeslToml,
};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use tracing::{info_span, span::EnteredSpan};

/// A longer running task to load a package.
pub(crate) struct LoadPackageTask {
    manifest: ProjectManifest,
    origin: PackageOrigin,
    sender: Sender<LoadPackageMessage>,
}

/// Request WESL project discovery starting in a given folder.
/// Does not load the dependencies.
#[derive(PartialEq, Clone, Debug)]
pub(crate) struct DiscoverArgument {
    pub(crate) path: AbsPathBuf,
    /// Whether to look at the parent folders for a `wesl.toml`.
    pub(crate) search_parents: bool,
}

impl LoadPackageTask {
    /// Create a new [`LoadPackageTask`] for loading a local project.
    pub(crate) fn discover_local(
        discover: &DiscoverArgument,
        sender: Sender<LoadPackageMessage>,
    ) -> Option<Self> {
        let manifest = ProjectManifest::discover(&discover.path, discover.search_parents)?;
        Some(Self {
            manifest,
            origin: PackageOrigin::Local,
            sender,
        })
    }

    pub(crate) const fn new(
        manifest: ProjectManifest,
        origin: PackageOrigin,
        sender: Sender<LoadPackageMessage>,
    ) -> Self {
        Self {
            manifest,
            origin,
            sender,
        }
    }

    /// Run the [`LoadPackageTask`] and report progress, if any.
    pub(crate) fn run(&self) {
        if let Err(error) = self.try_run() {
            self.sender.send(LoadPackageMessage::Error {
                error: error.to_string(),
                source: None,
            });
        }
    }

    fn try_run(&self) -> anyhow::Result<()> {
        let project = match &self.manifest {
            ProjectManifest::WeslToml(manifest_path) => {
                let wesl_toml = WeslToml::from_slice(&std::fs::read(manifest_path)?)?;
                let root = manifest_path.parent().join(&wesl_toml.root);
                WeslPackage {
                    manifest: manifest_path.clone(),
                    display_name: None,
                    root: if std::fs::metadata(&root)?.is_file() {
                        WeslPackageRoot::File(root)
                    } else {
                        WeslPackageRoot::Folder(root)
                    },
                    origin: self.origin,
                    // TODO: Load the wesl_toml.dependencies, see https://github.com/wgsl-analyzer/wgsl-analyzer/issues/976
                    dependencies: Vec::new(),
                    edition: Edition::from_str(&wesl_toml.edition)?,
                }
            },
            ProjectManifest::ProjectJson(manifest_path) => bail!("project json not supported"),
        };

        self.sender.send(LoadPackageMessage::Finished { project });
        Ok(())
    }

    #[expect(
        clippy::unused_self,
        reason = "Dependency loading is not implemented, so this does nothing useful. See https://github.com/wgsl-analyzer/wgsl-analyzer/issues/976 "
    )]
    pub(crate) const fn has_exited(&self) -> bool {
        true
    }

    #[expect(
        clippy::unused_self,
        reason = "Dependency loading is not implemented, so this does nothing useful. See https://github.com/wgsl-analyzer/wgsl-analyzer/issues/976 "
    )]
    pub(crate) const fn join(&self) {}
}

/// An enum containing either progress messages, an error,
/// or the loaded project.
#[derive(Debug, Clone)]
pub enum LoadPackageMessage {
    Finished {
        project: WeslPackage,
    },
    Error {
        error: String,
        source: Option<String>,
    },
    Progress {
        message: String,
    },
}
