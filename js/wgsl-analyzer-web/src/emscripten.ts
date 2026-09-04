/**
 * The emscripten module surface this package drives.
 *
 * Every member here is something the link flags in
 * `[target.wasm32-unknown-emscripten]` in `.cargo/config.toml` have to keep
 * alive: the `_`-prefixed ones come from `-sEXPORTED_FUNCTIONS`, `FS`,
 * `callMain` and `HEAPU8` from `-sEXPORTED_RUNTIME_METHODS`, and the `FS`
 * members from `-sFORCE_FILESYSTEM`. Nothing checks that at compile time —
 * `wgsl_analyzer.js` is generated — so `tests/smoke/lsp.test.ts` asserts each
 * one is really present against a linked build.
 */

import type { EmscriptenFs } from "./fs.js";

/** The emscripten module members this package touches. */
export interface EmscriptenModule {
	FS: EmscriptenFs;
	/** Replaced on memory growth, so always read it fresh. */
	HEAPU8: Uint8Array;
	callMain(args: readonly string[]): void;
	_malloc(size: number): number;
	_free(pointer: number): void;
	_lsp_stdin_push(pointer: number, length: number): number;
	_lsp_stdin_close(): void;
	_lsp_stdout_pop(pointer: number, capacity: number): number;
	_lsp_stdout_signal_ptr(): number;
}

export interface ModuleOptions {
	noInitialRun: boolean;
	printErr: (line: string) => void;
	onExit: (code: number) => void;
}

export type ModuleFactory = (options: ModuleOptions) => Promise<EmscriptenModule>;
