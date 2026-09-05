# wgsl-analyzer-web

Runs the `wgsl-analyzer` language server in a Web Worker.

The server is the `wgsl-analyzer` binary compiled to
`wasm32-unknown-emscripten`, running its ordinary `main_loop` and speaking LSP
over stdin and stdout. The package hosts it, seeds a workspace into the in-memory
filesystem, and exposes the message stream.

## Requirements

- **Rust nightly** with `rust-src`, because the build needs `-Zbuild-std`.
  The shipped `rust-std` for this target is compiled without the wasm `atomics`
  feature, so it cannot be linked with `-pthread`.
- **The emscripten SDK**, sourced into the shell (`source emsdk_env.sh`); `emcc`
  is not on `PATH` by default.
- **A cross-origin isolated page.** The build uses shared memory, so the host
  must send `Cross-Origin-Opener-Policy: same-origin` and
  `Cross-Origin-Embedder-Policy: require-corp`. Without them `SharedArrayBuffer`
  is unavailable and nothing starts.

## Building

```bash
rustup +nightly component add rust-src
source /path/to/emsdk/emsdk_env.sh

pnpm --filter wgsl-analyzer-web run build:wasm   # add --debug for a faster build
pnpm --filter wgsl-analyzer-web run build
```

`build:wasm` stages three files in `dist/`: `wgsl_analyzer.js`,
`wgsl_analyzer.wasm`, and, from `build`, `worker.js`. All three must be served
from the same directory, under exactly those names. The glue spawns its pthread
pool with `new Worker(new URL("wgsl_analyzer.js", import.meta.url))`, so
renaming the glue makes every pthread 404. That is why the build script renames
cargo's `wgsl-analyzer.js` back to `wgsl_analyzer.js`.

## Usage

```typescript
import { WgslAnalyzerServer } from "wgsl-analyzer-web";

const server = await WgslAnalyzerServer.start({
  baseUrl: "/wgsl-analyzer/",       // where the three files above are served
  root: "/workspace",
  files: {
    "wesl.toml": 'edition = "2026_pre"\n',
    "shaders/main.wesl": "fn main() {}\n",
  },
});

server.onMessage((message) => console.log("from server", message));
server.sendMessage({ jsonrpc: "2.0", id: 1, method: "initialize", params: { /* … */ } });
```

The package ships two adapters so editor clients work unmodified:

| Entry point | Gives you | Optional peer |
| --- | --- | --- |
| `wgsl-analyzer-web/jsonrpc` | vscode-jsonrpc `MessageReader`/`MessageWriter`, for `monaco-languageclient` | `vscode-jsonrpc` |
| `wgsl-analyzer-web/codemirror` | a `Transport`, for `@marimo-team/codemirror-languageserver` | `@marimo-team/codemirror-languageserver` |

See `js/examples/monaco` and `js/examples/codemirror`.

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

## Serve it compressed

A release `wgsl_analyzer.wasm` is around 4.8 MiB on disk, and it is mostly the
wasm code section, which compresses very well:

| | raw | gzip -9 | brotli -q 11 |
| --- | --- | --- | --- |
| `wgsl_analyzer.wasm` | 4.8 MiB | ~1.6 MiB | ~1.1 MiB |
| `wgsl_analyzer.js` | 64 KiB | 16 KiB | 14 KiB |

So `Content-Encoding` is worth more than anything else the host controls: gzip
takes roughly two thirds off the transfer and brotli a little over three
quarters. It is a one-time cost per version that the HTTP cache then keeps, but
serving the file uncompressed means every first visit pays 4.8 MiB.

The examples do not do this. Vite's dev and preview servers ship the file
uncompressed, so a local run is not representative of what a deployment costs —
measure against a server that has compression configured before drawing
conclusions about the payload.

The figures above are from one release build and will drift; `build:wasm` prints
the size of each artifact it stages.

## Push diagnostics come back empty

The server answers pull diagnostics (`textDocument/diagnostic`) correctly, but
`textDocument/publishDiagnostics` always carries an empty array. That holds even
for a file with an outright syntax error, and even though completion on the same
file works. This was reproduced with a hand-written client that advertises only
`publishDiagnostics`, so it is server-side rather than a quirk of any client
library.

Clients that advertise pull diagnostics (`monaco-languageclient` does) are
unaffected. Clients that rely on push (`@marimo-team/codemirror-languageserver`
does) will see no diagnostics; every other feature works.
