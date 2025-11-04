//! PGO (Profile-Guided Optimization) utilities.

use std::{
    env::consts::EXE_EXTENSION,
    ffi::OsStr,
    path::{Path, PathBuf},
};

use anyhow::Context as _;
use xshell::{Cmd, Shell, cmd};

use crate::flags::PgoTrainingCrate;

/// Decorates `wa_build_cmd` to add PGO instrumentation, and then runs the PGO instrumented
/// wgsl-analyzer on itself to gather a PGO profile.
pub(crate) fn gather_pgo_profile<'shell>(
    shell: &'shell Shell,
    ra_build_cmd: Cmd<'shell>,
    target: &str,
    train_crate: &PgoTrainingCrate,
) -> anyhow::Result<PathBuf> {
    let pgo_dir = std::path::absolute("wgsl-analyzer-pgo")?;
    // Clear out any stale profiles
    if pgo_dir.is_dir() {
        std::fs::remove_dir_all(&pgo_dir)?;
    }
    std::fs::create_dir_all(&pgo_dir)?;

    // Figure out a path to `llvm-profdata`
    let target_libdir = cmd!(shell, "rustc --print=target-libdir")
        .read()
        .context("cannot resolve target-libdir from rustc")?;
    let target_bindir = PathBuf::from(target_libdir).parent().unwrap().join("bin");
    let llvm_profdata = target_bindir
        .join("llvm-profdata")
        .with_extension(EXE_EXTENSION);

    // Build RA with PGO instrumentation
    let cmd_gather = ra_build_cmd.env(
        "RUSTFLAGS",
        format!("-Cprofile-generate={}", pgo_dir.to_str().unwrap()),
    );
    cmd_gather
        .run()
        .context("cannot build wgsl-analyzer with PGO instrumentation")?;

    let (train_path, label) = match &train_crate {
        PgoTrainingCrate::WgslAnalyzer => (PathBuf::from("."), "itself"),
        PgoTrainingCrate::GitHub(repo) => (
            download_package_for_training(shell, &pgo_dir, repo)?,
            repo.as_str(),
        ),
    };

    // Run RA either on itself or on a downloaded crate
    eprintln!("Training RA on {label}...");
    cmd!(
        shell,
        "target/{target}/release/wgsl-analyzer analysis-stats -q --run-all-ide-things {train_path}"
    )
    .run()
    .context("cannot generate PGO profiles")?;

    // Merge profiles into a single file
    let merged_profile = pgo_dir.join("merged.profdata");
    let profile_files = std::fs::read_dir(pgo_dir)?.filter_map(|entry| {
        let entry_path = entry.ok()?.path();
        (entry_path.extension() == Some(OsStr::new("profraw")))
            .then(|| entry_path.to_str().unwrap().to_owned())
    });
    cmd!(
        shell,
        "{llvm_profdata} merge {profile_files...} -o {merged_profile}"
    )
    .run()
    .context(
        "cannot merge PGO profiles. Do you have the rustup `llvm-tools` component installed?",
    )?;

    Ok(merged_profile)
}

/// Downloads a package from GitHub, stores it into `pgo_dir` and returns a path to it.
fn download_package_for_training(
    shell: &Shell,
    pgo_dir: &Path,
    repo: &str,
) -> anyhow::Result<PathBuf> {
    let mut splits = repo.splitn(2, '@');
    let repository = splits.next().unwrap();
    let revision = splits.next();

    // FIXME: switch to `--revision` here around 2035 or so
    #[expect(clippy::as_conversions, reason = "safe conversion")]
    let revision = if let Some(revision) = revision {
        &["--branch", revision] as &[&str]
    } else {
        &[]
    };

    let normalized_path = repository.replace('/', "-");
    let target_path = pgo_dir.join(normalized_path);
    cmd!(
        shell,
        "git clone --depth 1 https://github.com/{repo} {revision...} {target_path}"
    )
    .run()
    .with_context(|| "cannot download PGO training package from {repo}")?;

    Ok(target_path)
}

/// Helper function to create a build command for wgsl-analyzer
pub(crate) fn build_command<'shell>(
    shell: &'shell Shell,
    command: &str,
    target_name: &str,
    features: &[&str],
) -> Cmd<'shell> {
    cmd!(
        shell,
        "cargo {command} --manifest-path ./crates/wgsl-analyzer/Cargo.toml --bin wgsl-analyzer --target {target_name} {features...} --release"
    )
}

pub(crate) fn apply_pgo_to_cmd<'shell>(
    cmd: Cmd<'shell>,
    profile_path: &Path,
) -> Cmd<'shell> {
    cmd.env(
        "RUSTFLAGS",
        format!("-Cprofile-use={}", profile_path.to_str().unwrap()),
    )
}
