use std::path::{Path, PathBuf};

use xshell::cmd;

type Result<T, E = anyhow::Error> = std::result::Result<T, E>;

const HELP_STR: &str =
    "Usage: cargo run --bin package -- --target linux-x64 [--no-prebuilt-binary] [--install]";

#[derive(Debug)]
struct Args {
    target: String,
    install: bool,
    no_prebuilt_binary: bool,
}

fn parse_args() -> Result<Args, lexopt::Error> {
    let mut parser = lexopt::Parser::from_env();

    let mut target = None;
    let mut install = false;
    let mut no_prebuilt_binary = false;

    while let Some(arg) = parser.next()? {
        match arg {
            lexopt::Arg::Long("help") => {
                println!("{}", HELP_STR);
                std::process::exit(0);
            }
            lexopt::Arg::Long("target") => {
                target = Some(parser.value()?.into_string()?);
            }
            lexopt::Arg::Long("install") => install = true,
            lexopt::Arg::Long("no-prebuilt-binary") => no_prebuilt_binary = true,
            _ => return Err(arg.unexpected()),
        }
    }
    Ok(Args {
        target: target.ok_or("missing argument --target")?,
        install,
        no_prebuilt_binary,
    })
}

// Windows needs a workaround for npm.cmd, see https://github.com/rust-lang/rust/issues/42791
#[cfg(windows)]
const NPM: &'static str = "npm.cmd";
#[cfg(not(windows))]
const NPM: &str = "npm";

#[cfg(windows)]
const CODE: &'static str = "code.cmd";
#[cfg(not(windows))]
const CODE: &str = "code";

fn main() -> Result<()> {
    let args = parse_args()?;

    let sh = xshell::Shell::new()?;

    let extension = package(&sh, &args.target, !args.no_prebuilt_binary)?;

    if args.install {
        cmd!(sh, "{CODE} --install-extension {extension} --force").run()?;
    }

    Ok(())
}

const TARGETS: &[(&str, &str)] = &[
    ("win32-x64", "x86_64-pc-windows-msvc"),
    ("win32-ia32", "x86_64-pc-windows-msvc"),
    ("win32-arm64", "aarch64-pc-windows-msvc"),
    ("linux-x64", "x86_64-unknown-linux-gnu"),
    ("linux-arm64", "aarch64-unknown-linux-gnu"),
    ("alpine-x64", "x86_64-unknown-linux-musl"),
    ("alpine-arm64", "aarch64-unknown-linux-musl"),
    ("darwin-x64", "x86_64-apple-darwin"),
    ("darwin-arm64", "aarch64-apple-darwin"),
    // linux-armhf
];

fn compile(sh: &xshell::Shell, rust_target: &str) -> Result<PathBuf> {
    cmd!(sh, "rustup target add {rust_target}").run()?;
    let output =
        cmd!(sh, "cargo build --release --package wgsl_analyzer --target {rust_target} --message-format=json").read()?;

    let executable_path = serde_json::Deserializer::from_str(&output)
        .into_iter::<serde_json::Value>()
        .filter_map(Result::ok)
        .filter(|value| {
            value["reason"] == "compiler-artifact"
                && value["target"]["crate_types"]
                    .as_array()
                    .map_or(false, |arr| arr.iter().any(|v| v == "bin"))
        })
        .filter_map(|value| value["executable"].as_str().map(|s| s.to_owned()))
        .next()
        .ok_or_else(|| anyhow::anyhow!("cargo json output doesn't report executable path"))?;

    Ok(PathBuf::from(executable_path))
}

fn package(sh: &xshell::Shell, target: &str, prebuilt_binary: bool) -> Result<PathBuf> {
    let (_, rust_target) = TARGETS.iter().find(|(t, _)| *t == target).ok_or_else(|| {
        let target_names = TARGETS
            .iter()
            .map(|(t, _)| *t)
            .collect::<Vec<_>>()
            .join(", ");
        anyhow::anyhow!("invalid target, expected one of {}", target_names)
    })?;

    if !Path::new("editors/code").exists() {
        Err(anyhow::anyhow!("./editors/code folder does not exist, run this script from the root of the repository."))?;
    }
    let out = Path::new("editors/code/out");
    let mut dst = out.join("wgsl_analyzer");
    if prebuilt_binary {
        let src = compile(sh, rust_target)?;
        sh.create_dir(out)?;
        if let Some(ext) = src.extension() {
            dst.set_extension(ext);
        }
        sh.copy_file(src, &dst)?;
    }

    {
        let _dir = sh.push_dir("editors/code");
        cmd!(
            sh,
            "{NPM} run package --silent -- -o wgsl-analyzer-{target}.vsix --target {target}"
        )
        .run()?;
    }

    sh.remove_path(&dst)?;

    Ok(PathBuf::from(format!(
        "editors/code/wgsl-analyzer-{}.vsix",
        target
    )))
}
