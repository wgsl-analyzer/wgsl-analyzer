/**
 * Hosts the emscripten module: boots it, seeds the workspace, and carries the
 * message stream.
 */

import type { ModuleFactory } from "./emscripten.js";
import { seedWorkspace, writeFile } from "./fs.js";
import type { WorkspaceFiles } from "./protocol.js";

export interface HostOptions {
	/** Absolute path of the workspace inside the in-memory filesystem. */
	readonly root: string;
	/** Files to seed, keyed by path relative to {@link HostOptions.root}. */
	readonly files: WorkspaceFiles;
	/** argv for `main()`. Empty selects the default `lsp-server` subcommand. */
	readonly args: readonly string[];
	/** Receives each parsed JSON-RPC message from the server. */
	readonly onMessage: (message: unknown) => void;
	/** Receives the server's stderr, line by line. */
	readonly onStderr: (line: string) => void;
	/** Called if `main()` returns. */
	readonly onExit: (code: number) => void;
}

/** A running server. Once it has exited, every method does nothing. */
export interface Host {
	/** Sends one JSON-RPC message to the server. */
	send(message: unknown): void;
	/** Creates or replaces a workspace file, given its path relative to the root. */
	writeFile(relativePath: string, contents: string | Uint8Array): void;
	/** Removes a workspace file, if present. */
	deleteFile(relativePath: string): void;
}

/** Boots the server and resolves once `main()` has been started. */
export async function startHost(factory: ModuleFactory, options: HostOptions): Promise<Host> {
	const { root } = options;
	const resolve = (relativePath: string): string => `${root}/${relativePath.replace(/^\/+/, "")}`;

	// Messages for the server, which pulls them one at a time. It never closes.
	const inbox = new TransformStream<string, string>();
	const reader = inbox.readable.getReader();
	const writer = inbox.writable.getWriter();

	// With `-sEXIT_RUNTIME`, calling into the module after `main()` returns is an
	// error, and the page can still send messages after that.
	let exited = false;
	const module = await factory({
		noInitialRun: true,
		printErr: options.onStderr,
		onExit: (code) => {
			exited = true;
			// Messages are delivered from microtasks too, so those sent before exiting come first.
			queueMicrotask(() => options.onExit(code));
		},
		lspNextMessage: () => reader.read().then(({ value }) => value as string),
		lspOnMessage: (body) => options.onMessage(JSON.parse(body)),
	});

	// Needs to run after WasmFS was iniialized.
	seedWorkspace(module.FS, root, options.files);
	module.FS.chdir(root);
	// WasmFS's `chdir` does not go through `FS.handleError`, so a failure is
	// otherwise silent.
	if (module.FS.cwd() !== root) throw new Error(`cannot change directory to ${root}`);

	// Returns immediately: -sPROXY_TO_PTHREAD runs main() on a pthread.
	module.callMain(options.args);

	return {
		send: (message) => {
			if (!exited) void writer.write(JSON.stringify(message));
		},
		writeFile: (relativePath, contents) => {
			if (!exited) writeFile(module.FS, resolve(relativePath), contents);
		},
		deleteFile: (relativePath) => {
			const path = resolve(relativePath);
			if (!exited && module.FS.analyzePath(path).exists) module.FS.unlink(path);
		},
	};
}
