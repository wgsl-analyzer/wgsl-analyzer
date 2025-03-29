use std::{
    env::{self, args},
    io::stderr,
};

use lsp_server::Connection;
use lsp_types::InitializeParams;
use paths::AbsPathBuf;
use tracing::{info, warn};
use tracing_subscriber::fmt::Subscriber;
use wgsl_analyzer::{
    Result,
    config::{Config, TraceConfig},
    from_json,
    main_loop::main_loop,
};

const VERSION: &str = "0.9.10";

fn get_cwd_as_abs_path() -> Result<AbsPathBuf, std::io::Error> {
    info!("Getting current working directory as absolute path");
    let cwd = env::current_dir()?;
    Ok(AbsPathBuf::assert(cwd))
}

fn main() -> Result<()> {
    if args().any(|arg| arg == "--version") {
        #[expect(clippy::print_stdout, reason = "intended behavior")]
        {
            println!("{VERSION}");
        };
        return Ok(());
    }

    let (connection, _io_threads) = Connection::stdio();
    let (initialize_id, initialize_params) = connection.initialize_start()?;
    let initialize_params: InitializeParams = from_json("InitializeParams", &initialize_params)?;

    // Root path of current open folder
    let root_path = match &initialize_params.workspace_folders {
        Some(workspace_folders) => {
            if workspace_folders.len() > 1 {
                warn!("Multiple workspace folders detected. Using the first one.");
            }

            if let Some(first_folder) = workspace_folders.first() {
                match first_folder.uri.to_file_path() {
                    Ok(path) => match AbsPathBuf::try_from(path) {
                        Ok(abs_path) => abs_path,
                        Err(_) => get_cwd_as_abs_path()?,
                    },
                    Err(()) => get_cwd_as_abs_path()?,
                }
            } else {
                get_cwd_as_abs_path()?
            }
        },
        None => get_cwd_as_abs_path()?,
    };

    let initialize_result = lsp_types::InitializeResult {
        capabilities: wgsl_analyzer::server_capabilities(),
        server_info: Some(lsp_types::ServerInfo {
            name: String::from("wgsl-analyzer"),
            version: None,
        }),
    };
    let initialize_result = serde_json::to_value(initialize_result)?;
    connection.initialize_finish(initialize_id, initialize_result)?;

    let mut config = Config::new(root_path);
    if let Some(options) = initialize_params.initialization_options {
        config.data.update(&options);
    }

    setup_logging(&config.data.trace);
    info!("Initialized");
    main_loop(config, connection)
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
