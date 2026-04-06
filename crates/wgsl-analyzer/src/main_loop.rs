//! The main loop of `wgsl-analyzer` responsible for dispatching LSP
//! requests/replies and notifications back to the client.

use std::{
    fmt::{self, Write as _},
    ops::Div as _,
    panic::AssertUnwindSafe,
    time::{Duration, Instant},
};

use base_db::SourceDatabase as _;
use crossbeam_channel::{Receiver, select};
use hir::database::DefDatabase as _;
// use ide_db::base_db::{SourceDatabase, SourceRootDatabase, VfsPath};
use lsp_server::{Connection, Notification, Request};
use lsp_types as lt;
use lt::notification::Notification as _;
use project_model::PackageKey;
use salsa::{Cancelled, Durability};
use stdx::thread::ThreadIntent;
use tracing::{Level, error, span};
use triomphe::Arc;
use vfs::{FileId, VfsPath, loader::LoadingProgress};

use crate::handlers::notification::{
    handle_cancel, handle_did_change_configuration, handle_did_change_text_document,
    handle_did_change_watched_files, handle_did_close_text_document, handle_did_open_text_document,
    handle_did_save_text_document,
};
use crate::{
    Result,
    config::Config,
    diagnostics::{DiagnosticsGeneration, NativeDiagnosticsFetchKind, fetch_native_diagnostics},
    discover::{DiscoverArgument, LoadPackageMessage, LoadPackageTask},
    dispatch::{NotificationDispatcher, RequestDispatcher},
    global_state::{GlobalState, file_id_to_url},
    handlers::{self, notification::handle_did_change_workspace_folders},
    lsp::{
        self, from_proto,
        utilities::{Progress, notification_is},
    },
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
    QueuedTask(DeferredTask),
    Vfs(vfs::loader::Message),
    // Flycheck(FlycheckMessage),
    // TestResult(CargoTestMessage),
    LoadPackage(LoadPackageMessage),
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
            Self::LoadPackage(_) => write!(formatter, "Event::LoadPackage"),
        }
    }
}

#[derive(Debug)]
#[expect(clippy::enum_variant_names, reason = "Not relevant")]
pub(crate) enum DeferredTask {
    CheckIfIndexed(lt::Url),
}

#[derive(Debug)]
pub(crate) enum Task {
    Response(lsp_server::Response),
    DiscoverProject(DiscoverArgument),
    Retry(lsp_server::Request),
    Diagnostics(DiagnosticsTaskKind),
    // DiscoverTest(lsp::ext::DiscoverTestResults),
    PrimeCaches(PrimeCachesProgress),
}

#[derive(Debug)]
pub(crate) enum DiagnosticsTaskKind {
    Syntax(DiagnosticsGeneration, Vec<(FileId, Vec<lt::Diagnostic>)>),
    Semantic(DiagnosticsGeneration, Vec<(FileId, Vec<lt::Diagnostic>)>),
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
        let debug_non_verbose =
            |notification: &Notification, formatter: &mut fmt::Formatter<'_>| {
                formatter
                    .debug_struct("Notification")
                    .field("method", &notification.method)
                    .finish()
            };

        match self {
            Self::Lsp(lsp_server::Message::Notification(notification)) => {
                if notification_is::<lt::notification::DidOpenTextDocument>(notification)
                    || notification_is::<lt::notification::DidChangeTextDocument>(notification)
                {
                    return debug_non_verbose(notification, formatter);
                }
            },
            Self::Task(Task::Response(response)) => {
                return formatter
                    .debug_struct("Response")
                    .field("id", &response.id)
                    .field("error", &response.error)
                    .finish();
            },
            Self::Lsp(_)
            | Self::Task(_)
            | Self::QueuedTask(_)
            | Self::Vfs(_)
            | Self::LoadPackage(_) => (),
        }
        match self {
            Self::Lsp(message) => fmt::Debug::fmt(message, formatter),
            Self::Task(task) => fmt::Debug::fmt(task, formatter),
            Self::QueuedTask(task) => fmt::Debug::fmt(task, formatter),
            Self::Vfs(message) => fmt::Debug::fmt(message, formatter),
            // Event::Flycheck(it) => fmt::Debug::fmt(it, formatter),
            // Event::TestResult(it) => fmt::Debug::fmt(it, formatter),
            Self::LoadPackage(message) => fmt::Debug::fmt(message, formatter),
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

        for workspace_root in self.config.workspace_roots() {
            self.request_project_discover(
                DiscoverArgument {
                    path: workspace_root.clone(),
                    search_parents: false,
                },
                &"startup".to_owned(),
            );
        }

        while let Ok(event) = self.next_event(&inbox) {
            let Some(event) = event else {
                anyhow::bail!("client exited without proper shutdown sequence");
            };
            if matches!(
                &event,
                Event::Lsp(lsp_server::Message::Notification(Notification { method, .. }))
                if method == lt::notification::Exit::METHOD
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
        let additional_filters = additional_patterns.map(|pattern| lt::DocumentFilter {
            language: None,
            scheme: None,
            pattern: (Some(pattern)),
        });

        let mut selectors = vec![
            lt::DocumentFilter {
                language: None,
                scheme: None,
                pattern: Some("**/*.wgsl".into()),
            },
            lt::DocumentFilter {
                language: None,
                scheme: None,
                pattern: Some("**/*.wesl".into()),
            },
            lt::DocumentFilter {
                language: None,
                scheme: None,
                pattern: Some("**/wesl.toml".into()),
            },
        ];
        selectors.extend(additional_filters);

        let save_registration_options = lt::TextDocumentSaveRegistrationOptions {
            include_text: Some(false),
            text_document_registration_options: lt::TextDocumentRegistrationOptions {
                document_selector: Some(selectors),
            },
        };

        let registration = lt::Registration {
            id: "textDocument/didSave".to_owned(),
            method: "textDocument/didSave".to_owned(),
            register_options: Some(serde_json::to_value(save_registration_options).unwrap()),
        };
        self.send_request::<lt::request::RegisterCapability>(
            lt::RegistrationParams {
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
                        lsp::extensions::Health::Ok => lt::MessageType::INFO,
                        lsp::extensions::Health::Warning => lt::MessageType::WARNING,
                        lsp::extensions::Health::Error => lt::MessageType::ERROR,
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

            recv(self.load_package_receiver) -> task =>
                task.map(Event::LoadPackage),
        }
        .map(Some)
    }

    fn trigger_garbage_collection(&mut self) {
        if cfg!(test) {
            // Slow tests run the main loop in multiple threads, but GC isn't thread safe.
            return;
        }

        self.analysis_host.trigger_garbage_collection();
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
        if tracing::enabled!(tracing::Level::TRACE) {
            let task_queue_len = self.task_pool.handle.len();
            if task_queue_len > 0 {
                tracing::trace!("task queue len: {task_queue_len}");
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
                let mut last_progress_report = None;
                self.handle_vfs_message(message, &mut last_progress_report);
                // Coalesce many VFS event into a single loop turn
                while let Ok(message) = self.loader.receiver.try_recv() {
                    self.handle_vfs_message(message, &mut last_progress_report);
                }
                if let Some((message, fraction)) = last_progress_report {
                    self.report_progress(
                        "Roots Scanned",
                        &Progress::Report,
                        Some(message),
                        Some(fraction),
                        None,
                    );
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
            Event::LoadPackage(message) => {
                self.handle_load_package_message(message);
                // Coalesce many project discovery events into a single loop turn.
                while let Ok(message) = self.load_package_receiver.try_recv() {
                    self.handle_load_package_message(message);
                }
            },
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
                    self.send_request::<lt::request::SemanticTokensRefresh>((), |_, _| ());
                }

                // Refresh code lens if the client supports it.
                if self.config.code_lens_refresh() {
                    self.send_request::<lt::request::CodeLensRefresh>((), |_, _| ());
                }

                // Refresh inlay hints if the client supports it.
                if self.config.inlay_hints_refresh() {
                    self.send_request::<lt::request::InlayHintRefreshRequest>((), |_, _| ());
                }

                if self.config.diagnostics_refresh() {
                    self.send_request::<lt::request::WorkspaceDiagnosticRefresh>((), |_, _| ());
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

            let current_revision = self.analysis_host.raw_database().nonce_and_revision().1;
            // no work is currently being done, now we can block a bit and clean up our garbage
            if self.task_pool.handle.is_empty()
                && self.fmt_pool.handle.is_empty()
                && current_revision != self.last_gc_revision
            {
                self.trigger_garbage_collection();
                self.last_gc_revision = current_revision;
            }
        }

        self.cleanup_load_package_tasks();

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
                .filter_map(|(file_id, excluded)| {
                    (excluded == vfs::FileExcluded::No).then_some(file_id)
                })
                // .filter(|&file_id| {
                //     let source_root = database.file_source_root(file_id.0).source_root_id(database);
                //     // Only publish diagnostics for files in the workspace, not from crates.io deps
                //     // or the sysroot.
                //     // While theoretically these should never have errors, we have quite a few false
                //     // positives particularly in the stdlib, and those diagnostics would stay around
                //     // forever if we emitted them here.
                //     !database.source_root(source_root).source_root(database).is_library()
                // })
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
                    let fetch_semantic = self.vfs_done && self.load_package_jobs_active == 0;
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
            Task::DiscoverProject(argument) => {
                if let Some(load_package_task) =
                    LoadPackageTask::discover_local(&argument, self.load_package_sender.clone())
                {
                    if self.load_package_jobs_active == 0 {
                        self.report_progress("Project loading", &Progress::Begin, None, None, None);
                    }
                    self.load_package_jobs_active += 1;
                    let last_entry = self.load_package_tasks.len();
                    self.load_package_tasks.push(load_package_task);
                    self.load_package_tasks[last_entry].run();
                }
            },
        }
    }

    fn handle_vfs_message(
        &mut self,
        message: vfs::loader::Message,
        last_progress_report: &mut Option<(String, f64)>,
    ) {
        let _p = tracing::info_span!("GlobalState::handle_vfs_message").entered();
        let is_changed = matches!(message, vfs::loader::Message::Changed { .. });
        match message {
            vfs::loader::Message::Changed { files } | vfs::loader::Message::Loaded { files } => {
                let _p =
                    tracing::info_span!("GlobalState::handle_vfs_message{changed/load}").entered();
                let vfs = &mut self.vfs.write().0;
                for (path, contents) in files {
                    let path = VfsPath::from(path);
                    // if the file is in mem docs, it's managed by the client via notifications
                    // so only set it if its not in there
                    if !self.in_memory_documents.contains(&path)
                        && (is_changed || vfs.file_id(&path).is_none())
                    {
                        vfs.set_file_contents(path, contents);
                    }
                }
            },
            vfs::loader::Message::Progress {
                n_total,
                n_done,
                dir: directory, // spellchecker:disable-line
                config_version,
            } => {
                let _p = span!(Level::INFO, "GlobalState::handle_vfs_message/progress").entered();
                stdx::always!(config_version <= self.vfs_config_version);

                let (n_done, state) = match n_done {
                    LoadingProgress::Started => {
                        self.vfs_span =
                            Some(span!(Level::INFO, "vfs_load", total = n_total).entered());
                        (0, Progress::Begin)
                    },
                    LoadingProgress::Progress(n_done) => (n_done.min(n_total), Progress::Report),
                    LoadingProgress::Finished => {
                        self.vfs_span = None;
                        (n_total, Progress::End)
                    },
                };

                self.vfs_progress_config_version = config_version;
                self.vfs_done = state == Progress::End;

                let mut message = format!("{n_done}/{n_total}");
                if let Some(directory) = directory {
                    write!(
                        message,
                        ": {}",
                        match directory.strip_prefix(self.config.root_path()) {
                            Some(relative_path) => relative_path.as_utf8_path(),
                            None => directory.as_ref(),
                        }
                    );
                }

                match state {
                    Progress::Begin => self.report_progress(
                        "Roots Scanned",
                        &state,
                        Some(message),
                        Some(Progress::fraction(n_done, n_total)),
                        None,
                    ),
                    // Don't send too many notifications while batching, sending progress reports
                    // serializes notifications on the mainthread at the moment which slows us down
                    Progress::Report => {
                        if last_progress_report.is_none() {
                            self.report_progress(
                                "Roots Scanned",
                                &state,
                                Some(message.clone()),
                                Some(Progress::fraction(n_done, n_total)),
                                None,
                            );
                        }

                        *last_progress_report =
                            Some((message, Progress::fraction(n_done, n_total)));
                    },
                    Progress::End => {
                        last_progress_report.take();
                        self.report_progress(
                            "Roots Scanned",
                            &state,
                            Some(message),
                            Some(Progress::fraction(n_done, n_total)),
                            None,
                        );
                    },
                }
            },
        }
    }

    #[expect(clippy::unused_self, reason = "wip")]
    #[expect(clippy::needless_pass_by_ref_mut, reason = "wip")]
    fn handle_queued_task(
        &mut self,
        task: DeferredTask,
    ) {
        match task {
            DeferredTask::CheckIfIndexed(uri) => {},
        }
    }

    fn handle_load_package_message(
        &mut self,
        message: LoadPackageMessage,
    ) {
        let title = "Project loading";
        match message {
            LoadPackageMessage::Finished { project } => {
                self.load_package_jobs_active = self.load_package_jobs_active.strict_sub(1);
                if self.load_package_jobs_active == 0 {
                    self.report_progress(title, &Progress::End, None, None, None);
                    self.wants_to_switch = Some("fetched project".to_owned());
                }

                let mut packages = self.packages.write();
                packages.set(PackageKey::from_package(&project), project);
            },
            LoadPackageMessage::Progress { message } => {
                if self.load_package_jobs_active > 0 {
                    self.report_progress(title, &Progress::Report, Some(message), None, None);
                }
            },
            LoadPackageMessage::Error { error, source } => {
                let message = format!("Project discovery failed: {error}");
                self.show_and_log_error(message.clone(), source);

                self.load_package_jobs_active = self.load_package_jobs_active.strict_sub(1);
                if self.load_package_jobs_active == 0 {
                    self.report_progress(title, &Progress::End, Some(message), None, None);
                }
            },
        }
    }

    /// Drop any package loading processes that have exited, due to
    /// finishing or erroring.
    fn cleanup_load_package_tasks(&mut self) {
        let mut active_handles = vec![];

        for mut task in self.load_package_tasks.drain(..) {
            if !task.has_exited() {
                active_handles.push(task);
            }
        }
        self.load_package_tasks = active_handles;
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
        dispatcher.on_sync_mut::<lt::request::Shutdown>(|state, ()| {
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

        // FIXME: Some of these NO_RETRY could be retries if the file they are interested didn't change.
        // All other request handlers
        dispatcher
            .on::<NO_RETRY, lt::request::GotoDefinition>(handlers::request::handle_goto_definition)
            .on::<RETRY, lt::request::Completion>(handlers::request::handle_completion)
            .on_fmt_thread::<lt::request::Formatting>(handlers::request::handle_formatting)
            .on::<RETRY, lt::request::FoldingRangeRequest>(handlers::request::handle_folding_range)
            .on::<NO_RETRY, lsp::extensions::HoverRequest>(handlers::request::handle_hover)
            .on::<NO_RETRY, lt::request::Shutdown>(handlers::request::handle_shutdown)
            .on::<NO_RETRY, lt::request::InlayHintRequest>(handlers::request::handle_inlay_hints)
            .on_with_vfs_default::<lt::request::DocumentDiagnosticRequest>(
                handlers::request::handle_document_diagnostics,
                handlers::request::empty_diagnostic_report,
                || {
                    let code = i32::try_from(lt::error_codes::SERVER_CANCELLED)
                        .expect("LSP error code must fit in i32");
                    let message = "server cancelled the request".to_owned();
                    let value = lt::DiagnosticServerCancellationData {
                        retrigger_request: true,
                    };
                    let data = serde_json::to_value(value).ok();
                    lsp_server::ResponseError {
                        code,
                        message,
                        data,
                    }
                },
            )
            .on::<NO_RETRY, lsp::extensions::ViewSyntaxTree>(handlers::request::view_syntax_tree)
            .on::<NO_RETRY, lsp::extensions::DebugCommand>(handlers::request::debug_command)
            .on::<NO_RETRY, lsp::extensions::FullSource>(handlers::request::full_source)
            .on::<NO_RETRY, lt::request::SignatureHelpRequest>(
                handlers::request::handle_signature_help,
            )
            .finish();
    }

    /// Handles an incoming notification.
    fn on_notification(
        &mut self,
        notification: lsp_server::Notification,
    ) {
        let _p =
            span!(Level::INFO, "GlobalState::on_notification", notification.method = ?notification.method).entered();
        NotificationDispatcher {
            notification: Some(notification),
            global_state: self,
        }
        .on_sync_mut::<lt::notification::Cancel>(handle_cancel)
        .on_sync_mut::<lt::notification::DidOpenTextDocument>(handle_did_open_text_document)
        .on_sync_mut::<lt::notification::DidChangeTextDocument>(handle_did_change_text_document)
        .on_sync_mut::<lt::notification::DidCloseTextDocument>(handle_did_close_text_document)
        .on_sync_mut::<lt::notification::DidSaveTextDocument>(handle_did_save_text_document)
        .on_sync_mut::<lt::notification::DidChangeConfiguration>(handle_did_change_configuration)
        .on_sync_mut::<lt::notification::DidChangeWorkspaceFolders>(
            handle_did_change_workspace_folders,
        )
        .on_sync_mut::<lt::notification::DidChangeWatchedFiles>(handle_did_change_watched_files)
        .finish();
    }
}
