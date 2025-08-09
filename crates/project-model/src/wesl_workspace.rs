//! See [`WeslWorkspace`].

use std::str::from_utf8;
use std::{ffi, ops, path, process};

use anyhow::Context;
use base_db::Env;
use la_arena::{Arena, Idx};
use paths::{AbsPath, AbsPathBuf, Utf8Path, Utf8PathBuf};
use rustc_hash::{FxHashMap, FxHashSet};
use serde::Deserialize;
use serde_json::from_value;
use span::Edition;
use stdx::process::spawn_with_streaming_output;
use wesl_metadata::MetadataCommand;

use crate::InvocationStrategy;
use crate::ManifestPath;

const MINIMUM_TOOLCHAIN_VERSION_SUPPORTING_LOCKFILE_PATH: semver::Version = semver::Version {
    major: 1,
    minor: 82,
    patch: 0,
    pre: semver::Prerelease::EMPTY,
    build: semver::BuildMetadata::EMPTY,
};

/// [`WeslWorkspace`] represents the logical structure of, well, a WESL
/// workspace. It pretty closely mirrors `wesl metadata` output.
///
/// Note that internally, `wgsl-analyzer` uses a different structure:
/// `PackageGraph`. `PackageGraph` is lower-level: it knows only about the packages,
/// while this knows about `Packages`, a purely wesl-related
/// concept.
///
/// We use absolute paths here, `wesl metadata` guarantees to always produce
/// abs paths.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WeslWorkspace {
    packages: Arena<PackageData>,
    workspace_root: AbsPathBuf,
    target_directory: AbsPathBuf,
    manifest_path: ManifestPath,
    is_virtual_workspace: bool,
    /// Environment variables set in the `.wesl/config` file.
    config_env: Env,
    requires_rustc_private: bool,
}

impl ops::Index<Package2> for WeslWorkspace {
    type Output = PackageData;
    fn index(
        &self,
        index: Package2,
    ) -> &PackageData {
        &self.packages[index]
    }
}

/// Describes how to set the rustc source directory.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RustLibSource {
    /// Explicit path for the rustc source directory.
    Path(AbsPathBuf),
    /// Try to automatically detect where the rustc source directory is.
    Discover,
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct WeslConfig {
    /// Extra includes to add to the VFS.
    pub extra_includes: Vec<AbsPathBuf>,
    /// Extra args to pass to the `wesl` command.
    pub extra_args: Vec<String>,
    /// Extra env vars to set when invoking the `wesl` command
    pub extra_env: FxHashMap<String, Option<String>>,
    pub invocation_strategy: InvocationStrategy,
    /// Optional path to use instead of `target` when building
    pub target_dir: Option<Utf8PathBuf>,
    /// Load the project without any dependencies
    pub no_deps: bool,
}

pub type Package2 = Idx<PackageData>;

/// Information associated with a WESL package
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PackageData {
    /// Version given in the `wesl.toml`
    pub version: semver::Version,
    /// Name as given in the `wesl.toml`
    pub name: String,
    /// Repository as given in the `wesl.toml`
    pub repository: Option<String>,
    /// Path containing the `wesl.toml`
    pub manifest: ManifestPath,
    /// Does this package come from the local filesystem (and is editable)?
    pub is_local: bool,
    /// List of packages this package depends on
    pub dependencies: Vec<PackageDependency>,
    /// Rust edition for this package
    pub edition: Edition,
    /// String representation of package id
    pub id: String,
    /// Authors as given in the `wesl.toml`
    pub authors: Vec<String>,
    /// Description as given in the `wesl.toml`
    pub description: Option<String>,
    /// Homepage as given in the `wesl.toml`
    pub homepage: Option<String>,
    /// License as given in the `wesl.toml`
    pub license: Option<String>,
    /// License file as given in the `wesl.toml`
    pub license_file: Option<Utf8PathBuf>,
    /// Readme file as given in the `wesl.toml`
    pub readme: Option<Utf8PathBuf>,
    /// The contents of [package.metadata.wgsl-analyzer]
    pub metadata: WgslAnalyzerPackageMetaData,
}

#[derive(Deserialize, Default, Debug, Clone, Eq, PartialEq)]
pub struct WgslAnalyzerPackageMetaData {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PackageDependency {
    pub pkg: Package2,
    pub name: String,
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct WeslMetadataConfig {
    /// Extra args to pass to the wesl command.
    pub extra_args: Vec<String>,
    /// Extra env vars to set when invoking the wesl command
    pub extra_env: FxHashMap<String, Option<String>>,
    /// The target dir for this workspace load.
    pub target_dir: Utf8PathBuf,
}

// Deserialize helper for the wesl metadata
#[derive(Deserialize, Default)]
struct PackageMetadata {
    #[serde(rename = "wgsl-analyzer")]
    wgsl_analyzer: Option<WgslAnalyzerPackageMetaData>,
}

impl WeslWorkspace {
    /// Fetches the metadata for the given `wesl_toml` manifest.
    /// A successful result may contain another metadata error if the initial fetching failed but
    /// the `--no-deps` retry succeeded.
    ///
    /// The sysroot is used to set the `RUSTUP_TOOLCHAIN` env var when invoking wesl
    /// to ensure that the rustup proxy uses the correct toolchain.
    pub fn fetch_metadata(
        wesl_toml: &ManifestPath,
        current_dir: &AbsPath,
        config: &WeslMetadataConfig,
        no_deps: bool,
        locked: bool,
        progress: &dyn Fn(String),
    ) -> anyhow::Result<(wesl_metadata::Metadata, Option<anyhow::Error>)> {
        let res = Self::fetch_metadata_(wesl_toml, current_dir, config, no_deps, locked, progress);
        if let Ok((_, Some(ref e))) = res {
            tracing::warn!(
                %wesl_toml,
                ?e,
                "`wesl metadata` failed, but retry with `--no-deps` succeeded"
            );
        }
        res
    }

    fn fetch_metadata_(
        wesl_toml: &ManifestPath,
        current_dir: &AbsPath,
        config: &WeslMetadataConfig,
        no_deps: bool,
        locked: bool,
        progress: &dyn Fn(String),
    ) -> anyhow::Result<(wesl_metadata::Metadata, Option<anyhow::Error>)> {
        let wesl = crate::command("wesl", current_dir, &config.extra_env);
        let mut meta = MetadataCommand::new();
        meta.wesl_path(wesl.get_program());
        wesl.get_envs()
            .for_each(|(var, val)| _ = meta.env(var, val.unwrap_or_default()));
        meta.manifest_path(wesl_toml.to_path_buf());
        meta.current_dir(current_dir);

        let mut other_options = vec![];
        // wesl metadata only supports a subset of flags of what wesl usually accepts, and usually
        // the only relevant flags for metadata here are unstable ones, so we pass those along
        // but nothing else
        let mut extra_args = config.extra_args.iter();
        while let Some(arg) = extra_args.next() {
            if arg == "-Z" {
                if let Some(arg) = extra_args.next() {
                    other_options.push("-Z".to_owned());
                    other_options.push(arg.to_owned());
                }
            }
        }

        if no_deps {
            other_options.push("--no-deps".to_owned());
        }

        meta.other_options(other_options);

        // FIXME: Fetching metadata is a slow process, as it might require
        // calling crates.io. We should be reporting progress here, but it's
        // unclear whether wesl itself supports it.
        progress("wesl metadata: started".to_owned());

        let res = (|| -> anyhow::Result<(_, _)> {
            let mut errored = false;
            let output =
                spawn_with_streaming_output(meta.wesl_command(), &mut |_| (), &mut |line| {
                    errored = errored || line.starts_with("error") || line.starts_with("warning");
                    if errored {
                        progress("wesl metadata: ?".to_owned());
                        return;
                    }
                    progress(format!("wesl metadata: {line}"));
                })?;
            if !output.status.success() {
                progress(format!("wesl metadata: failed {}", output.status));
                let error = wesl_metadata::Error::WeslMetadata {
                    stderr: String::from_utf8(output.stderr)?,
                }
                .into();
                if !no_deps {
                    // If we failed to fetch metadata with deps, try again without them.
                    // This makes r-a still work partially when offline.
                    if let Ok((metadata, _)) = Self::fetch_metadata_(
                        wesl_toml,
                        current_dir,
                        config,
                        true,
                        locked,
                        progress,
                    ) {
                        return Ok((metadata, Some(error)));
                    }
                }
                return Err(error);
            }
            let stdout = from_utf8(&output.stdout)?
                .lines()
                .find(|line| line.starts_with('{'))
                .ok_or(wesl_metadata::Error::NoJson)?;
            Ok((wesl_metadata::MetadataCommand::parse(stdout)?, None))
        })()
        .with_context(|| format!("Failed to run `{:?}`", meta.wesl_command()));
        progress("wesl metadata: finished".to_owned());
        res
    }

    pub fn new(
        mut meta: wesl_metadata::Metadata,
        ws_manifest_path: ManifestPath,
        wesl_config_env: Env,
        is_sysroot: bool,
    ) -> WeslWorkspace {
        let mut pkg_by_id = FxHashMap::default();
        let mut packages = Arena::default();
        let mut targets = Arena::default();

        let workspace_root = AbsPathBuf::assert(meta.root_package_directory);
        let target_directory = AbsPathBuf::assert(meta.target_directory);
        let mut is_virtual_workspace = true;
        let mut requires_rustc_private = false;

        meta.packages.sort_by(|a, b| a.id.cmp(&b.id));
        for meta_pkg in meta.packages {
            let wesl_metadata::Package {
                name,
                version,
                id,
                manifest_path,
                repository,
                edition,
                metadata,
                authors,
                description,
                homepage,
                license,
                license_file,
                readme,
                ..
            } = meta_pkg;
            let meta = from_value::<PackageMetadata>(metadata).unwrap_or_default();
            let edition = match edition {
                wesl_metadata::Edition::E2024 => Edition::Edition2024,
                _ => {
                    tracing::error!("Unsupported edition `{:?}`", edition);
                    Edition::CURRENT
                },
            };
            let manifest = ManifestPath::try_from(AbsPathBuf::assert(manifest_path)).unwrap();
            is_virtual_workspace &= manifest != ws_manifest_path;
            let pkg = packages.alloc(PackageData {
                id: id.repr.clone(),
                name: name.to_string(),
                version,
                manifest: manifest.clone(),
                is_local: true,
                edition,
                repository,
                authors,
                description,
                homepage,
                license,
                license_file,
                readme,
                dependencies: Vec::new(),
                metadata: meta.wgsl_analyzer.unwrap_or_default(),
            });
            let pkg_data = &mut packages[pkg];
            pkg_by_id.insert(id, pkg);
        }
        for mut node in meta.resolve.map_or_else(Vec::new, |it| it.nodes) {
            let &source = pkg_by_id.get(&node.id).unwrap();
            node.renamed_dependencies.sort_by(|a, b| a.pkg.cmp(&b.pkg));
            let dependencies = node.renamed_dependencies;
            for (dep_node) in dependencies {
                let &pkg = pkg_by_id.get(&dep_node.pkg).unwrap();
                let dep = PackageDependency {
                    name: dep_node.name.to_string(),
                    pkg,
                };
                packages[source].dependencies.push(dep);
            }
        }

        WeslWorkspace {
            packages,
            workspace_root,
            target_directory,
            manifest_path: ws_manifest_path,
            is_virtual_workspace,
            requires_rustc_private,
            config_env: wesl_config_env,
        }
    }

    pub fn packages(&self) -> impl ExactSizeIterator<Item = Package2> + '_ {
        self.packages.iter().map(|(id, _pkg)| id)
    }

    pub fn workspace_root(&self) -> &AbsPath {
        &self.workspace_root
    }

    pub fn manifest_path(&self) -> &ManifestPath {
        &self.manifest_path
    }

    pub fn target_directory(&self) -> &AbsPath {
        &self.target_directory
    }

    pub fn package_flag(
        &self,
        package: &PackageData,
    ) -> String {
        if self.is_unique(&package.name) {
            package.name.clone()
        } else {
            format!("{}:{}", package.name, package.version)
        }
    }

    pub fn parent_manifests(
        &self,
        manifest_path: &ManifestPath,
    ) -> Option<Vec<ManifestPath>> {
        let mut found = false;
        let parent_manifests = self
            .packages()
            .filter_map(|pkg| {
                if !found && &self[pkg].manifest == manifest_path {
                    found = true
                }
                self[pkg].dependencies.iter().find_map(|dep| {
                    (&self[dep.pkg].manifest == manifest_path).then(|| self[pkg].manifest.clone())
                })
            })
            .collect::<Vec<ManifestPath>>();

        // some packages has this pkg as dep. return their manifests
        if !parent_manifests.is_empty() {
            return Some(parent_manifests);
        }

        // this pkg is inside this wesl workspace, fallback to workspace root
        if found {
            return Some(vec![
                ManifestPath::try_from(self.workspace_root().join("wesl.toml")).ok()?,
            ]);
        }

        // not in this workspace
        None
    }

    fn is_unique(
        &self,
        name: &str,
    ) -> bool {
        self.packages.iter().filter(|(_, v)| v.name == name).count() == 1
    }

    pub fn is_virtual_workspace(&self) -> bool {
        self.is_virtual_workspace
    }

    pub fn env(&self) -> &Env {
        &self.config_env
    }

    pub fn requires_rustc_private(&self) -> bool {
        self.requires_rustc_private
    }
}
