use anyhow::Context as _;
use xshell::{Shell, cmd};

use crate::flags::BuildWeb;

impl BuildWeb {
    pub(crate) fn run(
        &self,
        shell: &Shell,
    ) -> anyhow::Result<()> {
        let _dir = shell.push_dir("./js");

        let cmd_suffix = if self.release { "" } else { ":debug" };

        // Check build requirements
        cmd!(shell, "rustc +nightly --version")
            .run()
            .context("a nightly toolchain is required to build the web package")?;

        cmd!(shell, "emcc --version")
            .run()
            .context("`emscripten` is required to build the web package")?;

        // `build:wasm` stages wgsl_analyzer.{js,wasm}, `build` stages worker.js
        // beside them. A host needs all three, so both steps run.
        if cfg!(unix) {
            cmd!(shell, "pnpm --version")
                .run()
                .context("`pnpm` is required to build the web package")?;

            cmd!(shell, "pnpm install --frozen-lockfile").run()?;

            // Run build
            cmd!(
                shell,
                "pnpm --filter wgsl-analyzer-web run build:wasm{cmd_suffix}"
            )
            .run()?;

            cmd!(shell, "pnpm --filter wgsl-analyzer-web run build").run()?;
        } else {
            cmd!(shell, "cmd.exe /c pnpm --version")
                .run()
                .context("`pnpm` is required to build the web package")?;

            cmd!(shell, "cmd.exe /c pnpm install --frozen-lockfile").run()?;

            // Run build
            cmd!(
                shell,
                "cmd.exe /c pnpm --filter wgsl-analyzer-web run build:wasm{cmd_suffix}"
            )
            .run()?;

            cmd!(
                shell,
                "cmd.exe /c pnpm --filter wgsl-analyzer-web run build"
            )
            .run()?;
        }

        Ok(())
    }
}
