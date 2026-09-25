/**
 * End-to-end smoke test for the `wasm32-unknown-emscripten` build, under Node.
 *
 * This drives the emscripten glue through `src/host.ts`, as `src/worker.ts` does, rather than
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
import {
	AbstractMessageReader,
	AbstractMessageWriter,
	createMessageConnection,
	type DataCallback,
	type Disposable,
	Message,
	type MessageReader,
	type MessageWriter,
	type NotificationMessage,
	ResponseError,
} from "vscode-jsonrpc/node";

// Type-only, so these are erased and do not hoist a runtime import of `dist/`.
import type { EmscriptenModule, ModuleFactory } from "../../dist/emscripten.js";
import type { EmscriptenFs } from "../../dist/fs.js";
import type { Host } from "../../dist/host.js";
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
const RUNTIME_METHODS: readonly (keyof EmscriptenModule)[] = [
	"FS",
	"callMain",
	"stringToNewUTF8",
	"UTF8ToString",
	"addFunction",
];

/** `-sEXPORTED_FUNCTIONS`, verbatim. */
const WASM_EXPORTS: readonly string[] = [
	"_main",
	"_free",
	"_lsp_push_message",
	"_lsp_set_on_message",
];

/**
 * The `FS` members `src/fs.ts` calls. These exist only because of
 * `-sFORCE_FILESYSTEM`: WasmFS emits just the JS API it can prove it needs, so
 * dropping that flag leaves `Module.FS` present but hollow — which a check on
 * the name `FS` alone would not notice.
 */
const FS_MEMBERS: readonly (keyof EmscriptenFs)[] = [
	"mkdirTree",
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
	[ASSETS, "wgsl_analyzer.js", "cargo xtask build-web"],
	[ASSETS, "wgsl_analyzer.wasm", "cargo xtask build-web"],
	[DIST, "host.js", "pnpm --filter wgsl-analyzer-web run build"],
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
			"Run `cargo xtask build-web` from the repository root, `pnpm` from js/:",
			...commands.map((command) => `  ${command}`),
			"",
			"build-web needs emcc on PATH (source emsdk_env.sh) and a nightly",
			"toolchain with rust-src, because it links with -Zbuild-std.",
		].join("\n"),
	);
}

/** `ServerCancelled`: the request lost a race with a change, typically the workspace load. */
const SERVER_CANCELLED = -32802;

/** The id `dispose` sends `shutdown` with. A string, so it cannot collide with the connection's. */
const SHUTDOWN_ID = "shutdown";

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

/** Feeds the connection every message `startHost` delivers. */
class HostReader extends AbstractMessageReader implements MessageReader {
	#callback: DataCallback | undefined;

	override listen(callback: DataCallback): Disposable {
		this.#callback = callback;
		return {
			dispose: () => {
				this.#callback = undefined;
			},
		};
	}

	deliver(message: Message): void {
		this.#callback?.(message);
	}
}

/** Sends the connection's messages to the host it is attached to. */
class HostWriter extends AbstractMessageWriter implements MessageWriter {
	#host: Host | undefined;

	attach(host: Host): void {
		this.#host = host;
	}

	write(message: Message): Promise<void> {
		this.#host?.send(message);
		return Promise.resolve();
	}

	end(): void {}
}

async function startServer() {
	const load = <T>(directory: string, name: string): Promise<T> =>
		import(pathToFileURL(join(directory, name)).href) as Promise<T>;
	const { startHost } = await load<typeof import("../../dist/host.js")>(DIST, "host.js");
	const { default: createWgslAnalyzer } = await load<{ default: ModuleFactory }>(
		ASSETS,
		"wgsl_analyzer.js",
	);

	const stderr: string[] = [];
	const received: Message[] = [];
	const exited = Promise.withResolvers<number>();

	let stopped = false;
	/** Whether the `shutdown` response arrived before `onExit`. */
	let shutdownAnswered = false;

	// `EXIT_RUNTIME=1` makes emscripten's Node `quit_` assign the server's status
	// to `process.exitCode`, which node:test also owns. Left alone that either
	// masks a failing run or reddens a passing one.
	const inheritedExitCode = process.exitCode;

	// Captured for the export assertions, which the host does not expose.
	let module!: EmscriptenModule;
	const factory: ModuleFactory = async (options) => {
		module = await createWgslAnalyzer(options);
		return module;
	};

	function recordStderr(line: string): void {
		stderr.push(line);
		if (stderr.length > 500) stderr.shift();
	}

	function describeState(headline: string): string {
		const summarize = (message: Message): string => {
			if (Message.isRequest(message)) return `request  id=${message.id} ${message.method}`;
			if (Message.isResponse(message)) {
				const outcome = message.error
					? `ERROR ${message.error.code} ${message.error.message}`
					: "ok";
				return `response id=${message.id} ${outcome}`;
			}
			return `notify   ${(message as NotificationMessage).method}`;
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
		].join("\n");
	}

	async function withTimeout<T>(promise: Promise<T>, what: string, ms = STEP_MS): Promise<T> {
		let timer: ReturnType<typeof setTimeout> | undefined;
		const timeout = new Promise<never>((_, reject) => {
			timer = setTimeout(
				() => reject(new Error(describeState(`timed out after ${ms} ms waiting for ${what}`))),
				ms,
			);
		});
		try {
			return await Promise.race([promise, timeout]);
		} finally {
			clearTimeout(timer);
		}
	}

	const reader = new HostReader();
	const writer = new HostWriter();
	const log = (line: string): void => recordStderr(`[jsonrpc] ${line}`);
	const connection = createMessageConnection(reader, writer, {
		error: log,
		warn: log,
		info: log,
		log,
	});
	// `switch_workspaces` sends `client/registerCapability` unprompted.
	connection.onRequest(() => null);
	// Listening before `startHost`, so nothing the server sends is missed.
	connection.listen();

	async function request<T>(method: string, params: object, ms = STEP_MS): Promise<T> {
		// Retriggered as a client would, rather than relying on the request
		// arriving after the workspace has loaded.
		for (;;) {
			try {
				return await withTimeout(
					connection.sendRequest<T>(method, params),
					`the response to ${method}`,
					ms,
				);
			} catch (error) {
				if (!(error instanceof ResponseError)) throw error;
				const data = error.data as { retriggerRequest?: boolean } | undefined;
				if (error.code !== SERVER_CANCELLED || !data?.retriggerRequest) {
					throw new Error(describeState(`${method} failed: ${error.code} ${error.message}`), {
						cause: error,
					});
				}
			}
			await new Promise((resolve) => setTimeout(resolve, 50));
		}
	}

	const notify = (method: string, params: object): Promise<void> =>
		connection.sendNotification(method, params);

	const host: Host = await startHost(factory, {
		root: ROOT,
		files: FILES,
		args: [],
		onMessage: (message) => {
			received.push(message as Message);
			reader.deliver(message as Message);
		},
		onStderr: recordStderr,
		onExit: (code) => {
			shutdownAnswered = received.some(
				(message) => Message.isResponse(message) && message.id === SHUTDOWN_ID,
			);
			stopped = true;
			exited.resolve(code);
		},
	});
	// Nothing was written before this: the connection answers server requests on a later turn.
	writer.attach(host);

	return {
		module,
		stderr,
		received,
		request,
		notify,
		describeState,
		get shutdownAnswered() {
			return shutdownAnswered;
		},

		/**
		 * shutdown -> exit, back to back, then wait for the runtime to unwind.
		 *
		 * The `shutdown` response is the last thing the server writes, and it is
		 * not awaited here: `main()` returns only once the host has received every
		 * message, so the response still arrives before the exit.
		 *
		 * Both go straight to the host rather than through the connection, which
		 * dispatches a response only on a later turn, one the exit may beat.
		 */
		async dispose() {
			if (stopped) return exited.promise;
			host.send({ jsonrpc: "2.0", id: SHUTDOWN_ID, method: "shutdown" });
			host.send({ jsonrpc: "2.0", method: "exit" });
			try {
				return await withTimeout(exited.promise, "the exit");
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

/** Tells this run apart from `lsp.no-wait-async.test.ts`, which imports it. */
const VARIANT = "waitAsync" in Atomics ? "" : " (postMessage fallback)";

describe(`wasm32-unknown-emscripten build${VARIANT}`, () => {
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
	});

	it("completes an LSP handshake", { timeout: STEP_MS }, async () => {
		const result = await server.request<InitializeResult>("initialize", {
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
		// it proves request routing and a structured response body survive the
		// transport in both directions.
		const result = await server.request<DocumentDiagnosticReport>("textDocument/diagnostic", {
			textDocument: { uri: ENTRY_URI },
		});
		assert.ok(result, server.describeState("diagnostic returned no result"));
		assert.equal(result.kind, "full");
		assert.equal(Array.isArray(result.items), true);
	});

	it("carries a large message", { timeout: STEP_MS }, async () => {
		// Far larger than anything above, so the copy into the module goes through a
		// fresh `malloc` that may grow the shared heap. A truncated body is not valid
		// JSON, and the server would drop it.
		server.notify("textDocument/didOpen", {
			textDocument: {
				uri: `file://${ROOT}/oversized.wesl`,
				languageId: "wesl",
				version: 1,
				text: `${ENTRY_SOURCE}// ${"x".repeat(96 * 1024)}\n`,
			},
		});

		const result = await server.request<DocumentDiagnosticReport>("textDocument/diagnostic", {
			textDocument: { uri: ENTRY_URI },
		});
		assert.equal(result.kind, "full");
		assert.ok(
			!server.stderr.some((line) => line.includes("malformed LSP message")),
			server.describeState("the server dropped a message as malformed"),
		);
	});

	it("received messages", () => {
		assert.notEqual(server.received.length, 0, "no messages arrived at all");
	});

	it("shuts down and exits with 0", { timeout: STEP_MS }, async () => {
		const code = await server.dispose();
		assert.equal(code, 0, server.describeState(`the server exited with ${code}`));
		assert.ok(
			server.shutdownAnswered,
			server.describeState("the shutdown response did not arrive before the exit"),
		);
	});
});
