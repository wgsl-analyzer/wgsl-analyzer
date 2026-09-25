# wgsl-analyzer-web

Runs the `wgsl-analyzer` language server in a Web Worker.

The server is the `wgsl-analyzer` binary compiled to
`wasm32-unknown-emscripten`, running its ordinary `main_loop`. One crossing into
or out of wasm carries exactly one complete message body. The package hosts it,
seeds a workspace into the in-memory filesystem, and exposes the message stream.

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
directions. That is the whole interface, there are 2 examples showcasing how to use it:

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

- Serve `worker.js`, `wgsl_analyzer.js` and `wgsl_analyzer.wasm` from one
  directory under those exact names, as above.
- Be cross-origin isolated, or `SharedArrayBuffer` is missing and nothing starts.
- Hand `sendMessage` one complete message body.
- Treat `onMessage`'s argument as one complete message body.
- Expect stderr through `printErr`, not the LSP stream.

Filesystem access uses `-sWASMFS`, emscripten's wasm-side multithreaded
filesystem. It needs `-sFORCE_FILESYSTEM` alongside it, because WasmFS emits
only the JS filesystem API it can prove it needs and seeding the workspace calls
that API directly.
