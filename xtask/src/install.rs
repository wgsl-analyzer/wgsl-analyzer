//! Installs wgsl-analyzer language server and/or editor plugin.

use std::{env, path::PathBuf, str};

use anyhow::{Context as _, bail, format_err};
use xshell::{Shell, cmd};

use crate::flags::{self, Malloc, PgoTrainingCrate};

impl flags::Install {
    pub(crate) fn run(
        self,
        shell: &Shell,
    ) -> anyhow::Result<()> {
        if cfg!(target_os = "macos") {
            fix_path_for_mac(shell).context("Fix path for mac")?;
        }
        if let Some(server) = self.server() {
            install_server(shell, &server).context("install server")?;
        }
        if let Some(client) = self.client() {
            install_client(shell, &client).context("install client")?;
        }
        Ok(())
    }
}

#[derive(Clone)]
pub(crate) struct ClientOptions {
    pub(crate) code_binary: Option<String>,
}

const VS_CODES: &[&str] = &[
    "code",
    "code-exploration",
    "code-insiders",
    "codium",
    "code-oss",
];

pub(crate) struct ServerOptions {
    pub(crate) malloc: Malloc,
    pub(crate) dev_rel: bool,
    pub(crate) pgo: Option<PgoTrainingCrate>,
}

fn fix_path_for_mac(shell: &Shell) -> anyhow::Result<()> {
    let mut vscode_path: Vec<PathBuf> = {
        const COMMON_APP_PATH: &str =
            "/Applications/Visual Studio Code.app/Contents/Resources/app/bin";
        const ROOT_DIR: &str = "";
        let home_dir = shell.var("HOME").map_err(|error| {
            format_err!(
                "Failed getting HOME from environment with error: {}.",
                error
            )
        })?;
        #[expect(clippy::string_add, reason = "more concise")]
        [ROOT_DIR, &home_dir]
            .into_iter()
            .map(|directory| directory.to_owned() + COMMON_APP_PATH)
            .map(PathBuf::from)
            .filter(|path| path.exists())
            .collect()
    };

    if !vscode_path.is_empty() {
        let variables = shell
            .var_os("PATH")
            .context("Could not get PATH variable from env.")?;

        let mut paths = env::split_paths(&variables).collect::<Vec<_>>();
        paths.append(&mut vscode_path);
        let new_paths = env::join_paths(paths).context("build env PATH")?;
        shell.set_var("PATH", new_paths);
    }

    Ok(())
}

fn install_client(
    shell: &Shell,
    client_options: &ClientOptions,
) -> anyhow::Result<()> {
    let _dir = shell.push_dir("./editors/code");

    // Package extension.
    if cfg!(unix) {
        cmd!(shell, "npm --version")
            .run()
            .context("`npm` is required to build the VS Code plugin")?;
        cmd!(shell, "npm ci").run()?;

        cmd!(shell, "npm run package --scripts-prepend-node-path").run()?;
    } else {
        cmd!(shell, "cmd.exe /c npm --version")
            .run()
            .context("`npm` is required to build the VS Code plugin")?;
        cmd!(shell, "cmd.exe /c npm ci").run()?;

        cmd!(shell, "cmd.exe /c npm run package").run()?;
    }

    // Find the appropriate VS Code binary.
    let selected_code = client_options.code_binary.as_deref();
    let candidates: &[&str] = selected_code
        .as_ref()
        .map_or(VS_CODES, std::slice::from_ref);
    let code = candidates
        .iter()
        .copied()
        .find(|&bin| {
            if cfg!(unix) {
                cmd!(shell, "{bin} --version").read().is_ok()
            } else {
                cmd!(shell, "cmd.exe /c {bin}.cmd --version").read().is_ok()
            }
        })
        .ok_or_else(|| {
            format_err!(
                "Cannot execute `{} --version`. Perhaps it is not in $PATH?",
                candidates[0]
            )
        })?;

    // Install & verify.
    let installed_extensions = if cfg!(unix) {
        cmd!(
            shell,
            "{code} --install-extension wgsl-analyzer.vsix --force"
        )
        .run()?;
        cmd!(shell, "{code} --list-extensions").read()?
    } else {
        cmd!(
            shell,
            "cmd.exe /c {code}.cmd --install-extension wgsl-analyzer.vsix --force"
        )
        .run()?;
        cmd!(shell, "cmd.exe /c {code}.cmd --list-extensions").read()?
    };

    if !installed_extensions.contains("wgsl-analyzer") {
        bail!(
            "Could not install the Visual Studio Code extension. \
            Please make sure you have at least Node.js 22 (see https://github.com/wgsl-analyzer/wgsl-analyzer/tree/main/editors/code/package.json#L48) together with the latest version of VS Code installed and try again. \
            Note that installing via xtask install does not work for VS Code Remote, instead you will need to install the .vsix manually."
        );
    }

    Ok(())
}

fn install_server(
    shell: &Shell,
    options: &ServerOptions,
) -> anyhow::Result<()> {
    let features = options.malloc.to_features();
    let profile = if options.dev_rel {
        "dev-rel"
    } else {
        "release"
    };

    let cmd = cmd!(
        shell,
        "cargo install --path crates/wgsl-analyzer --profile={profile} --locked --force {features...}"
    );
    cmd.run()?;
    Ok(())
}
