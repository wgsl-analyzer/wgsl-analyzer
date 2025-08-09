//! Handles lowering of build-system specific workspace information
//! (`wesl metadata` or `wesl-project.json`) into representation stored
//! in the salsa database -- `PackageGraph`.

use std::{collections::VecDeque, fmt, fs, iter, ops::Deref, sync, thread};

use anyhow::Context;
use base_db::{
    PackageBuilderId, PackageDisplayName, PackageGraphBuilder, PackageName, PackageOrigin,
    PackageWorkspaceData, DependencyBuilder, Env, LangPackageOrigin, ProcMacroLoadingError,
    ProcMacroPaths, TargetLayoutLoadResult,
};
use intern::{Symbol, sym};
use paths::{AbsPath, AbsPathBuf, Utf8PathBuf};
use rustc_hash::{FxHashMap, FxHashSet};
use semver::Version;
use span::{Edition, FileId};
use tracing::instrument;
use triomphe::Arc;

use crate::{
    InvocationStrategy, ManifestPath, Package2, ProjectJson, ProjectManifest, WeslConfig, WeslSourceWorkspaceConfig, WeslWorkspace,
    project_json::{Package, PackageArrayIdx},
    toolchain_info::{QueryConfig, version},
    utf8_stdout,
    wesl_workspace::{PackageData, RustLibSource, WeslMetadataConfig},
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
    /// The toolchain version used by this workspace.
    pub toolchain: Option<Version>,
    /// Additional includes to add for the VFS.
    pub extra_includes: Vec<AbsPathBuf>,
    /// Set `cfg(test)` for local packages
    pub set_test: bool,
}

#[derive(Clone)]
#[allow(clippy::large_enum_variant)]
pub enum ProjectWorkspaceKind {
    /// Project workspace was discovered by running `wesl metadata` and `rustc --print sysroot`.
    Wesl {
        /// The workspace as returned by `wesl metadata`.
        wesl: WeslWorkspace,
        /// Additional `wesl metadata` error. (only populated if retried fetching via `--no-deps` succeeded).
        error: Option<Arc<anyhow::Error>>,
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
    // //
    /// Project with a set of disjoint files, not belonging to any particular workspace.
    /// Backed by basic sysroot packages for basic completion and highlighting.
    DetachedFile {
        /// The file in question.
        file: ManifestPath,
    },
}

impl fmt::Debug for ProjectWorkspace {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        // Make sure this isn't too verbose.
        let Self {
            kind,
            toolchain,
            extra_includes,
            set_test,
        } = self;
        match kind {
            ProjectWorkspaceKind::Wesl {
                wesl,
                error: _,
            } => f
                .debug_struct("Wesl")
                .field("root", &wesl.workspace_root().file_name())
                .field("n_packages", &wesl.packages().len())
                .field("n_extra_includes", &extra_includes.len())
                .field("toolchain", &toolchain)
                .field("set_test", set_test)
                .finish(),
            ProjectWorkspaceKind::Json(project) => {
                let mut debug_struct = f.debug_struct("Json");
                debug_struct
                    .field("n_packages", &project.n_packages())
                    .field("toolchain", &toolchain)
                    .field("n_extra_includes", &extra_includes.len())
                    .field("set_test", set_test);

                debug_struct.finish()
            },
            ProjectWorkspaceKind::DetachedFile { file } => f
                .debug_struct("DetachedFiles")
                .field("file", &file)
                .field("toolchain", &toolchain)
                .field("n_extra_includes", &extra_includes.len())
                .field("set_test", set_test)
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
        let res = match manifest {
            ProjectManifest::ProjectJson(project_json) => {
                let file = fs::read_to_string(project_json)
                    .with_context(|| format!("Failed to read json file {project_json}"))?;
                let data = serde_json::from_str(&file)
                    .with_context(|| format!("Failed to deserialize json file {project_json}"))?;
                let project_location = project_json.parent().to_path_buf();
                let project_json: ProjectJson =
                    ProjectJson::new(Some(project_json.clone()), &project_location, data);
                ProjectWorkspace::load_inline(project_json, config, progress)
            },
            ProjectManifest::WeslToml(wesl_toml) => {
                ProjectWorkspace::load_wesl(wesl_toml, config, progress)?
            },
        };

        Ok(res)
    }

    fn load_wesl(
        wesl_toml: &ManifestPath,
        config: &WeslConfig,
        progress: &(dyn Fn(String) + Sync),
    ) -> Result<ProjectWorkspace, anyhow::Error> {
        progress("discovering sysroot".to_owned());
        let WeslConfig {
            extra_args,
            extra_env,
            extra_includes,
            no_deps,
            ..
        } = config;
        let workspace_dir = wesl_toml.parent();

        // Resolve the `wesl.toml` to the workspace root as we base the `target` dir off of it.
        let mut cmd = crate::command("wesl", workspace_dir, extra_env);
        cmd.args([
            "locate-project",
            "--workspace",
            "--manifest-path",
            wesl_toml.as_str(),
        ]);
        let wesl_toml = &match utf8_stdout(&mut cmd) {
            Ok(output) => {
                #[derive(serde::Deserialize)]
                struct Root {
                    root: Utf8PathBuf,
                }
                match serde_json::from_str::<Root>(&output) {
                    Ok(object) => ManifestPath::try_from(AbsPathBuf::assert(object.root))
                        .expect("manifest path should be absolute"),
                    Err(e) => {
                        tracing::error!(%e, %wesl_toml, "failed fetching wesl workspace root");
                        wesl_toml.clone()
                    },
                }
            },
            Err(e) => {
                tracing::error!(%e, %wesl_toml, "failed fetching wesl workspace root");
                wesl_toml.clone()
            },
        };
        let workspace_dir = wesl_toml.parent();

        progress("querying project metadata".to_owned());
        let toolchain_config = QueryConfig::WeslRs(&wesl_toml);
        let toolchain = version::get(toolchain_config, extra_env)
            .inspect_err(|e| {
                tracing::error!(%e,
                    "failed fetching toolchain version for {wesl_toml:?} workspace"
                )
            })
            .ok()
            .flatten();

        let target_dir = config
            .target_dir
            .clone()
            .unwrap_or_else(|| workspace_dir.join("target").into());

        // We spawn a bunch of processes to query various information about the workspace's
        // toolchain and sysroot
        // We can speed up loading a bit by spawning all of these processes in parallel (especially
        // on systems were process spawning is delayed)
        let join = thread::scope(|s| {
            let wesl_metadata = s.spawn(|| {
                WeslWorkspace::fetch_metadata(
                    wesl_toml,
                    workspace_dir,
                    &WeslMetadataConfig {
                        extra_args: extra_args.clone(),
                        extra_env: extra_env.clone(),
                        target_dir: target_dir.clone(),
                    },
                    *no_deps,
                    false,
                    progress,
                )
            });
            thread::Result::Ok((
                wesl_metadata.join()?,
            ))
        });

        let (
            wesl_metadata,
        ) = match join {
            Ok(it) => it,
            Err(e) => std::panic::resume_unwind(e),
        };

        let (meta, error) = wesl_metadata.with_context(|| {
            format!("Failed to read Wesl metadata from wesl.toml file {wesl_toml}, {toolchain:?}",)
        })?;
        let wesl = WeslWorkspace::new(meta, wesl_toml.clone(), wesl_config_extra_env, false);
        Ok(ProjectWorkspace {
            kind: ProjectWorkspaceKind::Wesl {
                wesl,
                error: error.map(Arc::new),
            },
            toolchain,
            extra_includes: extra_includes.clone(),
            set_test: *set_test,
        })
    }

    pub fn load_inline(
        mut project_json: ProjectJson,
        config: &WeslConfig,
        progress: &(dyn Fn(String) + Sync),
    ) -> ProjectWorkspace {
        let sysroot_project = project_json.sysroot_project.take();
        let query_config = todo!();
        let toolchain = version::get(query_config, &config.extra_env).ok().flatten();
        let project_root = project_json.project_root();
        let target_dir = config
            .target_dir
            .clone()
            .unwrap_or_else(|| project_root.join("target").into());

        ProjectWorkspace {
            kind: ProjectWorkspaceKind::Json(project_json),
            toolchain,
            extra_includes: config.extra_includes.clone(),
            set_test: config.set_test,
        }
    }

    pub fn load_detached_file(
        detached_file: &ManifestPath,
        config: &WeslConfig,
    ) -> anyhow::Result<ProjectWorkspace> {
        let dir = detached_file.parent();
        let query_config = QueryConfig::WeslRs(detached_file);
        let toolchain = version::get(query_config, &config.extra_env).ok().flatten();
        let target_dir = config
            .target_dir
            .clone()
            .unwrap_or_else(|| dir.join("target").into());

        Ok(ProjectWorkspace {
            kind: ProjectWorkspaceKind::DetachedFile {
                file: detached_file.to_owned(),
            },
            toolchain,
            extra_includes: config.extra_includes.clone(),
            set_test: config.set_test,
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
            ProjectWorkspaceKind::Wesl { wesl, .. } => wesl.manifest_path(),
            ProjectWorkspaceKind::Json(project) => project.manifest_or_root(),
            ProjectWorkspaceKind::DetachedFile { file, .. } => file,
        }
    }

    pub fn workspace_root(&self) -> &AbsPath {
        match &self.kind {
            ProjectWorkspaceKind::Wesl { wesl, .. } => wesl.workspace_root(),
            ProjectWorkspaceKind::Json(project) => project.project_root(),
            ProjectWorkspaceKind::DetachedFile { file, .. } => file.parent(),
        }
    }

    pub fn manifest(&self) -> Option<&ManifestPath> {
        match &self.kind {
            ProjectWorkspaceKind::Wesl { wesl, .. } => Some(wesl.manifest_path()),
            ProjectWorkspaceKind::Json(project) => project.manifest(),
            ProjectWorkspaceKind::DetachedFile { .. } => {
                None
            },
        }
    }

    pub fn buildfiles(&self) -> Vec<AbsPathBuf> {
        match &self.kind {
            ProjectWorkspaceKind::Json(project) => project
                .packages()
                .filter_map(|(_, package)| package.build.as_ref().map(|build| build.build_file.clone()))
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
            ProjectWorkspaceKind::Wesl {
                wesl,
                error: _,
            } => {
                wesl.packages()
                    .map(|pkg| {
                        let is_local = wesl[pkg].is_local;
                        let pkg_root = wesl[pkg].manifest.parent().to_path_buf();
                        let mut include = vec![pkg_root.clone()];
                        let mut exclude = vec![pkg_root.join(".git")];
                        if is_local {
                            include.extend(self.extra_includes.iter().cloned());

                            exclude.push(pkg_root.join("target"));
                        } else {
                            exclude.push(pkg_root.join("tests"));
                            exclude.push(pkg_root.join("examples"));
                            exclude.push(pkg_root.join("benches"));
                        }
                        PackageRoot {
                            is_local,
                            include,
                            exclude,
                        }
                    })
                    .collect()
            },
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
            ProjectWorkspaceKind::Wesl { wesl, .. } => {
                wesl.packages().len()
            },
            ProjectWorkspaceKind::DetachedFile { .. } => 0,
        }
    }

    pub fn to_package_graph(
        &self,
        load: FileLoader<'_>,
        extra_env: &FxHashMap<String, Option<String>>,
    ) -> (PackageGraphBuilder) {
        let _p = tracing::info_span!("ProjectWorkspace::to_package_graph").entered();

        let Self {
            kind,
            ..
        } = self;
        let (package_graph, proc_macros) = match kind {
            ProjectWorkspaceKind::Json(project) => project_json_to_package_graph(
                load,
                project,
                extra_env,
                self.set_test,
                false,
                package_ws_data,
            ),
            ProjectWorkspaceKind::Wesl {
                wesl,
                error: _,
            } => wesl_to_package_graph(
                load,
                wesl,
                self.set_test,
            ),
            ProjectWorkspaceKind::DetachedFile { file, .. } => detached_file_to_package_graph(
                load,
                file,
                self.set_test,
                package_ws_data,
            ),
        };

        package_graph
    }

    pub fn eq_ignore_build_data(
        &self,
        other: &Self,
    ) -> bool {
        let Self {
            kind,
            sysroot,
            rustc_cfg,
            toolchain,
            target_layout,
            cfg_overrides,
            ..
        } = self;
        let Self {
            kind: o_kind,
            sysroot: o_sysroot,
            rustc_cfg: o_rustc_cfg,
            toolchain: o_toolchain,
            target_layout: o_target_layout,
            cfg_overrides: o_cfg_overrides,
            ..
        } = other;
        (match (kind, o_kind) {
            (
                ProjectWorkspaceKind::Wesl {
                    wesl,
                    rustc,
                    build_scripts: _,
                    error: _,
                },
                ProjectWorkspaceKind::Wesl {
                    wesl: o_wesl,
                    rustc: o_rustc,
                    build_scripts: _,
                    error: _,
                },
            ) => wesl == o_wesl && rustc == o_rustc,
            (ProjectWorkspaceKind::Json(project), ProjectWorkspaceKind::Json(o_project)) => {
                project == o_project
            },
            (
                ProjectWorkspaceKind::DetachedFile { file },
                ProjectWorkspaceKind::DetachedFile { file: o_file },
            ) => file == o_file,
            _ => return false,
        }) && sysroot == o_sysroot
            && rustc_cfg == o_rustc_cfg
            && toolchain == o_toolchain
            && target_layout == o_target_layout
            && cfg_overrides == o_cfg_overrides
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
    extra_env: &FxHashMap<String, Option<String>>,
    set_test: bool,
    package_ws_data: Arc<PackageWorkspaceData>,
) -> (PackageGraphBuilder) {
    let mut res = (PackageGraphBuilder::default(), ProcMacroPaths::default());
    let (package_graph, proc_macros) = &mut res;
    let project_root = Arc::new(project.project_root().to_path_buf());
    let idx_to_package_id: FxHashMap<PackageArrayIdx, _> = project
        .packages()
        .filter_map(|(idx, package)| Some((idx, package, load(&package.root_module)?)))
        .map(
            |(
                idx,
                Package {
                    display_name,
                    edition,
                    version,
                    env,
                    repository,
                    ..
                },
                file_id,
            )| {
                let env = env.clone().into_iter().collect();
                let package_graph_package_id = package_graph.add_package_root(
                    file_id,
                    *edition,
                    display_name.clone(),
                    version.clone(),
                    None,
                    env,
                    package_ws_data.clone(),
                );
                debug!(
                    ?package_graph_package_id,
                    package = display_name
                        .as_ref()
                        .map(|name| name.canonical_name().as_str()),
                    "added root to package graph"
                );
                (idx, package_graph_package_id)
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
    res
}

fn wesl_to_package_graph(
    load: FileLoader<'_>,
    wesl: &WeslWorkspace,
    set_test: bool,
    package_ws_data: Arc<PackageWorkspaceData>,
) -> (PackageGraphBuilder, ProcMacroPaths) {
    let _p = tracing::info_span!("wesl_to_package_graph").entered();
    let mut res = (PackageGraphBuilder::default());
    let (package_graph) = &mut res;

    // Mapping of a package to its library target
    let mut pkg_to_lib_package = FxHashMap::default();
    let mut pkg_packages = FxHashMap::default();

    // Next, create packages for each package, target pair
    for pkg in wesl.packages() {
        let mut lib_tgt = None;

        // Set deps to the core, std and to the lib target of the current package
        for &(from, kind) in pkg_packages.get(&pkg).into_iter().flatten() {
            // Add dep edge of all targets to the package's lib target
            if let Some((to, name)) = lib_tgt.clone() {
                if to != from {
                    // For root projects with dashes in their name,
                    // wesl metadata does not do any normalization,
                    // so we do it ourselves currently
                    let name = PackageName::normalize_dashes(&name);
                    add_dep(package_graph, from, name, to);
                }
            }
        }
    }

    let mut delayed_dev_deps = vec![];

    // Now add a dep edge from all targets of upstream to the lib
    // target of downstream.
    for pkg in wesl.packages() {
        for dep in &wesl[pkg].dependencies {
            let Some(&to) = pkg_to_lib_package.get(&dep.pkg) else {
                continue;
            };
            let Some(targets) = pkg_packages.get(&pkg) else {
                continue;
            };

            let name = PackageName::new(&dep.name).unwrap();
            for &(from, kind) in targets {
                add_dep(package_graph, from, name.clone(), to)
            }
        }
    }

    for (from, name, to) in delayed_dev_deps {
        add_dep(package_graph, from, name, to);
    }
    res
}

fn detached_file_to_package_graph(
    load: FileLoader<'_>,
    detached_file: &ManifestPath,
    set_test: bool,
    package_ws_data: Arc<PackageWorkspaceData>,
) -> (PackageGraphBuilder, ProcMacroPaths) {
    let _p = tracing::info_span!("detached_file_to_package_graph").entered();
    let mut package_graph = PackageGraphBuilder::default();
    let file_id = match load(detached_file) {
        Some(file_id) => file_id,
        None => {
            error!("Failed to load detached file {:?}", detached_file);
            return (package_graph, FxHashMap::default());
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
        None,
        Env::default(),
        PackageOrigin::Local {
            repo: None,
            name: display_name.map(|n| n.canonical_name().to_owned()),
        },
        false,
        Arc::new(detached_file.parent().to_path_buf()),
        package_ws_data,
    );
    (package_graph, FxHashMap::default())
}

fn add_target_package_root(
    package_graph: &mut PackageGraphBuilder,
    proc_macros: &mut ProcMacroPaths,
    wesl: &WeslWorkspace,
    pkg: &PackageData,
    file_id: FileId,
    wesl_name: &str,
    origin: PackageOrigin,
    package_ws_data: Arc<PackageWorkspaceData>,
) -> PackageBuilderId {
    let edition = pkg.edition;
    let mut env = wesl.env().clone();
    let package_id = package_graph.add_package_root(
        file_id,
        edition,
        Some(PackageDisplayName::from_canonical_name(wesl_name)),
        Some(pkg.version.to_string()),
        env,
        origin,
        package_ws_data,
    );
    package_id
}

#[derive(Default, Debug)]
struct SysrootPublicDeps {
    deps: Vec<(PackageName, PackageBuilderId, bool)>,
}

impl SysrootPublicDeps {
    /// Makes `from` depend on the public sysroot packages.
    fn add_to_package_graph(
        &self,
        package_graph: &mut PackageGraphBuilder,
        from: PackageBuilderId,
    ) {
        for (name, package, prelude) in &self.deps {
            add_dep_with_prelude(package_graph, from, name.clone(), *package, *prelude, true);
        }
    }
}

fn extend_package_graph_with_sysroot(
    package_graph: &mut PackageGraphBuilder,
    mut sysroot_package_graph: PackageGraphBuilder,
    mut sysroot_proc_macros: ProcMacroPaths,
) -> (SysrootPublicDeps, Option<PackageBuilderId>) {
    let mut pub_deps = vec![];
    let mut libproc_macro = None;
    for cid in sysroot_package_graph.iter() {
        if let PackageOrigin::Lang(lang_package) = sysroot_package_graph[cid].basic.origin {
            match lang_package {
                LangPackageOrigin::Test
                | LangPackageOrigin::Alloc
                | LangPackageOrigin::Core
                | LangPackageOrigin::Std => pub_deps.push((
                    PackageName::normalize_dashes(&lang_package.to_string()),
                    cid,
                    !matches!(lang_package, LangPackageOrigin::Test | LangPackageOrigin::Alloc),
                )),
                LangPackageOrigin::ProcMacro => libproc_macro = Some(cid),
                LangPackageOrigin::Other => (),
            }
        }
    }

    let mut marker_set = vec![];
    for &(_, cid, _) in pub_deps.iter() {
        marker_set.extend(sysroot_package_graph.transitive_deps(cid));
    }
    if let Some(cid) = libproc_macro {
        marker_set.extend(sysroot_package_graph.transitive_deps(cid));
    }

    marker_set.sort();
    marker_set.dedup();

    // Remove all packages except the ones we are interested in to keep the sysroot graph small.
    let removed_mapping = sysroot_package_graph.remove_packages_except(&marker_set);
    let mapping = package_graph.extend(sysroot_package_graph, &mut sysroot_proc_macros);

    // Map the id through the removal mapping first, then through the package graph extension mapping.
    pub_deps.iter_mut().for_each(|(_, cid, _)| {
        *cid = mapping[&removed_mapping[cid.into_raw().into_u32() as usize].unwrap()]
    });
    if let Some(libproc_macro) = &mut libproc_macro {
        *libproc_macro =
            mapping[&removed_mapping[libproc_macro.into_raw().into_u32() as usize].unwrap()];
    }

    (SysrootPublicDeps { deps: pub_deps }, libproc_macro)
}

fn add_dep(
    graph: &mut PackageGraphBuilder,
    from: PackageBuilderId,
    name: PackageName,
    to: PackageBuilderId,
) {
    add_dep_inner(graph, from, DependencyBuilder::new(name, to))
}

fn add_dep_with_prelude(
    graph: &mut PackageGraphBuilder,
    from: PackageBuilderId,
    name: PackageName,
    to: PackageBuilderId,
    prelude: bool,
    sysroot: bool,
) {
    add_dep_inner(
        graph,
        from,
        DependencyBuilder::with_prelude(name, to, prelude, sysroot),
    )
}

fn add_dep_inner(
    graph: &mut PackageGraphBuilder,
    from: PackageBuilderId,
    dep: DependencyBuilder,
) {
    if let Err(err) = graph.add_dep(from, dep) {
        tracing::warn!("{}", err)
    }
}
