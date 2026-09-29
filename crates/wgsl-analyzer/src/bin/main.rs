//! Driver for wgsl-analyzer.
//!
//! Based on cli flags, either spawns an LSP server, or runs a batch analysis.

#![expect(clippy::print_stdout, clippy::print_stderr, reason = "CLI tool")]

use std::{env, fs, path::PathBuf, process::ExitCode, str::FromStr as _, sync::Arc};

use anyhow::Context as _;
use lsp_server::{Connection, Message, Notification};
use lsp_types::{
    InitializeParams, InitializeResult, MessageType, Notification as _, ServerInfo,
    ShowMessageNotification, ShowMessageParams, WorkspaceFolders,
};
use paths::{AbsPathBuf, Utf8Component, Utf8Path, Utf8PathBuf, Utf8Prefix};
use tracing::info;
use tracing_subscriber::fmt::writer::BoxMakeWriter;
use wgsl_analyzer::{
    Result,
    cli::flags,
    config::{Config, ConfigChange, ConfigErrors},
    from_json,
};

#[cfg(target_os = "emscripten")]
mod emscripten_io;

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

    if let Err(error) = setup_logging(flags.log_file.clone()) {
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
                move || run_server(None),
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

fn setup_logging(log_file_flag: Option<PathBuf>) -> anyhow::Result<()> {
    if cfg!(windows)
        // This is required so that windows finds our pdb that is placed right beside the exe.
        // By default it doesn't look at the folder the exe resides in, only in the current working
        // directory which we set to the project workspace.
        // https://docs.microsoft.com/en-us/windows-hardware/drivers/debugger/general-environment-variables
        // https://docs.microsoft.com/en-us/windows/win32/api/dbghelp/nf-dbghelp-syminitialize
        && let Ok(path) = env::current_exe()
            && let Some(path) = path.parent()
    {
        // SAFETY: This is always safe to call on Windows.
        unsafe {
            env::set_var("_NT_SYMBOL_PATH", path);
        }
    }

    if env::var("RUST_BACKTRACE").is_err() {
        // SAFETY: Environment locks are used.
        unsafe {
            env::set_var("RUST_BACKTRACE", "short");
        }
    }

    let log_file = env::var("WA_LOG_FILE")
        .ok()
        .map(PathBuf::from)
        .or(log_file_flag);
    let log_file = match log_file {
        Some(path) => {
            if let Some(parent) = path.parent() {
                drop(fs::create_dir_all(parent));
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
        profile_filter: env::var("WA_PROFILE").ok(),
        json_profile_filter: std::env::var("WA_PROFILE_JSON").ok(),
    }
    .init()?;

    Ok(())
}

/// Parts of wgsl-analyzer can use a lot of stack space, and some operating systems only give us
/// 1 MB by default (for example, Windows), so this spawns a new thread with hopefully sufficient stack
/// space.
fn with_extra_thread<ThreadName, Function>(
    thread_name: ThreadName,
    thread_intent: stdx::thread::ThreadIntent,
    function: Function,
) -> anyhow::Result<()>
where
    ThreadName: Into<String>,
    Function: FnOnce() -> anyhow::Result<()> + Send + 'static,
{
    let handle = stdx::thread::Builder::new(thread_intent, thread_name).spawn(function)?;
    handle.join()?;
    Ok(())
}

fn run_server(startup_notice: Option<String>) -> anyhow::Result<()> {
    #[cfg(target_os = "emscripten")]
    let (connection, io_threads) = emscripten_io::connection();
    #[cfg(not(target_os = "emscripten"))]
    let (connection, io_threads) = Connection::stdio();

    rayon::ThreadPoolBuilder::new()
        .thread_name(|ix| format!("RayonWorker{ix}"))
        .stack_size(stdx::thread::DEFAULT_STACK_SIZE)
        .build_global()
        .unwrap();

    wgsl_analyzer::session::run_session(
        connection,
        wgsl_analyzer::session::IoThreads::Stdio(io_threads),
        startup_notice,
    )
}
