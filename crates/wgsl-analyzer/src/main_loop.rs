//! The main loop of `rust-analyzer` responsible for dispatching LSP
//! requests/replies and notifications back to the client.

use std::{
    fmt,
    ops::Div as _,
    panic::AssertUnwindSafe,
    time::{Duration, Instant},
};

use always_assert::always;
use crossbeam_channel::{Receiver, select};
// use ide_db::base_db::{SourceDatabase, SourceRootDatabase, VfsPath};
use lsp_server::{Connection, Notification, Request};
use lsp_types::{TextDocumentIdentifier, notification::Notification as _};
use stdx::thread::ThreadIntent;
use tracing::{Level, error, span};
// use vfs::{loader::LoadingProgress, AbsPathBuf, FileId};

use crate::{
    config::Config,
    diagnostics::{DiagnosticsGeneration, NativeDiagnosticsFetchKind, fetch_native_diagnostics},
    global_state::{
        GlobalState,
        //  FetchBuildDataResponse,
        //  FetchWorkspaceRequest,
        // FetchWorkspaceResponse,
        file_id_to_url,
        url_to_file_id,
    },
    lsp::{
        from_proto, to_proto,
        utils::{Progress, notification_is},
    },
    reload::ProjectWorkspaceProgress,
};

use crate::lsp;
use crate::lsp::utils::is_cancelled;
use base_db::SourceDatabase as _;
use std::sync::Arc;

use hir_def::HirFileId;
use hir_def::db::DefDatabase as _;
use hir_def::module_data::{ImportValue, ModuleItem};
use salsa::Durability;
use tracing::info;
use vfs::FileId;

use crate::{
    Result,
    dispatch::{NotificationDispatcher, RequestDispatcher},
    handlers,
    lsp::ext,
};

#[inline]
pub fn main_loop(
    config: Config,
    connection: Connection,
) -> anyhow::Result<()> {
    tracing::info!("initial config: {:#?}", config);

    // Windows scheduler implements priority boosts: if thread waits for an
    // event (like a condvar), and event fires, priority of the thread is
    // temporary bumped. This optimization backfires in our case: each time the
    // `main_loop` schedules a task to run on a threadpool, the worker threads
    // gets a higher priority, and (on a machine with fewer cores) displaces the
    // main loop! We work around this by marking the main loop as a
    // higher-priority thread.
    //
    // https://docs.microsoft.com/en-us/windows/win32/procthread/scheduling-priorities
    // https://docs.microsoft.com/en-us/windows/win32/procthread/priority-boosts
    // https://github.com/rust-lang/rust-analyzer/issues/2835
    #[cfg(windows)]
    unsafe {
        use windows_sys::Win32::System::Threading::*;
        let thread = GetCurrentThread();
        let thread_priority_above_normal = 1;
        SetThreadPriority(thread, thread_priority_above_normal);
    }

    GlobalState::new(connection.sender, config).run(connection.receiver)
}

enum Event {
    Lsp(lsp_server::Message),
    Task(Task),
    QueuedTask(QueuedTask),
    Vfs(vfs::loader::Message),
    // Flycheck(FlycheckMessage),
    // TestResult(CargoTestMessage),
}

impl fmt::Display for Event {
    fn fmt(
        &self,
        #[expect(clippy::min_ident_chars, reason = "trait method")] f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Lsp(_) => write!(f, "Event::Lsp"),
            Self::Task(_) => write!(f, "Event::Task"),
            Self::Vfs(_) => write!(f, "Event::Vfs"),
            // Self::Flycheck(_) => write!(f, "Event::Flycheck"),
            Self::QueuedTask(_) => write!(f, "Event::QueuedTask"),
            // Event::TestResult(_) => write!(f, "Event::TestResult"),
        }
    }
}

#[derive(Debug)]
pub(crate) enum QueuedTask {
    CheckIfIndexed(lsp_types::Url),
    CheckProcMacroSources(Vec<FileId>),
}

#[derive(Debug)]
pub(crate) enum Task {
    Response(lsp_server::Response),
    Diagnostics(DiagnosticsTaskKind),
    // Diagnostics(Vec<(FileId, Vec<lsp_types::Diagnostic>)>),
    FetchWorkspace(ProjectWorkspaceProgress),

    // DiscoverLinkedProjects(DiscoverProjectParam),
    Retry(lsp_server::Request),
    // DiscoverTest(lsp::ext::DiscoverTestResults),
    // PrimeCaches(PrimeCachesProgress),
}

#[derive(Debug)]
pub(crate) enum DiagnosticsTaskKind {
    Syntax(
        DiagnosticsGeneration,
        Vec<(FileId, Vec<lsp_types::Diagnostic>)>,
    ),
    Semantic(
        DiagnosticsGeneration,
        Vec<(FileId, Vec<lsp_types::Diagnostic>)>,
    ),
}

impl fmt::Debug for Event {
    fn fmt(
        &self,
        #[expect(clippy::min_ident_chars, reason = "trait method")] f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let debug_non_verbose = |not: &Notification, formatter: &mut fmt::Formatter<'_>| {
            formatter
                .debug_struct("Notification")
                .field("method", &not.method)
                .finish()
        };

        match self {
            Self::Lsp(lsp_server::Message::Notification(not)) => {
                if notification_is::<lsp_types::notification::DidOpenTextDocument>(not)
                    || notification_is::<lsp_types::notification::DidChangeTextDocument>(not)
                {
                    return debug_non_verbose(not, f);
                }
            },
            Self::Task(Task::Response(resp)) => {
                return f
                    .debug_struct("Response")
                    .field("id", &resp.id)
                    .field("error", &resp.error)
                    .finish();
            },
            Self::Lsp(_) | Self::Task(_) | Self::QueuedTask(_) | Self::Vfs(_) => (),
        }
        match self {
            Self::Lsp(it) => fmt::Debug::fmt(it, f),
            Self::Task(it) => fmt::Debug::fmt(it, f),
            Self::QueuedTask(it) => fmt::Debug::fmt(it, f),
            Self::Vfs(it) => fmt::Debug::fmt(it, f),
            // Event::Flycheck(it) => fmt::Debug::fmt(it, f),
            // Event::TestResult(it) => fmt::Debug::fmt(it, f),
            // Event::DiscoverProject(it) => fmt::Debug::fmt(it, f),
        }
    }
}

impl GlobalState {
    #[expect(clippy::needless_pass_by_value, reason = "intentional")]
    fn run(
        mut self,
        inbox: Receiver<lsp_server::Message>,
    ) -> Result<()> {
        self.update_status_or_notify();

        // if self.config.did_save_text_document_dynamic_registration() {
        //     let additional_patterns = self
        //         .config
        //         .discover_workspace_config()
        //         .map(|cfg| cfg.files_to_watch.clone().into_iter())
        //         .into_iter()
        //         .flatten()
        //         .map(|f| format!("**/{f}"));
        //     self.register_did_save_capability(additional_patterns);
        // }

        // if self.config.discover_workspace_config().is_none() {
        //     self.fetch_workspaces_queue.request_op(
        //         "startup".to_owned(),
        //         FetchWorkspaceRequest { path: None, force_crate_graph_reload: false },
        //     );
        //     if let Some((cause, FetchWorkspaceRequest { path, force_crate_graph_reload })) =
        //         self.fetch_workspaces_queue.should_start_op()
        //     {
        //         self.fetch_workspaces(cause, path, force_crate_graph_reload);
        //     }
        // }

        while let Ok(event) = self.next_event(&inbox) {
            let Some(event) = event else {
                anyhow::bail!("client exited without proper shutdown sequence");
            };
            if matches!(
                &event,
                Event::Lsp(lsp_server::Message::Notification(Notification { method, .. }))
                if method == lsp_types::notification::Exit::METHOD
            ) {
                return Ok(());
            }
            self.handle_event(event);
        }

        Err(anyhow::anyhow!(
            "A receiver has been dropped, something panicked!"
        ))
    }

    #[expect(clippy::needless_pass_by_ref_mut, reason = "intentional")]
    fn update_status_or_notify(&mut self) {
        let status = self.current_status();
        // if self.last_reported_status != status {
        //     self.last_reported_status = status.clone();

        //     if self.config.server_status_notification() {
        //         self.send_notification::<lsp::ext::ServerStatusNotification>(status);
        //     } else if let (
        //         health @ (lsp::ext::Health::Warning | lsp::ext::Health::Error),
        //         Some(message),
        //     ) = (status.health, &status.message)
        //     {
        //         let open_log_button = tracing::enabled!(tracing::Level::ERROR)
        //             && (self.fetch_build_data_error().is_err()
        //                 || self.fetch_workspace_error().is_err());
        //         self.show_message(
        //             match health {
        //                 lsp::ext::Health::Ok => lsp_types::MessageType::INFO,
        //                 lsp::ext::Health::Warning => lsp_types::MessageType::WARNING,
        //                 lsp::ext::Health::Error => lsp_types::MessageType::ERROR,
        //             },
        //             message.clone(),
        //             open_log_button,
        //         );
        //     }
        // }
    }

    #[expect(clippy::unused_self, reason = "wip")]
    pub(super) const fn fetch_workspace_error(&self) -> Result<(), String> {
        Ok(())
    }

    fn next_event(
        &self,
        inbox: &Receiver<lsp_server::Message>,
    ) -> Result<Option<Event>, crossbeam_channel::RecvError> {
        // Make sure we reply to formatting requests ASAP so the editor doesn't block
        if let Ok(task) = self.fmt_pool.receiver.try_recv() {
            return Ok(Some(Event::Task(task)));
        }

        select! {
            recv(inbox) -> msg =>
                return Ok(msg.ok().map(Event::Lsp)),

            recv(self.task_pool.receiver) -> task =>
                task.map(Event::Task),

            recv(self.deferred_task_queue.receiver) -> task =>
                task.map(Event::QueuedTask),

            recv(self.fmt_pool.receiver) -> task =>
                task.map(Event::Task),

            recv(self.loader.receiver) -> task =>
                task.map(Event::Vfs),

            // recv(self.flycheck_receiver) -> task =>
            //     task.map(Event::Flycheck),

            // recv(self.test_run_receiver) -> task =>
            //     task.map(Event::TestResult),

            // recv(self.discover_receiver) -> task =>
            //     task.map(Event::DiscoverProject),
        }
        .map(Some)
    }

    fn handle_event(
        &mut self,
        event: Event,
    ) {
        let loop_start = Instant::now();
        let _p = tracing::info_span!("GlobalState::handle_event", event = %event).entered();

        let event_dbg_msg = format!("{event:?}");
        tracing::debug!(?loop_start, ?event, "handle_event");
        if tracing::enabled!(tracing::Level::INFO) {
            let task_queue_len = self.task_pool.handle.len();
            if task_queue_len > 0 {
                tracing::info!("task queue len: {}", task_queue_len);
            }
        }

        let was_quiescent = self.is_quiescent();
        match event {
            Event::Lsp(msg) => match msg {
                lsp_server::Message::Request(req) => self.on_new_request(loop_start, req),
                lsp_server::Message::Response(response) => self.complete_request(response),
                lsp_server::Message::Notification(notification) => {
                    self.on_notification(notification);
                },
            },
            Event::QueuedTask(task) => {
                let _p = tracing::info_span!("GlobalState::handle_event/queued_task").entered();
                self.handle_queued_task(task);
                // Coalesce multiple task events into one loop turn
                while let Ok(task) = self.deferred_task_queue.receiver.try_recv() {
                    self.handle_queued_task(task);
                }
            },
            Event::Task(task) => {
                let _p = tracing::info_span!("GlobalState::handle_event/task").entered();
                let mut prime_caches_progress = Vec::new();

                self.handle_task(&mut prime_caches_progress, task);
                // Coalesce multiple task events into one loop turn
                while let Ok(task) = self.task_pool.receiver.try_recv() {
                    self.handle_task(&mut prime_caches_progress, task);
                }
            },
            Event::Vfs(message) => {
                let _p = tracing::info_span!("GlobalState::handle_event/vfs").entered();
                self.handle_vfs_msg(message);
                // Coalesce many VFS event into a single loop turn
                while let Ok(message) = self.loader.receiver.try_recv() {
                    self.handle_vfs_msg(message);
                }
            },
        }

        let state_changed = self.process_changes();
        if state_changed {
            self.update_diagnostics();
        }

        let changes = self.diagnostics.take_changes();

        if state_changed {
            // Update import paths?
            #[expect(
                clippy::single_match,
                reason = "changing to if let gives a warning about drop order"
            )]
            match changes.as_ref() {
                Some(changes) => {
                    for file_id in changes {
                        let module = self
                            .analysis_host
                            .raw_database()
                            .module_info(HirFileId::from(*file_id));
                        for item in module.items() {
                            if let ModuleItem::Import(import) = item {
                                let import = module.get(*import);
                                if let ImportValue::Path(path) = &import.value {
                                    let parent_path = self
                                        .analysis_host
                                        .raw_database()
                                        .file_path(*file_id)
                                        .parent()
                                        .unwrap();
                                    let import_path = parent_path.join(path).unwrap();

                                    let params =
                                        lsp::ext::import_text_document::ImportTextDocumentParams {
                                            uri: import_path.to_string(),
                                        };

                                    self.send_request::<lsp::ext::ImportTextDocument>(
                                        params,
                                        |_, _| {},
                                    );
                                }
                            }
                        }
                    }
                },
                _ => {},
            }
        }

        if let Some(diagnostic_changes) = changes {
            for file_id in diagnostic_changes {
                let url = file_id_to_url(&self.vfs.read().unwrap().0, file_id);
                let diagnostics = self.diagnostics.diagnostics_for(file_id).cloned().collect();
                self.send_notification::<lsp_types::notification::PublishDiagnostics>(
                    lsp_types::PublishDiagnosticsParams {
                        uri: url,
                        diagnostics,
                        version: None,
                    },
                );
            }
        }
    }

    fn update_diagnostics(&mut self) {
        let db = self.analysis_host.raw_database();
        let generation = self.diagnostics.next_generation();
        let subscriptions = {
            let vfs = &self.vfs.read().unwrap().0;
            self.mem_docs
                .iter()
                .map(|path| vfs.file_id(path).unwrap())
                // .filter_map(|(file_id, excluded)| {
                //     (excluded == vfs::FileExcluded::No).then_some(file_id)
                // })
                .filter(|&file_id| {
                    let source_root = db.file_source_root(file_id);
                    // Only publish diagnostics for files in the workspace, not from crates.io deps
                    // or the sysroot.
                    // While theoretically these should never have errors, we have quite a few false
                    // positives particularly in the stdlib, and those diagnostics would stay around
                    // forever if we emitted them here.
                    !db.source_root(source_root).is_library
                })
                .collect::<std::sync::Arc<_>>()
        };
        tracing::trace!("updating notifications for {:?}", subscriptions);
        // Split up the work on multiple threads, but we don't wanna fill the entire task pool with
        // diagnostic tasks, so we limit the number of tasks to a quarter of the total thread pool.
        let max_tasks = self.config.main_loop_num_threads().div(4).max(1);
        let chunk_length = subscriptions
            .len()
            .checked_div(max_tasks)
            .expect("max(1) above");
        let remainder = subscriptions
            .len()
            .checked_rem(max_tasks)
            .expect("max(1) above");

        let mut start = 0;
        for task_idx in 0..max_tasks {
            let extra = usize::from(task_idx < remainder);
            let end = start + chunk_length + extra;
            let slice = start..end;
            if slice.is_empty() {
                break;
            }
            // Diagnostics are triggered by the user typing
            // so we run them on a latency sensitive thread.
            let snapshot = self.snapshot();
            self.task_pool
                .handle
                .spawn_with_sender(ThreadIntent::LatencySensitive, {
                    #[expect(clippy::clone_on_ref_ptr, reason = "copied from r-a")]
                    let subscriptions = subscriptions.clone();
                    // Do not fetch semantic diagnostics (and populate query results) if we haven't even
                    // loaded the initial workspace yet.
                    let fetch_semantic =
                    self.vfs_done /*&& self.fetch_workspaces_queue.last_op_result().is_some() */;
                    move |sender| {
                        // We aren't observing the semantics token cache here
                        let snapshot = AssertUnwindSafe(&snapshot);
                        let Ok(diags) = std::panic::catch_unwind(|| {
                            fetch_native_diagnostics(
                                &snapshot,
                                #[expect(clippy::clone_on_ref_ptr, reason = "copied from r-a")]
                                subscriptions.clone(),
                                slice.clone(),
                                NativeDiagnosticsFetchKind::Syntax,
                            )
                        }) else {
                            return;
                        };
                        sender
                            .send(Task::Diagnostics(DiagnosticsTaskKind::Syntax(
                                generation, diags,
                            )))
                            .unwrap();

                        if fetch_semantic {
                            let Ok(diags) = std::panic::catch_unwind(|| {
                                fetch_native_diagnostics(
                                    &snapshot,
                                    #[expect(clippy::clone_on_ref_ptr, reason = "copied from r-a")]
                                    subscriptions.clone(),
                                    slice.clone(),
                                    NativeDiagnosticsFetchKind::Semantic,
                                )
                            }) else {
                                return;
                            };
                            sender
                                .send(Task::Diagnostics(DiagnosticsTaskKind::Semantic(
                                    generation, diags,
                                )))
                                .unwrap();
                        }
                    }
                });
            start = end;
        }
    }

    /// Registers and handles a request. This should only be called once per incoming request.
    fn on_new_request(
        &mut self,
        request_received: Instant,
        req: Request,
    ) {
        let _p =
            span!(Level::INFO, "GlobalState::on_new_request", req.method = ?req.method).entered();
        self.register_request(&req, request_received);
        self.on_request(req);
    }

    fn on_request(
        &mut self,
        request: lsp_server::Request,
    ) {
        let mut dispatcher = RequestDispatcher {
            request: Some(request),
            global_state: self,
        };
        dispatcher.on_sync_mut::<lsp_types::request::Shutdown>(|state, ()| {
            state.shutdown_requested = true;
            Ok(())
        });

        match &mut dispatcher {
            RequestDispatcher {
                request: Some(request),
                global_state: this,
            } if this.shutdown_requested => {
                #[expect(clippy::as_conversions, reason = "Defined by JSON RPC")]
                this.respond(lsp_server::Response::new_err(
                    request.id.clone(),
                    lsp_server::ErrorCode::InvalidRequest as i32,
                    "Shutdown already requested.".to_owned(),
                ));
                return;
            },
            _ => (),
        }

        const RETRY: bool = true;
        const NO_RETRY: bool = false;

        dispatcher
            .on::<NO_RETRY, lsp_types::request::GotoDefinition>(
                handlers::request::handle_goto_definition,
            )
            .on::<RETRY, lsp_types::request::Completion>(handlers::request::handle_completion)
            .on_fmt_thread::<lsp_types::request::Formatting>(handlers::request::handle_formatting)
            .on::<NO_RETRY, lsp_types::request::HoverRequest>(handlers::request::handle_hover)
            .on::<NO_RETRY, lsp_types::request::Shutdown>(handlers::request::handle_shutdown)
            .on::<NO_RETRY, lsp_types::request::InlayHintRequest>(
                handlers::request::handle_inlay_hints,
            )
            .on::<NO_RETRY, lsp::ext::SyntaxTree>(handlers::request::show_syntax_tree)
            .on::<NO_RETRY, lsp::ext::DebugCommand>(handlers::request::debug_command)
            .on::<NO_RETRY, lsp::ext::FullSource>(handlers::request::full_source)
            .finish();
    }

    #[expect(clippy::needless_pass_by_ref_mut, reason = "wip")]
    #[expect(clippy::ptr_arg, reason = "wip")]
    fn handle_task(
        &mut self,
        prime_caches_progress: &mut Vec<()>,
        task: Task,
    ) {
        match task {
            Task::Response(response) => self.respond(response),
            Task::Retry(req) if !self.is_completed(&req) => self.on_request(req),
            Task::Retry(_) => (),
            Task::Diagnostics(kind) => {
                self.diagnostics.set_native_diagnostics(kind);
            },
            Task::FetchWorkspace(progress) => {
                let (state, msg) = match progress {
                    ProjectWorkspaceProgress::Begin => (Progress::Begin, None),
                    ProjectWorkspaceProgress::Report(msg) => (Progress::Report, Some(msg)),
                    ProjectWorkspaceProgress::End(_) => {
                        self.switch_workspaces();
                        (Progress::End, None)
                    },
                };

                self.report_progress("Fetching", &state, msg, None);
            },
        }
    }

    #[expect(clippy::unused_self, reason = "wip")]
    #[expect(clippy::needless_pass_by_ref_mut, reason = "wip")]
    fn handle_vfs_msg(
        &mut self,
        message: vfs::loader::Message,
    ) {
        let _p = tracing::info_span!("GlobalState::handle_vfs_msg").entered();
        let is_changed = matches!(message, vfs::loader::Message::Changed { .. });
        match message {
            vfs::loader::Message::Changed { files } | vfs::loader::Message::Loaded { files } => {},
            vfs::loader::Message::Progress {
                n_total,
                n_done,
                dir,
                config_version,
            } => {},
        }
    }

    #[expect(clippy::unused_self, reason = "wip")]
    #[expect(clippy::needless_pass_by_ref_mut, reason = "wip")]
    fn handle_queued_task(
        &mut self,
        task: QueuedTask,
    ) {
        match task {
            QueuedTask::CheckIfIndexed(uri) => {},
            QueuedTask::CheckProcMacroSources(modified_rust_files) => {},
        }
    }

    fn complete_request(
        &mut self,
        response: lsp_server::Response,
    ) {
        let handler = self
            .req_queue
            .outgoing
            .complete(response.id.clone())
            .expect("received response for unknown request");
        handler(self, response);
    }

    /// Handles an incoming notification.
    fn on_notification(
        &mut self,
        notification: lsp_server::Notification,
    ) {
        let _p =
            span!(Level::INFO, "GlobalState::on_notification", notification.method = ?notification.method).entered();
        use crate::handlers::notification as handlers;
        use lsp_types::notification;
        NotificationDispatcher {
            not: Some(notification),
            global_state: self,
        }
        .on_sync_mut::<notification::DidOpenTextDocument>(handlers::handle_did_open_text_document)
        .on_sync_mut::<notification::DidChangeTextDocument>(
            handlers::handle_did_change_text_document,
        )
        .on_sync_mut::<notification::DidCloseTextDocument>(handlers::handle_did_close_text_document)
        .on_sync_mut::<notification::DidSaveTextDocument>(handlers::handle_did_save_text_document)
        .on_sync_mut::<lsp_types::notification::DidChangeConfiguration>(
            handlers::handle_did_change_configuration,
        )
        .on_sync_mut::<lsp_types::notification::DidChangeWatchedFiles>(
            handlers::handle_did_change_watched_files,
        )
        .finish();
    }

    pub fn update_configuration(
        &mut self,
        config: Config,
    ) {
        let old_config = std::mem::replace(&mut self.config, Arc::new(config));

        if old_config.data().custom_imports != self.config.data().custom_imports {
            self.analysis_host
                .raw_database_mut()
                .set_custom_imports_with_durability(
                    Arc::new(self.config.data().custom_imports.clone()),
                    Durability::HIGH,
                );
        }

        if old_config.data().shader_defs != self.config.data().shader_defs {
            self.analysis_host
                .raw_database_mut()
                .set_shader_defs_with_durability(
                    Arc::new(self.config.data().shader_defs.clone()),
                    Durability::HIGH,
                );
        }
    }
}
