# wgsl-analyzer-web

Runs the `wgsl-analyzer` language server in a Web Worker.

The server is the `wgsl-analyzer` binary compiled to
`wasm32-unknown-emscripten`, running its ordinary `main_loop`. Input crosses into
wasm as the usual `Content-Length` framed LSP stream; output crosses back one
complete message body per call, with the framing checked and stripped on the Rust
side. The package hosts it, seeds a workspace into the in-memory filesystem, and
exposes the message stream.

## Building

```bash
# `rust-std` for the `emscripten` target is compiled without the wasm `atomics` feature, so it cannot be linked with `-pthread`
rustup +nightly component add rust-src
source /path/to/emsdk/emsdk_env.sh

cargo xtask build-web   # add --release for the configuration that ships
```

The command stages three files in `dist/assets/`: `wgsl_analyzer.js` and
`wgsl_analyzer.wasm` from the cargo build, and `worker.js` from
`pnpm --filter wgsl-analyzer-web run build`, which it runs for you. All three
must be served from the same directory, under exactly those names. The glue
spawns its pthread pool with
`new Worker(new URL("wgsl_analyzer.js", import.meta.url))`, so renaming the glue
makes every pthread 404. That is why the xtask renames cargo's
`wgsl-analyzer.js` back to `wgsl_analyzer.js`.

They live in `dist/assets/` rather than beside the module output in `dist/` so
that the directory holds nothing else: a host can point a static file server
straight at it. `wgsl-analyzer-web/assets/*` resolves there too, for bundlers
that would rather ask the package than hardcode a path.

Note `dist/worker.js` also exists and is not the one to serve. `tsc` compiles
every file under `src/` so that `worker.ts` is typechecked along with the rest,
and its unbundled output lands there; `dist/assets/worker.js` from esbuild is
the real artifact.

## Usage

`WgslAnalyzerServer.sendMessage` and `.onMessage` carry parsed JSON-RPC objects in both
directions. That is the whole interface, and it is all any LSP client needs — the package ships
no client adapters, because every client wants a slightly different shape and each one is about
thirty lines:

| Example | Adapts to |
| --- | --- |
| [`js/examples/monaco/src/transport.ts`](../examples/monaco/src/transport.ts) | the `IMessageTransport` that `monaco.lsp` takes |
| [`js/examples/codemirror/src/transport.ts`](../examples/codemirror/src/transport.ts) | the `Transport` that `@marimo-team/codemirror-languageserver` takes |

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

The server does not use `Connection::stdio()`, Instead the emscripten build swaps
in a transport built out of two JavaScript functions the wasm module imports,
linked in with `--js-library` and `--pre-js`:

| File | Role |
| --- | --- |
| [`emscripten_io.rs`](../../crates/wgsl-analyzer/src/bin/emscripten_io.rs) | the Rust `Read`/`Write` endpoints: a `BufReader` inbound, a frame accumulator outbound |
| [`emscripten-io.js`](../../crates/wgsl-analyzer/src/bin/emscripten-io.js) | the two imports, proxied to the runtime thread |
| [`emscripten-io-pre.js`](../../crates/wgsl-analyzer/src/bin/emscripten-io-pre.js) | the queue and the `Module` methods this package calls |

The Rust module's docs work through why, and are worth reading before changing
either side. The short version: `Read::read` treats a count of zero as end of
input, so a read has to block until a frame arrives, and emscripten's
`__proxy: 'sync'` plus `__async` is what lets it do that on a pthread while the
runtime thread's event loop stays free.

What that leaves for the host:

- Serve `worker.js`, `wgsl_analyzer.js` and `wgsl_analyzer.wasm` from one
  directory under those exact names, as above.
- Be cross-origin isolated, or `SharedArrayBuffer` is missing and nothing starts.
- Call `Module.lspStart({ onOutput })` before `callMain`, then drive the server
  through the `pushBytes` and `closeInput` it returns — `closeInput` at shutdown,
  so the reader thread unwinds instead of staying parked. `lspStart` is the
  entire host-facing API, and `WgslAnalyzerServer` drives it for you.
- Treat `onOutput`'s argument as one complete message body. The bytes are a view
  into wasm memory that is only valid for the duration of the call, and the view
  is over shared memory, so decode a `slice()` rather than the view itself.
- Expect stderr through `printErr`, not the LSP stream.

Filesystem access uses `-sWASMFS`, emscripten's wasm-side multithreaded
filesystem. It needs `-sFORCE_FILESYSTEM` alongside it, because WasmFS emits
only the JS filesystem API it can prove it needs and seeding the workspace calls
that API directly.
