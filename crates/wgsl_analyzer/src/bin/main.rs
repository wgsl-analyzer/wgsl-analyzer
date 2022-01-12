use lsp_server::Connection;
use lsp_types::InitializeParams;
use tracing::info;
use wgsl_analyzer::{config::Config, from_json, main_loop::main_loop, Result};

fn main() -> Result<()> {
    setup_logging();

    let (connection, _io_threads) = Connection::stdio();
    let (initialize_id, initialize_params) = connection.initialize_start()?;
    let initialize_params: InitializeParams = from_json("InitializeParams", initialize_params)?;

    let initialize_result = lsp_types::InitializeResult {
        capabilities: wgsl_analyzer::server_capabilities(),
        server_info: Some(lsp_types::ServerInfo {
            name: String::from("wgsl_analyzer"),
            version: None,
        }),
    };
    let initialize_result = serde_json::to_value(initialize_result).unwrap();
    connection.initialize_finish(initialize_id, initialize_result)?;
    info!("initialized");

    let mut config = Config::default();
    if let Some(options) = initialize_params.initialization_options {
        config.update(options);
    }

    main_loop(config, connection)
}

fn setup_logging() {
    tracing_subscriber::fmt::Subscriber::builder()
        .with_ansi(false)
        .with_max_level(tracing::Level::INFO)
        .with_writer(std::io::stderr)
        .with_env_filter("salsa::derived=warn,info")
        .init();
}
