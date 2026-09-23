//! Builds the `wgsl-analyzer-web` package: the `wasm32-unknown-emscripten`
//! binary, and the JavaScript that hosts it.

use std::path::{Path, PathBuf};

use anyhow::{Context as _, bail};
use xshell::{Cmd, Shell, cmd};

use crate::{flags::BuildWeb, project_root};

const TARGET: &str = "wasm32-unknown-emscripten";

/// Relative to the project root.
const PACKAGE_ROOT: &str = "js/wgsl-analyzer-web";

/// The glue is renamed from the bin name `wgsl_analyzer.js` back to the crate
/// name `wgsl-analyzer.js`: emcc emits pthread bootstrap code that does
/// `new Worker(new URL("wgsl_analyzer.js", import.meta.url))`, so the original
/// name needs to be restored.
const ARTIFACTS: &[(&str, &str)] = &[
    ("wgsl-analyzer.js", "wgsl_analyzer.js"),
    ("wgsl_analyzer.wasm", "wgsl_analyzer.wasm"),
];

impl BuildWeb {
    pub(crate) fn run(
        &self,
        shell: &Shell,
    ) -> anyhow::Result<()> {
        check_requirements(shell)?;

        // A host needs three files out of `dist/assets`: the cargo build
        // supplies `wgsl_analyzer.{js,wasm}`, the package build `worker.js`.
        build_wasm(shell, self.release)?;
        build_package(shell)?;
        // Staging comes last because `build:lib` clears `dist` first.
        let assets = stage_artifacts(shell, self.release)?;

        println!("build-web: staged the web package in {}", assets.display());
        Ok(())
    }
}

fn check_requirements(shell: &Shell) -> anyhow::Result<()> {
    cmd!(shell, "rustc +nightly --version")
        .run()
        .context("a nightly toolchain is required to build the web package")?;

    cmd!(shell, "emcc --version")
        .run()
        .context("`emscripten` is required to build the web package")?;

    pnpm(shell, &["--version"])
        .run()
        .context("`pnpm` is required to build the web package")?;

    Ok(())
}

fn build_wasm(
    shell: &Shell,
    release: bool,
) -> anyhow::Result<()> {
    let release_flag: &[&str] = if release { &["--release"] } else { &[] };

    // The shipped rust-std for this target is built without the wasm `atomics`
    // feature, so it cannot be linked with -pthread. std has to be rebuilt.
    let mut command = cmd!(
        shell,
        "cargo +nightly build -Zbuild-std=std,panic_unwind --package wgsl-analyzer --bin wgsl-analyzer --target {TARGET} {release_flag...}"
    )
    // RUSTFLAGS in the environment would override the ones defined in the
    // `[target.wasm32-unknown-emscripten]` section in .cargo/config.toml.
    .env_remove("RUSTFLAGS")
    .env_remove("CARGO_ENCODED_RUSTFLAGS");

    if release {
        command = tune_for_size(shell, command);
    }

    command
        .run()
        .context("cannot build wgsl-analyzer for the web")
}

/// Size tuning, release only: `-Oz` slows the link down considerably.
fn tune_for_size<'shell>(
    shell: &Shell,
    command: Cmd<'shell>,
) -> Cmd<'shell> {
    // Appended rather than defaulted so that flags the caller already set are kept.
    let cflags = match shell.var("EMCC_CFLAGS") {
        Ok(existing) => format!("{} -Oz", existing.trim()),
        Err(_) => "-Oz".to_owned(),
    };
    let mut command = command.env("EMCC_CFLAGS", cflags.trim());

    // Mirrors the shell idiom `${VAR:-default}`: a value already in the
    // environment wins, so any of these can be overridden per invocation.
    let defaults = [
        ("CARGO_PROFILE_RELEASE_OPT_LEVEL", "z"),
        ("CARGO_PROFILE_RELEASE_LTO", "fat"),
        ("CARGO_PROFILE_RELEASE_CODEGEN_UNITS", "1"),
        ("CARGO_PROFILE_RELEASE_INCREMENTAL", "false"),
    ];
    for (name, value) in defaults {
        if shell.var_os(name).is_none_or(|set| set.is_empty()) {
            command = command.env(name, value);
        }
    }

    command
}

/// Copies the cargo output into `dist/assets/` and returns that directory.
fn stage_artifacts(
    shell: &Shell,
    release: bool,
) -> anyhow::Result<PathBuf> {
    let profile = if release { "release" } else { "debug" };
    let built = project_root().join("target").join(TARGET).join(profile);
    let assets = shell.create_dir(Path::new(PACKAGE_ROOT).join("dist").join("assets"))?;

    for &(from, to) in ARTIFACTS {
        let source = built.join(from);
        if !shell.path_exists(&source) {
            bail!("expected artifact is missing: {}", source.display());
        }
        let destination = assets.join(to);
        shell.copy_file(&source, &destination)?;
        println!("build-web: {to} ({:.1} MB)", megabytes(&destination)?);
    }

    Ok(assets)
}

fn build_package(shell: &Shell) -> anyhow::Result<()> {
    let _directory = shell.push_dir("./js");
    pnpm(shell, &["install", "--frozen-lockfile"]).run()?;
    pnpm(shell, &["--filter", "wgsl-analyzer-web", "run", "build"]).run()?;
    Ok(())
}

/// Spawn a `pnpm` process with the given `arguments`.
fn pnpm<'shell>(
    shell: &'shell Shell,
    arguments: &[&str],
) -> Cmd<'shell> {
    if cfg!(unix) {
        cmd!(shell, "pnpm {arguments...}")
    } else {
        // `pnpm` is a `.cmd` shim on Windows, which cannot be spawned directly.
        cmd!(shell, "cmd.exe /c pnpm {arguments...}")
    }
}

fn megabytes(path: &Path) -> anyhow::Result<f64> {
    let bytes = path
        .metadata()
        .with_context(|| format!("cannot read the size of {}", path.display()))?
        .len();
    #[expect(clippy::as_conversions, reason = "for display only")]
    Ok(bytes as f64 / 1000.0 / 1000.0)
}
