/**
 * Worker entry point: boots the emscripten module, seeds the workspace, and
 * bridges the server's stdin and stdout to the page.
 *
 * This file is bundled to `dist/worker.js` and must be served next to
 * `wgsl_analyzer.js`. The dynamic import below resolves relative to it, and the
 * glue spawns its pthread pool with
 * `new Worker(new URL("wgsl_analyzer.js", import.meta.url))`.
 *
 * Both directions go through the `emscripten-stdio` bridge rather than
 * `Module.stdin`/`Module.stdout`. That crate's module docs explain why
 * emscripten's own stdin cannot carry an LSP stream.
 */

import { createFrameDecoder, encodeFrame } from "./framing.js";
import { type EmscriptenFs, seedWorkspace, writeFile } from "./fs.js";
import type { HostMessage, WorkerMessage } from "./protocol.js";

/** The emscripten module members this package touches. */
interface EmscriptenModule {
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

interface ModuleOptions {
	noInitialRun: boolean;
	printErr: (line: string) => void;
	onExit: (code: number) => void;
}

type ModuleFactory = (options: ModuleOptions) => Promise<EmscriptenModule>;

/** Size of the reusable buffer stdout is drained through. */
const OUTPUT_CHUNK = 64 * 1024;

const post = (message: WorkerMessage): void => {
	self.postMessage(message);
};

let instance: EmscriptenModule | null = null;
let root = "/workspace";

self.onmessage = (event: MessageEvent<HostMessage>): void => {
	const message = event.data;
	switch (message.type) {
		case "boot":
			boot(message).catch((error: unknown) => {
				post({
					type: "error",
					message: String(error),
					stack: error instanceof Error ? error.stack : undefined,
				});
			});
			break;
		case "lsp":
			if (instance) pushToStdin(instance, encodeFrame(message.message));
			break;
		case "writeFile":
			if (instance) writeFile(instance.FS, `${root}/${message.path}`, message.contents);
			break;
		case "deleteFile":
			if (instance?.FS.analyzePath(`${root}/${message.path}`).exists) {
				instance.FS.unlink(`${root}/${message.path}`);
			}
			break;
		case "close":
			instance?._lsp_stdin_close();
			break;
		default:
			break;
	}
};

async function boot(message: Extract<HostMessage, { type: "boot" }>): Promise<void> {
	root = message.root;

	// Left external by the bundler so it resolves next to dist/worker.js at
	// runtime rather than being inlined.
	const factory = (await import("./wgsl_analyzer.js")) as unknown as { default: ModuleFactory };

	const module = await factory.default({
		noInitialRun: true,
		// The bridge leaves fd 2 alone, so stderr still arrives through
		// emscripten's own line-buffered path.
		printErr: (line) => post({ type: "stderr", line }),
		onExit: (code) => post({ type: "exit", code }),
	});
	instance = module;

	// Seed after the factory resolves rather than from `preRun`. `preRun` runs
	// before `__wasm_call_ctors`, which is harmless for the JS filesystem but
	// would touch WasmFS before its static constructors have run. This point is
	// still comfortably before `main()`.
	seedWorkspace(module.FS, root, message.files);
	module.FS.chdir(root);

	startOutputPump(module);
	post({ type: "ready" });

	// Returns immediately: -sPROXY_TO_PTHREAD runs main() on a pthread.
	module.callMain(message.args);
}

/** Copies one framed message into wasm memory and hands it to the server. */
function pushToStdin(module: EmscriptenModule, bytes: Uint8Array): void {
	const pointer = module._malloc(bytes.length);
	if (pointer === 0) {
		post({ type: "error", message: "out of memory while queuing stdin" });
		return;
	}
	try {
		module.HEAPU8.set(bytes, pointer);
		const result = module._lsp_stdin_push(pointer, bytes.length);
		if (result !== 0) {
			post({ type: "error", message: `lsp_stdin_push failed: ${result}` });
		}
	} finally {
		module._free(pointer);
	}
}

/**
 * Forwards the server's stdout to the page.
 *
 * The bridge bumps a counter and issues a wasm `memory.atomic.notify` on every
 * write, so this waits rather than polls. `Atomics.waitAsync` is what makes
 * that legal here: unlike `Atomics.wait` it does not block the caller, so the
 * worker's event loop stays free.
 */
function startOutputPump(module: EmscriptenModule): void {
	const pushChunk = createFrameDecoder(
		(message) => post({ type: "lsp", message }),
		(reason) => post({ type: "stderr", line: `[framing] ${reason}` }),
	);
	const outputPointer = module._malloc(OUTPUT_CHUNK);

	const drain = (): void => {
		for (;;) {
			const count = module._lsp_stdout_pop(outputPointer, OUTPUT_CHUNK);
			if (count <= 0) {
				if (count < 0) post({ type: "error", message: `lsp_stdout_pop failed: ${count}` });
				return;
			}
			// Consumed fully before the next pop overwrites the buffer, so no copy
			// is needed here.
			pushChunk(module.HEAPU8.subarray(outputPointer, outputPointer + count));
		}
	};

	const signalIndex = module._lsp_stdout_signal_ptr() >>> 2;
	const canWait =
		typeof Atomics.waitAsync === "function" && module.HEAPU8.buffer instanceof SharedArrayBuffer;

	if (!canWait) {
		post({
			type: "stderr",
			line: "[wgsl-analyzer-web] Atomics.waitAsync unavailable, polling stdout every 5ms",
		});
		drain();
		setInterval(drain, 5);
		return;
	}

	const wait = (): void => {
		// Rebuilt every time: ALLOW_MEMORY_GROWTH replaces the heap views.
		const signal = new Int32Array(module.HEAPU8.buffer);
		// Read the counter before draining, so a write that lands mid-drain still
		// leaves the value different and the wait returns immediately.
		const observed = Atomics.load(signal, signalIndex);
		drain();
		const result = Atomics.waitAsync(signal, signalIndex, observed);
		if (result.async) {
			void result.value.then(wait);
		} else {
			// Already changed. Yield first so this cannot starve the event loop.
			setTimeout(wait, 0);
		}
	};
	wait();
}
