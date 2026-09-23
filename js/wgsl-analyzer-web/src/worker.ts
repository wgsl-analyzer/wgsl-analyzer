/**
 * Worker entry point: boots the emscripten module, seeds the workspace, and
 * bridges the server's input and output to the page.
 *
 * This file is bundled to `dist/worker.js` and must be served next to
 * `wgsl_analyzer.js`. The dynamic import below resolves relative to it, and the
 * glue spawns its pthread pool with
 * `new Worker(new URL("wgsl_analyzer.js", import.meta.url))`.
 */

import type { EmscriptenModule, LspTransport } from "./emscripten.js";
import { encodeFrame } from "./framing.js";
import { seedWorkspace, writeFile } from "./fs.js";
import type { HostMessage, WorkerMessage } from "./protocol.js";

const decoder = new TextDecoder();

const post = (message: WorkerMessage): void => {
	self.postMessage(message);
};

let instance: EmscriptenModule | null = null;
let transport: LspTransport | null = null;
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
			transport?.pushBytes(encodeFrame(message.message));
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
			transport?.closeInput();
			break;
		default:
			break;
	}
};

async function boot(message: Extract<HostMessage, { type: "boot" }>): Promise<void> {
	root = message.root;

	// Left external by the bundler so it resolves next to dist/worker.js at
	// runtime rather than being inlined.
	const { default: createWgslAnalyzer } = await import("./wgsl_analyzer.js");

	const module = await createWgslAnalyzer({
		noInitialRun: true,
		printErr: (line) => post({ type: "stderr", line }),
		onExit: (code) => post({ type: "exit", code }),
	});
	instance = module;

	// Before `callMain`, which is when the server can first write.
	//
	// `slice()` because `bytes` views the shared wasm heap, and `TextDecoder` is not
	// obliged to accept one.
	transport = module.lspStart({
		onOutput: (bytes) => {
			try {
				post({ type: "lsp", message: JSON.parse(decoder.decode(bytes.slice())) });
			} catch (error) {
				post({ type: "stderr", line: `[lsp] malformed JSON body: ${String(error)}` });
			}
		},
	});

	// Seed after the factory resolves rather than from `preRun`. `preRun` runs
	// before `__wasm_call_ctors`, which is harmless for the JS filesystem but
	// would touch WasmFS before its static constructors have run. This point is
	// still comfortably before `main()`.
	seedWorkspace(module.FS, root, message.files);
	module.FS.chdir(root);

	post({ type: "ready" });

	// Returns immediately: -sPROXY_TO_PTHREAD runs main() on a pthread.
	module.callMain(message.args);
}
