# wgsl-analyzer in CodeMirror

A minimal CodeMirror 6 editor driven by `wgsl-analyzer` running in a Web Worker,
wired up with `@marimo-team/codemirror-languageserver`.

## Running

Build the package first. The example copies its artifacts into
`public/wgsl-analyzer/`.

```bash
rustup +nightly component add rust-src
source /path/to/emsdk/emsdk_env.sh

pnpm --filter wgsl-analyzer-web run build:wasm   # --debug builds faster
pnpm --filter wgsl-analyzer-web run build
pnpm --filter wgsl-analyzer-codemirror-example run dev
```

Then open the printed URL and try completion, hover, or go-to-definition.

## Notes

- The dev server sets `Cross-Origin-Opener-Policy` and
  `Cross-Origin-Embedder-Policy` in `vite.config.ts`. They are required: the
  server is built with pthreads and needs `SharedArrayBuffer`.
- `@marimo-team/codemirror-languageserver` defines its own minimal `Transport`
  interface, so the adapter in `wgsl-analyzer-web/codemirror` is a thin wrapper
  with no extra JSON-RPC dependency.

## Diagnostics do not appear here

This client relies on push diagnostics (`textDocument/publishDiagnostics`), and
the server publishes an empty array every time. That holds even for a file with
an outright syntax error, and even though completion on the same file returns
hundreds of items. The server answers pull diagnostics
(`textDocument/diagnostic`) correctly, which is why the Monaco example does show
errors.

This is server-side, not an artifact of this library. It reproduces with a
hand-written client that advertises only `publishDiagnostics`. Completion, hover
and go-to-definition all work here.
