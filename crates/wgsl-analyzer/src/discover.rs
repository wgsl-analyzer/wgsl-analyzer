//! Infrastructure for lazy project discovery and loading. Currently only support wesl.toml discovery.
use std::str::FromStr as _;

use anyhow::{Context as _, bail};
use base_db::input::{PackageName, PackageOrigin};
use crossbeam_channel::Sender;
use edition::Edition;
use paths::AbsPathBuf;
use project_model::{
    ManifestPath, PackageDependency, PackageKey, ProjectManifest, WeslPackage, WeslPackageRoot,
    WeslToml,
};

/// A longer running task to load a package.
#[derive(Debug, Clone)]
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

    /// Create a new [`LoadPackageTask`] for loading a local project.
    pub(crate) fn discover_local(
        discover: &DiscoverArgument,
        sender: Sender<LoadPackageMessage>,
    ) -> Option<Self> {
        let manifest = ProjectManifest::discover(&discover.path, discover.search_parents)?;
        Some(Self::new(manifest, PackageOrigin::Local, sender))
    }

    pub(crate) fn package_key(&self) -> PackageKey {
        PackageKey::from_manifest_path(match &self.manifest {
            ProjectManifest::ProjectJson(manifest_path)
            | ProjectManifest::WeslToml(manifest_path) => manifest_path.clone(),
        })
    }

    /// Run the [`LoadPackageTask`] and report progress, if any.
    pub(crate) fn run(&self) {
        if let Err(error) = self.try_run() {
            self.send(LoadPackageMessage::Error {
                error: error.to_string(),
                source: None,
            });
        }
    }

    fn send(
        &self,
        message: LoadPackageMessage,
    ) {
        if let Err(error) = self.sender.send(message) {
            tracing::warn!("load package task failed to send {}", error);
        }
    }

    fn try_run(&self) -> anyhow::Result<()> {
        let project = match &self.manifest {
            ProjectManifest::WeslToml(manifest_path) => {
                let bytes = std::fs::read(manifest_path)
                    .with_context(|| format!("failed to read manifest file '{manifest_path}'"))?;
                let wesl_toml = WeslToml::from_slice(&bytes).with_context(|| {
                    format!("unable to parse contents of manifest '{manifest_path}'")
                })?;
                let root = manifest_path.parent().absolutize(&wesl_toml.root);
                if std::fs::metadata(&root)?.is_file() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "wesl.toml root must point at a folder",
                    )
                    .into());
                }

                let dependencies = wesl_toml
                    .dependencies
                    .into_iter()
                    .map(|(name, dependency)| {
                        let Ok(name) = PackageName::new(&name) else {
                            return Err(DependencyError::InvalidName(name));
                        };

                        Ok(match (dependency.path, dependency.package) {
                            (None, None) => PackageDependency::Library {
                                name: name.clone(),
                                package: name.to_string(),
                            },
                            (None, Some(package)) => PackageDependency::Library { name, package },
                            (Some(path), None) => {
                                let path = ManifestPath::try_from(
                                    manifest_path.parent().absolutize(path).join("wesl.toml"),
                                )
                                .map_err(|_path| DependencyError::InvalidPath(name.clone()))?;
                                PackageDependency::Path { name, path }
                            },
                            (Some(path), Some(package)) => {
                                return Err(DependencyError::Ambiguous(name));
                            },
                        })
                    })
                    .collect::<Result<Vec<_>, DependencyError>>()?;

                for dependency in &dependencies {
                    match dependency {
                        PackageDependency::Path { name, path } => {
                            self.send(LoadPackageMessage::Dependency {
                                task: Self::new(
                                    ProjectManifest::WeslToml(path.clone()),
                                    PackageOrigin::Local,
                                    self.sender.clone(),
                                ),
                            });
                        },
                        PackageDependency::Library { name, package } => {
                            // TODO: Loading libraries is not yet implemented, see https://github.com/wgsl-analyzer/wgsl-analyzer/issues/976
                            tracing::warn!("Loading libraries is not supported yet");
                        },
                    }
                }

                let metadata = std::fs::metadata(&root)
                    .with_context(|| format!("failed to get metadata of root file '{root}'"))?;
                let edition = Edition::from_str(&wesl_toml.edition).with_context(|| format!("manifest '{manifest_path}' specifies an invalid value for `edition`, found '{}'", wesl_toml.edition))?;
                WeslPackage {
                    manifest: manifest_path.clone(),
                    display_name: manifest_path.parent().file_name().map(str::to_owned),
                    root,
                    origin: self.origin,
                    dependencies,
                    edition,
                }
            },
            ProjectManifest::ProjectJson(manifest_path) => bail!("project json not supported"),
        };

        self.send(LoadPackageMessage::Finished { project });
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

#[derive(Debug)]
pub enum DependencyError {
    Ambiguous(PackageName),
    InvalidName(String),
    InvalidPath(PackageName),
}
impl std::fmt::Display for DependencyError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Ambiguous(name) => write!(
                f,
                "Package {name} is both a path dependency and a library dependency. Choose one, not both."
            ),
            Self::InvalidName(name) => {
                write!(f, "Package {name} is an invalid WESL name.")
            },
            Self::InvalidPath(name) => write!(f, "Package {name} has an invalid path."),
        }
    }
}
impl std::error::Error for DependencyError {}

/// An enum containing either progress messages, an error,
/// or the loaded project.
#[derive(Debug, Clone)]
pub enum LoadPackageMessage {
    Finished {
        project: WeslPackage,
    },
    Dependency {
        task: LoadPackageTask,
    },
    Error {
        error: String,
        source: Option<String>,
    },
    Progress {
        message: String,
    },
}
