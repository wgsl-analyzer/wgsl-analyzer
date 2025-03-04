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
    lsp::{from_proto, utils::apply_document_changes},
    mem_docs::DocumentData,
    reload,
};

pub fn handle_did_open_text_document(
    state: &mut GlobalState,
    params: DidOpenTextDocumentParams,
) -> Result<()> {
    let path = match from_proto::vfs_path(&params.text_document.uri) {
        Ok(path) => path,
        Err(error) => {
            error!("Invalid path in DidOpenTextDocument: {}", error);
            return Ok(());
        },
    };

    let file_id = {
        let mut vfs = state.vfs.write().unwrap();
        vfs.0
            .set_file_contents(path.clone(), Some(params.text_document.text.into_bytes()));
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
    params: DidChangeTextDocumentParams,
) -> anyhow::Result<()> {
    let _p = tracing::info_span!("handle_did_change_text_document").entered();

    if let Ok(path) = from_proto::vfs_path(&params.text_document.uri) {
        let Some(DocumentData { version, data }) = state.mem_docs.get_mut(&path) else {
            tracing::error!(?path, "unexpected DidChangeTextDocument");
            return Ok(());
        };
        // The version passed in DidChangeTextDocument is the version after all edits are applied
        // so we should apply it before the vfs is notified.
        *version = params.text_document.version;

        let new_contents = apply_document_changes(
            state.config.negotiated_encoding(),
            std::str::from_utf8(data).unwrap(),
            params.content_changes,
        )
        .into_bytes();
        // if *data != new_contents {
        //     data.clone_from(&new_contents);
        //     state.vfs.write().0.set_file_contents(path, Some(new_contents));
        // }
    }
    Ok(())
}

pub fn handle_did_close_text_document(
    state: &mut GlobalState,
    params: DidCloseTextDocumentParams,
) -> Result<()> {
    let _path = from_proto::vfs_path(&params.text_document.uri)
        .context("invalid path in did_change_text_document")?;

    state.send_notification::<PublishDiagnostics>(PublishDiagnosticsParams {
        uri: params.text_document.uri,
        diagnostics: vec![],
        version: None,
    });

    Ok(())
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "handlers should have this signature"
)]
pub fn handle_did_save_text_document(
    state: &mut GlobalState,
    params: DidSaveTextDocumentParams,
) -> Result<()> {
    let path = from_proto::vfs_path(&params.text_document.uri)
        .context("invalid path in did_change_text_document")?;
    Ok(())
}

pub(crate) fn handle_did_change_configuration(
    state: &mut GlobalState,
    _params: DidChangeConfigurationParams,
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
        |this, resp| {
            tracing::debug!("config update response: '{:?}", resp);
            let lsp_server::Response { error, result, .. } = resp;

            match (error, result) {
                (Some(err), _) => {
                    tracing::error!("failed to fetch the server settings: {:?}", err);
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
    params: DidChangeWatchedFilesParams,
) -> anyhow::Result<()> {
    for change in params.changes.iter().unique_by(|&it| &it.uri) {
        if let Ok(path) = from_proto::abs_path(&change.uri) {
            state.loader.handle.invalidate(path);
        }
    }
    Ok(())
}
