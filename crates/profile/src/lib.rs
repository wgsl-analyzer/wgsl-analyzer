//! A collection of tools for profiling rust-analyzer.

#[cfg(feature = "cpu_profiler")]
mod google_cpu_profiler;
mod memory_usage;
mod stop_watch;

use std::cell::RefCell;
use std::{env, fs, process};

pub use crate::{
    memory_usage::{Bytes, MemoryUsage},
    stop_watch::{StopWatch, StopWatchSpan},
};

thread_local!(static IN_SCOPE: RefCell<bool> = const { RefCell::new(false) });

/// A wrapper around `google_cpu_profiler`.
///
/// Usage:
/// 1. Install `gperf_tools` (<https://github.com/gperftools/gperftools>), probably packaged with your Linux distro.
/// 2. Build with `cpu_profiler` feature.
/// 3. Run the code, the *raw* output would be in the `./out.profile` file.
/// 4. Install pprof for visualization (<https://github.com/google/pprof>).
/// 5. Bump sampling frequency to once per ms: `export CPUPROFILE_FREQUENCY=1000`
/// 6. Use something like `pprof -svg target/release/rust-analyzer ./out.profile` to see the results.
///
/// For example, here's how I run profiling on `NixOS`:
///
/// ```bash
/// $ bat -p shell.nix
/// with import <nixpkgs> {};
/// mkShell {
///   buildInputs = [ gperftools ];
///   shellHook = ''
///     export LD_LIBRARY_PATH="${gperftools}/lib:"
///   '';
/// }
/// $ set -x CPUPROFILE_FREQUENCY 1000
/// $ nix-shell --run 'cargo test --release --package rust-analyzer --lib -- benchmarks::benchmark_integrated_highlighting --exact --nocapture'
/// $ pprof -svg target/release/deps/rust_analyzer-8739592dc93d63cb crates/rust-analyzer/out.profile > profile.svg
/// ```
///
/// See this diff for how to profile completions:
///
/// <https://github.com/rust-lang/rust-analyzer/pull/5306>
#[derive(Debug)]
pub struct CpuSpan {
    _private: (),
}

#[must_use]
pub fn cpu_span() -> CpuSpan {
    #[cfg(feature = "cpu_profiler")]
    {
        google_cpu_profiler::start("./out.profile".as_ref());
    }

    #[cfg(not(feature = "cpu_profiler"))]
    #[expect(clippy::print_stderr, reason = "CLI tool")]
    {
        eprintln!(
            r#"cpu profiling is disabled, uncomment `default = [ "cpu_profiler" ]` in Cargo.toml to enable."#
        );
    }

    CpuSpan { _private: () }
}

#[cfg(feature = "cpu_profiler")]
impl Drop for CpuSpan {
    #[expect(clippy::print_stderr, reason = "this is a debugging utility")]
    fn drop(&mut self) {
        google_cpu_profiler::stop();
        let profile_data = env::current_dir().unwrap().join("out.profile");
        eprintln!("Profile data saved to:\n\n    {}\n", profile_data.display());
        #[expect(clippy::disallowed_methods, reason = "this is not a rust tool")]
        let mut cmd = process::Command::new("pprof");
        cmd.arg("-svg")
            .arg(env::current_exe().unwrap())
            .arg(&profile_data);
        let out = cmd.output();
        #[expect(clippy::use_debug, reason = "debugging")]
        match out {
            Ok(out) if out.status.success() => {
                let svg = profile_data.with_extension("svg");
                fs::write(&svg, out.stdout).unwrap();
                eprintln!("Profile rendered to:\n\n    {}\n", svg.display());
            },
            _ => {
                eprintln!("Failed to run:\n\n   {cmd:?}\n");
            },
        }
    }
}

#[must_use]
pub fn memory_usage() -> MemoryUsage {
    MemoryUsage::now()
}
