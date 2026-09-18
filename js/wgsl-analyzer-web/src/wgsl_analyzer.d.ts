/**
 * Stands in for the emscripten glue.
 *
 * The real `wgsl_analyzer.js` is produced by `scripts/build-wasm.ts` and only
 * exists in `dist/`, next to the bundled worker that imports it at runtime. This
 * declaration lets the worker typecheck without it being present in `src/`.
 */

import type { ModuleFactory } from "./emscripten.js";

declare const createWgslAnalyzer: ModuleFactory;
export default createWgslAnalyzer;
