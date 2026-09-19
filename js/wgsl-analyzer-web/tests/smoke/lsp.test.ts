/**
 * End-to-end smoke test for the `wasm32-unknown-emscripten` build, under Node.
 *
 * This drives the emscripten glue the way `src/worker.ts` does rather than
 * through `WgslAnalyzerServer.start()`, which is unusable here: it resolves
 * against `globalThis.location.href` and constructs a web `Worker`. The glue
 * itself is environment-agnostic because `-sENVIRONMENT` is deliberately unset,
 * so under Node it uses `worker_threads`, `createRequire` and `node:fs`.
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
import type { EmscriptenModule, LspTransport, ModuleFactory } from "../../dist/emscripten.js";
import type { EmscriptenFs } from "../../dist/fs.js";
import type { WorkspaceFiles } from "../../dist/protocol.js";

const DIST = join(import.meta.dirname, "..", "..", "dist");
const ASSETS = join(DIST, "assets");

const BOOT_MS = Number(process.env["WA_SMOKE_BOOT_MS"] ?? 180_000);
const STEP_MS = Number(process.env["WA_SMOKE_STEP_MS"] ?? 60_000);
const TOTAL_MS = Number(process.env["WA_SMOKE_TOTAL_MS"] ?? 420_000);

// The contract with .cargo/config.toml

// Typed as keys rather than plain strings, so dropping a member from
// `EmscriptenModule`/`EmscriptenFs` without revisiting the link flags is a
// compile error here rather than a silently weaker assertion.

/** `-sEXPORTED_RUNTIME_METHODS`, verbatim. */
const RUNTIME_METHODS: readonly (keyof EmscriptenModule)[] = ["FS", "callMain"];

/** `-sEXPORTED_FUNCTIONS`, verbatim. */
const WASM_EXPORTS: readonly string[] = ["_main"];

/** What `--pre-js=…/emscripten-io-pre.js` installs on the module. */
const TRANSPORT_METHODS: readonly (keyof EmscriptenModule)[] = ["lspStart"];

/** What `lspStart` hands back. Asserted separately, since the type is structural. */
const TRANSPORT_ENDS: readonly (keyof LspTransport)[] = ["pushBytes", "closeInput"];

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

	let stopped = false;

	// `EXIT_RUNTIME=1` makes emscripten's Node `quit_` assign the server's status
	// to `process.exitCode`, which node:test also owns. Left alone that either
	// masks a failing run or reddens a passing one.
	const inheritedExitCode = process.exitCode;

	let heapShared: boolean | undefined;

	// The server writes a raw byte stream, so frames are reassembled rather than
	// parsed per chunk.
	const decodeChunk = createFrameDecoder(deliver, (reason) => framingErrors.push(reason));
	const pushChunk = (bytes: Uint8Array): void => {
		heapShared ??= bytes.buffer instanceof SharedArrayBuffer;
		decodeChunk(bytes);
	};

	const module = await createWgslAnalyzer({
		noInitialRun: true,
		printErr: (line: string) => {
			stderr.push(line);
			if (stderr.length > 500) stderr.shift();
		},
		onExit: (code: number) => {
			stopped = true;
			exited.resolve(code);
		},
	});

	// Before `callMain`, which is when the server can first write. `onOutput` runs
	// on this thread: `lsp_js_write` is proxied here from the server's writer
	// pthread, so the view into wasm memory is valid for the call and the decoder
	// copies out of it synchronously.
	const transport = module.lspStart({ onOutput: pushChunk });

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
			...(framingErrors.length
				? ["", "framing errors:", ...framingErrors.map((r) => `  ${r}`)]
				: []),
		].join("\n");
	}

	function send(message: unknown): void {
		// Queued on this thread; the server's reader pthread picks it up through a
		// proxied `lsp_js_read`. Silently dropped after `closeInput`, which is only
		// reached from `dispose`.
		transport.pushBytes(encodeFrame(message));
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

	// Returns immediately under -sPROXY_TO_PTHREAD=1, which runs main() on a
	// pthread.
	module.callMain([]);

	return {
		module,
		transport,
		received,
		request,
		notify,
		waitFor,
		describeState,
		get framingErrors() {
			return framingErrors;
		},
		get heapShared() {
			return heapShared;
		},

		/**
		 * shutdown -> exit -> close the input, then wait for the runtime to
		 * unwind.
		 *
		 * The order matters. The `shutdown` response is the last thing the server
		 * writes, and it has to arrive before `proc_exit` terminates the threads.
		 * Closing the input is what lets `lsp_server`'s reader finish so
		 * `io_threads.join()` returns: on any path where the main loop bailed
		 * early the reader is still parked inside a proxied `lsp_js_read`, and the
		 * strong-referenced proxied-main worker would keep Node alive
		 * indefinitely.
		 */
		async dispose() {
			if (stopped) return exited.promise;
			try {
				await request("shutdown", null);
			} catch {
				// Fall through: closing the input still has to happen.
			}
			notify("exit", null);
			transport.closeInput();
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
				stopped = true;
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
		for (const name of TRANSPORT_METHODS) {
			assert.equal(
				typeof server.module[name],
				"function",
				`Module.${name} is missing. It is installed by `
					+ "--pre-js=crates/wgsl-analyzer/src/bin/emscripten-io-pre.js; check that "
					+ "link-arg, and the --js-library beside it, in "
					+ "[target.wasm32-unknown-emscripten] in .cargo/config.toml.",
			);
		}
		for (const name of TRANSPORT_ENDS) {
			assert.equal(
				typeof server.transport[name],
				"function",
				`lspStart did not return ${name}; the --pre-js is stale against ` + "src/emscripten.ts.",
			);
		}
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

	// After the handshake rather than beside the other link-flag assertions:
	// `heapShared` stays `undefined` until the server's first write, and the
	// handshake is the first test that guarantees one.
	it("runs on shared memory", () => {
		assert.equal(
			server.heapShared,
			true,
			"the server's output was not a view into shared memory, so -pthread "
				+ "did not reach the link",
		);
	});

	it("answers a pull-diagnostics request", { timeout: STEP_MS }, async () => {
		// Only the shape is asserted: the server returns a well-formed empty
		// report while the VFS is still loading. That is still worth having —
		// it proves request routing and a structured response body survive the
		// transport in both directions.
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

	it("shuts down and exits with 0", { timeout: STEP_MS }, async () => {
		const code = await server.dispose();
		assert.equal(code, 0, server.describeState(`the server exited with ${code}`));
	});
});
