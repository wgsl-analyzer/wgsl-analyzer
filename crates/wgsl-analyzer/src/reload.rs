use ide::base_db::input::SourceRoot;
use paths::AbsPathBuf;
use stdx::thread::ThreadIntent;
use tracing::info;
use vfs::file_set::FileSetConfig;

use crate::{global_state::GlobalState, lsp, main_loop::Task, operation_queue::Cause};

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

#[derive(Clone, Debug)]
pub(crate) enum ProjectWorkspace {
    Test,
}

impl ProjectWorkspace {
    /// Returns the roots for the current `ProjectWorkspace`
    /// The return type contains the path and whether or not
    /// the root is a member of the current workspace.
    pub(crate) const fn to_roots() -> Vec<PackageRoot> {
        Vec::new()
    }
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

        // if self.config.linked_or_discovered_projects().is_empty()
        //     && self.config.detached_files().is_empty()
        // {
        //     status.health |= lsp::ext::Health::Warning;
        //     message.push_str("Failed to discover workspace.\n");
        //     message.push_str("Consider adding the `Cargo.toml` of the workspace to the [`linkedProjects`](https://rust-analyzer.github.io/manual.html#rust-analyzer.linkedProjects) setting.\n\n");
        // }
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
                let linked_projects = vec![];
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
                    let workspaces = linked_projects;
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

        let glob_pattern = format!("{}/**/*.{{wgsl,wesl}}", self.config.root_path());

        let registration_options = lsp_types::DidChangeWatchedFilesRegistrationOptions {
            watchers: vec![lsp_types::FileSystemWatcher {
                glob_pattern: lsp_types::GlobPattern::String(glob_pattern),
                kind: None,
            }],
        };
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
        info!("Registered");
    }
}

#[derive(Default, Debug)]
pub(crate) struct SourceRootConfig {
    pub fsc: FileSetConfig,
    pub local_filesets: Vec<usize>,
}

impl SourceRootConfig {
    pub(crate) fn partition(
        &self,
        vfs: &vfs::Vfs,
    ) -> Vec<SourceRoot> {
        //let _p = profile::span("SourceRootConfig::partition");
        self.fsc
            .partition(vfs)
            .into_iter()
            .enumerate()
            .map(|(index, file_set)| {
                let is_local = self.local_filesets.contains(&index);
                if is_local {
                    SourceRoot::new_local(file_set)
                } else {
                    SourceRoot::new_library(file_set)
                }
            })
            .collect()
    }
}
