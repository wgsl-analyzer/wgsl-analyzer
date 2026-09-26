/**
 * Unit tests for `src/host.ts`, against a fake module.
 *
 * These import from `dist/` rather than `src/`, so they also assert that `tsc`
 * emitted something loadable. Run `pnpm run build` first.
 */

import assert from "node:assert/strict";
import { describe, it, type TestContext } from "node:test";

import type { EmscriptenModule, ModuleOptions } from "../../dist/emscripten.js";
import { type HostOptions, startHost } from "../../dist/host.js";

/** One recorded call: the method name followed by the arguments it received. */
type Call = [method: string, ...args: unknown[]];

interface Config {
	/** Runs inside `_lsp_push_message`. */
	readonly pushMessage?: (body: string) => void;
	/** Runs after each delivered message is recorded. */
	readonly onMessage?: (message: unknown) => void;
}

interface Fake {
	readonly host: Awaited<ReturnType<typeof startHost>>;
	readonly calls: Call[];
	/** What the host passed to the factory, so a test can play the module's side. */
	readonly options: ModuleOptions;
	readonly messages: unknown[];
	/** Hands `body` to the output function, as the server's writer thread does. */
	emit(body: string): void;
}

/** The table index the fake `addFunction` returns. */
const OUTPUT_INDEX = 42;

/** Starts a host over a fake module. */
async function start(config: Config = {}): Promise<Fake> {
	const calls: Call[] = [];
	const strings = new Map<number, string>();
	const allocate = (text: string): number => {
		const pointer = strings.size + 1;
		strings.set(pointer, text);
		return pointer;
	};
	let output: ((pointer: number) => void) | undefined;
	let cwd = "/";
	let options!: ModuleOptions;

	const emit = (body: string): void => output?.(allocate(body));

	const module: EmscriptenModule = {
		FS: {
			mkdirTree: (path) => calls.push(["mkdirTree", path]),
			writeFile: (path, data) => calls.push(["writeFile", path, data]),
			unlink: (path) => calls.push(["unlink", path]),
			chdir: (path) => {
				cwd = path;
			},
			cwd: () => cwd,
			analyzePath: (path) => ({ exists: calls.some((c) => c[0] === "writeFile" && c[1] === path) }),
		},
		callMain: (args) => calls.push(["callMain", args]),
		stringToNewUTF8: allocate,
		UTF8ToString: (pointer) => strings.get(pointer) ?? "",
		addFunction: (fn) => {
			calls.push(["addFunction"]);
			output = fn;
			return OUTPUT_INDEX;
		},
		_free: (pointer) => calls.push(["_free", pointer]),
		_lsp_push_message: (pointer) => {
			const body = strings.get(pointer) ?? "";
			calls.push(["_lsp_push_message", body]);
			config.pushMessage?.(body);
		},
		_lsp_set_on_message: (index) => calls.push(["_lsp_set_on_message", index]),
	};

	const messages: unknown[] = [];
	const hostOptions: HostOptions = {
		root: "/workspace",
		files: { "a.wgsl": "x" },
		args: [],
		onMessage: (message) => {
			messages.push(message);
			config.onMessage?.(message);
		},
		onStderr: () => {},
		onExit: () => {},
	};
	const host = await startHost((given) => {
		options = given;
		return Promise.resolve(module);
	}, hostOptions);
	return { host, calls, options, messages, emit };
}

const named = (calls: Call[], method: string): Call[] => calls.filter((c) => c[0] === method);

/** Collects what the host rethrows from a microtask, instead of letting it fail the run. */
function catchRethrown(t: TestContext): (() => void)[] {
	const rethrown: (() => void)[] = [];
	t.mock.method(globalThis, "queueMicrotask", (callback: () => void) => rethrown.push(callback));
	return rethrown;
}

describe("startHost", () => {
	it("seeds, changes directory, then starts main", async () => {
		const { calls } = await start();
		assert.deepEqual(named(calls, "writeFile"), [["writeFile", "/workspace/a.wgsl", "x"]]);
		assert.deepEqual(calls.at(-1), ["callMain", []]);
	});

	it("registers the output function before starting main", async () => {
		const { calls } = await start();
		const registered = calls.findIndex((c) => c[0] === "_lsp_set_on_message");
		assert.deepEqual(calls[registered], ["_lsp_set_on_message", OUTPUT_INDEX]);
		assert.ok(calls.findIndex((c) => c[0] === "addFunction") < registered);
		assert.ok(registered < calls.findIndex((c) => c[0] === "callMain"));
	});

	it("sends each message as one string, and frees it", async () => {
		const { host, calls } = await start();
		host.send({ id: 1 });
		assert.deepEqual(named(calls, "_lsp_push_message"), [["_lsp_push_message", '{"id":1}']]);
		assert.equal(named(calls, "_free").length, 1);
	});

	it("frees the string when the push throws", async () => {
		const { host, calls } = await start({
			pushMessage: () => {
				throw new Error("boom");
			},
		});
		assert.throws(() => host.send({}), /boom/);
		assert.equal(named(calls, "_free").length, 1);
	});

	it("delivers each message in order, before the output function returns", async () => {
		const { messages, emit } = await start();
		emit('{"id":1}');
		emit('{"id":2}');
		emit('{"id":3}');
		assert.deepEqual(messages, [{ id: 1 }, { id: 2 }, { id: 3 }]);
	});

	it("keeps delivering after onMessage throws, and rethrows from a microtask", async (t) => {
		const rethrown = catchRethrown(t);
		const { messages, emit } = await start({
			onMessage: (message) => {
				if ((message as { id: number }).id === 1) throw new Error("boom");
			},
		});
		assert.doesNotThrow(() => emit('{"id":1}'));
		emit('{"id":2}');
		emit('{"id":3}');
		assert.deepEqual(messages, [{ id: 1 }, { id: 2 }, { id: 3 }]);
		assert.equal(rethrown.length, 1);
		assert.throws(() => rethrown[0]?.(), /boom/);
	});

	it("rethrows output that is not JSON from a microtask", async (t) => {
		const rethrown = catchRethrown(t);
		const { messages, emit } = await start();
		assert.doesNotThrow(() => emit("{"));
		assert.deepEqual(messages, []);
		assert.equal(rethrown.length, 1);
		assert.throws(() => rethrown[0]?.(), SyntaxError);
	});

	it("lets the runtime's exit unwind out of the output function", async (t) => {
		const rethrown = catchRethrown(t);
		const fake: Fake = await start({
			onMessage: () => {
				fake.options.onExit(0);
				throw new Error("ExitStatus");
			},
		});
		assert.throws(() => fake.emit('{"id":1}'), /ExitStatus/);
		assert.deepEqual(rethrown, []);
	});

	it("lets onMessage send synchronously", async () => {
		const fake: Fake = await start({ onMessage: (message) => fake.host.send(message) });
		fake.emit('{"id":1}');
		assert.deepEqual(named(fake.calls, "_lsp_push_message"), [["_lsp_push_message", '{"id":1}']]);
	});

	it("calls nothing in the module once main has exited", async () => {
		const { host, options, calls } = await start();
		options.onExit(0);
		const before = calls.length;
		host.send({});
		host.writeFile("b.wgsl", "y");
		host.deleteFile("a.wgsl");
		assert.equal(calls.length, before);
	});

	it("resolves paths against the root, ignoring leading slashes", async () => {
		const { host, calls } = await start();
		host.writeFile("/dir/b.wgsl", "y");
		host.deleteFile("//a.wgsl");
		assert.deepEqual(named(calls, "writeFile").at(-1), ["writeFile", "/workspace/dir/b.wgsl", "y"]);
		assert.deepEqual(named(calls, "unlink"), [["unlink", "/workspace/a.wgsl"]]);
	});
});
