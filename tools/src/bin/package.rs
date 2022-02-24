use std::path::{Path, PathBuf};

use xshell::{cmd, cp, pushd, rm_rf};

type Result<T, E = anyhow::Error> = std::result::Result<T, E>;

const HELP_STR: &str = "Usage: cargo run --bin package --target linux-x64";

#[derive(Debug)]
struct Args {
    target: String,
    install: bool,
}

fn parse_args() -> Result<Args, lexopt::Error> {
    let mut parser = lexopt::Parser::from_env();

    let mut target = None;
    let mut install = false;

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
            _ => return Err(arg.unexpected()),
        }
    }
    Ok(Args {
        target: target.ok_or("missing argument --target")?,
        install,
    })
}

fn main() -> Result<()> {
    let args = parse_args()?;

    let extension = package(&args.target)?;

    if args.install {
        cmd!("code --install-extension {extension} --force").run()?;
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

fn compile(rust_target: &str) -> Result<PathBuf> {
    cmd!("rustup target add {rust_target}").run()?;
    let output =
        cmd!("cargo build --release --package wgsl_analyzer --target {rust_target} --message-format=json").read()?;

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

fn package(target: &str) -> Result<PathBuf> {
    let (_, rust_target) = TARGETS
        .iter()
        .find(|(t, _)| *t == target)
        .ok_or_else(|| anyhow::anyhow!("invalid target"))?;

    let src = compile(rust_target)?;
    let out = Path::new("editors/code/out");
    let dst = out.join("wgsl_analyzer");

    xshell::mkdir_p(out)?;

    cp(src, &dst)?;

    let _dir = pushd("editors/code")?;
    cmd!("npm run package --silent -- -o wgsl-analyzer-{target}.vsix --target {target}").run()?;

    rm_rf(&dst)?;

    Ok(PathBuf::from(format!(
        "editors/code/wgsl-analyzer-{}.vsix",
        target
    )))
}
