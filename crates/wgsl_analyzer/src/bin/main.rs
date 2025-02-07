use std::{env::args, io::stderr};

use lsp_server::Connection;
use lsp_types::InitializeParams;
use tracing::info;
use tracing_subscriber::fmt::Subscriber;
use wgsl_analyzer::{
    config::{Config, TraceConfig},
    from_json,
    main_loop::main_loop,
    Result,
};

const VERSION: &str = "0.9.4";

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

    let initialize_result = lsp_types::InitializeResult {
        capabilities: wgsl_analyzer::server_capabilities(),
        server_info: Some(lsp_types::ServerInfo {
            name: String::from("wgsl_analyzer"),
            version: None,
        }),
    };
    let initialize_result = serde_json::to_value(initialize_result)?;
    connection.initialize_finish(initialize_id, initialize_result)?;

    let mut config = Config::default();
    if let Some(options) = initialize_params.initialization_options {
        config.update(&options);
    }

    setup_logging(&config.trace);
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
