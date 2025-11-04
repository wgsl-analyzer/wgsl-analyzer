//! The main loop of `rust-analyzer` responsible for dispatching LSP
//! requests/replies and notifications back to the client.

use std::{
    fmt,
    ops::Div as _,
    panic::AssertUnwindSafe,
    time::{Duration, Instant},
};

use crossbeam_channel::{Receiver, select};
// use ide_db::base_db::{SourceDatabase, SourceRootDatabase, VfsPath};
use lsp_server::{Connection, Notification, Request};
use lsp_types::notification::Notification as _;
use stdx::thread::ThreadIntent;
use tracing::{Level, error, span};
// use vfs::{loader::LoadingProgress, AbsPathBuf, FileId};

use crate::{
    config::Config,
    diagnostics::{DiagnosticsGeneration, NativeDiagnosticsFetchKind, fetch_native_diagnostics},
    global_state::{FetchWorkspaceResponse, GlobalState, file_id_to_url},
    lsp::{
        from_proto,
        utilities::{Progress, notification_is},
    },
    reload::ProjectWorkspaceProgress,
};

use crate::lsp;
use base_db::SourceDatabase as _;
use triomphe::Arc;

use salsa::Cancelled;
use vfs::{AbsPathBuf, FileId};

use crate::{
    Result,
    dispatch::{NotificationDispatcher, RequestDispatcher},
    handlers,
};

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
    {
        use windows_sys::Win32::System::Threading::{GetCurrentThread, SetThreadPriority};
        // SAFETY: The safety of GetCurrentThread is undocumented.
        let thread = unsafe { GetCurrentThread() };
        let thread_priority_above_normal = 1;
        // SAFETY: The safety of SetThreadPriority is undocumented.
        unsafe {
            SetThreadPriority(thread, thread_priority_above_normal);
        }
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
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Lsp(_) => write!(formatter, "Event::Lsp"),
            Self::Task(_) => write!(formatter, "Event::Task"),
            Self::Vfs(_) => write!(formatter, "Event::Vfs"),
            // Self::Flycheck(_) => write!(formatter, "Event::Flycheck"),
            Self::QueuedTask(_) => write!(formatter, "Event::QueuedTask"),
            // Event::TestResult(_) => write!(formatter, "Event::TestResult"),
        }
    }
}

#[derive(Debug)]
#[expect(clippy::enum_variant_names, reason = "Not relevant")]
pub(crate) enum QueuedTask {
    CheckIfIndexed(lsp_types::Url),
}

#[derive(Debug)]
pub(crate) enum Task {
    Response(lsp_server::Response),
    Diagnostics(DiagnosticsTaskKind),
    // Diagnostics(Vec<(FileId, Vec<lsp_types::Diagnostic>)>),
    FetchWorkspace(ProjectWorkspaceProgress),

    // DiscoverLinkedProjects(DiscoverProjectParameter),
    Retry(lsp_server::Request),
    // DiscoverTest(lsp::ext::DiscoverTestResults),
    PrimeCaches(PrimeCachesProgress),
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

#[derive(Debug)]
pub(crate) enum DiscoverProjectParam {
    Buildfile(AbsPathBuf),
    Path(AbsPathBuf),
}

#[derive(Debug)]
pub(crate) enum PrimeCachesProgress {
    Begin,
    Report(()),
    End { cancelled: bool },
}

impl fmt::Debug for Event {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
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
                    return debug_non_verbose(not, formatter);
                }
            },
            Self::Task(Task::Response(response)) => {
                return formatter
                    .debug_struct("Response")
                    .field("id", &response.id)
                    .field("error", &response.error)
                    .finish();
            },
            Self::Lsp(_) | Self::Task(_) | Self::QueuedTask(_) | Self::Vfs(_) => (),
        }
        match self {
            Self::Lsp(message) => fmt::Debug::fmt(message, formatter),
            Self::Task(task) => fmt::Debug::fmt(task, formatter),
            Self::QueuedTask(task) => fmt::Debug::fmt(task, formatter),
            Self::Vfs(message) => fmt::Debug::fmt(message, formatter),
            // Event::Flycheck(it) => fmt::Debug::fmt(it, formatter),
            // Event::TestResult(it) => fmt::Debug::fmt(it, formatter),
            // Event::DiscoverProject(it) => fmt::Debug::fmt(it, formatter),
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

        if self.config.did_save_text_document_dynamic_registration() {
            let additional_patterns = self
                .config
                .discover_workspace_config()
                .map(|cfg| cfg.files_to_watch.clone().into_iter())
                .into_iter()
                .flatten()
                .map(|file| format!("**/{file}"));
            self.register_did_save_capability(additional_patterns);
        }

        // if self.config.discover_workspace_config().is_none() {
        //     self.fetch_workspaces_queue.request_operation(
        //         "startup".to_owned(),
        //         FetchWorkspaceRequest {
        //             path: None,
        //             force_crate_graph_reload: false,
        //         },
        //     );
        //     if let Some((
        //         cause,
        //         FetchWorkspaceRequest {
        //             path,
        //             force_crate_graph_reload,
        //         },
        //     )) = self.fetch_workspaces_queue.should_start_operation()
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

    fn register_did_save_capability(
        &mut self,
        additional_patterns: impl Iterator<Item = String>,
    ) {
        let additional_filters = additional_patterns.map(|pattern| lsp_types::DocumentFilter {
            language: None,
            scheme: None,
            pattern: (Some(pattern)),
        });

        let mut selectors = vec![
            lsp_types::DocumentFilter {
                language: None,
                scheme: None,
                pattern: Some("**/*.rs".into()),
            },
            lsp_types::DocumentFilter {
                language: None,
                scheme: None,
                pattern: Some("**/Cargo.toml".into()),
            },
            lsp_types::DocumentFilter {
                language: None,
                scheme: None,
                pattern: Some("**/Cargo.lock".into()),
            },
        ];
        selectors.extend(additional_filters);

        let save_registration_options = lsp_types::TextDocumentSaveRegistrationOptions {
            include_text: Some(false),
            text_document_registration_options: lsp_types::TextDocumentRegistrationOptions {
                document_selector: Some(selectors),
            },
        };

        let registration = lsp_types::Registration {
            id: "textDocument/didSave".to_owned(),
            method: "textDocument/didSave".to_owned(),
            register_options: Some(serde_json::to_value(save_registration_options).unwrap()),
        };
        self.send_request::<lsp_types::request::RegisterCapability>(
            lsp_types::RegistrationParams {
                registrations: vec![registration],
            },
            |_, _| (),
        );
    }

    fn update_status_or_notify(&mut self) {
        let status = self.current_status();
        if self.last_reported_status != status {
            self.last_reported_status = status.clone();

            if self.config.server_status_notification() {
                self.send_notification::<lsp::extensions::ServerStatusNotification>(status);
            } else if let (
                health @ (lsp::extensions::Health::Warning | lsp::extensions::Health::Error),
                Some(message),
            ) = (status.health, &status.message)
            {
                let open_log_button = tracing::enabled!(tracing::Level::ERROR)
                    &&
                    // (self.fetch_build_data_error().is_err() || 
                    self.fetch_workspace_error().is_err()
                    // )
                    ;
                self.show_message(
                    match health {
                        lsp::extensions::Health::Ok => lsp_types::MessageType::INFO,
                        lsp::extensions::Health::Warning => lsp_types::MessageType::WARNING,
                        lsp::extensions::Health::Error => lsp_types::MessageType::ERROR,
                    },
                    message.clone(),
                    open_log_button,
                );
            }
        }
    }

    #[expect(clippy::unused_self, reason = "wip")]
    #[expect(clippy::unnecessary_wraps, reason = "wip")]
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
            recv(inbox) -> message =>
                return Ok(message.ok().map(Event::Lsp)),

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

    #[expect(clippy::cognitive_complexity, reason = "deprecated lint")]
    #[expect(clippy::too_many_lines, reason = "TODO")]
    fn handle_event(
        &mut self,
        event: Event,
    ) {
        let loop_start = Instant::now();
        let _p = tracing::info_span!("GlobalState::handle_event", event = %event).entered();

        let event_debug_message = format!("{event:?}");
        tracing::debug!(?loop_start, ?event, "handle_event");
        if tracing::enabled!(tracing::Level::INFO) {
            let task_queue_len = self.task_pool.handle.length();
            if task_queue_len > 0 {
                tracing::info!("task queue len: {}", task_queue_len);
            }
        }

        let was_quiescent = self.is_quiescent();
        match event {
            Event::Lsp(message) => match message {
                lsp_server::Message::Request(request) => self.on_new_request(loop_start, request),
                lsp_server::Message::Notification(notification) => {
                    self.on_notification(notification);
                },
                lsp_server::Message::Response(response) => self.complete_request(response),
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

                for progress in prime_caches_progress {}
            },
            Event::Vfs(message) => {
                let _p = tracing::info_span!("GlobalState::handle_event/vfs").entered();
                self.handle_vfs_message(message);
                // Coalesce many VFS event into a single loop turn
                while let Ok(message) = self.loader.receiver.try_recv() {
                    self.handle_vfs_message(message);
                }
            },
            // Event::Flycheck(message) => {
            //     let _p = tracing::info_span!("GlobalState::handle_event/flycheck").entered();
            //     self.handle_flycheck_message(message);
            //     // Coalesce many flycheck updates into a single loop turn
            //     while let Ok(message) = self.flycheck_receiver.try_recv() {
            //         self.handle_flycheck_message(message);
            //     }
            // },
            // Event::TestResult(message) => {
            //     let _p = tracing::info_span!("GlobalState::handle_event/test_result").entered();
            //     self.handle_cargo_test_message(message);
            //     // Coalesce many test result event into a single loop turn
            //     while let Ok(message) = self.test_run_receiver.try_recv() {
            //         self.handle_cargo_test_message(message);
            //     }
            // },
            // Event::DiscoverProject(message) => {
            //     self.handle_discover_message(message);
            //     // Coalesce many project discovery events into a single loop turn.
            //     while let Ok(message) = self.discover_receiver.try_recv() {
            //         self.handle_discover_message(message);
            //     }
            // },
        }
        let event_handling_duration = loop_start.elapsed();
        let (state_changed, memdocs_added_or_removed) = if self.vfs_done {
            if let Some(cause) = self.wants_to_switch.take() {
                self.switch_workspaces(&cause);
            }
            (
                self.process_changes(),
                self.in_memory_documents.take_changes(),
            )
        } else {
            (false, false)
        };

        if self.is_quiescent() {
            let became_quiescent = !was_quiescent;
            // if became_quiescent {
            //     if self.config.check_on_save(None)
            //         && self.config.flycheck_workspace(None)
            //         && !self.fetch_build_data_queue.op_requested()
            //     {
            //         // Project has loaded properly, kick off initial flycheck
            //         self.flycheck
            //             .iter()
            //             .for_each(|flycheck| flycheck.restart_workspace(None));
            //     }
            //     if self.config.prefill_caches() {
            //         self.prime_caches_queue
            //             .request_op("became quiescent".to_owned(), ());
            //     }
            // }

            let client_refresh = became_quiescent || state_changed;
            if client_refresh {
                // Refresh semantic tokens if the client supports it.
                if self.config.semantic_tokens_refresh() {
                    // self.semantic_tokens_cache.lock().clear();
                    self.send_request::<lsp_types::request::SemanticTokensRefresh>((), |_, _| ());
                }

                // Refresh code lens if the client supports it.
                if self.config.code_lens_refresh() {
                    self.send_request::<lsp_types::request::CodeLensRefresh>((), |_, _| ());
                }

                // Refresh inlay hints if the client supports it.
                if self.config.inlay_hints_refresh() {
                    self.send_request::<lsp_types::request::InlayHintRefreshRequest>((), |_, _| ());
                }

                if self.config.diagnostics_refresh() {
                    self.send_request::<lsp_types::request::WorkspaceDiagnosticRefresh>(
                        (),
                        |_, _| (),
                    );
                }
            }

            let project_or_mem_docs_changed =
                became_quiescent || state_changed || memdocs_added_or_removed;
            if project_or_mem_docs_changed
                && !self.config.text_document_diagnostic()
                && self.config.publish_diagnostics(None)
            {
                self.update_diagnostics();
            }
            if project_or_mem_docs_changed && self.config.test_explorer() {
                // self.update_tests();
            }
        }

        if let Some(diagnostic_changes) = self.diagnostics.take_changes() {
            for file_id in diagnostic_changes {
                let uri = file_id_to_url(&self.vfs.read().0, file_id);
                let version = from_proto::vfs_path(&uri).ok().and_then(|path| {
                    self.in_memory_documents
                        .get(&path)
                        .map(|document_data| document_data.version)
                });

                let diagnostics = self
                    .diagnostics
                    .diagnostics_for(file_id)
                    .cloned()
                    .collect::<Vec<_>>();
                self.publish_diagnostics(uri, version, diagnostics);
            }
        }

        // if self.config.cargo_autoreload_config(None)
        //     || self.config.discover_workspace_config().is_some()
        // {
        //     if let Some((
        //         cause,
        //         FetchWorkspaceRequest {
        //             path,
        //             force_crate_graph_reload,
        //         },
        //     )) = self.fetch_workspaces_queue.should_start_op()
        //     {
        //         self.fetch_workspaces(cause, path, force_crate_graph_reload);
        //     }
        // }

        if !self.fetch_workspaces_queue.operation_in_progress() {
            // if let Some((cause, ())) = self.fetch_build_data_queue.should_start_op() {
            //     self.fetch_build_data(cause);
            // } else if let Some((cause, (change, paths))) =
            //     self.fetch_proc_macros_queue.should_start_op()
            // {
            //     self.fetch_proc_macros(cause, change, paths);
            // }
        }

        if let Some((cause, ())) = self.prime_caches_queue.should_start_operation() {
            self.prime_caches(cause);
        }

        self.update_status_or_notify();

        let loop_duration = loop_start.elapsed();
        if loop_duration > Duration::from_millis(100) && was_quiescent {
            tracing::warn!(
                "overly long loop turn took {loop_duration:?} (event handling took {event_handling_duration:?}): {event_debug_message}"
            );
            self.poke_wgsl_analyzer_developer(format!(
                "overly long loop turn took {loop_duration:?} (event handling took {event_handling_duration:?}): {event_debug_message}"
            ));
        }
    }

    #[expect(clippy::needless_pass_by_ref_mut, reason = "wip")]
    #[expect(clippy::needless_pass_by_value, reason = "wip")]
    fn prime_caches(
        &mut self,
        cause: String,
    ) {
        tracing::debug!(%cause, "will prime caches");
        let num_worker_threads = self.config.prime_caches_number_of_threads();

        self.task_pool
            .handle
            .spawn_with_sender(ThreadIntent::Worker, {
                let analysis = AssertUnwindSafe(self.snapshot().analysis);
                move |sender| {
                    sender
                        .send(Task::PrimeCaches(PrimeCachesProgress::Begin))
                        .unwrap();
                    let result: Result<(), Cancelled> = Ok(());
                    sender
                        .send(Task::PrimeCaches(PrimeCachesProgress::End {
                            cancelled: result.is_err(),
                        }))
                        .unwrap();
                }
            });
    }

    fn update_diagnostics(&mut self) {
        let database = self.analysis_host.raw_database();
        let generation = self.diagnostics.next_generation();
        let subscriptions = {
            let vfs = &self.vfs.read().0;
            self.in_memory_documents
                .iter()
                .map(|path| vfs.file_id(path).unwrap())
                // .filter_map(|(file_id, excluded)| {
                //     (excluded == vfs::FileExcluded::No).then_some(file_id)
                // })
                .filter(|&file_id| {
                    let source_root = database.file_source_root(file_id.0);
                    // Only publish diagnostics for files in the workspace, not from crates.io deps
                    // or the sysroot.
                    // While theoretically these should never have errors, we have quite a few false
                    // positives particularly in the stdlib, and those diagnostics would stay around
                    // forever if we emitted them here.
                    !database.source_root(source_root).is_library()
                })
                .map(|file_id| {
                    file_id.0
                })
                .collect::<Arc<_>>()
        };
        tracing::trace!("updating notifications for {:?}", subscriptions);
        // Split up the work on multiple threads, but we don't wanna fill the entire task pool with
        // diagnostic tasks, so we limit the number of tasks to a quarter of the total thread pool.
        let max_tasks = self.config.main_loop_number_of_threads().div(4).max(1);
        let chunk_length = subscriptions
            .len()
            .checked_div(max_tasks)
            .expect("max(1) above");
        let remainder = subscriptions
            .len()
            .checked_rem(max_tasks)
            .expect("max(1) above");

        let mut start = 0;
        for task_index in 0..max_tasks {
            let extra = usize::from(task_index < remainder);
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
                    let subscriptions = subscriptions.clone();
                    // Do not fetch semantic diagnostics (and populate query results) if we haven't even
                    // loaded the initial workspace yet.
                    let fetch_semantic = self.vfs_done
                        && self
                            .fetch_workspaces_queue
                            .last_operation_result()
                            .is_some();
                    move |sender| {
                        // We aren't observing the semantics token cache here
                        let snapshot = AssertUnwindSafe(&snapshot);
                        let Ok(diags) = std::panic::catch_unwind(|| {
                            fetch_native_diagnostics(
                                &snapshot,
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
        request: Request,
    ) {
        let _p =
            span!(Level::INFO, "GlobalState::on_new_request", request.method = ?request.method)
                .entered();
        self.register_request(&request, request_received);
        self.on_request(request);
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
            .on::<NO_RETRY, lsp::extensions::HoverRequest>(handlers::request::handle_hover)
            .on::<NO_RETRY, lsp_types::request::Shutdown>(handlers::request::handle_shutdown)
            .on::<NO_RETRY, lsp_types::request::InlayHintRequest>(
                handlers::request::handle_inlay_hints,
            )
            // FIXME: Some of these NO_RETRY could be retries if the file they are interested didn't change.
            // All other request handlers
            .on_with_vfs_default::<lsp_types::request::DocumentDiagnosticRequest>(handlers::request::handle_document_diagnostics, handlers::request::empty_diagnostic_report, || lsp_server::ResponseError {
                code: i32::try_from(lsp_types::error_codes::SERVER_CANCELLED).expect("LSP error code must fit in i32"),
                message: "server cancelled the request".to_owned(),
                data: serde_json::to_value(lsp_types::DiagnosticServerCancellationData {
                    retrigger_request: true
                }).ok(),
            })
            .on::<NO_RETRY, lsp::extensions::SyntaxTree>(handlers::request::show_syntax_tree)
            .on::<NO_RETRY, lsp::extensions::DebugCommand>(handlers::request::debug_command)
            .on::<NO_RETRY, lsp::extensions::FullSource>(handlers::request::full_source)
            .finish();
    }

    #[expect(clippy::ignored_unit_patterns, reason = "wip")]
    #[expect(clippy::match_same_arms, reason = "wip")]
    fn handle_task(
        &mut self,
        prime_caches_progress: &mut Vec<PrimeCachesProgress>,
        task: Task,
    ) {
        match task {
            Task::Response(response) => self.respond(response),
            Task::Retry(request) if !self.is_completed(&request) => self.on_request(request),
            Task::Retry(_) => (),
            Task::Diagnostics(kind) => {
                self.diagnostics.set_native_diagnostics(kind);
            },
            Task::PrimeCaches(progress) => match progress {
                PrimeCachesProgress::Begin => prime_caches_progress.push(progress),
                PrimeCachesProgress::Report(_) => {
                    match prime_caches_progress.last_mut() {
                        Some(last @ PrimeCachesProgress::Report(_)) => {
                            // Coalesce subsequent update events.
                            *last = progress;
                        },
                        _ => prime_caches_progress.push(progress),
                    }
                },
                PrimeCachesProgress::End { .. } => prime_caches_progress.push(progress),
            },
            Task::FetchWorkspace(progress) => {
                let (state, message) = match progress {
                    ProjectWorkspaceProgress::Begin => (Progress::Begin, None),
                    ProjectWorkspaceProgress::Report(message) => (Progress::Report, Some(message)),
                    ProjectWorkspaceProgress::End(workspaces, force_crate_graph_reload) => {
                        let response = FetchWorkspaceResponse {
                            workspaces,
                            force_crate_graph_reload,
                        };
                        // self.fetch_workspaces_queue.op_completed(response);
                        if let Err(error) = self.fetch_workspace_error() {
                            error!("FetchWorkspaceError: {error}");
                        }
                        self.wants_to_switch = Some("fetched workspace".to_owned());
                        self.diagnostics.clear_check_all();
                        (Progress::End, None)
                    },
                };

                self.report_progress("Fetching", &state, message, None, None);
            },
        }
    }

    #[expect(clippy::unused_self, reason = "wip")]
    #[expect(clippy::needless_pass_by_ref_mut, reason = "wip")]
    fn handle_vfs_message(
        &mut self,
        message: vfs::loader::Message,
    ) {
        let _p = tracing::info_span!("GlobalState::handle_vfs_message").entered();
        let is_changed = matches!(message, vfs::loader::Message::Changed { .. });
        match message {
            vfs::loader::Message::Changed { files } | vfs::loader::Message::Loaded { files } => {},
            vfs::loader::Message::Progress {
                n_total,
                n_done,
                dir: directory, // spellchecker:disable-line
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
        }
    }

    fn complete_request(
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

    pub(crate) fn update_configuration(
        &mut self,
        config: Config,
    ) {
        let _old_config = std::mem::replace(&mut self.config, Arc::new(config));
    }
}
