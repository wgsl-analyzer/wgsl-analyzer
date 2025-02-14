use paths::AbsPathBuf;
use tracing::info;
use vfs::file_set::FileSetConfig;

use ide::base_db::input::SourceRoot;

use crate::{global_state::GlobalState, main_loop::Task};

/// `PackageRoot` describes a package root folder.
/// Which may be an external dependency, or a member of
/// the current workspace.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct PackageRoot {
    /// Is from the local filesystem and may be edited
    pub is_local: bool,
    pub include: Vec<AbsPathBuf>,
    pub exclude: Vec<AbsPathBuf>,
}

#[derive(Clone, Debug)]
pub enum ProjectWorkspace {
    Test,
}

impl ProjectWorkspace {
    /// Returns the roots for the current `ProjectWorkspace`
    /// The return type contains the path and whether or not
    /// the root is a member of the current workspace
    pub fn to_roots(&self) -> Vec<PackageRoot> {
        Vec::new()
    }
}

#[derive(Debug)]
pub enum ProjectWorkspaceProgress {
    Begin,
    Report(String),
    End(Vec<anyhow::Result<ProjectWorkspace>>),
}

impl GlobalState {
    pub fn fetch_workspaces(&mut self) {
        self.task_pool.handle.spawn_with_sender(move |sender| {
            sender
                .send(Task::FetchWorkspace(ProjectWorkspaceProgress::Begin))
                .unwrap();
            let workspaces = vec![Ok(ProjectWorkspace::Test)];
            sender
                .send(Task::FetchWorkspace(ProjectWorkspaceProgress::End(
                    workspaces,
                )))
                .unwrap();
        });
    }

    pub fn switch_workspaces(&mut self) {
        let glob_pattern = format!("{}/**/*.wgsl", self.config.root_path.display());

        let registration_options = lsp_types::DidChangeWatchedFilesRegistrationOptions {
            watchers: vec![lsp_types::FileSystemWatcher {
                glob_pattern: lsp_types::GlobPattern::String(glob_pattern),
                kind: None,
            }],
        };
        let registration = lsp_types::Registration {
            id: "workspace/didChangeWatchedFiles".to_string(),
            method: "workspace/didChangeWatchedFiles".to_string(),
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
pub struct SourceRootConfig {
    pub fsc: FileSetConfig,
    pub local_filesets: Vec<usize>,
}

impl SourceRootConfig {
    pub(crate) fn partition(&self, vfs: &vfs::Vfs) -> Vec<SourceRoot> {
        //let _p = profile::span("SourceRootConfig::partition");
        self.fsc
            .partition(vfs)
            .into_iter()
            .enumerate()
            .map(|(idx, file_set)| {
                let is_local = self.local_filesets.contains(&idx);
                if is_local {
                    SourceRoot::new_local(file_set)
                } else {
                    SourceRoot::new_library(file_set)
                }
            })
            .collect()
    }
}
