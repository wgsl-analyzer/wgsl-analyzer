use std::{
    env::{self, args as arguments},
    io::stderr,
    path::PathBuf,
};

use lsp_server::Connection;
use lsp_types::InitializeParams;
use paths::{AbsPathBuf, Utf8PathBuf};
use tracing::{info, warn};
use tracing_subscriber::fmt::Subscriber;
use wgsl_analyzer::{
    Result,
    config::{Config, ConfigChange, ConfigErrors, TraceConfig},
    from_json,
    main_loop::main_loop,
};

const VERSION: &str = "0.9.8";

fn get_cwd_as_abs_path() -> Result<AbsPathBuf, std::io::Error> {
    info!("Getting current working directory as absolute path");
    let cwd = env::current_dir()?;
    Ok(AbsPathBuf::assert(
        camino::Utf8Path::new(cwd.to_str().unwrap()).into(),
    ))
}

fn main() -> Result<()> {
    if arguments().any(|arg| arg == "--version") {
        #[expect(clippy::print_stdout, reason = "intended behavior")]
        {
            println!("{VERSION}");
        };
        return Ok(());
    }

    run_server()
}

fn run_server() -> anyhow::Result<()> {
    let (connection, io_threads) = Connection::stdio();

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
        root_uri,
        capabilities,
        workspace_folders,
        initialization_options,
        client_info,
        ..
    } = from_json::<lsp_types::InitializeParams>("InitializeParameters", &initialize_parameters)?;

    let root_path = if let Some(it) = root_uri
        .and_then(|it| it.to_file_path().ok())
        .map(patch_path_prefix)
        .and_then(|it| Utf8PathBuf::from_path_buf(it).ok())
        .and_then(|it| AbsPathBuf::try_from(it).ok())
    {
        it
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

    let workspace_roots = workspace_folders
        .map(|workspaces| {
            workspaces
                .into_iter()
                .filter_map(|it| it.uri.to_file_path().ok())
                .map(patch_path_prefix)
                .filter_map(|it| Utf8PathBuf::from_path_buf(it).ok())
                .filter_map(|it| AbsPathBuf::try_from(it).ok())
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
                MessageType, ShowMessageParams,
                notification::{Notification as _, ShowMessage},
            };
            let not = lsp_server::Notification::new(
                ShowMessage::METHOD.to_owned(),
                ShowMessageParams {
                    typ: MessageType::WARNING,
                    message: error_sink.to_string(),
                },
            );
            connection
                .sender
                .send(lsp_server::Message::Notification(not))
                .unwrap();
        }
    }

    let server_capabilities = wgsl_analyzer::server_capabilities(&config);

    let initialize_result = lsp_types::InitializeResult {
        capabilities: server_capabilities,
        server_info: Some(lsp_types::ServerInfo {
            name: String::from("wgsl-analyzer"),
            version: Some(wgsl_analyzer::version().to_string()),
        }),
        offset_encoding: None,
    };

    let initialize_result = serde_json::to_value(initialize_result).unwrap();

    if let Err(error) = connection.initialize_finish(initialize_id, initialize_result) {
        if error.channel_is_disconnected() {
            io_threads.join()?;
        }
        return Err(error.into());
    }

    // if config.discover_workspace_config().is_none()
    //     && !config.has_linked_projects()
    //     && config.detached_files().is_empty()
    // {
    //     config.rediscover_workspaces();
    // }

    // If the io_threads have an error, there's usually an error on the main
    // loop too because the channels are closed. Ensure we report both errors.
    match (
        wgsl_analyzer::main_loop(config, connection),
        io_threads.join(),
    ) {
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

fn setup_logging(trace: &TraceConfig) {
    let level = if trace.extension { "debug" } else { "info" };
    let filter = format!(
        "{default},salsa=warn,naga=warn,lsp_server={lsp_server}",
        default = level,
        lsp_server = if trace.server { "debug" } else { "info" }
    );

    Subscriber::builder()
        .with_ansi(false)
        .with_writer(stderr)
        .with_env_filter(filter)
        .init();
}
