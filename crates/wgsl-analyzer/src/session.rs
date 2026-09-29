//! Entry point of a single LSP session: initialization handshake, then the main loop.

use std::{env, path::PathBuf};

use anyhow::Context;
use lsp_server::Connection;
use paths::Utf8PathBuf;
use vfs::AbsPathBuf;

use crate::{
    config::{Config, ConfigChange, ConfigErrors},
    from_json,
};

/// Handles to the I/O threads shuttling the messages of a [`Connection`], joined when
/// the session ends to surface transport errors.
pub enum IoThreads {
    /// The stdio transport of a standalone server.
    #[cfg(target_os = "emscripten")]
    Stdio(crate::emscripten_io::IoThreads),
    #[cfg(not(target_os = "emscripten"))]
    Stdio(lsp_server::IoThreads),
}

impl IoThreads {
    fn join(self) -> anyhow::Result<()> {
        match self {
            Self::Stdio(io_threads) => Ok(io_threads.join()?),
        }
    }
}

/// Runs a full LSP session over `connection`: waits for the client's `initialize`,
/// negotiates capabilities, then runs the main loop until the client disconnects or
/// requests shutdown.
///
/// # Errors
///
/// Returns an error if the connection breaks down or the main loop exits abnormally.
///
/// # Panics
///
/// Panics if the connection fails.
#[expect(clippy::too_many_lines, reason = "TODO")]
pub fn run_session(
    connection: Connection,
    io_threads: IoThreads,
    startup_notice: Option<String>,
) -> anyhow::Result<()> {
    tracing::info!("server version {} will start", crate::version());

    let (initialize_id, initialize_parameters) = match connection.initialize_start() {
        Ok((initialize_id, initialize_parameters)) => (initialize_id, initialize_parameters),
        Err(error) => {
            if error.channel_is_disconnected() {
                io_threads.join()?;
            }
            return Err(error.into());
        },
    };

    tracing::info!("InitializeParameters: {}", initialize_parameters);
    let lsp_types::InitializeParams {
        #[expect(deprecated, reason = "migration TODO")]
        root_uri,
        capabilities,
        workspace_folders_initialize_params,
        initialization_options,
        client_info,
        process_id: _,
        locale: _,
        #[expect(deprecated, reason = "unused")]
            root_path: _,
        trace: _,
        work_done_progress_params: _,
    } = from_json::<lsp_types::InitializeParams, _>(
        "InitializeParameters",
        &initialize_parameters,
    )?;

    let root_path = if let Some(path) = root_uri
        .and_then(|uri| uri.to_file_path().ok())
        .map(patch_path_prefix)
        .and_then(|path| Utf8PathBuf::from_path_buf(path).ok())
        .and_then(|path| AbsPathBuf::try_from(path).ok())
    {
        path
    } else {
        let cwd = env::current_dir()?;
        AbsPathBuf::assert_utf8(cwd)
    };

    if let Some(client_info) = &client_info {
        tracing::info!(
            "Client '{}' {}",
            client_info.name,
            client_info.version.as_deref().unwrap_or_default()
        );
    }

    let workspace_roots = workspace_folders_initialize_params
        .workspace_folders
        .and_then(|workspaces| match workspaces {
            lsp_types::WorkspaceFolders::WorkspaceFolderList(workspace_folders) => {
                Some(workspace_folders)
            },
            lsp_types::WorkspaceFolders::Null => None,
        })
        .map(|workspaces| {
            workspaces
                .into_iter()
                .filter_map(|folder| folder.uri.to_file_path().ok())
                .map(patch_path_prefix)
                .filter_map(|path| Utf8PathBuf::from_path_buf(path).ok())
                .filter_map(|path| AbsPathBuf::try_from(path).ok())
                .collect::<Vec<_>>()
        })
        .filter(|workspaces| !workspaces.is_empty())
        .unwrap_or_else(|| vec![root_path.clone()]);
    let mut config = Config::new(root_path, capabilities, workspace_roots, client_info);
    if let Some(json) = initialization_options {
        let mut change = ConfigChange::default();
        change.change_client_config(json);

        let error_sink: ConfigErrors;
        (config, error_sink, _) = config.apply_change(change);

        if !error_sink.is_empty() {
            use lsp_types::{
                MessageType, Notification as _, ShowMessageNotification, ShowMessageParams,
            };
            let notification = lsp_server::Notification::new(
                ShowMessageNotification::METHOD.into(),
                ShowMessageParams {
                    kind: MessageType::Warning,
                    message: error_sink.to_string(),
                },
            );
            connection
                .sender
                .send(lsp_server::Message::Notification(notification))
                .unwrap();
        }
    }

    let server_capabilities = crate::server_capabilities(&config);

    let initialize_result = lsp_types::InitializeResult {
        capabilities: server_capabilities,
        server_info: Some(lsp_types::ServerInfo {
            name: String::from("wgsl-analyzer"),
            version: Some(crate::version().to_string()),
        }),
    };

    let initialize_result = serde_json::to_value(initialize_result).unwrap();

    if let Err(error) = connection.initialize_finish(initialize_id, initialize_result) {
        if error.channel_is_disconnected() {
            io_threads.join()?;
        }
        return Err(error.into());
    }

    if let Some(notice) = startup_notice {
        use lsp_types::{
            MessageType, Notification as _, ShowMessageNotification, ShowMessageParams,
        };
        let notification = lsp_server::Notification::new(
            ShowMessageNotification::METHOD.into(),
            ShowMessageParams {
                kind: MessageType::Warning,
                message: notice,
            },
        );
        connection
            .sender
            .send(lsp_server::Message::Notification(notification))
            .unwrap();
    }

    // If the io_threads have an error, there's usually an error on the main
    // loop too because the channels are closed. Ensure we report both errors.
    match (crate::main_loop(config, connection), io_threads.join()) {
        (Err(loop_e), Err(join_e)) => anyhow::bail!("{loop_e}\n{join_e}"),
        (Ok(()), Err(join_e)) => anyhow::bail!("{join_e}"),
        (Err(loop_e), Ok(())) => anyhow::bail!("{loop_e}"),
        (Ok(()), Ok(())) => {},
    }

    tracing::info!("server did shut down");
    Ok(())
}

fn patch_path_prefix(path: PathBuf) -> PathBuf {
    use std::path::{Component, Prefix};
    if cfg!(windows) {
        // VS Code might report paths with the file drive in lowercase, but this can mess
        // with env vars set by tools and build scripts executed by w-a such that it invalidates
        // cargo's compilations unnecessarily. https://github.com/rust-lang/rust-analyzer/issues/14683
        // So we just uppercase the drive letter here unconditionally.
        // (doing it conditionally is a pain because std::path::Prefix always reports uppercase letters on windows)
        let mut components = path.components();
        match components.next() {
            Some(Component::Prefix(prefix)) => {
                let prefix = match prefix.kind() {
                    Prefix::Disk(disk_letter) => {
                        format!("{}:", char::from(disk_letter).to_ascii_uppercase())
                    },
                    Prefix::VerbatimDisk(disk_letter) => {
                        format!(r"\\?\{}:", char::from(disk_letter).to_ascii_uppercase())
                    },
                    Prefix::Verbatim(_)
                    | Prefix::VerbatimUNC(..)
                    | Prefix::DeviceNS(_)
                    | Prefix::UNC(..) => return path,
                };
                PathBuf::new().join(prefix).join(components)
            },
            _ => path,
        }
    } else {
        path
    }
}

#[test]
#[cfg(windows)]
fn patch_path_prefix_works() {
    assert_eq!(
        patch_path_prefix(r"c:\foo\bar".into()),
        PathBuf::from(r"C:\foo\bar")
    );
    assert_eq!(
        patch_path_prefix(r"\\?\c:\foo\bar".into()),
        PathBuf::from(r"\\?\C:\foo\bar")
    );
}
