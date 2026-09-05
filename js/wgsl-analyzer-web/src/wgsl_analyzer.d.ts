/**
 * Type surface for the emscripten glue.
 *
 * The real `wgsl_analyzer.js` is produced by `scripts/build-wasm.mjs` and only
 * exists in `dist/`, next to the bundled worker that imports it at runtime. This
 * declaration lets the worker typecheck without it being present in `src/`.
 */
declare const createWgslAnalyzer: (options: unknown) => Promise<unknown>;
export default createWgslAnalyzer;
