# wgsl-analyzer

wgsl-analyzer is a [language server](https://microsoft.github.io/language-server-protocol/) plugin for the [WGSL Shading language](https://gpuweb.github.io/gpuweb/wgsl/).

It comes with a [VS Code](https://code.visualstudio.com/) plugin located in [./editors/code](./editors/code), but due to the nature of the language server protocol it should be possible to create plugins for other editors as well.

## Installation

TODO - vscode extension published in marketplace

## Building from source

The lsp server can be built using `cargo build --release -p wgsl_analyzer`.

The vscode extension can either be built as a platform-specific extension which bundles the language server binary, or as a platform-independant one.

**Platform independant extension:**

`cd editors/code && npm run package`

**Platform-specific extension:**

Copy the server binary (either `wgsl_analyzer` or `wgsl_analyzer.exe`) to `./editors/code/out`, the run 
`npm run package -- --target <target> -o wgsl_analyzer-<target>.vsix` where the target is one of the targets listed as [platform-specific extension targets](https://code.visualstudio.com/api/working-with-extensions/publishing-extension#platformspecific-extensions).

## Design

The design is heavily inspired (and in large parts copied from) [rust-analyzer](https://github.com/rust-analyzer/rust-analyzer).
See [rust-analyzer/docs/dev/architecture.md](https://github.com/rust-analyzer/rust-analyzer/blob/master/docs/dev/architecture.md) for a summary of the architecture.