/**
 * The emscripten module surface this package drives.
 */

import type { EmscriptenFs } from "./fs.js";

/** The emscripten module members this package touches. */
export interface EmscriptenModule {
	FS: EmscriptenFs;
	// Functions provided by emscripten, see https://emscripten.org/docs/api_reference/index.html.
	callMain(args: readonly string[]): void;
	_free(pointer: number): void;
	stringToNewUTF8(text: string): number;
	UTF8ToString(pointer: number): string;
	addFunction(fn: (pointer: number) => void, signature: "vp"): number;
	// Functions provided by the language server, see crates/wgsl-analyzer/src/bin/emscripten_io.rs.
	_lsp_push_message(message: number): void;
	_lsp_set_on_message(onMessage: number): void;
}

export interface ModuleOptions {
	noInitialRun: boolean;
	printErr: (line: string) => void;
	onExit: (code: number) => void;
}

export type ModuleFactory = (options: ModuleOptions) => Promise<EmscriptenModule>;
