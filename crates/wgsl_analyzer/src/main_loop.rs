use crate::config::Config;
use crate::dispatch::NotificationDispatcher;
use crate::global_state::file_id_to_url;
use crate::lsp_utils::is_cancelled;
use crate::{dispatch::RequestDispatcher, global_state::GlobalState};
use crate::{handlers, lsp_ext, Result};
use base_db::SourceDatabase;
use crossbeam_channel::{select, Receiver};
use lsp_server::Connection;
use salsa::Durability;
use std::sync::Arc;
use std::time::Instant;
use vfs::FileId;

pub fn main_loop(config: Config, connection: Connection) -> Result<()> {
    GlobalState::new(connection.sender, config).run(connection.receiver)
}

#[derive(Debug)]
pub enum Event {
    Lsp(lsp_server::Message),
    Task(Task),
}
#[derive(Debug)]
pub enum Task {
    Response(lsp_server::Response),
    Diagnostics(Vec<(FileId, Vec<lsp_types::Diagnostic>)>),
}

impl GlobalState {
    fn run(mut self, receiver: Receiver<lsp_server::Message>) -> Result<()> {
        while let Some(event) = self.next_event(&receiver) {
            self.handle_event(event)?;
        }

        Err(anyhow::anyhow!(
            "client exited without proper shutdown sequence"
        ))
    }

    fn next_event(&self, lsp_receiver: &Receiver<lsp_server::Message>) -> Option<Event> {
        select! {
            recv(lsp_receiver) -> msg => msg.ok().map(Event::Lsp),
            recv(self.task_pool.receiver) -> task => Some(Event::Task(task.unwrap())),
        }
    }

    fn handle_event(&mut self, event: Event) -> Result<()> {
        let start_time = Instant::now();

        match event {
            Event::Lsp(msg) => match msg {
                lsp_server::Message::Request(req) => self.on_request(req, start_time)?,
                lsp_server::Message::Response(response) => self.complete_request(response),
                lsp_server::Message::Notification(notification) => {
                    self.on_notification(notification)?
                }
            },
            Event::Task(task) => match task {
                Task::Response(response) => self.respond(response),
                Task::Diagnostics(diagnostics_per_file) => {
                    for (file_id, diagnostics) in diagnostics_per_file {
                        self.diagnostics
                            .set_native_diagnostics(file_id, diagnostics)
                    }
                }
            },
        }

        let state_changed = self.process_changes();
        if state_changed {
            self.update_diagnostics();
        }

        if let Some(diagnostic_changes) = self.diagnostics.take_changes() {
            for file_id in diagnostic_changes {
                let url = file_id_to_url(&self.vfs.read().unwrap().0, file_id);
                let diagnostics = self.diagnostics.diagnostics_for(file_id).cloned().collect();
                // let version = from_proto::vfs_path(&url)
                // .map(|path| self.mem_docs.get(&path).map(|it| it.version))
                // .unwrap_or_default();

                self.send_notification::<lsp_types::notification::PublishDiagnostics>(
                    lsp_types::PublishDiagnosticsParams {
                        uri: url,
                        diagnostics,
                        version: None,
                    },
                );
            }
        }

        Ok(())
    }

    fn update_diagnostics(&mut self) {
        let snapshot = self.snapshot();
        let relevant_files: Vec<_> = self
            .vfs
            .read()
            .unwrap()
            .0
            .iter()
            .map(|(file_id, _)| file_id)
            .collect();

        let diagnostics_config = self.config.diagnostics();

        self.task_pool.handle.spawn(move || {
            let diagnostics = relevant_files
                .into_iter()
                .filter_map(|file_id| {
                    let diagnostics =
                        handlers::publish_diagnostics(&snapshot, &diagnostics_config, file_id)
                            .map_err(|err| {
                                if !is_cancelled(&*err) {
                                    tracing::error!("Failed to compute diagnostics: {:?}", err);
                                }
                            })
                            .ok()?;

                    Some((file_id, diagnostics))
                })
                .collect();

            Task::Diagnostics(diagnostics)
        });
    }

    fn on_request(&mut self, req: lsp_server::Request, start_time: Instant) -> Result<()> {
        self.register_request(&req, start_time);

        RequestDispatcher::new(Some(req), self)
            .on::<lsp_types::request::GotoDefinition>(handlers::handle_goto_definition)
            .on::<lsp_types::request::Completion>(handlers::handle_completion)
            .on::<lsp_types::request::Formatting>(handlers::handle_formatting)
            .on::<lsp_types::request::HoverRequest>(handlers::handle_hover)
            .on::<lsp_types::request::Shutdown>(handlers::handle_shutdown)
            .on::<lsp_ext::SyntaxTree>(handlers::show_syntax_tree)
            .on::<lsp_ext::DebugCommand>(handlers::debug_command)
            .on::<lsp_ext::FullSource>(handlers::full_source)
            .on::<lsp_ext::InlayHints>(handlers::handle_inlay_hints)
            .finish();

        Ok(())
    }

    fn complete_request(&mut self, response: lsp_server::Response) {
        let handler = self
            .req_queue
            .outgoing
            .complete(response.id.clone())
            .expect("received response for unknown request");

        handler(self, response)
    }

    fn on_notification(&mut self, notification: lsp_server::Notification) -> Result<()> {
        NotificationDispatcher::new(Some(notification), self)
            .on::<lsp_types::notification::DidOpenTextDocument>(
                text_notifications::did_open_text_document,
            )?
            .on::<lsp_types::notification::DidChangeTextDocument>(
                text_notifications::did_change_text_document,
            )?
            .on::<lsp_types::notification::DidCloseTextDocument>(
                text_notifications::did_close_text_document,
            )?
            .on::<lsp_types::notification::DidSaveTextDocument>(
                text_notifications::did_save_text_document,
            )?
            .on::<lsp_types::notification::DidChangeConfiguration>(|this, _params| {
                // As stated in https://github.com/microsoft/language-server-protocol/issues/676,
                // this notification's parameters should be ignored and the actual config queried separately.
                this.send_request::<lsp_ext::RequestConfiguration>((), |this, resp| {
                    let lsp_server::Response { error, result, .. } = resp;

                    match (error, result) {
                        (Some(err), _) => {
                            tracing::error!("Failed to fetch the server settings: {:?}", err)
                        }
                        (None, Some(configs)) => {
                            // Note that json can be null according to the spec if the client can't
                            // provide a configuration. This is handled in Config::update below.
                            let mut config = Config::clone(&*this.config);
                            config.update(configs);
                            this.update_configuration(config);
                        }
                        (None, None) => tracing::error!(
                            "Received empty server settings response from the client"
                        ),
                    }
                });
                Ok(())
            })?
            .finish();
        Ok(())
    }

    pub fn update_configuration(&mut self, config: Config) {
        let old_config = std::mem::replace(&mut self.config, Arc::new(config));

        if old_config.custom_imports != self.config.custom_imports {
            self.analysis_host
                .raw_database_mut()
                .set_custom_imports_with_durability(
                    Arc::new(self.config.custom_imports.clone()),
                    Durability::HIGH,
                );
        }

        if old_config.shader_defs != self.config.shader_defs {
            self.analysis_host
                .raw_database_mut()
                .set_shader_defs_with_durability(
                    Arc::new(self.config.shader_defs.clone()),
                    Durability::HIGH,
                );
        }
    }
}

mod text_notifications {
    use anyhow::Context;
    use lsp_types::{
        DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
        DidSaveTextDocumentParams,
    };
    use tracing::error;

    use crate::{from_proto, global_state::GlobalState, lsp_utils::apply_document_changes, Result};

    pub fn did_open_text_document(
        state: &mut GlobalState,
        params: DidOpenTextDocumentParams,
    ) -> Result<()> {
        let path = match from_proto::vfs_path(&params.text_document.uri) {
            Ok(path) => path,
            Err(e) => {
                error!("Invalid path in DidOpenTextDocument: {}", e);
                return Ok(());
            }
        };

        state
            .vfs
            .write()
            .unwrap()
            .0
            .set_file_contents(path, Some(params.text_document.text.into_bytes()));

        Ok(())
    }

    pub fn did_change_text_document(
        state: &mut GlobalState,
        params: DidChangeTextDocumentParams,
    ) -> Result<()> {
        let path = from_proto::vfs_path(&params.text_document.uri)
            .context("invalid path in did_change_text_document")?;

        let vfs = &mut state.vfs.write().unwrap().0;
        let file_id = vfs.file_id(&path).unwrap();
        let mut text = String::from_utf8(vfs.file_contents(file_id).to_vec()).unwrap();
        apply_document_changes(&mut text, params.content_changes);

        vfs.set_file_contents(path, Some(text.into_bytes()));

        Ok(())
    }

    pub fn did_close_text_document(
        _state: &mut GlobalState,
        params: DidCloseTextDocumentParams,
    ) -> Result<()> {
        let _path = from_proto::vfs_path(&params.text_document.uri)
            .context("invalid path in did_change_text_document")?;

        Ok(())
    }

    pub fn did_save_text_document(
        _state: &mut GlobalState,
        params: DidSaveTextDocumentParams,
    ) -> Result<()> {
        let _path = from_proto::vfs_path(&params.text_document.uri)
            .context("invalid path in did_change_text_document")?;
        Ok(())
    }
}
