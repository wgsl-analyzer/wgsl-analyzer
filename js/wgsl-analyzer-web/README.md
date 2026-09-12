# wgsl-analyzer-web

Runs the `wgsl-analyzer` language server in a Web Worker.

The server is the `wgsl-analyzer` binary compiled to
`wasm32-unknown-emscripten`, running its ordinary `main_loop` and speaking LSP
over stdin and stdout. The package hosts it, seeds a workspace into the in-memory
filesystem, and exposes the message stream.

## Building

```bash
# `rust-std` for the `emscripten` target is compiled without the wasm `atomics` feature, so it cannot be linked with `-pthread`
rustup +nightly component add rust-src
source /path/to/emsdk/emsdk_env.sh

cd js
pnpm --filter wgsl-analyzer-web run build:wasm   # add --debug for a faster build
pnpm --filter wgsl-analyzer-web run build
```

Between them the two commands stage three files in `dist/assets/`:
`wgsl_analyzer.js` and `wgsl_analyzer.wasm` from `build:wasm`, and `worker.js`
from `build`. All three must be served from the same directory, under exactly
those names. The glue spawns its pthread pool with
`new Worker(new URL("wgsl_analyzer.js", import.meta.url))`, so renaming the glue
makes every pthread 404. That is why the build script renames cargo's
`wgsl-analyzer.js` back to `wgsl_analyzer.js`.

They live in `dist/assets/` rather than beside the module output in `dist/` so
that the directory holds nothing else: a host can point a static file server
straight at it. `wgsl-analyzer-web/assets/*` resolves there too, for bundlers
that would rather ask the package than hardcode a path.

## Usage

The package ships two adapters so editor clients work unmodified:

| Entry point | Gives you | Optional peer |
| --- | --- | --- |
| `wgsl-analyzer-web/jsonrpc` | vscode-jsonrpc `MessageReader`/`MessageWriter`, for `monaco-languageclient` | `vscode-jsonrpc` |
| `wgsl-analyzer-web/codemirror` | a `Transport`, for `@marimo-team/codemirror-languageserver` | `@marimo-team/codemirror-languageserver` |

See `js/examples/monaco` and `js/examples/codemirror`.

## Requirements

**A cross-origin isolated page.** The build uses shared memory since the language server relies on
[pthreads](https://emscripten.org/docs/porting/pthreads.html), so the host must send
`Cross-Origin-Opener-Policy: same-origin` and `Cross-Origin-Embedder-Policy: require-corp`. Without
them `SharedArrayBuffer` is unavailable and nothing starts, further information is available in the
official [docs](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Cross-Origin-Embedder-Policy#features_that_depend_on_cross-origin_isolation).

## Editing files

Edits to an open document should go through `textDocument/didChange` as usual.

`writeFile` and `deleteFile` exist for changing the *set* of files. The server's
filesystem watcher cannot observe the in-memory filesystem, so follow either with
a `workspace/didChangeWatchedFiles` notification.

The worker seeds the workspace after the module is ready but before `main()`
runs, deliberately not from `preRun`. `preRun` executes before
`__wasm_call_ctors`, so seeding there would touch WasmFS before its static
constructors have run.

## What the host has to get right

The server keeps using `Connection::stdio()` unchanged. The adaptation happens
below Rust's `std::io`, where the linker redirects `read`, `readv`, `write` and
`writev` into the [`emscripten-stdio`](../../crates/emscripten-stdio) crate. Its
module docs work through why emscripten's own stdin cannot carry an LSP stream,
which is worth reading before changing either side.

What that leaves for the host:

- Serve `worker.js`, `wgsl_analyzer.js` and `wgsl_analyzer.wasm` from one
  directory under those exact names, as above.
- Be cross-origin isolated, or `SharedArrayBuffer` is missing and nothing starts.
- Drain stdout when the counter at `_lsp_stdout_signal_ptr()` changes. The worker
  waits on it with `Atomics.waitAsync`, which does not block its event loop.
  Without `waitAsync` it falls back to a 5 ms poll and says so on stderr.
- Expect stderr through `printErr`, not the LSP stream. fd 2 is left unwrapped so
  tracing and panics still reach the console. The flip side is that any stray
  write to stdout corrupts the protocol.

Filesystem access uses `-sWASMFS`, emscripten's wasm-side multithreaded
filesystem, so file reads from the server's task pools are not proxied to the
runtime thread the way the legacy JS filesystem would require. It needs
`-sFORCE_FILESYSTEM` alongside it, because WasmFS emits only the JS filesystem
API it can prove it needs and seeding the workspace calls that API directly.
