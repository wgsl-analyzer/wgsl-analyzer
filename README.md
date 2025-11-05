# wgsl-analyzer

<!-- markdownlint-disable no-inline-html -->
<p align="center">
  <img
    src="https://github.com/wgsl-analyzer/wgsl-analyzer/blob/main/logo.svg"
    alt="wgsl-analyzer logo">
</p>
<!-- markdownlint-restore no-inline-html -->

[![Discord](https://img.shields.io/discord/691052431525675048.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/dZJ3JTbhaU)

wgsl-analyzer is a [language server][ls] plugin for the [WebGPU Shading language][WGSL] (WGSL).
It also supports [WGSL Extended Shader Language][WESL] (WESL).
Note: (support for WESL is experimental and in-progress)

You can use it with any editor that supports the [Language Server Protocol][lsp] (for example, VS Code, Vim, Emacs, Zed).

wgsl-analyzer's features include go-to-definition, type checking, code completion, and much more is planned.
wgsl-analyzer also supports integrated formatting (with wgslfmt) and integrated diagnostics (with naga).

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

## Building from source

The lsp server can be built using `cargo build --release -p wgsl-analyzer`.

The VS Code extension can either be built as a platform-specific extension which bundles the language server binary, or as a platform-independent one.

**1. Install node modules:**

`npm --prefix editors/code install`

**2. Package extension:** (choose one)

Platform independent extension:

`npm --prefix editors/code run package`

Platform-specific extension:

Copy the server binary (either `wgsl-analyzer` or `wgsl-analyzer.exe`) into `./editors/code/server/`:

```bash
mkdir editors/code/server
cp target/release/wgsl-analyzer editors/code/server/wgsl-analyzer
```

Next, run:

```bash
npm --prefix editors/code run package -- --target <target> -o wgsl-analyzer-<target>.vsix
```

Example: `npm --prefix editors/code run package -- --target linux-x64 -o wgsl-analyzer-linux-x64.vsix`

where the target is one of the targets listed as [platform-specific extension targets](https://code.visualstudio.com/api/working-with-extensions/publishing-extension#platformspecific-extensions).

The output is a file such as `editors/code/wgsl-analyzer-linux-x64.vsix`

**3. Install the extension:**

Open the vsix with VS Code, for example, by running the VS Code command (<kbd>F1</kbd>):

`> Extensions: Install from VSIX...`
