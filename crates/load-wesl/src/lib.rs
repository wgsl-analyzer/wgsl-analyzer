//! Loads a WESL project
//!

use base_db::{PackageGraph, SourceRoot, SourceRootId, change::Change};
use crossbeam_channel::{Receiver, unbounded};
use ide_db::RootDatabase;
use itertools::Itertools as _;
use project_model::{PackageRoot, ProjectManifest, ProjectWorkspace, WeslConfig};
use rustc_hash::FxHashMap;
use std::{collections::hash_map::Entry, mem, path::Path};
use triomphe::Arc;
use vfs::{
    AbsPath, AbsPathBuf, VfsPath,
    file_set::FileSetConfig,
    loader::{Handle, LoadingProgress},
};
#[derive(Debug)]
pub struct LoadWeslConfig {}

pub fn load_workspace_at(
    root: &Path,
    cargo_config: &WeslConfig,
    load_config: &LoadWeslConfig,
    progress: &(dyn Fn(String) + Sync),
) -> anyhow::Result<(RootDatabase, vfs::Vfs)> {
    let root = AbsPathBuf::assert_utf8(std::env::current_dir()?.join(root));
    let root = ProjectManifest::discover_single(&root)?;
    let workspace = ProjectWorkspace::load(root, cargo_config, progress)?;

    load_workspace(workspace, load_config)
}

pub fn load_workspace(
    ws: ProjectWorkspace,
    load_config: &LoadWeslConfig,
) -> anyhow::Result<(RootDatabase, vfs::Vfs)> {
    let lru_cap = std::env::var("WA_LRU_CAP")
        .ok()
        .and_then(|it| it.parse::<u16>().ok());
    let mut db = RootDatabase::new(lru_cap);

    let vfs = load_workspace_into_db(ws, load_config, &mut db)?;

    Ok((db, vfs))
}

// This variant of `load_workspace` allows deferring the loading of rust-analyzer
// into an existing database, which is useful in certain third-party scenarios,
// now that `salsa` supports extending foreign databases (e.g. `RootDatabase`).
pub fn load_workspace_into_db(
    ws: ProjectWorkspace,
    load_config: &LoadWeslConfig,
    db: &mut RootDatabase,
) -> anyhow::Result<vfs::Vfs> {
    let (sender, receiver) = unbounded();
    let mut vfs = vfs::Vfs::default();
    let mut loader = {
        let loader = vfs_notify::NotifyHandle::spawn(sender);
        Box::new(loader)
    };

    tracing::debug!(?load_config, "LoadCargoConfig");
    let crate_graph = ws.to_package_graph(&mut |path: &AbsPath| {
        let contents = loader.load_sync(path);
        let path = vfs::VfsPath::from(path.to_path_buf());
        vfs.set_file_contents(path.clone(), contents);
        vfs.file_id(&path)
            .and_then(|(file_id, excluded)| (excluded == vfs::FileExcluded::No).then_some(file_id))
    });

    let project_folders = ProjectFolders::new(std::slice::from_ref(&ws), &[], None);
    loader.set_config(vfs::loader::Config {
        load: project_folders.load,
        watch: vec![],
        version: 0,
    });

    load_package_graph_into_db(
        crate_graph,
        project_folders.source_root_config,
        &mut vfs,
        &receiver,
        db,
    );

    Ok(vfs)
}

#[derive(Default)]
pub struct ProjectFolders {
    pub load: Vec<vfs::loader::Entry>,
    pub watch: Vec<usize>,
    pub source_root_config: SourceRootConfig,
}

impl ProjectFolders {
    pub fn new(
        workspaces: &[ProjectWorkspace],
        global_excludes: &[AbsPathBuf],
        user_config_dir_path: Option<&AbsPath>,
    ) -> ProjectFolders {
        let mut result = ProjectFolders::default();
        let mut fsc = FileSetConfig::builder();
        let mut local_filesets = vec![];

        // TODO: Do we need all this complexity?

        // Dedup source roots
        // Depending on the project setup, we can have duplicated source roots, or for example in
        // the case of the rustc workspace, we can end up with two source roots that are almost the
        // same but not quite, like:
        // PackageRoot { is_local: false, include: [AbsPathBuf(".../rust/src/tools/miri/cargo-miri")], exclude: [] }
        // PackageRoot {
        //     is_local: true,
        //     include: [AbsPathBuf(".../rust/src/tools/miri/cargo-miri"), AbsPathBuf(".../rust/build/x86_64-pc-windows-msvc/stage0-tools/x86_64-pc-windows-msvc/release/build/cargo-miri-85801cd3d2d1dae4/out")],
        //     exclude: [AbsPathBuf(".../rust/src/tools/miri/cargo-miri/.git"), AbsPathBuf(".../rust/src/tools/miri/cargo-miri/target")]
        // }
        //
        // The first one comes from the explicit rustc workspace which points to the rustc workspace itself
        // The second comes from the rustc workspace that we load as the actual project workspace
        // These `is_local` differing in this kind of way gives us problems, especially when trying to filter diagnostics as we don't report diagnostics for external libraries.
        // So we need to deduplicate these, usually it would be enough to deduplicate by `include`, but as the rustc example shows here that doesn't work,
        // so we need to also coalesce the includes if they overlap.

        let mut roots: Vec<_> = workspaces
            .iter()
            .flat_map(|ws| ws.to_roots())
            .update(|root| root.include.sort())
            .sorted_by(|a, b| a.include.cmp(&b.include))
            .collect();

        // map that tracks indices of overlapping roots
        let mut overlap_map = FxHashMap::<_, Vec<_>>::default();
        let mut done = false;

        while !mem::replace(&mut done, true) {
            // maps include paths to indices of the corresponding root
            let mut include_to_idx = FxHashMap::default();
            // Find and note down the indices of overlapping roots
            for (index, root) in roots
                .iter()
                .enumerate()
                .filter(|(_, it)| !it.include.is_empty())
            {
                for include in &root.include {
                    match include_to_idx.entry(include) {
                        Entry::Occupied(e) => {
                            overlap_map.entry(*e.get()).or_default().push(index);
                        },
                        Entry::Vacant(e) => {
                            e.insert(index);
                        },
                    }
                }
            }
            for (k, v) in overlap_map.drain() {
                done = false;
                for v in v {
                    let r = mem::replace(
                        &mut roots[v],
                        PackageRoot {
                            is_local: false,
                            include: vec![],
                            exclude: vec![],
                        },
                    );
                    roots[k].is_local |= r.is_local;
                    roots[k].include.extend(r.include);
                    roots[k].exclude.extend(r.exclude);
                }
                roots[k].include.sort();
                roots[k].exclude.sort();
                roots[k].include.dedup();
                roots[k].exclude.dedup();
            }
        }

        for root in roots.into_iter().filter(|it| !it.include.is_empty()) {
            let file_set_roots: Vec<VfsPath> =
                root.include.iter().cloned().map(VfsPath::from).collect();

            let entry = {
                let mut dirs = vfs::loader::Directories::default();
                dirs.extensions.push("wgsl".into());
                dirs.extensions.push("wesl".into());
                dirs.extensions.push("toml".into());
                dirs.include.extend(root.include);
                dirs.exclude.extend(root.exclude);
                for excl in global_excludes {
                    if dirs
                        .include
                        .iter()
                        .any(|incl| incl.starts_with(excl) || excl.starts_with(incl))
                    {
                        dirs.exclude.push(excl.clone());
                    }
                }

                vfs::loader::Entry::Directories(dirs)
            };

            if root.is_local {
                result.watch.push(result.load.len());
            }
            result.load.push(entry);

            if root.is_local {
                local_filesets.push(fsc.len() as u64);
            }
            fsc.add_file_set(file_set_roots)
        }

        for ws in workspaces.iter() {
            let mut file_set_roots: Vec<VfsPath> = vec![];
            let mut entries = vec![];

            if !file_set_roots.is_empty() {
                let entry = vfs::loader::Entry::Files(entries);
                result.watch.push(result.load.len());
                result.load.push(entry);
                local_filesets.push(fsc.len() as u64);
                fsc.add_file_set(file_set_roots)
            }
        }

        if let Some(user_config_path) = user_config_dir_path {
            let ratoml_path = {
                let mut p = user_config_path.to_path_buf();
                p.push("wgsl-analyzer.toml");
                p
            };

            let file_set_roots = vec![VfsPath::from(ratoml_path.to_owned())];
            let entry = vfs::loader::Entry::Files(vec![ratoml_path]);

            result.watch.push(result.load.len());
            result.load.push(entry);
            local_filesets.push(fsc.len() as u64);
            fsc.add_file_set(file_set_roots)
        }

        let fsc = fsc.build();
        result.source_root_config = SourceRootConfig {
            fsc,
            local_filesets,
        };

        result
    }
}

#[derive(Default, Debug)]
pub struct SourceRootConfig {
    pub fsc: FileSetConfig,
    pub local_filesets: Vec<u64>,
}

impl SourceRootConfig {
    pub fn partition(
        &self,
        vfs: &vfs::Vfs,
    ) -> Vec<SourceRoot> {
        self.fsc
            .partition(vfs)
            .into_iter()
            .enumerate()
            .map(|(index, file_set)| {
                let is_local = self.local_filesets.contains(&(index as u64));
                if is_local {
                    SourceRoot::new_local(file_set)
                } else {
                    SourceRoot::new_library(file_set)
                }
            })
            .collect()
    }

    /// Maps local source roots to their parent source roots by bytewise comparing of root paths .
    /// If a `SourceRoot` doesn't have a parent and is local then it is not contained in this mapping but it can be asserted that it is a root `SourceRoot`.
    pub fn source_root_parent_map(&self) -> FxHashMap<SourceRootId, SourceRootId> {
        let roots = self.fsc.roots();

        let mut map = FxHashMap::default();

        // See https://github.com/rust-lang/rust-analyzer/issues/17409
        //
        // We can view the connections between roots as a graph. The problem is
        // that this graph may contain cycles, so when adding edges, it is necessary
        // to check whether it will lead to a cycle.
        //
        // Since we ensure that each node has at most one outgoing edge (because
        // each SourceRoot can have only one parent), we can use a disjoint-set to
        // maintain the connectivity between nodes. If an edge’s two nodes belong
        // to the same set, they are already connected.
        let mut dsu = FxHashMap::default();
        fn find_parent(
            dsu: &mut FxHashMap<u64, u64>,
            id: u64,
        ) -> u64 {
            if let Some(&parent) = dsu.get(&id) {
                let parent = find_parent(dsu, parent);
                dsu.insert(id, parent);
                parent
            } else {
                id
            }
        }

        for (index, (root, root_id)) in roots.iter().enumerate() {
            if !self.local_filesets.contains(root_id)
                || map.contains_key(&SourceRootId(*root_id as u32))
            {
                continue;
            }

            for (root2, root2_id) in roots[..index].iter().rev() {
                if self.local_filesets.contains(root2_id)
                    && root_id != root2_id
                    && root.starts_with(root2)
                {
                    // check if the edge will create a cycle
                    if find_parent(&mut dsu, *root_id) != find_parent(&mut dsu, *root2_id) {
                        map.insert(
                            SourceRootId(*root_id as u32),
                            SourceRootId(*root2_id as u32),
                        );
                        dsu.insert(*root_id, *root2_id);
                    }

                    break;
                }
            }
        }

        map
    }
}

fn load_package_graph_into_db(
    package_graph: PackageGraph,
    source_root_config: SourceRootConfig,
    vfs: &mut vfs::Vfs,
    receiver: &Receiver<vfs::loader::Message>,
    db: &mut RootDatabase,
) {
    let mut analysis_change = Change::default();

    // wait until Vfs has loaded all roots
    for task in receiver {
        match task {
            vfs::loader::Message::Progress { n_done, .. } => {
                if n_done == LoadingProgress::Finished {
                    break;
                }
            },
            vfs::loader::Message::Loaded { files } | vfs::loader::Message::Changed { files } => {
                let _p =
                    tracing::info_span!("load_cargo::load_crate_craph/LoadedChanged").entered();
                for (path, contents) in files {
                    vfs.set_file_contents(path.into(), contents);
                }
            },
        }
    }
    let changes = vfs.take_changes();
    for (_, file) in changes {
        if let vfs::Change::Create(v, _) | vfs::Change::Modify(v, _) = file.change
            && let Ok(text) = String::from_utf8(v)
        {
            let path = vfs.file_path(file.file_id);
            analysis_change.change_file(file.file_id, Some(Arc::new(text)), path.clone());
        }
    }
    let source_roots = source_root_config.partition(vfs);
    analysis_change.set_roots(source_roots);

    analysis_change.set_package_graph(package_graph);

    db.apply_change(analysis_change);
}

// TODO: Port the tests
