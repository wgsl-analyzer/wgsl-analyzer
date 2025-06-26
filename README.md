# `wgsl-analyzer`

![wgsl-analyzer logo](logo.svg)

[![Discord](https://img.shields.io/discord/691052431525675048.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/dZJ3JTbhaU)

## What `wgsl-analyzer` is

`wgsl-analyzer` is a [language server](https://microsoft.github.io/language-server-protocol) plugin for the [WGSL Shading language](https://www.w3.org/TR/WGSL).
It also supports [WESL](https://wesl-lang.dev/)

It comes with a [VS Code](https://code.visualstudio.com) plugin located in [./editors/code](./editors/code).
Due to the nature of the language server protocol, it should be possible to create plugins for other editors as well.

## Installation

### VS Code

The extension is [published on the marketplace](https://marketplace.visualstudio.com/items?itemName=wgsl-analyzer.wgsl-analyzer), so you can simply download the extension like any other.

> [!NOTE]
> If you are not using a platform for which the vscode extension ships prebuilt binaries (currently only windows-x64, linux-x64 and macos-x64), then you need to compile the language server yourself:
>
> ```bash
> cargo install --git https://github.com/wgsl-analyzer/wgsl-analyzer.git wgsl-analyzer
> ```
>
> Specify the server path in the settings:
>
> ```json
> {
>     "wgsl-analyzer.server.path": "~/.cargo/bin/wgsl-analyzer"
> }
> ```

### Other editors

See: [Other Editors](./docs/book/src/other_editors.md)

## Configuration

Configuration for the VS Code plugin can be found in its subdirectory: [./editors/code/README.md](./editors/code/README.md).

## Building from source

The lsp server can be built using `cargo build --release -p wgsl-analyzer`.

The vscode extension can either be built as a platform-specific extension which bundles the language server binary, or as a platform-independent one.

**Install node modules:**

`cd editors/code && npm install`

**Platform independent extension:**

`cd editors/code && npm run package`

**Platform-specific extension:**

Copy the server binary (either `wgsl-analyzer` or `wgsl-analyzer.exe`) into `./editors/code/out/`, then run:

```bash
npm run package -- --target <target> -o wgsl-analyzer-<target>.vsix
```

where the target is one of the targets listed as [platform-specific extension targets](https://code.visualstudio.com/api/working-with-extensions/publishing-extension#platformspecific-extensions).

This can be done automatically with `cargo run --bin package -- --target linux-x64 --install`.

## Design

The design is heavily inspired (and in large parts copied from) [rust-analyzer](https://github.com/rust-lang/rust-analyzer).

See [wgsl-analyzer architecture](https://wgsl-analyzer.github.io/book/contributing/architecture.html) for a summary of the architecture.

(Also see [rust-analyzer architecture](https://rust-analyzer.github.io/book/contributing/architecture.html) for a summary of the original architecture.)
