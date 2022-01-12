use lsp_server::Connection;
use lsp_types::InitializeParams;
use tracing::info;
use wgsl_analyzer::{
    config::{Config, TraceConfig},
    from_json,
    main_loop::main_loop,
    Result,
};

fn main() -> Result<()> {
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

    let mut config = Config::default();
    if let Some(options) = initialize_params.initialization_options {
        config.update(options);
    }

    setup_logging(&config.trace);
    info!("Initialized");
    main_loop(config, connection)
}

fn setup_logging(trace: &TraceConfig) {
    let mut filter = String::from("warn,wgsl_analyzer=");
    filter.push_str(if trace.extension { "debug" } else { "info" });
    if trace.server {
        filter.push_str(",lsp_server=debug")
    }

    tracing_subscriber::fmt::Subscriber::builder()
        .with_ansi(false)
        .with_writer(std::io::stderr)
        .with_env_filter(filter)
        .init();
}
