/**
 * The emscripten module surface this package drives.
 */

import type { EmscriptenFs } from "./fs.js";

export interface LspStartOptions {
	/**
	 * Receives the server's output as it is written.
	 *
	 * The bytes are a view into wasm memory, valid only for the duration of the
	 * call, so a handler that does not consume them synchronously has to copy.
	 * They are also a raw stream rather than messages: one call is not one frame,
	 * so the handler needs a `Content-Length` parser over them.
	 */
	onOutput: (bytes: Uint8Array) => void;
}

/** The two ends of a started transport. */
export interface LspTransport {
	/** Queues one framed message for the server. */
	pushBytes(bytes: Uint8Array): void;
	/**
	 * Reports end of input, so the server's reader thread unwinds rather than
	 * staying parked on a read and holding the runtime open.
	 */
	closeInput(): void;
}

/** The emscripten module members this package touches. */
export interface EmscriptenModule {
	FS: EmscriptenFs;
	callMain(args: readonly string[]): void;
	/**
	 * Starts the transport and returns its two ends.
	 *
	 * Call this before `callMain`, which is when the server can first write.
	 * Throws if called twice.
	 */
	lspStart(options: LspStartOptions): LspTransport;
}

export interface ModuleOptions {
	noInitialRun: boolean;
	printErr: (line: string) => void;
	onExit: (code: number) => void;
}

export type ModuleFactory = (options: ModuleOptions) => Promise<EmscriptenModule>;
