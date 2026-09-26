/**
 * The emscripten module surface this package drives.
 */

import type { EmscriptenFs } from "./fs.js";

/** The emscripten module members this package touches. */
export interface EmscriptenModule {
	FS: EmscriptenFs;
	// Functions provided by emscripten, see https://emscripten.org/docs/api_reference/index.html.
	callMain(args: readonly string[]): void;
}

export interface ModuleOptions {
	noInitialRun: boolean;
	printErr: (line: string) => void;
	onExit: (code: number) => void;
	// The transport, called by emscripten/library.js.
	/**
	 * Resolves to the next message for the server. It may run inside another call into the
	 * module, so it must not call into the module itself, and it must never reject.
	 */
	lspNextMessage: () => Promise<string>;
	/** Receives one message from the server, from a microtask. */
	lspOnMessage: (body: string) => void;
}

export type ModuleFactory = (options: ModuleOptions) => Promise<EmscriptenModule>;
