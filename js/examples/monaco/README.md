# wgsl-analyzer in Monaco

A minimal Monaco editor driven by `wgsl-analyzer` running in a Web Worker, wired up with monaco's
own LSP client — the `monaco.lsp` namespace, bundled into `monaco-editor` since 0.55.

## Running

Build the package first. Vite serves its `dist/assets/` directly as `publicDir`, so there is
nothing to copy.

```bash
rustup +nightly component add rust-src
source /path/to/emsdk/emsdk_env.sh

cargo xtask build-web   # --release builds the configuration that ships

cd js
pnpm --filter wgsl-analyzer-monaco-example run dev
```

## Notes

- The dev server sets `Cross-Origin-Opener-Policy` and `Cross-Origin-Embedder-Policy` in
  `vite.config.ts`. They are required: the server is built with pthreads and needs
  `SharedArrayBuffer`.
- `monaco.lsp` is upstream alpha — its own README says it "is in alpha stage and might contain
  many bugs". It is used here anyway because it needs nothing but `monaco-editor` itself.
- `MonacoLspClient` sends `initialize` from its constructor with `rootUri: null` hardcoded, and
  exposes no hook to change it. That works here **only** because the worker calls `FS.chdir(root)`
  before `main()`, and the server falls back to the process working directory when `rootUri` is
  absent, so the workspace still resolves to `/workspace`. A server without that fallback could
  not be driven by this client.
- The editor worker is imported as `monaco-editor/editor/editor.worker?worker`, without the
  `esm/vs/` prefix most monaco recipes still show. The package's `exports` map rewrites `./*` to
  `./esm/vs/*.js`, so spelling the prefix out resolves to `esm/vs/esm/vs/…` and fails.
