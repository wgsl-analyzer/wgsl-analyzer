/**
 * Worker entry point: relays between the page and the {@link Host}.
 *
 * This file is bundled to `dist/worker.js` and must be served next to
 * `wgsl_analyzer.js`. The dynamic import below resolves relative to it, and the
 * glue spawns its pthread pool with
 * `new Worker(new URL("wgsl_analyzer.js", import.meta.url))`.
 */

import { type Host, startHost } from "./host.js";
import type { HostMessage, WorkerMessage } from "./protocol.js";

const post = (message: WorkerMessage): void => {
	self.postMessage(message);
};

let host: Host | null = null;

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
			host?.send(message.message);
			break;
		case "writeFile":
			host?.writeFile(message.path, message.contents);
			break;
		case "deleteFile":
			host?.deleteFile(message.path);
			break;
		default:
			break;
	}
};

async function boot(message: Extract<HostMessage, { type: "boot" }>): Promise<void> {
	// Left external by the bundler so it resolves next to dist/worker.js at
	// runtime rather than being inlined.
	const { default: createWgslAnalyzer } = await import("./wgsl_analyzer.js");

	host = await startHost(createWgslAnalyzer, {
		root: message.root,
		files: message.files,
		args: message.args,
		onMessage: (lsp) => post({ type: "lsp", message: lsp }),
		onStderr: (line) => post({ type: "stderr", line }),
		onExit: (code) => post({ type: "exit", code }),
	});

	post({ type: "ready" });
}
