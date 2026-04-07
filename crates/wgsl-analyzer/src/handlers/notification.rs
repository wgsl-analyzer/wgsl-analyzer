#![expect(
    clippy::unnecessary_wraps,
    reason = "handlers must have a specific signature"
)]

use std::ops::Not as _;

use itertools::Itertools as _;
use lsp_types::{
    CancelParams, DidChangeConfigurationParams, DidChangeTextDocumentParams,
    DidChangeWatchedFilesParams, DidChangeWorkspaceFoldersParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, DidSaveTextDocumentParams,
};
use paths::Utf8PathBuf;
use tracing::error;
use triomphe::Arc;
use vfs::AbsPathBuf;

use crate::{
    Result,
    config::{Config, ConfigChange},
    discover::DiscoverArgument,
    global_state::GlobalState,
    in_memory_documents::DocumentData,
    lsp::{from_proto, utilities::apply_document_changes},
};

pub(crate) fn handle_cancel(
    state: &mut GlobalState,
    parameters: CancelParams,
) -> anyhow::Result<()> {
    let id: lsp_server::RequestId = match parameters.id {
        lsp_types::NumberOrString::Number(id) => id.into(),
        lsp_types::NumberOrString::String(id) => id.into(),
    };
    state.cancel(id);
    Ok(())
}

pub(crate) fn handle_did_open_text_document(
    state: &mut GlobalState,
    parameters: DidOpenTextDocumentParams,
) -> Result<()> {
    let _p = tracing::info_span!("handle_did_open_text_document").entered();

    let path = match from_proto::vfs_path(&parameters.text_document.uri) {
        Ok(path) => path,
        Err(error) => {
            error!("Invalid path in DidOpenTextDocument: {}", error);
            return Ok(());
        },
    };

    let text_bytes = parameters.text_document.text.into_bytes();
    let already_exists = state
        .in_memory_documents
        .insert(
            path.clone(),
            DocumentData {
                version: parameters.text_document.version,
                data: text_bytes.clone(),
            },
        )
        .is_err();
    if already_exists {
        tracing::error!("duplicate DidOpenTextDocument: {}", path);
    }

    state
        .vfs
        .write()
        .0
        .set_file_contents(path.clone(), Some(text_bytes));

    if let Some(file_path) = path.as_path() {
        state.request_project_discover(
            DiscoverArgument {
                path: file_path.to_path_buf(),
                search_parents: true,
            },
            &"opened file".to_owned(),
        );
    }
    Ok(())
}

pub(crate) fn handle_did_change_text_document(
    state: &mut GlobalState,
    parameters: DidChangeTextDocumentParams,
) -> anyhow::Result<()> {
    let _p = tracing::info_span!("handle_did_change_text_document").entered();

    if let Ok(path) = from_proto::vfs_path(&parameters.text_document.uri) {
        let Some(DocumentData { version, data }) = state.in_memory_documents.get_mut(&path) else {
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
        if state.in_memory_documents.remove(&path).is_err() {
            tracing::error!("orphan DidCloseTextDocument: {}", path);
        }

        // Clear diagnostics also for excluded files, just in case.
        let value = state.vfs.read().0.file_id(&path);
        if let Some(file_id) = value {
            state.diagnostics.clear_native_for(file_id.0);
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
    let _p = tracing::info_span!("handle_did_save_text_document").entered();

    let path = match from_proto::vfs_path(&parameters.text_document.uri) {
        Ok(path) => path,
        Err(error) => {
            error!("Invalid path in DidSaveTextDocument: {}", error);
            return Ok(());
        },
    };

    // Re-fetch workspaces if the wesl.toml has changed
    if let Some(file_path) = path.as_path()
        && path.name_and_extension() == Some(("wesl", Some("toml")))
    {
        state.request_project_discover(
            DiscoverArgument {
                path: file_path.to_path_buf(),
                search_parents: false,
            },
            &"wesl.toml changed".to_owned(),
        );
    }

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

pub(crate) fn handle_did_change_workspace_folders(
    state: &mut GlobalState,
    parameters: DidChangeWorkspaceFoldersParams,
) -> anyhow::Result<()> {
    let config = Arc::make_mut(&mut state.config);

    for workspace in parameters.event.removed {
        let Ok(path) = workspace.uri.to_file_path() else {
            continue;
        };
        let Ok(path) = Utf8PathBuf::from_path_buf(path) else {
            continue;
        };
        let Ok(path) = AbsPathBuf::try_from(path) else {
            continue;
        };
        config.remove_workspace(&path);
    }

    let added: Vec<_> = parameters
        .event
        .added
        .into_iter()
        .filter_map(|folder| folder.uri.to_file_path().ok())
        .filter_map(|folder| Utf8PathBuf::from_path_buf(folder).ok())
        .filter_map(|folder| AbsPathBuf::try_from(folder).ok())
        .collect();
    config.add_workspaces(added.iter().cloned());

    state.refresh_packages();

    for workspace_root in added {
        state.request_project_discover(
            DiscoverArgument {
                path: workspace_root,
                search_parents: false,
            },
            &"client workspaces changed".to_owned(),
        );
    }

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
    for change in parameters
        .changes
        .iter()
        .unique_by(|&file_event| &file_event.uri)
    {
        if let Ok(path) = from_proto::absolute_path(&change.uri) {
            state.loader.handle.invalidate(path);
        }
    }
    Ok(())
}
