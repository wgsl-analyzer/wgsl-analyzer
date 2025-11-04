//! See <https://github.com/matklad/cargo-xtask/>.
//!
//! This binary defines various auxiliary build commands, which are not
//! expressible with just `cargo`. Notably, it provides tests via `cargo test -p xtask`
//! for code generation and `cargo xtask install` for installation of
//! wgsl-analyzer server and client.
//!
//! This binary is integrated into the `cargo` command line by using an alias in
//! `.cargo/config`.

#![allow(clippy::print_stdout, reason = "CLI tool")]
#![allow(clippy::print_stderr, reason = "CLI tool")]
#![allow(clippy::disallowed_types, reason = "not applicable")]

mod flags;

mod codegen;
mod dist;
mod install;
mod pgo;
mod publish;
mod release;
mod tidy;
mod utilities;

use std::{env as environment, path::PathBuf};

use anyhow::{Context as _, bail};
use xshell::{Shell, cmd as command};

fn main() -> anyhow::Result<()> {
    let flags = flags::Xtask::from_env_or_exit();

    let shell = &Shell::new()?;
    shell.change_dir(project_root());

    match flags.subcommand {
        flags::XtaskCmd::Install(command) => command.run(shell),
        flags::XtaskCmd::FuzzTests(_) => run_fuzzer(shell),
        flags::XtaskCmd::Release(command) => command.run(shell),
        flags::XtaskCmd::Dist(command) => command.run(shell),
        flags::XtaskCmd::PublishReleaseNotes(command) => command.run(shell),
        flags::XtaskCmd::Codegen(command) => command.run(shell),
        flags::XtaskCmd::Bb(command) => {
            {
                let _directory = shell.push_dir("./crates/wgsl-analyzer");
                command!(shell, "cargo build --release --features jemalloc").run()?;
            }
            shell.copy_file(
                "./target/release/wgsl-analyzer",
                format!("./target/wgsl-analyzer-{}", command.suffix),
            )?;
            Ok(())
        },
        flags::XtaskCmd::Tidy(command) => command.run(shell),
    }
}

/// Returns the path to the root directory of `wgsl-analyzer` project.
fn project_root() -> PathBuf {
    let directory = environment::var("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned());
    PathBuf::from(directory).parent().unwrap().to_owned()
}

fn run_fuzzer(shell: &Shell) -> anyhow::Result<()> {
    let _d = shell.push_dir("./crates/syntax");
    let _e = shell.push_env("RUSTUP_TOOLCHAIN", "nightly");
    if command!(shell, "cargo fuzz --help").read().is_err() {
        command!(shell, "cargo install cargo-fuzz").run()?;
    }

    // Expecting nightly rustc
    let out = command!(shell, "rustc --version").read()?;
    if !out.contains("nightly") {
        bail!("fuzz tests require nightly rustc")
    }

    command!(shell, "cargo fuzz run parser").run()?;
    Ok(())
}

fn date_iso(shell: &Shell) -> anyhow::Result<String> {
    command!(shell, "date -u +%Y-%m-%d")
        .read()
        .context("failed to get current date")
}

fn is_release_tag(tag: &str) -> bool {
    tag.len() == "yyyy-mm-dd".len() && tag.starts_with(|character: char| character.is_ascii_digit())
}
