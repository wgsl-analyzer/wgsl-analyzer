/**
 * wgsl-analyzer as a language server in a Web Worker.
 *
 * The server is the real `wgsl-analyzer` binary compiled to
 * `wasm32-unknown-emscripten`, running its ordinary `main_loop` and speaking LSP
 * over stdin and stdout. This module hosts it and exposes the message stream;
 * adapters for specific editor clients live in the `jsonrpc` and `codemirror`
 * entry points.
 *
 * The page must be cross-origin isolated. The build uses shared memory, so
 * without `Cross-Origin-Opener-Policy: same-origin` and
 * `Cross-Origin-Embedder-Policy: require-corp` nothing will start.
 */

import type { HostMessage, WorkerMessage, WorkspaceFiles } from "./protocol.js";

export type { WorkspaceFiles } from "./protocol.js";

export interface StartOptions {
	/**
	 * Directory serving `worker.js`, `wgsl_analyzer.js` and `wgsl_analyzer.wasm`.
	 * They must sit side by side. Defaults to `"/wgsl-analyzer/"`.
	 */
	readonly baseUrl?: string | URL;
	/** Absolute path of the workspace inside MEMFS. Defaults to `"/workspace"`. */
	readonly root?: string;
	/** Files to seed, keyed by path relative to {@link StartOptions.root}. */
	readonly files: WorkspaceFiles;
	/** argv for `main()`. Empty selects the default `lsp-server` subcommand. */
	readonly args?: readonly string[];
	/** Receives the server's stderr, line by line. */
	readonly onStderr?: (line: string) => void;
	/** Called if `main()` returns. */
	readonly onExit?: (code: number) => void;
}

export interface Disposable {
	dispose(): void;
}

/** A running wgsl-analyzer language server. */
export class WgslAnalyzerServer {
	readonly #worker: Worker;
	readonly #listeners = new Set<(message: unknown) => void>();
	readonly #root: string;
	#disposed = false;

	private constructor(worker: Worker, root: string) {
		this.#worker = worker;
		this.#root = root;
	}

	/** The workspace root as a URI, suitable for `initialize`. */
	get rootUri(): string {
		return `file://${this.#root}`;
	}

	/** Builds the URI of a workspace file, given its path relative to the root. */
	uriOf(relativePath: string): string {
		return `file://${this.#root}/${relativePath.replace(/^\/+/, "")}`;
	}

	/** Boots the server and resolves once it is ready to accept messages. */
	static async start(options: StartOptions): Promise<WgslAnalyzerServer> {
		const base = new URL(String(options.baseUrl ?? "/wgsl-analyzer/"), globalThis.location.href);
		const root = options.root ?? "/workspace";

		const worker = new Worker(new URL("worker.js", base), { type: "module" });
		const server = new WgslAnalyzerServer(worker, root);

		await new Promise<void>((resolve, reject) => {
			const onFirstMessage = (event: MessageEvent<WorkerMessage>): void => {
				const message = event.data;
				if (message.type === "ready") {
					worker.removeEventListener("message", onFirstMessage);
					resolve();
				} else if (message.type === "error") {
					worker.removeEventListener("message", onFirstMessage);
					reject(new Error(`${message.message}\n${message.stack ?? ""}`));
				}
			};
			worker.addEventListener("message", onFirstMessage);
			worker.addEventListener(
				"error",
				(event) => reject(new Error(`worker failed to load: ${event.message}`)),
				{ once: true },
			);

			const boot: HostMessage = {
				type: "boot",
				root,
				files: options.files,
				args: options.args ?? [],
			};
			worker.postMessage(boot);
		});

		worker.addEventListener("message", (event: MessageEvent<WorkerMessage>) => {
			const message = event.data;
			switch (message.type) {
				case "lsp":
					for (const listener of server.#listeners) listener(message.message);
					break;
				case "stderr":
					options.onStderr?.(message.line);
					break;
				case "exit":
					options.onExit?.(message.code);
					break;
				case "error":
					options.onStderr?.(`[worker] ${message.message}`);
					break;
				default:
					break;
			}
		});

		return server;
	}

	/** Sends one JSON-RPC message to the server. */
	sendMessage(message: unknown): void {
		if (this.#disposed) throw new Error("server has been disposed");
		const envelope: HostMessage = { type: "lsp", message };
		this.#worker.postMessage(envelope);
	}

	/** Subscribes to messages coming back from the server. */
	onMessage(listener: (message: unknown) => void): Disposable {
		this.#listeners.add(listener);
		return {
			dispose: () => {
				this.#listeners.delete(listener);
			},
		};
	}

	/**
	 * Creates or replaces a file in MEMFS.
	 *
	 * Needed only when the set of files changes. The server's filesystem watcher
	 * cannot observe MEMFS, so follow this with a
	 * `workspace/didChangeWatchedFiles` notification. Ordinary edits to an open
	 * document should go through `textDocument/didChange` and leave MEMFS alone.
	 */
	writeFile(relativePath: string, contents: string | Uint8Array): void {
		const message: HostMessage = { type: "writeFile", path: relativePath, contents };
		this.#worker.postMessage(message);
	}

	/** Removes a file from MEMFS. See {@link WgslAnalyzerServer.writeFile}. */
	deleteFile(relativePath: string): void {
		const message: HostMessage = { type: "deleteFile", path: relativePath };
		this.#worker.postMessage(message);
	}

	/** Stops the server and tears down the worker. */
	dispose(): void {
		if (this.#disposed) return;
		this.#disposed = true;
		this.#listeners.clear();

		// Close stdin first so the server's reader thread sees EOF and unwinds
		// cleanly, then tear the worker down on the next macrotask so that
		// message actually gets delivered.
		const close: HostMessage = { type: "close" };
		this.#worker.postMessage(close);
		setTimeout(() => this.#worker.terminate(), 0);
	}
}
