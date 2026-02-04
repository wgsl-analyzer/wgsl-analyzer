//! Handles lowering of build-system specific workspace information
//! (`wesl metadata` or `wesl-project.json`) into representation stored
//! in the salsa database -- `PackageGraph`.

use std::{collections::VecDeque, fmt, fs, iter, ops::Deref, sync, thread};

use anyhow::Context;
use base_db::{
    Dependency, FileId, LanguagePackageOrigin, PackageDisplayName, PackageGraph, PackageId,
    PackageName, PackageOrigin,
};
use edition::Edition;
use itertools::Itertools;
use paths::{AbsPath, AbsPathBuf, Utf8PathBuf};
use rustc_hash::{FxHashMap, FxHashSet};
use semver::Version;
use tracing::instrument;
use triomphe::Arc;

use crate::{
    InvocationStrategy, ManifestPath, ProjectJson, ProjectManifest,
    project_json::{Package, PackageArrayIdx},
    utf8_stdout,
    wesl_toml::WeslToml,
};
use tracing::{debug, error, info};

pub type FileLoader<'data> = &'data mut dyn for<'path> FnMut(&'path AbsPath) -> Option<FileId>;

/// `PackageRoot` describes a package root folder.
/// Which may be an external dependency, or a member of
/// the current workspace.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct PackageRoot {
    /// Is from the local filesystem and may be edited
    pub is_local: bool,
    /// Directories to include
    pub include: Vec<AbsPathBuf>,
    /// Directories to exclude
    pub exclude: Vec<AbsPathBuf>,
}

#[derive(Clone)]
pub struct ProjectWorkspace {
    pub kind: ProjectWorkspaceKind,
    /// Additional includes to add for the VFS.
    pub extra_includes: Vec<AbsPathBuf>,
}

#[derive(Clone)]
pub enum ProjectWorkspaceKind {
    /// Project workspace was discovered by running `wesl metadata`.
    Wesl {
        /// The workspace as returned by `wesl metadata`.
        wesl: WeslWorkspace,
    },
    /// Project workspace was specified using a `wesl-project.json` file.
    Json(ProjectJson),
    // FIXME: The primary limitation of this approach is that the set of detached files needs to be fixed at the beginning.
    // That's not the end user experience we should strive for.
    // Ideally, you should be able to just open a random detached file in existing WESL projects, and get the basic features working.
    // That needs some changes on the salsa-level though.
    // In particular, we should split the unified PackageGraph (which currently has maximal durability) into proper package graph, and a set of ad hoc roots (with minimal durability).
    // Then, we need to hide the graph behind the queries such that most queries look only at the proper package graph, and fall back to ad hoc roots only if there's no results.
    // After this, we should be able to tweak the logic in reload.rs to add newly opened files, which don't belong to any existing packages, to the set of the detached files.
    /// Project with a set of disjoint files, not belonging to any particular workspace.
    DetachedFile {
        /// The file in question.
        file: ManifestPath,
    },
}

/// Simplified wesl workspace
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WeslWorkspace {
    // packages: Arena<PackageData>,
    pub manifest_path: ManifestPath,
    pub is_virtual_workspace: bool,
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct WeslConfig {
    /// Extra includes to add to the VFS.
    pub extra_includes: Vec<AbsPathBuf>,
    /// Load the project without any dependencies
    pub no_deps: bool,
}

impl fmt::Debug for ProjectWorkspace {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        // Make sure this isn't too verbose.
        let Self {
            kind,
            extra_includes,
        } = self;
        match kind {
            ProjectWorkspaceKind::Wesl { wesl } => formatter
                .debug_struct("Wesl")
                .field("manifest_path", &wesl.manifest_path)
                .field("n_extra_includes", &extra_includes.len())
                .finish(),
            ProjectWorkspaceKind::Json(project) => formatter
                .debug_struct("Json")
                .field("n_packages", &project.n_packages())
                .field("n_extra_includes", &extra_includes.len())
                .finish(),
            ProjectWorkspaceKind::DetachedFile { file } => formatter
                .debug_struct("DetachedFiles")
                .field("file", &file)
                .field("n_extra_includes", &extra_includes.len())
                .finish(),
        }
    }
}

impl ProjectWorkspace {
    pub fn load(
        manifest: ProjectManifest,
        config: &WeslConfig,
        progress: &(dyn Fn(String) + Sync),
    ) -> anyhow::Result<ProjectWorkspace> {
        ProjectWorkspace::load_inner(&manifest, config, progress)
            .with_context(|| format!("Failed to load the project at {manifest}"))
    }

    fn load_inner(
        manifest: &ProjectManifest,
        config: &WeslConfig,
        progress: &(dyn Fn(String) + Sync),
    ) -> anyhow::Result<ProjectWorkspace> {
        let result = match manifest {
            ProjectManifest::ProjectJson(project_json) => {
                let file = fs::read_to_string(project_json)
                    .with_context(|| format!("Failed to read json file {project_json}"))?;
                let data = serde_json::from_str(&file)
                    .with_context(|| format!("Failed to deserialize json file {project_json}"))?;
                let project_location = project_json.parent().to_path_buf();
                let project_json: ProjectJson =
                    ProjectJson::new(Some(project_json.clone()), &project_location, data);
                ProjectWorkspace::load_inline(project_json, config)
            },
            ProjectManifest::WeslToml(wesl_toml) => {
                ProjectWorkspace::load_wesl(wesl_toml, config, progress)?
            },
        };

        Ok(result)
    }

    fn load_wesl(
        wesl_toml: &ManifestPath,
        config: &WeslConfig,
        progress: &(dyn Fn(String) + Sync),
    ) -> Result<ProjectWorkspace, anyhow::Error> {
        progress("discovering sysroot".to_owned());
        let WeslConfig { extra_includes, .. } = config;
        // TODO: Actually load the entire workspace

        progress("querying project metadata".to_owned());

        let wesl_config: WeslToml = toml::from_slice(&std::fs::read(wesl_toml)?)?;

        // TODO: Fetch the metadata about the dependencies

        let wesl = WeslWorkspace {
            manifest_path: wesl_toml.clone(),
            is_virtual_workspace: false,
        };

        Ok(ProjectWorkspace {
            kind: ProjectWorkspaceKind::Wesl { wesl },
            extra_includes: extra_includes.clone(),
        })
    }

    pub fn load_inline(
        project_json: ProjectJson,
        config: &WeslConfig,
    ) -> ProjectWorkspace {
        ProjectWorkspace {
            kind: ProjectWorkspaceKind::Json(project_json),
            extra_includes: config.extra_includes.clone(),
        }
    }

    pub fn load_detached_file(
        detached_file: &ManifestPath,
        config: &WeslConfig,
    ) -> anyhow::Result<ProjectWorkspace> {
        Ok(ProjectWorkspace {
            kind: ProjectWorkspaceKind::DetachedFile {
                file: detached_file.to_owned(),
            },
            extra_includes: config.extra_includes.clone(),
        })
    }

    pub fn load_detached_files(
        detached_files: Vec<ManifestPath>,
        config: &WeslConfig,
    ) -> Vec<anyhow::Result<ProjectWorkspace>> {
        detached_files
            .into_iter()
            .map(|file| Self::load_detached_file(&file, config))
            .collect()
    }

    pub fn manifest_or_root(&self) -> &AbsPath {
        match &self.kind {
            ProjectWorkspaceKind::Wesl { wesl, .. } => &wesl.manifest_path,
            ProjectWorkspaceKind::Json(project) => project.manifest_or_root(),
            ProjectWorkspaceKind::DetachedFile { file, .. } => file,
        }
    }

    pub fn workspace_root(&self) -> &AbsPath {
        match &self.kind {
            ProjectWorkspaceKind::Wesl { wesl, .. } => &wesl.manifest_path,
            ProjectWorkspaceKind::Json(project) => project.project_root(),
            ProjectWorkspaceKind::DetachedFile { file, .. } => file.parent(),
        }
    }

    pub fn manifest(&self) -> Option<&ManifestPath> {
        match &self.kind {
            ProjectWorkspaceKind::Wesl { wesl, .. } => Some(&wesl.manifest_path),
            ProjectWorkspaceKind::Json(project) => project.manifest(),
            ProjectWorkspaceKind::DetachedFile { .. } => None,
        }
    }

    pub fn buildfiles(&self) -> Vec<AbsPathBuf> {
        match &self.kind {
            ProjectWorkspaceKind::Json(project) => project
                .packages()
                .filter_map(|(_, package)| {
                    package.build.as_ref().map(|build| build.build_file.clone())
                })
                .map(|build_file| self.workspace_root().join(build_file))
                .collect(),
            _ => vec![],
        }
    }

    /// Returns the roots for the current `ProjectWorkspace`
    /// The return type contains the path and whether or not
    /// the root is a member of the current workspace
    pub fn to_roots(&self) -> Vec<PackageRoot> {
        match &self.kind {
            ProjectWorkspaceKind::Json(project) => project
                .packages()
                .map(|(_, package)| PackageRoot {
                    is_local: true,
                    include: package
                        .include
                        .iter()
                        .cloned()
                        .chain(self.extra_includes.iter().cloned())
                        .collect(),
                    exclude: package.exclude.clone(),
                })
                .collect::<FxHashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>(),
            ProjectWorkspaceKind::Wesl { wesl } => iter::once(PackageRoot {
                is_local: true,
                include: [wesl.manifest_path.to_path_buf()].to_vec(),
                exclude: Vec::new(),
            })
            .collect(),
            ProjectWorkspaceKind::DetachedFile { file, .. } => iter::once(PackageRoot {
                is_local: true,
                include: vec![file.to_path_buf()],
                exclude: Vec::new(),
            })
            .collect(),
        }
    }

    pub fn n_packages(&self) -> usize {
        match &self.kind {
            ProjectWorkspaceKind::Json(project) => project.n_packages(),
            ProjectWorkspaceKind::Wesl { .. } | ProjectWorkspaceKind::DetachedFile { .. } => 0,
        }
    }

    pub fn to_package_graph(
        &self,
        load: FileLoader<'_>,
    ) -> PackageGraph {
        let _p = tracing::info_span!("ProjectWorkspace::to_package_graph").entered();

        let Self { kind, .. } = self;
        let package_graph = match kind {
            ProjectWorkspaceKind::Json(project) => project_json_to_package_graph(load, project),
            ProjectWorkspaceKind::Wesl { wesl } => wesl_to_package_graph(load, wesl),
            ProjectWorkspaceKind::DetachedFile { file, .. } => {
                detached_file_to_package_graph(load, file)
            },
        };

        package_graph
    }

    pub fn eq_ignore_build_data(
        &self,
        other: &Self,
    ) -> bool {
        let Self { kind, .. } = self;
        let Self { kind: o_kind, .. } = other;
        (match (kind, o_kind) {
            (ProjectWorkspaceKind::Wesl { wesl }, ProjectWorkspaceKind::Wesl { wesl: o_wesl }) => {
                wesl == o_wesl
            },
            (ProjectWorkspaceKind::Json(project), ProjectWorkspaceKind::Json(o_project)) => {
                project == o_project
            },
            (
                ProjectWorkspaceKind::DetachedFile { file },
                ProjectWorkspaceKind::DetachedFile { file: o_file },
            ) => file == o_file,
            _ => return false,
        })
    }

    /// Returns `true` if the project workspace is [`Json`].
    ///
    /// [`Json`]: ProjectWorkspace::Json
    #[must_use]
    pub fn is_json(&self) -> bool {
        matches!(self.kind, ProjectWorkspaceKind::Json { .. })
    }
}

#[instrument(skip_all)]
fn project_json_to_package_graph(
    load: FileLoader<'_>,
    project: &ProjectJson,
) -> PackageGraph {
    let mut result = PackageGraph::default();
    let package_graph = &mut result;

    let idx_to_package_id: FxHashMap<PackageArrayIdx, PackageId> = project
        .packages()
        .filter_map(|(index, package)| Some((index, package, load(&package.root_module)?)))
        .map(
            |(
                index,
                Package {
                    display_name,
                    edition,
                    version,
                    env,
                    repository,
                    is_workspace_member,
                    ..
                },
                file_id,
            )| {
                let package_graph_package_id = package_graph.add_package_root(
                    file_id,
                    *edition,
                    display_name.clone(),
                    version.clone(),
                    if let Some(name) = display_name.clone() {
                        PackageOrigin::Local {
                            repository: repository.clone(),
                            name: Some(name.canonical_name().to_owned()),
                        }
                    } else {
                        PackageOrigin::Local {
                            repository: None,
                            name: None,
                        }
                    },
                );
                debug!(
                    ?package_graph_package_id,
                    package = display_name
                        .as_ref()
                        .map(|name| name.canonical_name().as_str()),
                    "added root to package graph"
                );
                (index, package_graph_package_id)
            },
        )
        .collect();
    debug!(map = ?idx_to_package_id);
    for (from_idx, package) in project.packages() {
        if let Some(&from) = idx_to_package_id.get(&from_idx) {
            for dep in &package.deps {
                if let Some(&to) = idx_to_package_id.get(&dep.package) {
                    add_dep(package_graph, from, dep.name.clone(), to);
                }
            }
        }
    }
    result
}

fn wesl_to_package_graph(
    _load: FileLoader<'_>,
    _wesl: &WeslWorkspace,
) -> PackageGraph {
    PackageGraph::default()
}

fn detached_file_to_package_graph(
    load: FileLoader<'_>,
    detached_file: &ManifestPath,
) -> PackageGraph {
    let _p = tracing::info_span!("detached_file_to_package_graph").entered();
    let mut package_graph = PackageGraph::default();
    let file_id = match load(detached_file) {
        Some(file_id) => file_id,
        None => {
            error!("Failed to load detached file {:?}", detached_file);
            return package_graph;
        },
    };
    let display_name = detached_file
        .file_stem()
        .map(PackageDisplayName::from_canonical_name);
    let detached_file_package = package_graph.add_package_root(
        file_id,
        Edition::CURRENT,
        display_name.clone(),
        None,
        PackageOrigin::Local {
            repository: None,
            name: display_name.map(|n| n.canonical_name().to_owned()),
        },
    );
    package_graph
}

fn add_dep(
    graph: &mut PackageGraph,
    from: PackageId,
    name: PackageName,
    to: PackageId,
) {
    add_dep_inner(graph, from, Dependency::new(name, to))
}

fn add_dep_inner(
    graph: &mut PackageGraph,
    from: PackageId,
    dep: Dependency,
) {
    if let Err(error) = graph.add_dep(from, dep) {
        tracing::warn!("{}", error)
    }
}
