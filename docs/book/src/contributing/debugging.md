# Debugging VS Code plugin and the language server

## Prerequisites

- Install [LLDB](https://lldb.llvm.org) and the [LLDB Extension].
- Open the root folder in VS Code. Here you can access the preconfigured debug setups.

![Debug options view](https://user-images.githubusercontent.com/36276403/74611090-92ec5380-5101-11ea-8a41-598f51f3f3e3.png)

[LLDB Extension]: <https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb>

- Install all TypeScript dependencies

  ```bash
  cd editors/code
  npm ci
  ```

## Common knowledge

- All debug configurations open a new `[Extension Development Host]` VS Code instance
where **only** the `wgsl-analyzer` extension being debugged is enabled.
- To activate the extension you need to open any WGSL project folder in `[Extension Development Host]`.

## Debug TypeScript VS Code extension

- `Run Installed Extension` - runs the extension with the globally installed `wgsl-analyzer` binary.
- `Run Extension (Debug Build)` - runs extension with the locally built LSP server (`target/debug/wgsl-analyzer`).

TypeScript debugging is configured to watch your source edits and recompile.
To apply changes to an already running debug process, press <kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>P</kbd>
and run the following command in your `[Extension Development Host]`

```text
> Developer: Reload Window
```

## Debug WGSL LSP server

- When attaching a debugger to an already running `wgsl-analyzer` server on Linux,
  you might need to enable `ptrace` for unrelated processes by running:

  ```bash
  echo 0 | sudo tee /proc/sys/kernel/yama/ptrace_scope
  ```

- By default, the LSP server is built without debug information.
  To enable it, you will need to change `Cargo.toml`:

  ```toml
    [profile.dev]
    debug = 2
  ```

1. Select `Run Extension (Debug Build)` to run your locally built `target/debug/wgsl-analyzer`.
2. In the original VS Code window once again select the `Attach To Server` debug configuration.
3. A list of running processes should appear. Select the `wgsl-analyzer` from this repo.
4. Navigate to `crates/wgsl-analyzer/src/main_loop.rs` and add a breakpoint to the `on_request` function.
5. Go back to the `[Extension Development Host]` instance and hover over a Rust variable and your breakpoint should hit.

If you need to debug the server from the very beginning, including its initialization
code, you can use the `--wait-dbg` command line argument or `WA_WAIT_DBG` environment variable.
The server will spin at the beginning of the `try_main` function (see `crates\wgsl-analyzer\src\bin\main.rs`)

```rust
let mut d = 4;
while d == 4 { // set a breakpoint here and change the value
	d = 4;
}
```

However for this to work, you will need to enable debug_assertions in your build

```bash
WGSLFLAGS='--cfg debug_assertions' cargo build --release
```

## Demo

- [Debugging TypeScript VScode extension](https://www.youtube.com/watch?v=T-hvpK6s4wM).
- [Debugging WGSL LSP server](https://www.youtube.com/watch?v=EaNb5rg4E0M).

## Troubleshooting

### Cannot find the `wgsl-analyzer` process

It could be a case of just jumping the gun.

The `wgsl-analyzer` is only started once the `onLanguage:wgsl` activation.

Make sure you open a WGSL file in the `[Extension Development Host]` and try again.

### Cannot connect to `wgsl-analyzer`

Make sure you have run `echo 0 | sudo tee /proc/sys/kernel/yama/ptrace_scope`.

By default this should reset back to 1 every time you log in.

### Breakpoints are never being hit

Check your version of `lldb`.
If it is version 6 and lower, use the `classic` adapter type.
It is `lldb.adapterType` in settings file.

If you are running `lldb` version 7, change the lldb adapter type to `bundled` or `native`.
