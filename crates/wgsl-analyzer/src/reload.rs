use hir::database::DefDatabase as _;
use ide::base_db::input::SourceRoot;
use lsp_types::FileSystemWatcher;
use paths::AbsPathBuf;
use project_model::PackageRoot;
use salsa::Durability;
use stdx::thread::ThreadIntent;
use tracing::info;
use triomphe::Arc;
use vfs::{VfsPath, file_set::FileSetConfig};

use crate::{
    config::Config, discover::DiscoverArgument, global_state::GlobalState, lsp, main_loop::Task,
    operation_queue::Cause,
};

impl GlobalState {
    /// Is the server quiescent?
    ///
    /// This indicates that we've fully loaded the projects and
    /// are ready to do semantic work.
    pub(crate) const fn is_quiescent(&self) -> bool {
        self.vfs_done
            && self.load_package_jobs_active == 0
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

    pub(crate) fn update_configuration(
        &mut self,
        config: Config,
    ) {
        let _p = tracing::info_span!("GlobalState::update_configuration").entered();
        let old_config = std::mem::replace(&mut self.config, Arc::new(config));

        if self.config.extensions() != old_config.extensions() {
            self.analysis_host
                .raw_database_mut()
                .set_extensions_with_durability(self.config.extensions(), Durability::MEDIUM);
        }
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

        if !message.is_empty() {
            status.message = Some(message.trim_end().to_owned());
        }

        status
    }

    pub fn request_project_discover(
        &self,
        discover: DiscoverArgument,
        cause: &Cause,
    ) {
        info!(%cause, "will discover project");

        // Don't try to analyze the projects when opening a stray file.
        // Libraries are handled by the fact that they are reachable
        // from one of the discovered projects in the current workspace.
        if !self.config.is_in_workspace(&discover.path) {
            return;
        }

        self.task_pool
            .handle
            .spawn_with_sender(ThreadIntent::Worker, move |sender| {
                let _p = tracing::info_span!("GlobalState::request_project_discover").entered();
                tracing::debug!(?discover.path, "discovering projects");
                sender.send(Task::DiscoverProject(discover)).unwrap();
            });
    }

    /// Cleans up the discovered packages.
    pub(crate) fn refresh_packages(&self) {
        let mut packages = self.packages.write();
        packages.retain(|_, project| self.config.is_in_workspace(&project.manifest));
        packages.retain_referenced();

        let workspace_roots = self.config.workspace_roots();
        if packages.is_empty() && !workspace_roots.is_empty() {
            tracing::warn!("no projects in {:?}", &workspace_roots);
        }
    }

    pub(crate) fn switch_workspaces(
        &mut self,
        cause: &Cause,
    ) {
        let _p = tracing::info_span!("GlobalState::switch_workspaces").entered();
        tracing::info!(%cause, "will switch workspaces");

        let package_roots = {
            let package_graph = self.packages.read();
            package_graph
                .iter()
                .map(|(_, package)| package.to_root())
                .collect::<Vec<_>>()
        };

        let watchers = to_file_system_watchers(
            &package_roots,
            self.config
                .did_change_watched_files_relative_pattern_support(),
        );

        let registration_options = lsp_types::DidChangeWatchedFilesRegistrationOptions { watchers };
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

        let (load, source_root_config) = to_load_and_source_root_config(package_roots);

        self.vfs_config_version += 1;
        self.loader.handle.set_config(vfs::loader::Config {
            load,
            // We rely on client side watching instead of making the vfs loader watch files
            watch: Vec::new(),
            version: self.vfs_config_version,
        });

        self.source_root_config = source_root_config;

        info!("did switch workspaces");
    }
}

pub(crate) fn to_load_and_source_root_config(
    package_roots: Vec<PackageRoot>
) -> (Vec<vfs::loader::Entry>, SourceRootConfig) {
    let mut load: Vec<vfs::loader::Entry> = Vec::new();
    let mut fsc = FileSetConfig::builder();
    let mut local_filesets = vec![];
    for root in package_roots {
        load.push(vfs::loader::Entry::Directories(vfs::loader::Directories {
            extensions: ["wgsl".to_owned(), "wesl".to_owned(), "toml".to_owned()].to_vec(),
            include: root.include,
            exclude: root.exclude,
        }));
        load.push(vfs::loader::Entry::Files(root.include_files));

        if root.origin.is_local() {
            local_filesets.push(fsc.len());
        }
        fsc.add_file_set([VfsPath::from(root.manifest.parent().to_path_buf())].to_vec());
    }
    let source_root_config = SourceRootConfig {
        fsc: fsc.build(),
        local_filesets,
    };
    (load, source_root_config)
}

fn to_file_system_watchers(
    package_roots: &[PackageRoot],
    supports_relative_patterns: bool,
) -> Vec<FileSystemWatcher> {
    if supports_relative_patterns {
        // When relative patterns are supported by the client, prefer using them
        package_roots
            .iter()
            .filter(|root| root.origin.is_local())
            .flat_map(|root| {
                root.include
                    .iter()
                    .flat_map(|base: &AbsPathBuf| {
                        [
                            (base.clone(), "**/*.wgsl"),
                            (base.clone(), "**/*.wesl"),
                            (base.clone(), "**/wesl.toml"),
                        ]
                    })
                    .chain(root.include_files.iter().filter_map(|file| {
                        let parent = file.parent()?.to_path_buf();
                        let file_name = file.file_name()?;
                        Some((parent, file_name))
                    }))
            })
            .map(|(base, pat)| lsp_types::FileSystemWatcher {
                glob_pattern: lsp_types::GlobPattern::Relative(lsp_types::RelativePattern {
                    base_uri: lsp_types::OneOf::Right(
                        lsp_types::Url::from_file_path(base).unwrap(),
                    ),
                    pattern: pat.to_owned(),
                }),
                kind: None,
            })
            .collect()
    } else {
        // When they're not, integrate the base to make them into absolute patterns
        package_roots
            .iter()
            .filter(|root| root.origin.is_local())
            .flat_map(|root| {
                root.include
                    .iter()
                    .flat_map(|base| {
                        [
                            format!("{base}/**/*.wgsl"),
                            format!("{base}/**/*.wesl"),
                            format!("{base}/**/wesl.toml"),
                        ]
                    })
                    .chain(
                        root.include_files
                            .iter()
                            .map(std::string::ToString::to_string),
                    )
            })
            .map(|glob_pattern| lsp_types::FileSystemWatcher {
                glob_pattern: lsp_types::GlobPattern::String(glob_pattern),
                kind: None,
            })
            .collect()
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
