use base_db::change::Change;
use crossbeam_channel::{Receiver, Sender, unbounded};
use ide::{Analysis, AnalysisHost, Cancellable};
use lsp_types::Url;
use nohash_hasher::IntMap;
use parking_lot::{
    MappedRwLockReadGuard, Mutex, RwLock, RwLockReadGuard, RwLockUpgradableReadGuard,
    RwLockWriteGuard,
};
use rustc_hash::FxHashMap;
use std::time::Instant;
use triomphe::Arc;
use vfs::{AbsPathBuf, FileId, Vfs, VfsPath};

use crate::{
    Result,
    config::{Config, ConfigErrors},
    diagnostics::DiagnosticCollection,
    in_memory_documents::InMemoryDocuments,
    line_index::{LineEndings, LineIndex, PositionEncoding},
    lsp::{from_proto, to_proto},
    main_loop::Task,
    operation_queue::{Cause, OperationQueue},
    reload::{ProjectWorkspace, SourceRootConfig},
    task_pool::{TaskPool, TaskQueue},
};

pub(crate) struct FetchWorkspaceRequest {
    pub(crate) path: Option<AbsPathBuf>,
    pub(crate) force_crate_graph_reload: bool,
}

pub(crate) struct FetchWorkspaceResponse {
    pub(crate) workspaces: Vec<anyhow::Result<ProjectWorkspace>>,
    pub(crate) force_crate_graph_reload: bool,
}

type RequestHandler = fn(&mut GlobalState, lsp_server::Response);
type RequestQueue = lsp_server::ReqQueue<(String, Instant), RequestHandler>;

// Enforces drop order
pub(crate) struct Handle<H, C> {
    pub handle: H,
    pub receiver: C,
}

/// `GlobalState` is the primary mutable state of the language server
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
    pub(crate) task_pool: Handle<TaskPool<Task>, Receiver<Task>>,
    pub(crate) fmt_pool: Handle<TaskPool<Task>, Receiver<Task>>,

    // status
    pub(crate) shutdown_requested: bool,
    pub(crate) last_reported_status: crate::lsp::extensions::ServerStatusParameters,

    // VFS
    pub(crate) loader: Handle<Box<dyn vfs::loader::Handle>, Receiver<vfs::loader::Message>>,
    pub(crate) vfs: Arc<RwLock<(vfs::Vfs, FxHashMap<FileId, LineEndings>)>>,
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
    pub(crate) mem_docs: InMemoryDocuments,
    pub(crate) config: Arc<Config>,
    pub(crate) config_errors: Option<ConfigErrors>,
    pub(crate) source_root_config: SourceRootConfig,
    // `workspaces` field stores the data we actually use, while the `OperationQueue`
    // stores the result of the last fetch.
    // If the fetch (partially) fails, we do not update the current value.
    pub(crate) workspaces: Arc<[ProjectWorkspace]>,

    // op queues
    pub(crate) fetch_workspaces_queue:
        OperationQueue<FetchWorkspaceRequest, FetchWorkspaceResponse>,
    // pub(crate) fetch_build_data_queue: OperationQueue<(), FetchBuildDataResponse>,
    // pub(crate) fetch_proc_macros_queue: OperationQueue<Vec<ProcMacroPaths>, bool>,
    pub(crate) prime_caches_queue: OperationQueue,
    pub(crate) discover_workspace_queue: OperationQueue,

    /// A deferred task queue.
    ///
    /// This queue is used for doing database-dependent work inside of sync
    /// handlers, as accessing the database may block latency-sensitive
    /// interactions and should be moved away from the main thread.
    ///
    /// For certain features, such as [`GlobalState::handle_discover_message`],
    /// this queue should run only *after* [`GlobalState::process_changes`] has
    /// been called.
    pub(crate) deferred_task_queue: TaskQueue,
}

/// An immutable snapshot of the world's state at a point in time.
pub(crate) struct GlobalStateSnapshot {
    pub(crate) config: Arc<Config>,
    pub(crate) analysis: Analysis,
    // pub(crate) check_fixes: CheckFixes,
    in_memory_documents: InMemoryDocuments,
    // pub(crate) semantic_tokens_cache: Arc<Mutex<FxHashMap<Url, SemanticTokens>>>,
    vfs: Arc<RwLock<(vfs::Vfs, FxHashMap<FileId, LineEndings>)>>,
    pub(crate) workspaces: Arc<[ProjectWorkspace]>,
    // used to signal semantic highlighting to fall back to syntax based highlighting until
    // proc-macros have been loaded
    // FIXME: Can we derive this from somewhere else?
    // pub(crate) flycheck: Arc<[FlycheckHandle]>,
}

impl std::panic::UnwindSafe for GlobalStateSnapshot {}

impl GlobalState {
    pub(crate) fn new(
        sender: Sender<lsp_server::Message>,
        config: Config,
    ) -> Self {
        let loader = {
            let (sender, receiver) = unbounded::<vfs::loader::Message>();
            let handle: vfs_notify::NotifyHandle = vfs::loader::Handle::spawn(sender);
            #[expect(clippy::as_conversions, reason = "tested to be valid")]
            let handle = Box::new(handle) as Box<dyn vfs::loader::Handle>;
            Handle { handle, receiver }
        };

        let task_pool = {
            let (sender, receiver) = unbounded();
            let handle = TaskPool::new_with_threads(sender, config.main_loop_num_threads());
            Handle { handle, receiver }
        };
        let fmt_pool = {
            let (sender, receiver) = unbounded();
            let handle = TaskPool::new_with_threads(sender, 1);
            Handle { handle, receiver }
        };

        let task_queue = {
            let (sender, receiver) = unbounded();
            TaskQueue { sender, receiver }
        };

        let mut analysis_host = AnalysisHost::new(None);
        // if let Some(capacities) = config.lru_query_capacities_config() {
        //     analysis_host.update_lru_capacities(capacities);
        // }
        // let (flycheck_sender, flycheck_receiver) = unbounded();
        // let (test_run_sender, test_run_receiver) = unbounded();

        // let (discover_sender, discover_receiver) = unbounded();

        let mut this = Self {
            sender,
            request_queue: RequestQueue::default(),
            task_pool,
            fmt_pool,
            loader,
            config: Arc::new(config.clone()),
            analysis_host,
            diagnostics: DiagnosticCollection::default(),
            mem_docs: InMemoryDocuments::default(),
            // semantic_tokens_cache: Arc::new(Default::default()),
            shutdown_requested: false,
            last_reported_status: crate::lsp::extensions::ServerStatusParameters {
                health: crate::lsp::extensions::Health::Error,
                quiescent: true,
                message: None,
            },
            source_root_config: SourceRootConfig::default(),
            // local_roots_parent_map: Arc::new(FxHashMap::default()),
            config_errors: None,

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

            // discover_handle: None,
            // discover_sender,
            // discover_receiver,
            vfs: Arc::new(RwLock::new((vfs::Vfs::default(), FxHashMap::default()))),
            vfs_config_version: 0,
            vfs_progress_config_version: 0,
            vfs_span: None,
            vfs_done: true,
            wants_to_switch: None,

            workspaces: Arc::from(Vec::new()),
            // crate_graph_file_dependencies: FxHashSet::default(),
            // detached_files: FxHashSet::default(),
            fetch_workspaces_queue: OperationQueue::default(),
            // fetch_build_data_queue: OperationQueue::default(),
            // fetch_proc_macros_queue: OperationQueue::default(),
            prime_caches_queue: OperationQueue::default(),
            discover_workspace_queue: OperationQueue::default(),

            deferred_task_queue: task_queue,
        };
        // Apply any required database inputs from the config.
        this.update_configuration(config);
        this
    }

    pub(crate) fn process_changes(&mut self) -> bool {
        let change = {
            let mut change = Change::new();
            let (vfs, line_endings_map) = &mut *self.vfs.write();
            let changed_files = vfs.take_changes();
            if changed_files.is_empty() {
                return false;
            }
            for file in changed_files.into_values() {
                let text = if let vfs::Change::Create(vector, _) | vfs::Change::Modify(vector, _) =
                    file.change
                {
                    String::from_utf8(vector).ok().map(|text| {
                        // FIXME: Consider doing normalization in the `vfs` instead? That allows
                        // getting rid of some locking
                        let (text, line_endings) = LineEndings::normalize(text);
                        line_endings_map.insert(file.file_id, line_endings);
                        Arc::new(text)
                    })
                } else {
                    None
                };
                let path = vfs.file_path(file.file_id);
                change.change_file(file.file_id, text, path.clone());
            }

            let roots = self.source_root_config.partition(vfs);
            change.set_roots(roots);

            change
        };

        self.analysis_host.apply_change(change);
        true
    }

    pub(crate) fn snapshot(&self) -> GlobalStateSnapshot {
        GlobalStateSnapshot {
            config: Arc::clone(&self.config),
            workspaces: Arc::clone(&self.workspaces),
            analysis: self.analysis_host.analysis(),
            vfs: Arc::clone(&self.vfs),
            // check_fixes: Arc::clone(&self.diagnostics.check_fixes),
            in_memory_documents: self.mem_docs.clone(),
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

    pub(crate) fn send_notification<N: lsp_types::notification::Notification>(
        &self,
        parameters: N::Params,
    ) {
        let not = lsp_server::Notification::new(N::METHOD.to_owned(), parameters);
        self.send(not.into());
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

                let not = lsp_server::Notification::new(
                    <lsp_types::notification::PublishDiagnostics as lsp_types::notification::Notification>::METHOD.to_owned(),
                    lsp_types::PublishDiagnosticsParams { uri, diagnostics, version },
                );
                sender.send(not.into());
            }
        });
    }
}

impl GlobalStateSnapshot {
    fn vfs_read(&self) -> MappedRwLockReadGuard<'_, vfs::Vfs> {
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
    vfs: &vfs::Vfs,
    id: FileId,
) -> Url {
    let path = vfs.file_path(id);
    let path = path.as_path().unwrap();
    to_proto::url_from_abs_path(path)
}

/// Returns `None` if the file was excluded.
pub(crate) fn url_to_file_id(
    vfs: &vfs::Vfs,
    url: &Url,
) -> anyhow::Result<Option<FileId>> {
    let path = from_proto::vfs_path(url)?;
    vfs_path_to_file_id(vfs, &path)
}

/// Returns `None` if the file was excluded.
pub(crate) fn vfs_path_to_file_id(
    vfs: &vfs::Vfs,
    vfs_path: &VfsPath,
) -> anyhow::Result<Option<FileId>> {
    let (file_id, excluded) = vfs
        .file_id(vfs_path)
        .ok_or_else(|| anyhow::format_err!("file not found: {vfs_path}"))?;
    match excluded {
        vfs::FileExcluded::Yes => Ok(None),
        vfs::FileExcluded::No => Ok(Some(file_id)),
    }
}
