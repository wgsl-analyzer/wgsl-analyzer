//! Driver for rust-analyzer.
//!
//! Based on cli flags, either spawns an LSP server, or runs a batch analysis

#![expect(clippy::print_stdout, clippy::print_stderr, reason = "CLI tool")]

use std::{
    env::{self, args as arguments},
    fs,
    io::stderr,
    path::PathBuf,
    process::ExitCode,
    sync::Arc,
};

use anyhow::Context as _;
use lsp_server::Connection;
use lsp_types::InitializeParams;
use paths::{AbsPathBuf, Utf8PathBuf};
use tracing::{info, warn};
use tracing_subscriber::fmt::{Subscriber, writer::BoxMakeWriter};
use wgsl_analyzer::{
    Result,
    cli::flags,
    config::{Config, ConfigChange, ConfigErrors, TraceConfig},
    from_json,
    main_loop::main_loop,
};

fn get_cwd_as_abs_path() -> Result<AbsPathBuf, std::io::Error> {
    info!("Getting current working directory as absolute path");
    let cwd = env::current_dir()?;
    Ok(AbsPathBuf::assert(
        camino::Utf8Path::new(cwd.to_str().unwrap()).into(),
    ))
}

fn main() -> Result<ExitCode> {
    let flags = flags::WgslAnalyzer::from_env_or_exit();

    #[cfg(debug_assertions)]
    if flags.wait_dbg || env::var("WA_WAIT_DBG").is_ok() {
        wait_for_debugger();
    }

    if let Err(error) = setup_logging2(flags.log_file.clone()) {
        eprintln!("Failed to setup logging: {error:#}");
    }

    let verbosity = flags.verbosity();

    #[expect(clippy::unimplemented, reason = "TODO")]
    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "future variants are not a current concern"
    )]
    match flags.subcommand {
        flags::WgslAnalyzerCmd::LspServer(command) => 'lsp_server: {
            if command.print_config_schema {
                // println!("{:#}", Config::json_schema());
                break 'lsp_server;
            }
            if command.version {
                println!("wgsl-analyzer {}", wgsl_analyzer::version());
                break 'lsp_server;
            }

            // wgsl-analyzer’s “main thread” is actually
            // a secondary latency-sensitive thread with an increased stack size.
            // We use this thread intent because any delay in the main loop
            // will make actions like hitting enter in the editor slow.
            with_extra_thread(
                "LspServer",
                stdx::thread::ThreadIntent::LatencySensitive,
                run_server,
            )?;
        },
        // flags::WgslAnalyzerCmd::Parse(cmd) => cmd.run()?,
        // flags::WgslAnalyzerCmd::Symbols(cmd) => cmd.run()?,
        // flags::WgslAnalyzerCmd::Highlight(cmd) => cmd.run()?,
        // flags::WgslAnalyzerCmd::AnalysisStats(cmd) => cmd.run(verbosity)?,
        // flags::WgslAnalyzerCmd::Diagnostics(cmd) => cmd.run()?,
        // flags::WgslAnalyzerCmd::UnresolvedReferences(cmd) => cmd.run()?,
        // flags::WgslAnalyzerCmd::Ssr(cmd) => cmd.run()?,
        // flags::WgslAnalyzerCmd::Search(cmd) => cmd.run()?,
        // flags::WgslAnalyzerCmd::Lsif(cmd) => {
        //     cmd.run(&mut std::io::stdout(), Some(project_model::RustLibSource::Discover))?
        // }
        // flags::WgslAnalyzerCmd::Scip(cmd) => cmd.run()?,
        // flags::WgslAnalyzerCmd::RunTests(cmd) => cmd.run()?,
        // flags::WgslAnalyzerCmd::RustcTests(cmd) => cmd.run()?,
        _ => unimplemented!("subcommand not implemented"),
    }
    Ok(ExitCode::SUCCESS)
}

#[cfg(debug_assertions)]
fn wait_for_debugger() {
    #[cfg(target_os = "windows")]
    {
        use windows_sys::Win32::System::Diagnostics::Debug::IsDebuggerPresent;
        // SAFETY: WinAPI generated code that is defensively marked `unsafe` but
        // in practice can not be used in an unsafe way.
        while unsafe { IsDebuggerPresent() } == 0 {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        let mut dummy = 4;
        while dummy == 4 {
            dummy = 4;
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}

#[expect(clippy::too_many_lines, reason = "main")]
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

    let workspace_roots = workspace_folders
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

    if config.discover_workspace_config().is_none()
    // && !config.has_linked_projects()
    // && config.detached_files().is_empty()
    {
        config.rediscover_workspaces();
    }

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

fn setup_logging2(log_file_flag: Option<PathBuf>) -> anyhow::Result<()> {
    if cfg!(windows) {
        // This is required so that windows finds our pdb that is placed right beside the exe.
        // By default it doesn't look at the folder the exe resides in, only in the current working
        // directory which we set to the project workspace.
        // https://docs.microsoft.com/en-us/windows-hardware/drivers/debugger/general-environment-variables
        // https://docs.microsoft.com/en-us/windows/win32/api/dbghelp/nf-dbghelp-syminitialize
        if let Ok(path) = env::current_exe()
            && let Some(path) = path.parent()
        {
            // SAFETY: This is always safe to call on Windows.
            unsafe {
                env::set_var("_NT_SYMBOL_PATH", path);
            }
        }
    }

    if env::var("WGSL_BACKTRACE").is_err() {
        // SAFETY: Environment locks are used.
        unsafe {
            env::set_var("WGSL_BACKTRACE", "short");
        }
    }

    let log_file = env::var("WA_LOG_FILE")
        .ok()
        .map(PathBuf::from)
        .or(log_file_flag);
    let log_file = match log_file {
        Some(path) => {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent);
            }
            Some(
                fs::File::create(&path)
                    .with_context(|| format!("cannot create log file at {}", path.display()))?,
            )
        },
        None => None,
    };

    let writer = log_file.map_or_else(
        || BoxMakeWriter::new(std::io::stderr),
        |file| BoxMakeWriter::new(Arc::new(file)),
    );

    wgsl_analyzer::tracing::Config {
        writer,
        // Deliberately enable all `error` logs if the user has not set WA_LOG, as there is usually
        // useful information in there for debugging.
        filter: env::var("WA_LOG")
            .ok()
            .unwrap_or_else(|| "error".to_owned()),
        chalk_filter: env::var("CHALK_DEBUG").ok(),
        profile_filter: env::var("WA_PROFILE").ok(),
        json_profile_filter: std::env::var("WA_PROFILE_JSON").ok(),
    }
    .init()?;

    Ok(())
}

const STACK_SIZE: usize = 1024 * 1024 * 8;

/// Parts of rust-analyzer can use a lot of stack space, and some operating systems only give us
/// 1 MB by default (eg. Windows), so this spawns a new thread with hopefully sufficient stack
/// space.
fn with_extra_thread(
    thread_name: impl Into<String>,
    thread_intent: stdx::thread::ThreadIntent,
    function: impl FnOnce() -> anyhow::Result<()> + Send + 'static,
) -> anyhow::Result<()> {
    let handle = stdx::thread::Builder::new(thread_intent)
        .name(thread_name.into())
        .stack_size(STACK_SIZE)
        .spawn(function)?;
    handle.join()?;
    Ok(())
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
