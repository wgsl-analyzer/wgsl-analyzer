//! In `wgsl-analyzer`, we maintain a strict separation between pure abstract
//! semantic project model and a concrete model of a particular build system.
//!
//! Pure model is represented by the [`base_db::PackageGraph`] from another package.
//!
//! In this crate, we are concerned with "real world" project models.
//!
//! Specifically, here we have a representation for a `wesl-rs` project
//! ([`WeslWorkspace`]) and for manually specified layout ([`ProjectJson`]).
//!
//! Roughly, the things we do here are:
//!
//! * Project discovery (where's the relevant wesl.toml for the current dir).
//! * Custom build steps (`build.rs` code generation and compilation of
//!   procedural macros).
//! * Lowering of concrete model to a [`base_db::PackageGraph`]

pub mod project_json;

mod manifest_path;
mod wesl_toml;
mod workspace;

use std::{
    collections, ffi, fmt,
    fs::{self, ReadDir, read_dir},
    io, path,
    process::{self, Command},
};

use anyhow::{Context as _, bail, format_err};
use paths::{AbsPath, AbsPathBuf, Utf8PathBuf};
use rustc_hash::FxHashSet;

pub use crate::{
    manifest_path::ManifestPath,
    project_json::{ProjectJson, ProjectJsonData},
    workspace::{FileLoader, PackageRoot, ProjectWorkspace, ProjectWorkspaceKind, WeslConfig},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectJsonFromCommand {
    /// The data describing this project, such as its dependencies.
    pub data: ProjectJsonData,
    /// The build system specific file that describes this project,
    /// such as a `my-project/BUCK` file.
    pub buildfile: AbsPathBuf,
}

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

    pub fn discover_single(path: &AbsPath) -> anyhow::Result<Self> {
        let mut candidates = Self::discover(path)?;
        let Some(result) = candidates.pop() else {
            bail!("no projects")
        };
        if !candidates.is_empty() {
            bail!("more than one project");
        }
        Ok(result)
    }

    pub fn discover(path: &AbsPath) -> io::Result<Vec<Self>> {
        if let Some(project_json) = find_in_parent_dirs(path, "wesl-project.json") {
            return Ok(vec![Self::ProjectJson(project_json)]);
        }
        if let Some(project_json) = find_in_parent_dirs(path, ".wesl-project.json") {
            return Ok(vec![Self::ProjectJson(project_json)]);
        }
        return find_wesl_toml(path)
            .map(|paths| paths.into_iter().map(ProjectManifest::WeslToml).collect());

        fn find_wesl_toml(path: &AbsPath) -> io::Result<Vec<ManifestPath>> {
            match find_in_parent_dirs(path, "wesl.toml") {
                Some(manifest_path) => Ok(vec![manifest_path]),
                None => Ok(find_wesl_toml_in_child_dir(read_dir(path)?)),
            }
        }

        fn find_in_parent_dirs(
            path: &AbsPath,
            target_file_name: &str,
        ) -> Option<ManifestPath> {
            if path.file_name().unwrap_or_default() == target_file_name
                && let Ok(manifest) = ManifestPath::try_from(path.to_path_buf())
            {
                return Some(manifest);
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

        // TODO: Remove this (see https://github.com/rust-lang/rust-analyzer/issues/17537 )
        fn find_wesl_toml_in_child_dir(entities: ReadDir) -> Vec<ManifestPath> {
            // Only one level down to avoid cycles the easy way and stop a runaway scan with large projects
            entities
                .filter_map(Result::ok)
                .map(|entry| entry.path().join("wesl.toml"))
                .filter(|path| path.exists())
                .map(Utf8PathBuf::from_path_buf)
                .filter_map(Result::ok)
                .map(AbsPathBuf::try_from)
                .filter_map(Result::ok)
                .filter_map(|path| path.try_into().ok())
                .collect()
        }
    }

    #[must_use]
    pub fn discover_all(paths: &[AbsPathBuf]) -> Vec<Self> {
        let mut result = paths
            .iter()
            .filter_map(|path| Self::discover(path.as_ref()).ok())
            .flatten()
            .collect::<FxHashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        result.sort();
        result
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

fn utf8_stdout(cmd: &mut Command) -> anyhow::Result<String> {
    let output = cmd.output().with_context(|| format!("{cmd:?} failed"))?;
    if !output.status.success() {
        match String::from_utf8(output.stderr) {
            Ok(stderr) if !stderr.is_empty() => {
                bail!("{cmd:?} failed, {}\nstderr:\n{stderr}", output.status)
            },
            _ => bail!("{cmd:?} failed, {}", output.status),
        }
    }
    let stdout = String::from_utf8(output.stdout)?;
    Ok(stdout.trim().to_owned())
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum InvocationStrategy {
    Once,
    #[default]
    PerWorkspace,
}

#[expect(
    clippy::disallowed_types,
    reason = "generic parameter allows for FxHashMap"
)]
pub fn command<H, CommandString: AsRef<ffi::OsStr>, WorkingDirectory: AsRef<path::Path>>(
    command: CommandString,
    working_directory: WorkingDirectory,
    extra_env: &collections::HashMap<String, Option<String>, H>,
) -> process::Command {
    #[expect(clippy::disallowed_methods, reason = "`toolchain::command`")]
    let mut command = process::Command::new(command);
    command.current_dir(working_directory);
    for env in extra_env {
        match env {
            (key, Some(value)) => command.env(key, value),
            (key, None) => command.env_remove(key),
        };
    }
    command
}
