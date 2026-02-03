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

mod env;
mod manifest_path;
mod wesl_toml;
mod workspace;

use std::{
    ffi, fmt,
    fs::{self, ReadDir, read_dir},
    io, path,
    process::{self, Command},
};

use anyhow::{Context, bail, format_err};
use paths::{AbsPath, AbsPathBuf, Utf8PathBuf};
use rustc_hash::FxHashSet;

pub use crate::{
    manifest_path::ManifestPath,
    project_json::{ProjectJson, ProjectJsonData},
    workspace::{FileLoader, PackageRoot, ProjectWorkspace, ProjectWorkspaceKind},
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
    pub fn from_manifest_file(path: AbsPathBuf) -> anyhow::Result<ProjectManifest> {
        let path = ManifestPath::try_from(path)
            .map_err(|path| format_err!("bad manifest path: {path}"))?;
        if path.file_name().unwrap_or_default() == "wesl-project.json" {
            return Ok(ProjectManifest::ProjectJson(path));
        }
        if path.file_name().unwrap_or_default() == ".wesl-project.json" {
            return Ok(ProjectManifest::ProjectJson(path));
        }
        if path.file_name().unwrap_or_default() == "wesl.toml" {
            return Ok(ProjectManifest::WeslToml(path));
        }
        bail!("project root must point to a wesl.toml or wesl-project.json file: {path}");
    }

    pub fn discover_single(path: &AbsPath) -> anyhow::Result<ProjectManifest> {
        let mut candidates = ProjectManifest::discover(path)?;
        let result = match candidates.pop() {
            None => bail!("no projects"),
            Some(it) => it,
        };

        if !candidates.is_empty() {
            bail!("more than one project");
        }
        Ok(result)
    }

    pub fn discover(path: &AbsPath) -> io::Result<Vec<ProjectManifest>> {
        if let Some(project_json) = find_in_parent_dirs(path, "wesl-project.json") {
            return Ok(vec![ProjectManifest::ProjectJson(project_json)]);
        }
        if let Some(project_json) = find_in_parent_dirs(path, ".wesl-project.json") {
            return Ok(vec![ProjectManifest::ProjectJson(project_json)]);
        }
        return find_wesl_toml(path)
            .map(|paths| paths.into_iter().map(ProjectManifest::WeslToml).collect());

        fn find_wesl_toml(path: &AbsPath) -> io::Result<Vec<ManifestPath>> {
            match find_in_parent_dirs(path, "wesl.toml") {
                Some(it) => Ok(vec![it]),
                None => Ok(find_wesl_toml_in_child_dir(read_dir(path)?)),
            }
        }

        fn find_in_parent_dirs(
            path: &AbsPath,
            target_file_name: &str,
        ) -> Option<ManifestPath> {
            if path.file_name().unwrap_or_default() == target_file_name {
                if let Ok(manifest) = ManifestPath::try_from(path.to_path_buf()) {
                    return Some(manifest);
                }
            }

            let mut curr = Some(path);

            while let Some(path) = curr {
                let candidate = path.join(target_file_name);
                if fs::metadata(&candidate).is_ok() {
                    if let Ok(manifest) = ManifestPath::try_from(candidate) {
                        return Some(manifest);
                    }
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
                .map(|it| it.path().join("wesl.toml"))
                .filter(|it| it.exists())
                .map(Utf8PathBuf::from_path_buf)
                .filter_map(Result::ok)
                .map(AbsPathBuf::try_from)
                .filter_map(Result::ok)
                .filter_map(|it| it.try_into().ok())
                .collect()
        }
    }

    pub fn discover_all(paths: &[AbsPathBuf]) -> Vec<ProjectManifest> {
        let mut result = paths
            .iter()
            .filter_map(|it| ProjectManifest::discover(it.as_ref()).ok())
            .flatten()
            .collect::<FxHashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        result.sort();
        result
    }

    pub fn manifest_path(&self) -> &ManifestPath {
        match self {
            ProjectManifest::ProjectJson(it) | ProjectManifest::WeslToml(it) => it,
        }
    }
}

impl fmt::Display for ProjectManifest {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        fmt::Display::fmt(self.manifest_path(), f)
    }
}

fn utf8_stdout(cmd: &mut Command) -> anyhow::Result<String> {
    let output = cmd.output().with_context(|| format!("{cmd:?} failed"))?;
    if !output.status.success() {
        match String::from_utf8(output.stderr) {
            Ok(stderr) if !stderr.is_empty() => {
                bail!("{:?} failed, {}\nstderr:\n{}", cmd, output.status, stderr)
            },
            _ => bail!("{:?} failed, {}", cmd, output.status),
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

pub fn command<H>(
    cmd: impl AsRef<ffi::OsStr>,
    working_directory: impl AsRef<path::Path>,
    extra_env: &std::collections::HashMap<String, Option<String>, H>,
) -> process::Command {
    // we are `toolchain::command``
    let mut cmd = process::Command::new(cmd);
    cmd.current_dir(working_directory);
    for env in extra_env {
        match env {
            (key, Some(val)) => cmd.env(key, val),
            (key, None) => cmd.env_remove(key),
        };
    }
    cmd
}
