# Other Editors

`wgsl-analyzer` works with any editor that supports the [Language Server Protocol](https://microsoft.github.io/language-server-protocol).

This page assumes that you have already [installed the `wgsl-analyzer` binary](./wgsl-analyzer_binary.md).

## Emacs (using lsp-mode)

- Assumes you are using `wgsl-mode`: <https://github.com/acowley/wgsl-mode>

1. Install the language server

    ```bash
    cargo install --git https://github.com/wgsl-analyzer/wgsl-analyzer wgsl-analyzer
    ```

2. Add the following to your init.el

    ```emacs-lisp
    (with-eval-after-load 'lsp-mode
    (add-to-list 'lsp-language-id-configuration '(wgsl-mode . "wgsl"))
    (lsp-register-client (make-lsp-client :new-connection (lsp-stdio-connection "wgsl-analyzer")
                                          :activation-fn (lsp-activate-on "wgsl")
                                          :server-id 'wgsl-analyzer)))
    ```

### [Eglot]

[Eglot] is the more minimalistic and lightweight LSP client for Emacs, integrates well with existing Emacs functionality and is built into Emacs starting from release 29.

After installing [Eglot], e.g. via `M-x package-install` (not needed from Emacs 29), you can enable it via the `M-x eglot` command or load it automatically in `wgsl-mode` via

```emacs-lisp
(add-hook 'wgsl-mode-hook 'eglot-ensure)
```

For more detailed instructions and options see the [Eglot manual](https://joaotavora.github.io/eglot) (also available from Emacs via `M-x info`) and the [Eglot readme](https://github.com/joaotavora/eglot/blob/master/README.md).

Eglot does not support the `wgsl-analyzer` extensions to the language-server protocol and does not aim to do so in the future.
The [eglot-x](https://github.com/nemethf/eglot-x#wgsl-analyzer-extensions) package adds experimental support for those LSP extensions.

[Eglot]: <https://github.com/joaotavora/eglot>

### LSP Mode

LSP-mode is the original LSP-client for emacs.
Compared to Eglot it has a larger codebase and supports more features, like LSP protocol extensions.
With extension packages like [LSP UI](https://github.com/emacs-lsp/lsp-mode) it offers a lot of visual eyecandy.
Further it integrates well with [DAP mode](https://github.com/emacs-lsp/dap-mode) for support of the Debug Adapter Protocol.

You can install LSP-mode via `M-x package-install` and then run it via the `M-x lsp` command or load it automatically in WGSL/WESL buffers with

```emacs-lisp
(add-hook 'wgsl-mode-hook 'lsp-deferred)
```

For more information on how to set up LSP mode and its extension package see the instructions in the [LSP mode manual](https://emacs-lsp.github.io/lsp-mode/page/installation).
Also see the [`wgsl-analyzer` section](https://emacs-lsp.github.io/lsp-mode/page/lsp-wgsl-analyzer) for `wgsl-analyzer` specific options and commands, which you can optionally bind to keys.

## Vim/Neovim

There are several LSP client implementations for Vim or Neovim:

### Using `coc-wgsl-analyzer`

1. Install coc.nvim by following the instructions at [coc.nvim](https://github.com/neoclide/coc.nvim) (Node.js required)

2. Run `:CocInstall coc-wgsl-analyzer` to install [`coc-wgsl-analyzer`](https://github.com/wgsl-analyzer/coc-wgsl-analyzer), this extension implements *most* of the features supported in the VS Code extension:
    - automatically install and upgrade stable/nightly releases
    - same configurations as VS Code extension, `wgsl-analyzer.server.path`, `wgsl-analyzer.cargo.features` etc.
    - same commands too, `wgsl-analyzer.analyzerStatus`, `wgsl-analyzer.ssr` etc.
    - inlay hints for variables and method chaining, *Neovim Only*

> [!NOTE]
> `coc-wgsl-analyzer` is capable of installing or updating the `wgsl-analyzer` binary on its own.

<!--  -->
> [!NOTE]
> for code actions, use `coc-codeaction-cursor` and `coc-codeaction-selected`; `coc-codeaction` and `coc-codeaction-line` are unlikely to be useful.

### Using LanguageClient-neovim

1. Install LanguageClient-neovim by following [the instructions](https://github.com/autozimu/LanguageClient-neovim)
    - The GitHub project wiki has extra tips on configuration

2. Configure by adding this to your Vim/Neovim config file (replacing the existing WGSL or WESL-specific line if it exists):

    ```vim
    let g:LanguageClient_serverCommands = {
    \ 'wgsl': ['wgsl-analyzer'],
    \ 'wesl': ['wgsl-analyzer'],
    \ }
    ```

### Using lsp

1. Install the `wgsl-analyzer` language server
2. Configure the `.wgsl` and `.wesl` filetype

    ```lua
    vim.api.nvim_create_autocmd({ "BufNewFile", "BufRead" }, {
      pattern = {'*.wgsl', '*.wesl'},
      callback = function()
        vim.bo.filetype = "wesl"
      end,
    })
    ```

> [!TIP]
> Create separate file associations for a conventional modular setup.

- Create `.config/nvim/ftdetect/wgsl.lua` and `.config/nvim/ftdetect/wesl.lua`.

    ```lua
    vim.api.nvim_create_autocmd({ "BufRead", "BufNewFile" }, { pattern = "*.wgsl",  command = "setfiletype wgsl" })
    ```

    ```lua
    vim.api.nvim_create_autocmd({ "BufRead", "BufNewFile" }, { pattern = "*.wesl",  command = "setfiletype wesl" })
    ```

3. Configure the nvim lsp

    ```lua
    local lspconfig = require('lspconfig')
    lspconfig.wgsl_analyzer.setup({})
    ```

### Using coc.nvim

- Requires CoC to be installed: <https://github.com/neoclide/coc.nvim>
- Requires cargo to be installed to build binaries:

1. Install the language server

    ```bash
    cargo install --git https://github.com/wgsl-analyzer/wgsl-analyzer.git wgsl-analyzer
    ```

    (if you are not familiar with using and setting up cargo, you might run into problems finding your binary.
    Ensure that $HOME/.cargo/bin is in your $PATH. More Info about $PATH: <https://linuxconfig.org/linux-path-environment-variable>)

2. open Neovim / Vim and type `:CocConfig` to configure coc.nvim.

3. under `.languageserver: { ... }` create a new field `"wgsl-analyzer-language-server"`. The field should look like this:

    ```jsonc
    //  {
    //    "languageserver": {
            "wgsl-analyzer-language-server": {
              "command": "wgsl-analyzer", // alternatively you can specify the absolute path to your binary.
              "filetypes": ["wgsl", "wesl"],
            },
    //      ...
    //  }
    ```

4. In order for your editor to recognize WGSL files as such, you need to put this into your `vim.rc`

    ```vim
    " Recognize wgsl
    au BufNewFile,BufRead *.wgsl set filetype=wgsl
    ```

### Using nvim-cmp/cmp_nvim_lsp

- Requires [nvim-cmp](https://github.com/hrsh7th/nvim-cmp) and [cmp_nvim_lsp](https://github.com/hrsh7th/cmp-nvim-lsp) to be installed. Your existing setup should look similar to this:

    ```lua
    local capabilities = vim.lsp.protocol.make_client_capabilities()
    capabilities = vim.tbl_deep_extend("force", capabilities, require("cmp_nvim_lsp").default_capabilities())

    local lspconfig = require("lspconfig")
    ```

- Pass capabilities to the `wgsl-analyzer` setup:

    ```lua
    lspconfig.wgsl_analyzer.setup({
       filetypes = { "wgsl", "wesl" },
       capabilities = capabilities,
    })
    ```


### YouCompleteMe

Install YouCompleteMe by following [the instructions](https://github.com/ycm-core/YouCompleteMe#installation).

`wgsl-analyzer` is the default in ycm, it should work out of the box.

### ALE

To use the LSP server in [ale](https://github.com/dense-analysis/ale):

```vim
let g:ale_linters = {'wgsl': ['analyzer'], 'wesl': ['analyzer']}
```

### nvim-lsp

Neovim 0.5 has built-in language server support.
For a quick start configuration of `wgsl-analyzer`, use [neovim/nvim-lspconfig](https://github.com/neovim/nvim-lspconfig#wgsl-analyzer).
Once `neovim/nvim-lspconfig` is installed, use `lua require'lspconfig'.wgsl_analyzer.setup({})` in your `init.vim`.

You can also pass LSP settings to the server:

```lua
lua << EOF
local lspconfig = require'lspconfig'

local on_attach = function(client)
  require'completion'.on_attach(client)
end

lspconfig.wgsl_analyzer.setup({
  on_attach = on_attach,
  settings = {
    ["wgsl-analyzer"] = {
      
    }
  }
})
EOF
```

If you are running Neovim 0.10 or later, you can enable inlay hints via `on_attach`:

```lua
lspconfig.wgsl_analyzer.setup({
  on_attach = function(client, bufnr)
    vim.lsp.inlay_hint.enable(true, { bufnr = bufnr })
  end
})
```

Note that the hints are only visible after `wgsl-analyzer` has finished loading **and** you have to edit the file to trigger a re-render.

### vim-lsp

vim-lsp is installed by following [the plugin instructions](https://github.com/prabirshrestha/vim-lsp).
It can be as simple as adding this line to your `.vimrc`:

```vim
Plug 'prabirshrestha/vim-lsp'
```

Next you need to register the `wgsl-analyzer` binary.
If it is available in `$PATH`, you may want to add this to your `.vimrc`:

```vim
if executable('wgsl-analyzer')
  au User lsp_setup call lsp#register_server({
    \   'name': 'wgsl-analyzer Language Server',
    \   'cmd': {server_info->['wgsl-analyzer']},
    \   'whitelist': ['wgsl', 'wesl'],
    \ })
endif
```

There is no dedicated UI for the server configuration, so you would need to send any options as a value of the `initialization_options` field, as described in the [Configuration](./configuration.md) section.
Here is an example of how to enable the proc-macro support:

```vim
if executable('wgsl-analyzer')
  au User lsp_setup call lsp#register_server({
    \   'name': 'wgsl-analyzer Language Server',
    \   'cmd': {server_info->['wgsl-analyzer']},
    \   'whitelist': ['wgsl', 'wesl'],
    \   'initialization_options': {
    \     'cargo': {
    \       'buildScripts': {
    \         'enable': v:true,
    \       },
    \     },
    \     'procMacro': {
    \       'enable': v:true,
    \     },
    \   },
    \ })
endif
```

## Sublime Text

### Sublime Text 4

Follow the instructions in [LSP-rust-analyzer](https://github.com/sublimelsp/LSP-rust-analyzer), but substitute `rust` with `wgsl` where applicable.

Install [LSP-file-watcher-chokidar](https://packagecontrol.io/packages/LSP-file-watcher-chokidar) to enable file watching (`workspace/didChangeWatchedFiles`).

### Sublime Text 3

- Install the [LSP package](https://packagecontrol.io/packages/LSP).
- From the command palette, run `LSP: Enable Language Server Globally` and select `wgsl-analyzer`.

If it worked, you should see "wgsl-analyzer, Line X, Column Y" on the left side of the status bar, and after waiting a bit, functionalities like tooltips on hovering over variables should become available.

If you get an error saying `No such file or directory: 'wgsl-analyzer'`, see the [`wgsl-analyzer` binary installation](./wgsl-analyzer_binary.md) section.

## GNOME Builder

No support.

## Eclipse IDE

No support.

## Kate Text Editor

Support for the language server protocol is built into Kate through the LSP plugin, which is included by default.

To change `wgsl-analyzer` config options, start from the following example and put it into Kate's "User Server Settings" tab (located under the LSP Client settings):

```json
{
  "servers": {
    "wgsl": {
      "command": ["wgsl-analyzer"],
      "url": "https://github.com/wgsl-analyzer/wgsl-analyzer",
      "highlightingModeRegex": "^WGSL$"
    },
    "wesl": {
      "command": ["wgsl-analyzer"],
      "url": "https://github.com/wgsl-analyzer/wgsl-analyzer",
      "highlightingModeRegex": "^WESL$"
    }
  }
}
```

Then click on apply, and restart the LSP server for your WGSL code or WESL project.

## juCi++

[juCi++](https://gitlab.com/cppit/jucipp) has built-in support for the language server protocol.

## Kakoune

[Kakoune](https://kakoune.org) supports LSP with the help of [`kak-lsp`](https://github.com/kak-lsp/kak-lsp).
Follow the [instructions](https://github.com/kak-lsp/kak-lsp#installation) to install `kak-lsp`.
To configure `kak-lsp`, refer to the [configuration section](https://github.com/kak-lsp/kak-lsp#configuring-kak-lsp).
It is about copying the [configuration file](https://github.com/kak-lsp/kak-lsp/blob/master/kak-lsp.toml) to the right place.
The latest versions should use `wgsl-analyzer` by default.

Finally, you need to configure Kakoune to talk to `kak-lsp` (see [Usage section](https://github.com/kak-lsp/kak-lsp#usage)).
A basic configuration will only get you LSP but you can also activate inlay diagnostics and auto-formatting on save.
The following might help you understand all of this:

```kakoune
eval %sh{kak-lsp --kakoune -s $kak_session}  # Not needed if you load it with plug.kak.
hook global WinSetOption filetype=(wgsl|wesl) %{
  # Enable LSP
  lsp-enable-window

  # Auto-formatting on save
  hook window BufWritePre .* lsp-formatting-sync

  # Configure inlay hints (only on save)
  hook window -group wgsl-inlay-hints BufWritePost .* wgsl-analyzer-inlay-hints
  hook -once -always window WinSetOption filetype=.* %{
    remove-hooks window wgsl-inlay-hints
  }
}
```

## Helix

[Helix](https://docs.helix-editor.com) supports LSP by default.
However, it will not install `wgsl-analyzer` automatically.
You can follow instructions for [installing the `wgsl-analyzer` binary](./wgsl-analyzer_binary.md).

## Visual Studio 2022

No support.

## Lapce

No support.

## Zed

No support.

## IntelliJ IDEs

This includes:

- IntelliJ IDEA Ultimate
- WebStorm
- PhpStorm
- PyCharm Professional
- DataSpell
- RubyMine
- CLion
- Aqua
- DataGrip
- GoLand
- Rider
- RustRover

No support.

See #207
