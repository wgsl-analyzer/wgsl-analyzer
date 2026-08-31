//! Infrastructure for lazy project discovery and loading. Currently only support wesl.toml discovery.
use std::{
    collections::BTreeMap,
    fs, io,
    str::{FromStr as _, from_utf8},
};

use anyhow::{Context as _, anyhow, bail};
use base_db::input::{PackageName, PackageOrigin};
use cargo_metadata::MetadataCommand;
use crossbeam_channel::Sender;
use edition::Edition;
use paths::AbsPathBuf;
use project_model::{
    ManifestPath, PackageDependency, PackageKey, ProjectManifest, WeslDependency, WeslManifest,
    WeslPackage, WeslPackageRoot,
};
use stdx::process::spawn_with_streaming_output;

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
        let manifest = ProjectManifest::discover(
            &discover.path,
            discover.search_parents,
            |manifest| -> bool { get_cargo_metadata(manifest).is_ok() },
        )?;
        Some(Self::new(manifest, PackageOrigin::Local, sender))
    }

    pub(crate) fn package_key(&self) -> PackageKey {
        PackageKey::from_manifest_path(match &self.manifest {
            ProjectManifest::ProjectJson(manifest_path)
            | ProjectManifest::WeslToml(manifest_path)
            | ProjectManifest::CargoToml(manifest_path) => manifest_path.clone(),
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
                let manifest = WeslManifest::from_slice(&bytes).with_context(|| {
                    format!("unable to parse contents of manifest '{manifest_path}'")
                })?;
                self.parse(manifest_path, &manifest)?
            },
            ProjectManifest::CargoToml(manifest_path) => {
                let manifest = get_cargo_metadata(manifest_path)?;
                self.parse(manifest_path, &manifest)?
            },
            ProjectManifest::ProjectJson(manifest_path) => {
                let file = fs::File::open(manifest_path)?;
                let reader = io::BufReader::new(file);

                // Read the JSON contents of the file as an instance of `User`.
                let manifest: WeslManifest =
                    serde_json::from_reader(reader).with_context(|| {
                        format!("unable to parse contents of manifest '{manifest_path}'")
                    })?;
                self.parse(manifest_path, &manifest)?
            },
        };

        self.send(LoadPackageMessage::Finished { project });
        Ok(())
    }

    fn parse(
        &self,
        manifest_path: &ManifestPath,
        wesl_toml: &WeslManifest,
    ) -> Result<WeslPackage, anyhow::Error> {
        let root = manifest_path.parent().join(&wesl_toml.root);
        if std::fs::metadata(&root)?.is_file() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "wesl.toml root must point at a folder",
            )
            .into());
        }
        let dependencies = wesl_toml
            .dependencies
            .iter()
            .map(|(name, dependency)| {
                let Ok(name) = PackageName::new(name) else {
                    return Err(DependencyError::InvalidName(name.clone()));
                };

                Ok(
                    match (dependency.path.clone(), dependency.package.clone()) {
                        (None, None) => PackageDependency::Library {
                            name: name.clone(),
                            package: name.to_string(),
                        },
                        (None, Some(package)) => PackageDependency::Library { name, package },
                        (Some(path), None) => {
                            let base = manifest_path.parent().join(path);
                            let wesl_toml = base.join("wesl.toml");
                            // TODO: this isn't always a manifest
                            let cargo_toml = base.join("Cargo.toml");
                            let project_json = base.join("wesl-project.json");
                            let dot_project_json = base.join(".wesl-project.json");
                            let manifest = [wesl_toml, cargo_toml, project_json, dot_project_json]
                                .into_iter()
                                .find(|candidate| fs::metadata(candidate).is_ok())
                                .ok_or_else(|| DependencyError::InvalidPath(name.clone()))?;
                            let path = ManifestPath::try_from(manifest)
                                .map_err(|_path| DependencyError::InvalidPath(name.clone()))?;
                            PackageDependency::Path { name, path }
                        },
                        (Some(path), Some(package)) => {
                            return Err(DependencyError::Ambiguous(name));
                        },
                    },
                )
            })
            .collect::<Result<Vec<_>, DependencyError>>()?;
        for dependency in &dependencies {
            match dependency {
                PackageDependency::Path { name, path } => {
                    self.send(LoadPackageMessage::Dependency {
                        task: Self::new(
                            ProjectManifest::from_manifest_path(path.clone())?,
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
        let edition = Edition::from_str(&wesl_toml.edition).with_context(|| {
            format!(
                "manifest '{manifest_path}' specifies an invalid value for `edition`, found '{}'",
                wesl_toml.edition
            )
        })?;
        Ok(WeslPackage {
            manifest: manifest_path.clone(),
            display_name: manifest_path.parent().file_name().map(str::to_owned),
            root,
            origin: self.origin,
            dependencies,
            edition,
        })
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

#[expect(clippy::print_stderr, reason = "tracing not working")]
fn get_cargo_metadata(manifest_path: &ManifestPath) -> Result<WeslManifest, anyhow::Error> {
    let cargo_path = toolchain::Tool::Cargo.path();
    let mut command = MetadataCommand::new();
    command.cargo_path(cargo_path);
    command.manifest_path(manifest_path);
    let mut errored = false;
    let output = spawn_with_streaming_output(command.cargo_command(), &mut |_| {}, &mut |line| {
        errored = errored || line.starts_with("error") || line.starts_with("warning");
        if errored {
            eprintln!("{line}");
            // progress("cargo metadata: ?".to_owned());
            // return;
        }
        // progress(format!("cargo metadata: {line}"));
    })
    .with_context(|| "spawn with streaming output failed")?;
    if !output.status.success() {
        // progress(format!("cargo metadata: failed {}", output.status));
        let error = cargo_metadata::Error::CargoMetadata {
            stderr: String::from_utf8(output.stderr.clone())
                .with_context(|| "stderr was not utf8")?,
        };
        return Err(error).with_context(|| {
            format!(
                "output status does not indicate success:\n{}",
                String::from_utf8(output.stderr).unwrap()
            )
        })?;
    }
    let stdout = from_utf8(&output.stdout)
        .with_context(|| "converting stdout to utf8 failed")?
        .lines()
        .find(|line| line.starts_with('{'))
        .ok_or(cargo_metadata::Error::NoJson)
        .with_context(|| "not json")?;
    let metadata = cargo_metadata::MetadataCommand::parse(stdout)
        .with_context(|| "unable to parse stdout as cargo metadata")?;
    let binding = metadata.workspace_packages();
    let find = binding
        .iter()
        .find(|package| package.manifest_path == manifest_path.as_str());
    let partial: WeslManifest = find
        .unwrap()
        .metadata
        .get("wgsl-analyzer")
        .map(|table| serde_json::from_value(table.clone()))
        .transpose()
        .with_context(|| anyhow!("no wgsl-analyzer table in {manifest_path}"))?
        .unwrap_or_default();
    let cargo_dependencies = find
        .unwrap()
        .dependencies
        .iter()
        .cloned()
        .map(|cargo_dependency| {
            (
                to_wesl_name(
                    &cargo_dependency
                        .rename
                        .unwrap_or_else(|| cargo_dependency.name.clone()),
                ),
                WeslDependency {
                    package: None,
                    path: cargo_dependency.path.map(paths::Utf8PathBuf::into_string),
                },
            )
        });
    let combined = partial.dependencies.into_iter().chain(cargo_dependencies);
    let manifest = WeslManifest {
        package_manager: Some("Cargo".to_owned()),
        dependencies: BTreeMap::from_iter(combined),
        ..partial
    };
    Ok(manifest)
}

fn to_wesl_name(package_name: &str) -> String {
    package_name.replace('-', "_")
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
