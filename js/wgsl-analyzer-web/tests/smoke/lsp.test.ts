/**
 * End-to-end smoke test for the `wasm32-unknown-emscripten` build, under Node.
 *
 * This drives the emscripten glue the way `src/worker.ts` does rather than
 * through `WgslAnalyzerServer.start()`, which is unusable here: it resolves
 * against `globalThis.location.href` and constructs a web `Worker`. The glue
 * itself is environment-agnostic because `-sENVIRONMENT` is deliberately unset,
 * so under Node it uses `worker_threads`, `createRequire` and `node:fs`.
 *
 * Framing and seeding are imported from `dist/` rather than reimplemented: a
 * harness carrying its own copy of the `Content-Length` parser would only be
 * testing the copy.
 *
 * Every `dist/` import is dynamic. A static import is hoisted above the module
 * body, so a missing artifact would fail with ERR_MODULE_NOT_FOUND before
 * `preflight` could explain which build step was skipped.
 *
 * Note this covers the module, not the browser. Node hands out
 * `SharedArrayBuffer` unconditionally, so a regression in the cross-origin
 * isolation requirement or in the `new Worker(..., {type: "module"})` path in
 * `src/index.ts` would still pass here.
 */

import assert from "node:assert/strict";
import { existsSync, statSync } from "node:fs";
import { join } from "node:path";
import { after, before, describe, it } from "node:test";
import { pathToFileURL } from "node:url";

// Type-only, so these are erased and do not hoist a runtime import of `dist/`.
import type { EmscriptenModule, ModuleFactory } from "../../dist/emscripten.js";
import type { EmscriptenFs } from "../../dist/fs.js";
import type { WorkspaceFiles } from "../../dist/protocol.js";

const DIST = join(import.meta.dirname, "..", "..", "dist");
const ASSETS = join(DIST, "assets");

const BOOT_MS = Number(process.env["WA_SMOKE_BOOT_MS"] ?? 180_000);
const STEP_MS = Number(process.env["WA_SMOKE_STEP_MS"] ?? 60_000);
const TOTAL_MS = Number(process.env["WA_SMOKE_TOTAL_MS"] ?? 420_000);

/** Buffer stdout is drained through. `src/worker.ts` uses the same 64 KiB. */
const OUTPUT_CHUNK = Number(process.env["WA_SMOKE_CHUNK"] ?? 64 * 1024);

/** Backstop drain interval. See the comment on the pump. */
const SAFETY_POLL_MS = 25;

// The contract with .cargo/config.toml

// Typed as keys rather than plain strings, so dropping a member from
// `EmscriptenModule`/`EmscriptenFs` without revisiting the link flags is a
// compile error here rather than a silently weaker assertion.

/** `-sEXPORTED_RUNTIME_METHODS`, verbatim. */
const RUNTIME_METHODS: readonly (keyof EmscriptenModule)[] = ["FS", "callMain", "HEAPU8"];

/**
 * `-sEXPORTED_FUNCTIONS`, verbatim. Deliberately not `keyof EmscriptenModule`:
 * the link flags are a superset of the interface, because `_main` is reached
 * through `callMain` and so the package never names it.
 */
const WASM_EXPORTS: readonly string[] = [
	"_main",
	"_malloc",
	"_free",
	"_lsp_stdin_push",
	"_lsp_stdin_close",
	"_lsp_stdout_pop",
	"_lsp_stdout_signal_ptr",
];

/**
 * The `FS` members `src/fs.ts` calls. These exist only because of
 * `-sFORCE_FILESYSTEM`: WasmFS emits just the JS API it can prove it needs, so
 * dropping that flag leaves `Module.FS` present but hollow — which a check on
 * the name `FS` alone would not notice.
 */
const FS_MEMBERS: readonly (keyof EmscriptenFs)[] = [
	"mkdir",
	"writeFile",
	"unlink",
	"chdir",
	"cwd",
	"analyzePath",
];

// This fixture mirrors js/examples/*/src/workspace.ts

const ROOT = "/workspace";
const ENTRY = "shaders/main.wesl";
const ENTRY_URI = `file://${ROOT}/${ENTRY}`;

const ENTRY_SOURCE = `const SCALE: f32 = 2.0;

fn double(value: f32) -> f32 {
	return value * SCALE;
}

@compute @workgroup_size(1)
fn main() {
	let doubled = double(21.0);
}
`;

const FILES: WorkspaceFiles = {
	// No `root` key: the loader defaults to ./shaders, as simple_wesl expects.
	"wesl.toml": 'edition = "2026_pre"\n',
	[ENTRY]: ENTRY_SOURCE,
};

const ARTIFACTS: readonly (readonly [directory: string, name: string, command: string])[] = [
	[ASSETS, "wgsl_analyzer.js", "pnpm --filter wgsl-analyzer-web run build:wasm"],
	[ASSETS, "wgsl_analyzer.wasm", "pnpm --filter wgsl-analyzer-web run build:wasm"],
	[DIST, "framing.js", "pnpm --filter wgsl-analyzer-web run build"],
	[DIST, "fs.js", "pnpm --filter wgsl-analyzer-web run build"],
];

function preflight() {
	const missing = ARTIFACTS.filter(([directory, name]) => !existsSync(join(directory, name)));
	if (missing.length === 0) return;
	const commands = [...new Set(missing.map(([, , command]) => command))];
	throw new Error(
		[
			`${missing.length} build artifact(s) are missing:`,
			...missing.map(([directory, name]) => `  - ${join(directory, name)}`),
			"",
			"Run, from the repository root:",
			...commands.map((command) => `  ${command}`),
			"",
			"build:wasm needs emcc on PATH (source emsdk_env.sh) and a nightly",
			"toolchain with rust-src, because it links with -Zbuild-std.",
		].join("\n"),
	);
}

type JsonRpcId = number | string;

interface JsonRpcError {
	readonly code: number;
	readonly message: string;
}

/** Server to client, expecting a reply. Distinguished by carrying both an id and a method. */
interface JsonRpcRequest {
	readonly id: JsonRpcId;
	readonly method: string;
}

interface JsonRpcResponse<T = unknown> {
	readonly id: JsonRpcId;
	readonly result?: T;
	readonly error?: JsonRpcError;
}

/** Neither a request nor a response: carries a method but no id. */
interface JsonRpcNotification {
	readonly method: string;
}

/** The `initialize` result members asserted below. */
interface InitializeResult {
	readonly capabilities: {
		readonly textDocumentSync?: unknown;
		readonly diagnosticProvider?: unknown;
	};
	readonly serverInfo?: { readonly name?: string };
}

/** The `textDocument/diagnostic` result members asserted below. */
interface DocumentDiagnosticReport {
	readonly kind: string;
	readonly items: readonly unknown[];
}

/** A pending `waitFor`. Each owns its own predicate, so only the outcome is shared. */
interface Waiter {
	/** Returns the first matching message not yet taken, or undefined. */
	readonly scan: () => unknown;
	resolve(message: unknown): void;
}

const isObject = (value: unknown): value is Record<string, unknown> =>
	typeof value === "object" && value !== null;

const isResponse = (message: unknown): message is JsonRpcResponse =>
	isObject(message) && "id" in message && ("result" in message || "error" in message);

const isRequest = (message: unknown): message is JsonRpcRequest =>
	isObject(message) && "id" in message && "method" in message;

async function startServer() {
	const load = <T>(directory: string, name: string): Promise<T> =>
		import(pathToFileURL(join(directory, name)).href) as Promise<T>;
	const { createFrameDecoder, encodeFrame } = await load<typeof import("../../dist/framing.js")>(
		DIST,
		"framing.js",
	);
	const { seedWorkspace } = await load<typeof import("../../dist/fs.js")>(DIST, "fs.js");
	const { default: createWgslAnalyzer } = await load<{ default: ModuleFactory }>(
		ASSETS,
		"wgsl_analyzer.js",
	);

	const stderr: string[] = [];
	const received: unknown[] = [];
	const consumed = new WeakSet<object>();
	const waiters = new Set<Waiter>();
	const framingErrors: string[] = [];
	const exited = Promise.withResolvers<number>();

	let wakeups = 0;
	let stopPump = () => {};

	// `EXIT_RUNTIME=1` makes emscripten's Node `quit_` assign the server's status
	// to `process.exitCode`, which node:test also owns. Left alone that either
	// masks a failing run or reddens a passing one.
	const inheritedExitCode = process.exitCode;

	const module = await createWgslAnalyzer({
		noInitialRun: true,
		// fd 2 is left unwrapped, so stderr still arrives through emscripten's
		// own line-buffered path; pthreads proxy it back to this thread.
		printErr: (line: string) => {
			stderr.push(line);
			if (stderr.length > 500) stderr.shift();
		},
		onExit: (code: number) => {
			// Synchronous inside `proc_exit`, which has already called
			// `terminateAllThreads()` and is about to set ABORT. No JS turn can
			// interleave, so this is the only point at which stopping the pump
			// is guaranteed not to call into a half-dead runtime.
			stopPump();
			exited.resolve(code);
		},
	});

	// Seeded after the factory resolves rather than from `preRun`: `preRun` runs
	// before `__wasm_call_ctors`, and WasmFS needs its static constructors.
	// Still comfortably before `main()`.
	seedWorkspace(module.FS, ROOT, FILES);
	module.FS.chdir(ROOT);
	// WasmFS's `chdir` does not go through `FS.handleError`, so a failure here
	// is otherwise silent.
	assert.equal(module.FS.cwd(), ROOT, "FS.chdir did not take effect");

	function describeState(headline: string): string {
		const summarize = (message: unknown): string => {
			if (isRequest(message)) return `request  id=${message.id} ${message.method}`;
			if (isResponse(message)) {
				const outcome = message.error
					? `ERROR ${message.error.code} ${message.error.message}`
					: "ok";
				return `response id=${message.id} ${outcome}`;
			}
			return `notify   ${(message as JsonRpcNotification).method}`;
		};
		const tail = received.slice(-12);
		return [
			headline,
			"",
			`server messages (${received.length} total, last ${tail.length}):`,
			...(tail.length ? tail.map((m) => `  ${summarize(m)}`) : ["  (none)"]),
			"",
			`stderr (last ${Math.min(stderr.length, 25)} of ${stderr.length}):`,
			...(stderr.length ? stderr.slice(-25).map((line) => `  ${line}`) : ["  (none)"]),
			"",
			`pump: ${wakeups} wakeups through _lsp_stdout_signal_ptr`,
			...(framingErrors.length
				? ["", "framing errors:", ...framingErrors.map((r) => `  ${r}`)]
				: []),
		].join("\n");
	}

	function send(message: unknown): void {
		const bytes = encodeFrame(message);
		const pointer = module._malloc(bytes.length);
		assert.notEqual(pointer, 0, "out of memory queuing stdin");
		try {
			module.HEAPU8.set(bytes, pointer);
			// 0 ok, -1 invalid pointer, -2 stdin closed.
			const status = module._lsp_stdin_push(pointer, bytes.length);
			assert.equal(status, 0, `_lsp_stdin_push returned ${status}`);
		} finally {
			module._free(pointer);
		}
	}

	function deliver(message: unknown): void {
		received.push(message);
		// A message carrying both `id` and `method` is a server-to-client
		// request. `switch_workspaces` sends `client/registerCapability`
		// unprompted, and its id comes from the server's own counter, which
		// collides numerically with ours — hence matching on shape, not just id.
		if (isRequest(message)) send({ jsonrpc: "2.0", id: message.id, result: null });
		for (const waiter of [...waiters]) {
			const match = waiter.scan();
			if (match !== undefined) {
				waiters.delete(waiter);
				waiter.resolve(match);
			}
		}
	}

	/**
	 * Resolves with the first matching message no earlier call has taken.
	 * Already-received messages are eligible, so nothing is lost by being
	 * awaited a turn late.
	 */
	function waitFor(
		predicate: (message: unknown) => boolean,
		what: string,
		ms = STEP_MS,
	): Promise<unknown> {
		const scan = (): unknown => {
			for (const message of received) {
				if (isObject(message) && !consumed.has(message) && predicate(message)) {
					consumed.add(message);
					return message;
				}
			}
			return undefined;
		};
		const immediate = scan();
		if (immediate !== undefined) return Promise.resolve(immediate);
		return new Promise((resolve, reject) => {
			// Assigned below, before any turn in which the timer or `deliver`
			// could reach it.
			let timer!: ReturnType<typeof setTimeout>;
			const waiter: Waiter = {
				scan,
				resolve: (message) => {
					clearTimeout(timer);
					resolve(message);
				},
			};
			timer = setTimeout(() => {
				waiters.delete(waiter);
				reject(new Error(describeState(`timed out after ${ms} ms waiting for ${what}`)));
			}, ms);
			waiters.add(waiter);
		});
	}

	let nextId = 0;
	async function request<T = unknown>(
		method: string,
		params: unknown,
		ms = STEP_MS,
	): Promise<JsonRpcResponse<T>> {
		const id = ++nextId;
		send({ jsonrpc: "2.0", id, method, params });
		return (await waitFor(
			(message) => isResponse(message) && message.id === id,
			`the response to ${method} (id ${id})`,
			ms,
		)) as JsonRpcResponse<T>;
	}

	const notify = (method: string, params: unknown): void =>
		send({ jsonrpc: "2.0", method, params });

	// `StdoutPipe::wake` bumps a counter and issues `memory.atomic.notify`. The
	// counter is read *before* draining, so a write landing at any point during
	// the drain leaves a different value and `waitAsync` returns synchronously
	// with "not-equal". Reading it after draining would lose wakeups; do not
	// "simplify" this ordering.
	//
	// `Atomics.waitAsync` does work on Node's main thread, but a pending wait
	// does not keep the event loop alive, and a harness that hangs instead of
	// failing is the worst outcome. So the production path runs, an unref'd
	// interval backstops it, and a test below asserts the production path
	// actually delivered — a pure poll would pass even with `wake` removed.
	const outputPointer = module._malloc(OUTPUT_CHUNK);
	assert.notEqual(outputPointer, 0, "out of memory allocating the stdout buffer");
	const pushChunk = createFrameDecoder(deliver, (reason) => framingErrors.push(reason));
	const signalIndex = module._lsp_stdout_signal_ptr() >>> 2;
	let stopped = false;

	const drain = () => {
		if (stopped) return;
		for (;;) {
			const count = module._lsp_stdout_pop(outputPointer, OUTPUT_CHUNK);
			if (count < 0) throw new Error(`_lsp_stdout_pop returned ${count}`);
			if (count === 0) return;
			// Consumed fully before the next pop overwrites it, so no copy.
			pushChunk(module.HEAPU8.subarray(outputPointer, outputPointer + count));
		}
	};

	const wait = () => {
		if (stopped) return;
		const signal = new Int32Array(module.HEAPU8.buffer);
		const observed = Atomics.load(signal, signalIndex);
		drain();
		if (stopped) return;
		const result = Atomics.waitAsync(signal, signalIndex, observed);
		if (result.async) {
			void result.value.then((outcome) => {
				if (outcome === "ok") wakeups += 1;
				wait();
			});
		} else {
			// Already changed; yield so this cannot starve the loop.
			setTimeout(wait, 0);
		}
	};

	const safety = setInterval(drain, SAFETY_POLL_MS);
	// Must not be what holds the process open, or a hang looks like progress.
	safety.unref();
	stopPump = () => {
		stopped = true;
		clearInterval(safety);
	};
	wait();

	// Started only once the pump is live, so the handshake cannot be written
	// before anything is draining it. Returns immediately under
	// -sPROXY_TO_PTHREAD=1, which runs main() on a pthread.
	module.callMain([]);

	return {
		module,
		received,
		request,
		notify,
		waitFor,
		describeState,
		get wakeups() {
			return wakeups;
		},
		get framingErrors() {
			return framingErrors;
		},

		/**
		 * shutdown -> exit -> close stdin, then wait for the runtime to unwind.
		 *
		 * The order matters. The `shutdown` response is the last thing the
		 * server writes and has to be drained before `proc_exit` terminates the
		 * threads. Closing stdin is what lets `lsp_server`'s reader finish so
		 * `io_threads.join()` returns: on any path where the main loop bailed
		 * early the reader is still parked in `read(0)`, and the strong-
		 * referenced proxied-main worker would keep Node alive indefinitely.
		 */
		async dispose() {
			if (stopped) return exited.promise;
			try {
				await request("shutdown", null);
			} catch {
				// Fall through: closing stdin still has to happen.
			}
			notify("exit", null);
			module._lsp_stdin_close();
			try {
				return await Promise.race([
					exited.promise,
					new Promise((_, reject) =>
						setTimeout(
							() => reject(new Error(describeState(`no exit within ${STEP_MS} ms`))),
							STEP_MS,
						),
					),
				]);
			} finally {
				stopPump();
				process.exitCode = inheritedExitCode;
			}
		},
	};
}

// While `main()` runs the proxied-main worker is strongly referenced, so Node
// will not exit on its own. unref means this only fires while something else
// holds the loop open — exactly the situation that needs breaking.
const watchdog = setTimeout(() => {
	process.stderr.write(
		`\nsmoke test exceeded ${TOTAL_MS} ms and is being killed; `
			+ "emscripten's pthread pool prevents a clean unwind.\n",
	);
	process.exit(1);
}, TOTAL_MS);
watchdog.unref();

describe("wasm32-unknown-emscripten build", () => {
	// Definitely assigned by `before`; if that throws, node:test fails the suite
	// rather than running the cases below against an unset value.
	let server!: Awaited<ReturnType<typeof startServer>>;

	before(
		async () => {
			preflight();
			const size = statSync(join(ASSETS, "wgsl_analyzer.wasm")).size;
			process.stderr.write(
				`# wgsl_analyzer.wasm is ${(size / 1024 / 1024).toFixed(1)} MB `
					+ `(${size > 20 * 1024 * 1024 ? "debug" : "release"} build)\n`,
			);
			server = await startServer();
		},
		{ timeout: BOOT_MS },
	);

	after(async () => {
		clearTimeout(watchdog);
		await server?.dispose().catch(() => {});
	});

	it("exports everything the link flags promise", () => {
		for (const name of RUNTIME_METHODS) {
			assert.notEqual(
				server.module[name],
				undefined,
				`Module.${name} is missing. Check -sEXPORTED_RUNTIME_METHODS in `
					+ "[target.wasm32-unknown-emscripten] in .cargo/config.toml.",
			);
		}
		// Read through an index view: WASM_EXPORTS mirrors the link flags, which
		// name more than `EmscriptenModule` declares.
		const exported = server.module as unknown as Record<string, unknown>;
		for (const name of WASM_EXPORTS) {
			assert.equal(
				typeof exported[name],
				"function",
				`Module.${name} is missing. Check -sEXPORTED_FUNCTIONS in `
					+ "[target.wasm32-unknown-emscripten] in .cargo/config.toml. Release "
					+ "builds run Binaryen meta-DCE, which drops anything not named there.",
			);
		}
		for (const member of FS_MEMBERS) {
			assert.equal(
				typeof server.module.FS[member],
				"function",
				`Module.FS.${member} is missing; -sFORCE_FILESYSTEM is what makes `
					+ "WasmFS emit the JS filesystem API that seeding calls.",
			);
		}
		assert.equal(
			server.module.HEAPU8.buffer instanceof SharedArrayBuffer,
			true,
			"the heap is not shared memory, so -pthread did not reach the link",
		);
		assert.notEqual(
			server.module._lsp_stdout_signal_ptr(),
			0,
			"_lsp_stdout_signal_ptr returned a null address",
		);
	});

	it("completes an LSP handshake", { timeout: STEP_MS }, async () => {
		const response = await server.request<InitializeResult>("initialize", {
			processId: null,
			clientInfo: { name: "wgsl-analyzer-web smoke test", version: "0" },
			rootUri: `file://${ROOT}`,
			workspaceFolders: [{ uri: `file://${ROOT}`, name: "workspace" }],
			capabilities: {
				general: { positionEncodings: ["utf-16"] },
				workspace: { workspaceFolders: true },
				textDocument: {
					synchronization: { dynamicRegistration: false },
					hover: { contentFormat: ["markdown", "plaintext"] },
					diagnostic: { dynamicRegistration: false, relatedDocumentSupport: false },
				},
			},
		});

		assert.equal(response.error, undefined, server.describeState("initialize failed"));
		const { result } = response;
		assert.ok(result, server.describeState("initialize returned no result"));
		assert.equal(typeof result.capabilities, "object");
		assert.equal(result.serverInfo?.name, "wgsl-analyzer");
		// Spot checks, so an empty object cannot pass as a capability set.
		assert.notEqual(result.capabilities.textDocumentSync, undefined);
		assert.notEqual(result.capabilities.diagnosticProvider, undefined);

		server.notify("initialized", {});
		server.notify("textDocument/didOpen", {
			textDocument: { uri: ENTRY_URI, languageId: "wesl", version: 1, text: ENTRY_SOURCE },
		});
	});

	it("answers a pull-diagnostics request", { timeout: STEP_MS }, async () => {
		// Only the shape is asserted: the server returns a well-formed empty
		// report while the VFS is still loading. That is still worth having —
		// it proves request routing and a structured response body survive
		// the writev bridge in both directions.
		const response = await server.request<DocumentDiagnosticReport>("textDocument/diagnostic", {
			textDocument: { uri: ENTRY_URI },
		});
		assert.equal(response.error, undefined, server.describeState("diagnostic failed"));
		const { result } = response;
		assert.ok(result, server.describeState("diagnostic returned no result"));
		assert.equal(result.kind, "full");
		assert.equal(Array.isArray(result.items), true);
	});

	it("reassembled every frame it received", () => {
		assert.deepEqual(server.framingErrors, [], "the frame decoder reported errors");
		assert.notEqual(server.received.length, 0, "no messages arrived at all");
	});

	it("was woken through _lsp_stdout_signal_ptr", () => {
		// Without this the safety poll would carry every byte on its own, so a
		// regression in StdoutPipe::wake would pass unnoticed here and leave the
		// browser with no wakeup path at all.
		assert.equal(
			server.wakeups > 0,
			true,
			server.describeState(
				"stdout never woke the host; every byte arrived through the safety poll. "
					+ "Suspect StdoutPipe::wake in crates/emscripten-stdio.",
			),
		);
	});

	it("shuts down and exits with 0", { timeout: STEP_MS }, async () => {
		const code = await server.dispose();
		assert.equal(code, 0, server.describeState(`the server exited with ${code}`));
	});
});
