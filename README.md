# wgsl-analyzer

<!-- markdownlint-disable no-inline-html -->
<p align="center">
  <img
    src="https://github.com/wgsl-analyzer/wgsl-analyzer/blob/main/logo.svg"
    alt="wgsl-analyzer logo">
</p>
<!-- markdownlint-restore no-inline-html -->

[![Discord](https://img.shields.io/discord/691052431525675048.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/dZJ3JTbhaU)

[![codecov](https://codecov.io/github/wgsl-analyzer/wgsl-analyzer/graph/badge.svg)](https://app.codecov.io/github/wgsl-analyzer/wgsl-analyzer/tree/main)

wgsl-analyzer is a [language server][ls] plugin for the [WebGPU Shading language][WGSL] (WGSL).
It also supports [WGSL Extended Shader Language][WESL] (WESL).
Note: (support for WESL is experimental and in-progress)

You can use it with any editor that supports the [Language Server Protocol][lsp] (for example, VS Code, Vim, Emacs, Zed).

wgsl-analyzer's features include go-to-definition, type checking, code completion, and much more is planned.
wgsl-analyzer also supports <!-- integrated formatting (with wgslfmt) and --> integrated diagnostics (with naga).

Internally, wgsl-analyzer is structured as a set of libraries for analyzing Rust code.
See [Architecture][architecture] in the manual.

It comes with a [VS Code][VS Code] extension located in [./editors/code](./editors/code).
Due to the nature of the language server protocol, it should be possible to create plugins for other editors as well.

## Quick Start

See the [installation guide in the manual](https://wgsl-analyzer.github.io/book/installation.html).

[ls]: https://microsoft.github.io/language-server-protocol
[WGSL]: https://www.w3.org/TR/WGSL
[WESL]: https://wesl-lang.dev/
[lsp]: https://microsoft.github.io/language-server-protocol/
[architecture]: https://wgsl-analyzer.github.io/book/contributing/architecture.html
[VS Code]: https://code.visualstudio.com

## [Bevy]

wgsl-analyzer does not yet support automatically inferring dependencies from package managers such as [Cargo].

Relevant issues:

- [Infer path dependencies from `Cargo.toml` #1393](<https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1393>)
- [WESL Dependencies](<https://github.com/wgsl-analyzer/wgsl-analyzer/issues/976>)

It works if you "vendor" [Bevy] and create a [WESL package manifest][`wesl.toml`] with the explicit configuration.

1. Clone [the `bevyengine/bevy` repository](https://github.com/bevyengine/bevy) locally.
2. Set up a [`wesl.toml`] in your crate with shaders (next to [`Cargo.toml`]).

### Example [`wesl.toml`] for use with [Bevy]

```toml
# wesl.toml
edition = "2026_pre"

# if you do not put your shaders in the default `./shaders` folder, specify the correct value here
# root = "assets/shaders"

# it does not hurt to have all of these listed even if you do not use them, but you can trim it to your needs
[dependencies]
bevy_anti_alias = { path = "../bevy/crates/bevy_anti_alias" }
bevy_core_pipeline = { path = "../bevy/crates/bevy_core_pipeline" }
bevy_dev_tools = { path = "../bevy/crates/bevy_dev_tools" }
bevy_feathers = { path = "../bevy/crates/bevy_feathers" }
bevy_gizmos_render = { path = "../bevy/crates/bevy_gizmos_render" }
bevy_pbr = { path = "../bevy/crates/bevy_pbr" }
bevy_post_process = { path = "../bevy/crates/bevy_post_process" }
bevy_render = { path = "../bevy/crates/bevy_render" }
bevy_solari = { path = "../bevy/crates/bevy_solari" }
bevy_sprite_render = { path = "../bevy/crates/bevy_sprite_render" }
bevy_ui_render = { path = "../bevy/crates/bevy_ui_render" }
constants = { path = "../bevy/constants" } # temporary bevy hack
```

See [wgsl-analyzer: Project Discovery](https://wgsl-analyzer.github.io/book/discovery.html) for more information.

[`wesl.toml`]: https://wesl-lang.dev/spec/WeslToml
[`Cargo.toml`]: https://doc.rust-lang.org/cargo/reference/manifest.html
[Bevy]: https://bevy.org/
[Cargo]: https://doc.rust-lang.org/cargo/

## Building from source

The easiest way to build the language server and extension and install it all in one command:

```bash
cargo xtask install
```

The following instructions describe the manual steps:

The LSP server can be built using `cargo build --release -p wgsl-analyzer`.

The VS Code extension can either be built as a platform-specific extension which bundles the language server binary, or as a platform-independent one.

**1. Install node modules:**

`pnpm --dir editors/code install`

**2. Package extension:** (choose one)

Platform independent extension:

`pnpm --dir editors/code run package`

Platform-specific extension:

Copy the server binary (either `wgsl-analyzer` or `wgsl-analyzer.exe`) into `./editors/code/server/`:

```bash
mkdir editors/code/server
cp target/release/wgsl-analyzer editors/code/server/wgsl-analyzer
```

Next, run:

```bash
pnpm --dir editors/code run package --target <target> -o wgsl-analyzer-<target>.vsix
```

Example: `pnpm --dir editors/code run package --target linux-x64 -o wgsl-analyzer-linux-x64.vsix`

where the target is one of the targets listed as [platform-specific extension targets](https://code.visualstudio.com/api/working-with-extensions/publishing-extension#platformspecific-extensions).

The output is a file such as `editors/code/wgsl-analyzer-linux-x64.vsix`

**3. Install the extension:**

Open the VSIX with VS Code, for example, by running the VS Code command (<kbd>F1</kbd>):

`> Extensions: Install from VSIX...`
