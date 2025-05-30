#![expect(
    clippy::unnecessary_wraps,
    reason = "handlers must have a specific signature"
)]

use std::ops::Not as _;

use anyhow::Context as _;
use itertools::Itertools as _;
use lsp_types::{
    DidChangeConfigurationParams, DidChangeTextDocumentParams, DidChangeWatchedFilesParams,
    DidCloseTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams,
    PublishDiagnosticsParams, notification::PublishDiagnostics,
};
use tracing::error;

use crate::{
    Result, // target_spec::TargetSpec,
    // try_default,
    config::{Config, ConfigChange},
    global_state::GlobalState,
    in_memory_documents::DocumentData,
    lsp::{from_proto, utilities::apply_document_changes},
    reload,
};

pub(crate) fn handle_did_open_text_document(
    state: &mut GlobalState,
    parameters: DidOpenTextDocumentParams,
) -> Result<()> {
    let path = match from_proto::vfs_path(&parameters.text_document.uri) {
        Ok(path) => path,
        Err(error) => {
            error!("Invalid path in DidOpenTextDocument: {}", error);
            return Ok(());
        },
    };

    let text_bytes = parameters.text_document.text.into_bytes();
    state.mem_docs.insert(
        path.clone(),
        DocumentData {
            version: parameters.text_document.version,
            data: text_bytes.clone(),
        },
    );

    let file_id = {
        let mut vfs = state.vfs.write().unwrap();
        vfs.0.set_file_contents(path.clone(), Some(text_bytes));
        vfs.0.file_id(&path)
    };

    // When the file gets closed, we hide the diagnostics, because the LSP does not give a good way to determine when a file has been deleted
    // If there are pre-existing diagnostics, send them now
    if let Some(file_id) = file_id {
        state.diagnostics.make_updated(file_id);
    }

    Ok(())
}

pub(crate) fn handle_did_change_text_document(
    state: &mut GlobalState,
    parameters: DidChangeTextDocumentParams,
) -> anyhow::Result<()> {
    let _p = tracing::info_span!("handle_did_change_text_document").entered();

    if let Ok(path) = from_proto::vfs_path(&parameters.text_document.uri) {
        let Some(DocumentData { version, data }) = state.mem_docs.get_mut(&path) else {
            tracing::error!(?path, "unexpected DidChangeTextDocument");
            return Ok(());
        };
        // The version passed in DidChangeTextDocument is the version after all edits are applied
        // so we should apply it before the vfs is notified.
        *version = parameters.text_document.version;

        let new_contents = apply_document_changes(
            state.config.negotiated_encoding(),
            std::str::from_utf8(data).unwrap(),
            parameters.content_changes,
        )
        .into_bytes();

        if *data != new_contents {
            data.clone_from(&new_contents);
            state
                .vfs
                .write()
                .unwrap()
                .0
                .set_file_contents(path, Some(new_contents));
        }
    }
    Ok(())
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "handlers should have this signature"
)]
pub(crate) fn handle_did_close_text_document(
    state: &mut GlobalState,
    parameters: DidCloseTextDocumentParams,
) -> anyhow::Result<()> {
    let _p = tracing::info_span!("handle_did_close_text_document").entered();

    if let Ok(path) = from_proto::vfs_path(&parameters.text_document.uri) {
        if state.mem_docs.remove(&path).is_err() {
            tracing::error!("orphan DidCloseTextDocument: {}", path);
        }

        // Clear diagnostics also for excluded files, just in case.
        let value = state.vfs.read().unwrap().0.file_id(&path);
        if let Some(file_id) = value {
            state.diagnostics.clear_native_for(file_id);
        }

        // state
        //     .semantic_tokens_cache
        //     .lock()
        //     .remove(&params.text_document.uri);

        if let Some(path) = path.as_path() {
            state.loader.handle.invalidate(path.to_path_buf());
        }
    }
    Ok(())
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "handlers should have this signature"
)]
pub(crate) fn handle_did_save_text_document(
    state: &mut GlobalState,
    parameters: DidSaveTextDocumentParams,
) -> Result<()> {
    let path = from_proto::vfs_path(&parameters.text_document.uri)
        .context("invalid path in did_change_text_document")?;
    Ok(())
}

pub(crate) fn handle_did_change_configuration(
    state: &mut GlobalState,
    _parameters: DidChangeConfigurationParams,
) -> anyhow::Result<()> {
    // As stated in https://github.com/microsoft/language-server-protocol/issues/676,
    // this notification's parameters should be ignored and the actual config queried separately.
    state.send_request::<lsp_types::request::WorkspaceConfiguration>(
        lsp_types::ConfigurationParams {
            items: vec![lsp_types::ConfigurationItem {
                scope_uri: None,
                section: Some("wgsl-analyzer".to_owned()),
            }],
        },
        |this, response| {
            tracing::debug!("config update response: '{:?}", response);
            let lsp_server::Response { error, result, .. } = response;

            match (error, result) {
                (Some(error), _) => {
                    tracing::error!("failed to fetch the server settings: {:?}", error);
                },
                (None, Some(mut configs)) => {
                    if let Some(json) = configs.get_mut(0) {
                        let config = Config::clone(&*this.config);
                        let mut change = ConfigChange::default();
                        change.change_client_config(json.take());

                        let (config, errors, _) = config.apply_change(change);
                        this.config_errors = errors.is_empty().not().then_some(errors);

                        // Client config changes neccesitates .update_config method to be called.
                        this.update_configuration(config);
                    }
                },
                (None, None) => {
                    tracing::error!("received empty server settings response from the client");
                },
            }
        },
    );

    Ok(())
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "handlers must have a given signature"
)]
pub(crate) fn handle_did_change_watched_files(
    state: &mut GlobalState,
    parameters: DidChangeWatchedFilesParams,
) -> anyhow::Result<()> {
    for change in parameters.changes.iter().unique_by(|&it| &it.uri) {
        if let Ok(path) = from_proto::absolute_path(&change.uri) {
            state.loader.handle.invalidate(path);
        }
    }
    Ok(())
}
