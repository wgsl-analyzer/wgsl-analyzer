<img src="https://raw.githubusercontent.com/wgsl-analyzer/wgsl-analyzer/master/logo.svg" alt="wgsl-analyzer logo">

#

[![Discord](https://img.shields.io/discord/691052431525675048.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/dZJ3JTbhaU)

## What wgsl-analyzer is

wgsl-analyzer is a [language server](https://microsoft.github.io/language-server-protocol/) plugin for the [WGSL Shading language](https://gpuweb.github.io/gpuweb/wgsl/).

It comes with a [VS Code](https://code.visualstudio.com/) plugin located in [./editors/code](./editors/code), but due to the nature of the language server protocol it should be possible to create plugins for other editors as well.

## Installation

### VS Code

The extension is [published on the marketplace](https://marketplace.visualstudio.com/items?itemName=wgsl-analyzer.wgsl-analyzer), so you can simply download the extension like any other.

If you are not using a platform for which the vscode extension ships prebuilt binaries (currently only windows-x64, linux-x64 and macos-x64), then you need to compile the language server yourself:

```sh
cargo install --git https://github.com/wgsl-analyzer/wgsl-analyzer.git wgsl_analyzer
```

Specify the server path in the settings:

```json
{
    "wgsl-analyzer.server.path": "~/.cargo/bin/wgsl_analyzer"
}
```

### Neovim / Vim (using coc.nvim)

- Requires CoC to be installed: https://github.com/neoclide/coc.nvim
- Requires cargo to be installed to build binaries: 

1. Install the language server

    ```sh
    cargo install --git https://github.com/wgsl-analyzer/wgsl-analyzer.git wgsl_analyzer
    ```
    
    (if you are not familiar with using and setting up cargo, you might run into problems finding your binary.
    Ensure that $HOME/.cargo/bin is in your $PATH. More Info about $PATH: https://linuxconfig.org/linux-path-environment-variable)

2. open Neovim / Vim and type `:CocConfig` to configure coc.nvim.

3. under `.languageserver: { ... }` create a new field named "wgsl". The field should look like this:

    ```jsonc
    //  {
    //    "languageserver": {
            "wgsl": {
              "command": "wgsl_analyzer", // alternatively you can specify the absolute path to your binary.
              "filetypes": ["wgsl"],
            },
    //      ...
    //  }
    ```

4. In order for your editor to recognize wgsl files as such, you need to put this into your `vim.rc`

    ```vim
    " Recognize wgsl
    au BufNewFile,BufRead *.wgsl set filetype=wgsl
    ```

### Neovim (using lsp)

1. Install the `wgsl_analyzer` language server
2. Configure the wgsl filetype

    ```lua
    vim.api.nvim_create_autocmd({ "BufNewFile", "BufRead" }, {
      pattern = "*.wgsl",
      callback = function()
        vim.bo.filetype = "wgsl"
      end,
    })
    ```

3. Configure the nvim lsp

    ```lua
    local lspconfig = require('lspconfig')
    lspconfig.wgsl_analyzer.setup({})
    ```

### Emacs (using lsp-mode)

 - Assumes you are using wgsl-mode: https://github.com/acowley/wgsl-mode

1. Install the language server 
```sh
cargo install --git https://github.com/wgsl-analyzer/wgsl-analyzer wgsl_analyzer
```
2. Add the following to your init.el
```elisp
(with-eval-after-load 'lsp-mode
  (add-to-list 'lsp-language-id-configuration '(wgsl-mode . "wgsl"))
  (lsp-register-client (make-lsp-client :new-connection (lsp-stdio-connection "wgsl_analyzer")
                                        :activation-fn (lsp-activate-on "wgsl")
                                        :server-id 'wgsl_analyzer)))
```

## Configuration

Configuration for the VS Code plugin can be found in its subdirectory: [./editors/code/README.md](./editors/code/README.md).

## Building from source

The lsp server can be built using `cargo build --release -p wgsl_analyzer`.

The vscode extension can either be built as a platform-specific extension which bundles the language server binary, or as a platform-independant one.

**Install node modules:**

`cd editors/code && npm install`

**Platform independent extension:**

`cd editors/code && npm run package`

**Platform-specific extension:**

Copy the server binary (either `wgsl_analyzer` or `wgsl_analyzer.exe`) into `./editors/code/out/`, then run
`npm run package -- --target <target> -o wgsl_analyzer-<target>.vsix` where the target is one of the targets listed as [platform-specific extension targets](https://code.visualstudio.com/api/working-with-extensions/publishing-extension#platformspecific-extensions).

This can be done automatically with `cargo run --bin package -- --target linux-x64 --install`.

## Design

The design is heavily inspired (and in large parts copied from) [rust-analyzer](https://github.com/rust-analyzer/rust-analyzer).
See [rust-analyzer/docs/dev/architecture.md](https://github.com/rust-analyzer/rust-analyzer/blob/master/docs/dev/architecture.md) for a summary of the architecture.


## Development

### Release process

The extension gets automatically packaged and released to the marketplace in CI.

1. update version in `package.json`
2. run `./tools/update_version.sh`
3. commit and tag with `vx.y.z`
4. `git push && git push --tags`
