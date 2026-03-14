use std::time::Instant;

use base_db::{
    SourceDatabase as _,
    change::Change as BaseDbChange,
    input::{Dependency, PackageData, PackageName, PackageOrigin},
};
use crossbeam_channel::{Receiver, Sender, unbounded};
use ide::{Analysis, AnalysisHost, Cancellable};
use lsp_types::Url;
use parking_lot::{MappedRwLockReadGuard, RwLock, RwLockReadGuard};
use project_model::{
    ManifestPath, PackageChange, PackageGraph, PackageKey, WeslPackage, WeslPackageRoot,
};
use rustc_hash::FxHashMap;
use salsa::Revision;
use tracing::Level;
use triomphe::Arc;
use vfs::{
    AbsPathBuf, Change as VfsChange, FileExcluded, FileId, Vfs, VfsPath,
    loader::{Handle, Message},
};
use vfs_notify::NotifyHandle;

use crate::{
    config::{Config, ConfigErrors},
    diagnostics::DiagnosticCollection,
    discover::{self, DiscoverArgument},
    in_memory_documents::InMemoryDocuments,
    line_index::{LineEndings, LineIndex},
    lsp::{from_proto, to_proto},
    main_loop::Task,
    operation_queue::{Cause, OperationQueue},
    reload::SourceRootConfig,
    task_pool::{DeferredTaskQueue, TaskPool},
};

type RequestHandler = fn(&mut GlobalState, lsp_server::Response);
type RequestQueue = lsp_server::ReqQueue<(String, Instant), RequestHandler>;

// Enforces drop order
pub(crate) struct HandleReceiver<H, C> {
    pub handle: H,
    pub receiver: C,
}

/// `GlobalState` is the primary mutable state of the language server.
///
/// The most interesting components are `vfs`, which stores a consistent
/// snapshot of the file systems, and `analysis_host`, which stores our
/// incremental salsa database.
///
/// Note that this struct has more than one impl in various modules!
#[doc(alias = "GlobalMess")]
pub(crate) struct GlobalState {
    pub(crate) sender: Sender<lsp_server::Message>,
    pub(crate) request_queue: RequestQueue,
    pub(crate) task_pool: HandleReceiver<TaskPool<Task>, Receiver<Task>>,
    pub(crate) fmt_pool: HandleReceiver<TaskPool<Task>, Receiver<Task>>,

    // status
    pub(crate) shutdown_requested: bool,
    pub(crate) last_reported_status: crate::lsp::extensions::ServerStatusParameters,

    // Project loading
    pub(crate) load_package_tasks: Vec<discover::LoadPackageTask>,
    pub(crate) load_package_sender: Sender<discover::LoadPackageMessage>,
    pub(crate) load_package_receiver: Receiver<discover::LoadPackageMessage>,
    pub(crate) load_package_jobs_active: u32,

    // VFS
    pub(crate) loader: HandleReceiver<Box<dyn Handle>, Receiver<Message>>,
    pub(crate) vfs: Arc<RwLock<(Vfs, FxHashMap<FileId, LineEndings>)>>,
    pub(crate) vfs_config_version: u32,
    pub(crate) vfs_progress_config_version: u32,
    pub(crate) vfs_done: bool,
    // used to track how long VFS loading takes. this can't be on `vfs::loader::Handle`,
    // as that handle's lifetime is the same as `GlobalState` itself.
    pub(crate) vfs_span: Option<tracing::span::EnteredSpan>,
    pub(crate) wants_to_switch: Option<Cause>,

    // pub(crate) vfs_config_version: u32,
    pub(crate) analysis_host: AnalysisHost,
    pub(crate) diagnostics: DiagnosticCollection,
    pub(crate) in_memory_documents: InMemoryDocuments,
    pub(crate) config: Arc<Config>,
    pub(crate) config_errors: Option<ConfigErrors>,
    pub(crate) source_root_config: SourceRootConfig,

    pub(crate) packages: Arc<RwLock<PackageGraph>>,

    // op queues
    pub(crate) prime_caches_queue: OperationQueue,

    /// A deferred task queue.
    ///
    /// This queue is used for doing database-dependent work inside of sync
    /// handlers, as accessing the database may block latency-sensitive
    /// interactions and should be moved away from the main thread.
    ///
    /// For certain features, such as [`GlobalState::handle_discover_message`],
    /// this queue should run only *after* [`GlobalState::process_changes`] has
    /// been called.
    pub(crate) deferred_task_queue: DeferredTaskQueue,

    pub(crate) last_gc_revision: Revision,
}

/// An immutable snapshot of the world's state at a point in time.
pub(crate) struct GlobalStateSnapshot {
    pub(crate) config: Arc<Config>,
    pub(crate) analysis: Analysis,
    // pub(crate) check_fixes: CheckFixes,
    in_memory_documents: InMemoryDocuments,
    // pub(crate) semantic_tokens_cache: Arc<Mutex<FxHashMap<Url, SemanticTokens>>>,
    vfs: Arc<RwLock<(Vfs, FxHashMap<FileId, LineEndings>)>>,
    // pub(crate) packages: Arc<[Package]>,
    // pub(crate) flycheck: Arc<[FlycheckHandle]>,
}

impl std::panic::UnwindSafe for GlobalStateSnapshot {}

impl GlobalState {
    pub(crate) fn new(
        sender: Sender<lsp_server::Message>,
        config: Config,
    ) -> Self {
        let loader = {
            let (sender, receiver) = unbounded::<Message>();
            let handle: NotifyHandle = Handle::spawn(sender);
            #[expect(clippy::as_conversions, reason = "tested to be valid")]
            let handle = Box::new(handle) as Box<dyn Handle>;
            HandleReceiver { handle, receiver }
        };

        let task_pool = {
            let (sender, receiver) = unbounded();
            let handle = TaskPool::new_with_threads(sender, config.main_loop_number_of_threads());
            HandleReceiver { handle, receiver }
        };
        let fmt_pool = {
            let (sender, receiver) = unbounded();
            let handle = TaskPool::new_with_threads(sender, 1);
            HandleReceiver { handle, receiver }
        };

        let task_queue = {
            let (sender, receiver) = unbounded();
            DeferredTaskQueue { sender, receiver }
        };

        let analysis_host = AnalysisHost::new(None);
        // if let Some(capacities) = config.lru_query_capacities_config() {
        //     analysis_host.update_lru_capacities(capacities);
        // }
        // let (flycheck_sender, flycheck_receiver) = unbounded();
        // let (test_run_sender, test_run_receiver) = unbounded();

        let (load_package_sender, load_package_receiver) = unbounded();
        let last_gc_revision = analysis_host.raw_database().nonce_and_revision().1;

        let mut this = Self {
            sender,
            request_queue: RequestQueue::default(),
            task_pool,
            fmt_pool,
            shutdown_requested: false,
            last_reported_status: crate::lsp::extensions::ServerStatusParameters {
                health: crate::lsp::extensions::Health::Error,
                quiescent: true,
                message: None,
            },
            load_package_tasks: Vec::new(),
            load_package_sender,
            load_package_receiver,
            load_package_jobs_active: 0,
            loader,
            vfs: Arc::new(RwLock::new((Vfs::default(), FxHashMap::default()))),
            vfs_config_version: 0,
            // semantic_tokens_cache: Arc::new(Default::default()),
            vfs_progress_config_version: 0,
            vfs_done: true,
            vfs_span: None,
            wants_to_switch: None,
            // local_roots_parent_map: Arc::new(FxHashMap::default()),

            // proc_macro_clients: Arc::from_iter([]),

            // build_deps_changed: false,

            // flycheck: Arc::from_iter([]),
            // flycheck_sender,
            // flycheck_receiver,
            // last_flycheck_error: None,

            // test_run_session: None,
            // test_run_sender,
            // test_run_receiver,
            // test_run_remaining_jobs: 0,
            analysis_host,
            diagnostics: DiagnosticCollection::default(),
            in_memory_documents: InMemoryDocuments::default(),
            config: Arc::new(config.clone()),
            config_errors: None,
            source_root_config: SourceRootConfig::default(),
            packages: Arc::new(RwLock::new(PackageGraph::default())),

            // crate_graph_file_dependencies: FxHashSet::default(),
            prime_caches_queue: OperationQueue::default(),

            deferred_task_queue: task_queue,
            last_gc_revision,
        };
        // Apply any required database inputs from the config.
        this.update_configuration(config);
        this
    }

    /// Returns whether any change happened or not.
    pub(crate) fn process_changes(&mut self) -> bool {
        let _p = tracing::span!(Level::INFO, "GlobalState::process_changes").entered();
        let mut modified_local_packages: FxHashMap<ManifestPath, PackageChange> =
            FxHashMap::default();

        let mut change = BaseDbChange::new();
        // VFS changes
        let (vfs, line_endings_map) = &mut *self.vfs.write();
        let changed_files = vfs.take_changes();

        // A file was added or deleted
        let mut has_structure_changes = false;
        for file in changed_files.into_values() {
            let vfs_path = vfs.file_path(file.file_id);

            if let Some(path) = vfs_path.as_path() {
                has_structure_changes |= file.is_created_or_deleted();
                // Update wesl.toml projects in the workspace
                if path.name_and_extension() == Some(("wesl", Some("toml")))
                    && self.config.is_in_workspace(path)
                    && let Ok(package_path) = ManifestPath::try_from(path.to_path_buf())
                {
                    let change = match file.change {
                        VfsChange::Create(_, _) | VfsChange::Modify(_, _) => PackageChange::Set,
                        VfsChange::Delete => PackageChange::Delete,
                    };
                    modified_local_packages.insert(package_path, change);
                }
            }
            // Clear native diagnostics when their file gets deleted
            if !file.exists() {
                self.diagnostics.clear_native_for(file.file_id);
            }

            let text =
                if let VfsChange::Create(vector, _) | VfsChange::Modify(vector, _) = file.change {
                    String::from_utf8(vector).ok().map(|text| {
                        // FIXME: Consider doing normalization in the `vfs` instead? That allows
                        // getting rid of some locking
                        let (text, line_endings) = LineEndings::normalize(text);
                        line_endings_map.insert(file.file_id, line_endings);
                        text
                    })
                } else {
                    None
                };
            change.change_file(file.file_id, text);
        }

        if has_structure_changes {
            let roots = self.source_root_config.partition(vfs);
            change.set_roots(roots);
        }
        // Package graph changes

        let mut packages = &mut *self.packages.write();
        for (path, modified) in modified_local_packages {
            match modified {
                PackageChange::Set => self.request_project_discover(
                    DiscoverArgument {
                        path: path.into(),
                        search_parents: false,
                    },
                    &"wesl.toml changed".to_owned(),
                ),
                PackageChange::Delete => {
                    if let Some(package_id) =
                        packages.package_id(&PackageKey::from_manifest_path(path))
                    {
                        packages.remove(package_id);
                    }
                },
            }
        }

        let changed_packages = packages.take_changes();
        for (id, package_change) in changed_packages {
            let package_data = packages.get(id).and_then(|package| {
                let vfs_path = match &package.root {
                    WeslPackageRoot::File(path) => vfs::VfsPath::from(path.clone()),
                    WeslPackageRoot::Folder(path) => {
                        // TODO: Support folders as the root https://github.com/wgsl-analyzer/wgsl-analyzer/issues/992
                        tracing::error!(
                            "Folders as the root are not supported at the moment {}",
                            path
                        );
                        return None;
                    },
                };
                let Some((root_file_id, root_file_excluded)) = vfs.file_id(&vfs_path) else {
                    // TODO: Properly report the error
                    tracing::error!("Could not find root file {}", &vfs_path);
                    return None;
                };
                if root_file_excluded == FileExcluded::Yes {
                    return None;
                }

                let dependencies = package
                    .dependencies
                    .iter()
                    .filter_map(|dependency| {
                        // TODO: Properly report the errors
                        let Some(package_id) = packages.package_id(&dependency.pkg) else {
                            tracing::error!("Could not find dependency {}", &dependency.name);
                            return None;
                        };
                        let Ok(name) = PackageName::new(&dependency.name) else {
                            tracing::error!("Invalid dependency name {}", &dependency.name);
                            return None;
                        };
                        Some(Dependency { package_id, name })
                    })
                    .collect();

                Some(PackageData {
                    root_file_id,
                    edition: package.edition,
                    display_name: package.display_name.clone(),
                    dependencies,
                    cyclic_dependencies: Vec::new(),
                    origin: package.origin,
                })
            });
            change.change_package(id, package_data);
        }

        if change.is_empty() {
            false
        } else {
            self.analysis_host.apply_change(change);
            true
        }
    }

    pub(crate) fn snapshot(&self) -> GlobalStateSnapshot {
        GlobalStateSnapshot {
            config: Arc::clone(&self.config),
            analysis: self.analysis_host.analysis(),
            in_memory_documents: self.in_memory_documents.clone(),
            vfs: Arc::clone(&self.vfs),
            // check_fixes: Arc::clone(&self.diagnostics.check_fixes),
            // packages: Arc::clone(&self.packages),
            // semantic_tokens_cache: Arc::clone(&self.semantic_tokens_cache),
            // flycheck: self.flycheck.clone(),
        }
    }

    pub(crate) fn send_request<R: lsp_types::request::Request>(
        &mut self,
        parameters: R::Params,
        handler: RequestHandler,
    ) {
        let request =
            self.request_queue
                .outgoing
                .register(R::METHOD.to_owned(), parameters, handler);
        self.send(request.into());
    }

    pub(crate) fn complete_request(
        &mut self,
        response: lsp_server::Response,
    ) {
        let handler = self
            .request_queue
            .outgoing
            .complete(response.id.clone())
            .expect("received response for unknown request");
        handler(self, response);
    }

    pub(crate) fn send_notification<N: lsp_types::notification::Notification>(
        &self,
        parameters: N::Params,
    ) {
        let notification = lsp_server::Notification::new(N::METHOD.to_owned(), parameters);
        self.send(notification.into());
    }

    pub(crate) fn register_request(
        &mut self,
        request: &lsp_server::Request,
        request_received: Instant,
    ) {
        self.request_queue.incoming.register(
            request.id.clone(),
            (request.method.clone(), request_received),
        );
    }

    pub(crate) fn respond(
        &mut self,
        response: lsp_server::Response,
    ) {
        if let Some((method, start)) = self.request_queue.incoming.complete(&response.id) {
            if let Some(error) = &response.error
                && error.message.starts_with("server panicked")
            {
                self.poke_wgsl_analyzer_developer(format!("{}, check the log", error.message));
            }

            let duration = start.elapsed();
            tracing::debug!(name: "message response", method, %response.id, duration = format_args!("{:0.2?}", duration));
            self.send(response.into());
        }
    }

    pub(crate) fn cancel(
        &mut self,
        request_id: lsp_server::RequestId,
    ) {
        if let Some(response) = self.request_queue.incoming.cancel(request_id) {
            self.send(response.into());
        }
    }

    pub(crate) fn is_completed(
        &self,
        request: &lsp_server::Request,
    ) -> bool {
        self.request_queue.incoming.is_completed(&request.id)
    }

    fn send(
        &self,
        message: lsp_server::Message,
    ) {
        self.sender.send(message).unwrap();
    }

    pub(crate) fn publish_diagnostics(
        &self,
        uri: Url,
        version: Option<i32>,
        mut diagnostics: Vec<lsp_types::Diagnostic>,
    ) {
        // We put this on a separate thread to avoid blocking the main thread with serialization work
        self.task_pool.handle.spawn_with_sender(stdx::thread::ThreadIntent::Worker, {
            let sender = self.sender.clone();
            move |_| {
                // VS Code assumes diagnostic messages to be non-empty strings, so we need to patch
                // empty diagnostics. Neither the docs of VS Code nor the LSP spec say whether
                // diagnostic messages are actually allowed to be empty or not and patching this
                // in the VS Code client does not work as the assertion happens in the protocol
                // conversion. So this hack is here to stay, and will be considered a hack
                // until the LSP decides to state that empty messages are allowed.

                // See https://github.com/rust-lang/rust-analyzer/issues/11404
                // See https://github.com/rust-lang/rust-analyzer/issues/13130
                let patch_empty = |message: &mut String| {
                    if message.is_empty() {
                        " ".clone_into(message);
                    }
                };

                for diagnostic in &mut diagnostics {
                    patch_empty(&mut diagnostic.message);
                    if let Some(dri) = &mut diagnostic.related_information {
                        for dri in dri {
                            patch_empty(&mut dri.message);
                        }
                    }
                }

                let notification = lsp_server::Notification::new(
                    <lsp_types::notification::PublishDiagnostics as lsp_types::notification::Notification>::METHOD.to_owned(),
                    lsp_types::PublishDiagnosticsParams { uri, diagnostics, version },
                );
                drop(sender.send(notification.into()));
            }
        });
    }
}

impl Drop for GlobalState {
    fn drop(&mut self) {
        self.analysis_host.trigger_cancellation();
    }
}

impl GlobalStateSnapshot {
    fn vfs_read(&self) -> MappedRwLockReadGuard<'_, Vfs> {
        RwLockReadGuard::map(self.vfs.read(), |(vfs, _)| vfs)
    }

    /// Returns `None` if the file was excluded.
    pub(crate) fn url_to_file_id(
        &self,
        url: &Url,
    ) -> anyhow::Result<Option<FileId>> {
        url_to_file_id(&self.vfs_read(), url)
    }

    pub(crate) fn file_id_to_url(
        &self,
        id: FileId,
    ) -> Url {
        file_id_to_url(&self.vfs.read().0, id)
    }

    pub(crate) fn file_line_index(
        &self,
        file_id: FileId,
    ) -> Cancellable<LineIndex> {
        let endings = self.vfs.read().1[&file_id];
        let index = self.analysis.line_index(file_id)?;
        let result = LineIndex {
            index,
            endings,
            encoding: self.config.capabilities().negotiated_encoding(),
        };
        Ok(result)
    }

    pub(crate) fn file_version(
        &self,
        file_id: FileId,
    ) -> Option<i32> {
        Some(
            self.in_memory_documents
                .get(self.vfs_read().file_path(file_id))?
                .version,
        )
    }
}

pub(crate) fn file_id_to_url(
    vfs: &Vfs,
    id: FileId,
) -> Url {
    let path = vfs.file_path(id);
    let path = path.as_path().unwrap();
    to_proto::url_from_abs_path(path)
}

/// Returns `None` if the file was excluded.
pub(crate) fn url_to_file_id(
    vfs: &Vfs,
    url: &Url,
) -> anyhow::Result<Option<FileId>> {
    let path = from_proto::vfs_path(url)?;
    vfs_path_to_file_id(vfs, &path)
}

/// Returns `None` if the file was excluded.
pub(crate) fn vfs_path_to_file_id(
    vfs: &Vfs,
    vfs_path: &VfsPath,
) -> anyhow::Result<Option<FileId>> {
    let (file_id, excluded) = vfs
        .file_id(vfs_path)
        .ok_or_else(|| anyhow::format_err!("file not found: {vfs_path}"))?;
    match excluded {
        FileExcluded::Yes => Ok(None),
        FileExcluded::No => Ok(Some(file_id)),
    }
}
