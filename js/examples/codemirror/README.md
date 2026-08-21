# wgsl-analyzer in CodeMirror

A minimal CodeMirror 6 editor driven by `wgsl-analyzer` running in a Web Worker, wired up with
`@marimo-team/codemirror-languageserver`.

## Running

Build the package first. Vite serves its `dist/assets/` directly as `publicDir`, so there is
nothing to copy.

```bash
rustup +nightly component add rust-src
source /path/to/emsdk/emsdk_env.sh

cd js
pnpm --filter wgsl-analyzer-web run build:wasm   # --debug builds faster
pnpm --filter wgsl-analyzer-web run build
pnpm --filter wgsl-analyzer-codemirror-example run dev
```

## Notes

- The dev server sets `Cross-Origin-Opener-Policy` and `Cross-Origin-Embedder-Policy` in
  `vite.config.ts`. They are required: the server is built with pthreads and needs
  `SharedArrayBuffer`.
- `@marimo-team/codemirror-languageserver` defines its own minimal `Transport` interface, so the
  adapter in `wgsl-analyzer-web/codemirror` is a thin wrapper with no extra JSON-RPC dependency.
