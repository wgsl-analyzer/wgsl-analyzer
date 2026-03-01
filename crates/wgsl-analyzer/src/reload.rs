use base_db::{PackageGraph, change::Change};
use ide::base_db::input::SourceRoot;
use itertools::Itertools;
use load_wesl::ProjectFolders;
use lsp_types::FileSystemWatcher;
use paths::AbsPathBuf;
use project_model::{ProjectWorkspace, ProjectWorkspaceKind, WeslConfig};
use stdx::thread::ThreadIntent;
use tracing::info;
use triomphe::Arc;
use vfs::{AbsPath, file_set::FileSetConfig};

use crate::{
    config::{Config, FilesWatcher},
    global_state::{FetchWorkspaceResponse, GlobalState},
    lsp,
    main_loop::Task,
    operation_queue::Cause,
};

/// `PackageRoot` describes a package root folder.
/// Which may be an external dependency, or a member of
/// the current workspace.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct PackageRoot {
    /// Is from the local filesystem and may be edited.
    pub is_local: bool,
    pub include: Vec<AbsPathBuf>,
    pub exclude: Vec<AbsPathBuf>,
}

#[derive(Debug)]
pub(crate) enum ProjectWorkspaceProgress {
    Begin,
    Report(String),
    End(Vec<anyhow::Result<ProjectWorkspace>>, bool),
}

impl GlobalState {
    /// Is the server quiescent?
    ///
    /// This indicates that we've fully loaded the projects and
    /// are ready to do semantic work.
    pub(crate) const fn is_quiescent(&self) -> bool {
        self.vfs_done
            && !self.fetch_workspaces_queue.operation_in_progress()
            // && !self.fetch_build_data_queue.operation_in_progress()
            // && !self.fetch_proc_macros_queue.operation_in_progress()
            && !self.discover_workspace_queue.operation_in_progress()
            && self.vfs_progress_config_version >= self.vfs_config_version
    }

    /// Is the server ready to respond to analysis dependent LSP requests?
    ///
    /// Unlike `is_quiescent`, this returns false when we're indexing
    /// the project, because we're holding the salsa lock and cannot
    /// respond to LSP requests that depend on salsa data.
    const fn is_fully_ready(&self) -> bool {
        self.is_quiescent() && !self.prime_caches_queue.operation_in_progress()
    }

    pub(crate) fn current_status(&self) -> lsp::extensions::ServerStatusParameters {
        let mut status = lsp::extensions::ServerStatusParameters {
            health: lsp::extensions::Health::Ok,
            quiescent: self.is_fully_ready(),
            message: None,
        };
        let mut message = String::new();

        // if !self.config.cargo_autoreload_config(None)
        //     && self.is_quiescent()
        //     && self.fetch_workspaces_queue.op_requested()
        //     && self.config.discover_workspace_config().is_none()
        // {
        //     status.health |= lsp::ext::Health::Warning;
        //     message.push_str("Auto-reloading is disabled and the workspace has changed, a manual workspace reload is required.\n\n");
        // }

        // if self.fetch_build_data_error().is_err() {
        //     status.health |= lsp::ext::Health::Warning;
        //     message.push_str("Failed to run build scripts of some packages.\n\n");
        // }
        // if let Some(error) = &self.config_errors {
        //     status.health |= lsp::ext::Health::Warning;
        //     format_to!(message, "{error}\n");
        // }
        // if let Some(error) = &self.last_flycheck_error {
        //     status.health |= lsp::ext::Health::Warning;
        //     message.push_str(error);
        //     message.push('\n');
        // }

        // TODO: Remove this, we should survive wgsl without a root
        if self.config.discovered_projects().is_empty() {
            status.health |= lsp::extensions::Health::Warning;
            message.push_str("Failed to discover workspace.\n");
            message.push_str("Consider adding a `wesl.toml` to your workspace.\n\n");
        }
        // if self.fetch_workspace_error().is_err() {
        //     status.health |= lsp::ext::Health::Error;
        //     message.push_str("Failed to load workspaces.");

        //     if self.config.has_linked_projects() {
        //         message.push_str(
        //             "`rust-analyzer.linkedProjects` have been specified, which may be incorrect. Specified project paths:\n",
        //         );
        //         message
        //             .push_str(&format!("    {}", self.config.linked_manifests().format("\n    ")));
        //         if self.config.has_linked_project_jsons() {
        //             message.push_str("\nAdditionally, one or more project jsons are specified")
        //         }
        //     }
        //     message.push_str("\n\n");
        // }

        if !message.is_empty() {
            status.message = Some(message.trim_end().to_owned());
        }

        status
    }

    #[expect(clippy::needless_pass_by_value, reason = "")]
    #[expect(clippy::needless_pass_by_ref_mut, reason = "")]
    pub(crate) fn fetch_workspaces(
        &mut self,
        cause: Cause,
        path: Option<AbsPathBuf>,
        force_crate_graph_reload: bool,
    ) {
        info!(%cause, "will fetch workspaces");

        self.task_pool
            .handle
            .spawn_with_sender(ThreadIntent::Worker, {
                let linked_projects = self.config.discovered_projects().to_vec();
                let wesl_config = WeslConfig::default(); // Could be loaded from self.config if we had that config key

                let is_quiescent = !(self.discover_workspace_queue.operation_in_progress()
                    || self.vfs_progress_config_version < self.vfs_config_version
                    || !self.vfs_done);

                move |sender| {
                    let progress = {
                        let sender = sender.clone();
                        move |message| {
                            sender
                                .send(Task::FetchWorkspace(ProjectWorkspaceProgress::Report(
                                    message,
                                )))
                                .unwrap();
                        }
                    };

                    sender
                        .send(Task::FetchWorkspace(ProjectWorkspaceProgress::Begin))
                        .unwrap();
                    let workspaces: Vec<_> = linked_projects
                        .iter()
                        .map(|manifest| {
                            project_model::ProjectWorkspace::load(manifest, &wesl_config, &progress)
                        })
                        .collect();
                    eprintln!("{workspaces:?}");

                    // TODO: Do we need to deduplicate?

                    info!(?workspaces, "did fetch workspaces");
                    sender
                        .send(Task::FetchWorkspace(ProjectWorkspaceProgress::End(
                            workspaces,
                            force_crate_graph_reload,
                        )))
                        .unwrap();
                }
            });
    }

    pub(crate) fn switch_workspaces(
        &mut self,
        cause: &Cause,
    ) {
        let _p = tracing::info_span!("GlobalState::switch_workspaces").entered();
        tracing::info!(%cause, "will switch workspaces");

        let Some(FetchWorkspaceResponse {
            workspaces,
            force_crate_graph_reload,
        }) = self.fetch_workspaces_queue.last_operation_result()
        else {
            return;
        };
        let switching_from_empty_workspace = self.workspaces.is_empty();

        info!(%cause, ?force_crate_graph_reload, %switching_from_empty_workspace);
        if self.fetch_workspace_error().is_err() && !switching_from_empty_workspace {
            if *force_crate_graph_reload {
                self.recreate_package_graph(cause, false);
            }
            // It only makes sense to switch to a partially broken workspace
            // if we don't have any workspace at all yet.
            return;
        }

        let workspaces = workspaces
            .iter()
            .filter_map(|workspace| workspace.as_ref().ok().cloned())
            .collect::<Vec<_>>();

        let same_workspaces = workspaces.len() == self.workspaces.len()
            && workspaces
                .iter()
                .zip(self.workspaces.iter())
                .all(|(discovered, current)| discovered.eq_ignore_build_data(current));

        if same_workspaces {
            if switching_from_empty_workspace {
                // Switching from empty to empty is a no-op
                return;
            }
            if *force_crate_graph_reload {
                self.recreate_package_graph(cause, switching_from_empty_workspace);
            }
            // Unchanged workspaces
            return;
        }
        self.workspaces = (workspaces).into();

        if matches!(self.config.files().watcher, FilesWatcher::Client) {
            let filter = self
                .workspaces
                .iter()
                .flat_map(project_model::ProjectWorkspace::to_roots)
                .filter(|root| root.is_local)
                .map(|root| root.include);

            let watchers: Vec<FileSystemWatcher> = if self
                .config
                .did_change_watched_files_relative_pattern_support()
            {
                // When relative patterns are supported by the client, prefer using them
                filter
                    .flat_map(|include| {
                        include.into_iter().flat_map(|base| {
                            [
                                (base.clone(), "**/*.{wgsl,wesl}"),
                                (base.clone(), "**/wesl.toml"),
                                (base, "**/wesl-project.json"),
                            ]
                        })
                    })
                    .map(|(base, pat)| lsp_types::FileSystemWatcher {
                        glob_pattern: lsp_types::GlobPattern::Relative(
                            lsp_types::RelativePattern {
                                base_uri: lsp_types::OneOf::Right(
                                    lsp_types::Url::from_file_path(base).unwrap(),
                                ),
                                pattern: pat.to_owned(),
                            },
                        ),
                        kind: None,
                    })
                    .collect()
            } else {
                // When they're not, integrate the base to make them into absolute patterns
                filter
                    .flat_map(|include| {
                        include.into_iter().flat_map(|base| {
                            [
                                format!("{base}/**/*.{{wgsl,wesl}}"),
                                format!("{base}/**/wesl.toml"),
                                format!("{base}/**/wesl-project.json"),
                            ]
                        })
                    })
                    .map(|glob_pattern| lsp_types::FileSystemWatcher {
                        glob_pattern: lsp_types::GlobPattern::String(glob_pattern),
                        kind: None,
                    })
                    .collect()
            };

            let registration_options =
                lsp_types::DidChangeWatchedFilesRegistrationOptions { watchers };
            let registration = lsp_types::Registration {
                id: "workspace/didChangeWatchedFiles".to_owned(),
                method: "workspace/didChangeWatchedFiles".to_owned(),
                register_options: Some(serde_json::to_value(registration_options).unwrap()),
            };
            self.send_request::<lsp_types::request::RegisterCapability>(
                lsp_types::RegistrationParams {
                    registrations: vec![registration],
                },
                |_, _| (),
            );
        }

        let files_config = self.config.files();
        let project_folders = ProjectFolders::new(
            &self.workspaces,
            &[],
            Config::user_config_dir_path().as_deref(),
        );

        let watch = match files_config.watcher {
            FilesWatcher::Client => vec![],
            FilesWatcher::Server => project_folders.watch,
        };
        self.vfs_config_version += 1;
        self.loader.handle.set_config(vfs::loader::Config {
            load: project_folders.load,
            watch,
            version: self.vfs_config_version,
        });
        self.source_root_config = project_folders.source_root_config;
        // self.local_roots_parent_map = Arc::new(self.source_root_config.source_root_parent_map());
        info!(?cause, "recreating the package graph");
        self.recreate_package_graph(cause, switching_from_empty_workspace);
        info!("did switch workspaces");
    }

    fn recreate_package_graph(
        &mut self,
        cause: &str,
        initial_build: bool,
    ) {
        eprintln!("AAAAAAAAAAAAAAAAAAAAAAAAAAA");
        info!(?cause, "Building package graph");
        self.report_progress(
            "Building package graph",
            &crate::lsp::utilities::Progress::Begin,
            None,
            None,
            None,
        );

        // crate graph construction relies on these paths, record them so when one of them gets
        // deleted or created we trigger a reconstruction of the crate graph
        self.package_graph_file_dependencies.clear();
        self.detached_files = self
            .workspaces
            .iter()
            .filter_map(|workspace| match &workspace.kind {
                ProjectWorkspaceKind::DetachedFile { file, .. } => Some(file.clone()),
                ProjectWorkspaceKind::Wesl { .. } | ProjectWorkspaceKind::Json(_) => None,
            })
            .collect();

        let (package_graph) = {
            // Create package graph from all the workspaces
            let vfs = &self.vfs.read().0;
            let mut load = |path: &AbsPath| {
                let vfs_path = vfs::VfsPath::from(path.to_path_buf());
                self.package_graph_file_dependencies
                    .insert(vfs_path.clone());
                let file_id = vfs.file_id(&vfs_path);
                let (file_id, excluded) = file_id?;
                (excluded == vfs::FileExcluded::No).then_some(file_id)
            };

            let mut package_graph = PackageGraph::default();
            for workspace in self.workspaces.iter() {
                let (other) = workspace.to_package_graph(&mut load);
                package_graph.extend(other);
            }
            package_graph.shrink_to_fit();
            package_graph
        };
        let mut change = Change::default();
        change.set_package_graph(package_graph);
        if initial_build {
            self.analysis_host.apply_change(change);
            self.finish_loading_package_graph();
        }

        self.report_progress(
            "Building package graph",
            &crate::lsp::utilities::Progress::End,
            None,
            None,
            None,
        );
    }

    pub(crate) fn finish_loading_package_graph(&mut self) {
        self.process_changes();
    }
}
