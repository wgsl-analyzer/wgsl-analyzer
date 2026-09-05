# wgsl-analyzer in Monaco

A minimal Monaco editor driven by `wgsl-analyzer` running in a Web Worker, wired
up with `monaco-languageclient`.

## Running

Build the package first. The example copies its artifacts into
`public/wgsl-analyzer/`.

```bash
rustup +nightly component add rust-src
source /path/to/emsdk/emsdk_env.sh

pnpm --filter wgsl-analyzer-web run build:wasm   # --debug builds faster
pnpm --filter wgsl-analyzer-web run build
pnpm --filter wgsl-analyzer-monaco-example run dev
```

Then open the printed URL. Try completion (Ctrl+Space), hover, go-to-definition
(F12), and introduce an error to see diagnostics.

## Notes

- The dev server sets `Cross-Origin-Opener-Policy` and
  `Cross-Origin-Embedder-Policy` in `vite.config.ts`. They are required: the
  server is built with pthreads and needs `SharedArrayBuffer`.
- The language is registered with `monaco.languages.register` rather than through
  an extension manifest. In the wrapper's `"classic"` mode the extension host that
  would process `contributes` is not loaded, so a manifest leaves the model as
  plaintext and the client's document selector never matches.
- `monacoWorkerFactory: configureDefaultWorkerFactory` is passed explicitly. The
  wrapper's fallback registers a worker factory with no loaders, so every lookup
  misses and monaco silently runs its workers on the main thread.
- `optimizeDeps.esbuildOptions.plugins` includes
  `@codingame/esbuild-import-meta-url-plugin`, without which dependency
  pre-bundling rewrites monaco-vscode-api's `new URL(..., import.meta.url)` worker
  references and the editor worker 404s.
