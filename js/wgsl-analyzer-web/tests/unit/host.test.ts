/**
 * Unit tests for `src/host.ts`, against a fake module.
 *
 * These import from `dist/` rather than `src/`, so they also assert that `tsc`
 * emitted something loadable. Run `pnpm run build` first.
 */

import assert from "node:assert/strict";
import { describe, it } from "node:test";

import type { EmscriptenModule, ModuleOptions } from "../../dist/emscripten.js";
import { type HostOptions, startHost } from "../../dist/host.js";

/** One recorded call: the method name followed by the arguments it received. */
type Call = [method: string, ...args: unknown[]];

interface Fake {
	readonly host: Awaited<ReturnType<typeof startHost>>;
	readonly calls: Call[];
	/** What the host passed to the factory, so a test can play the module's side. */
	readonly options: ModuleOptions;
	/** Everything the host reported, in order. */
	readonly events: unknown[];
}

/** Starts a host over a fake module. */
async function start(): Promise<Fake> {
	const calls: Call[] = [];
	let cwd = "/";
	let options!: ModuleOptions;

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
	};

	const events: unknown[] = [];
	const hostOptions: HostOptions = {
		root: "/workspace",
		files: { "a.wgsl": "x" },
		args: [],
		onMessage: (message) => events.push(message),
		onStderr: () => {},
		onExit: (code) => events.push(["exit", code]),
	};
	const host = await startHost((given) => {
		options = given;
		return Promise.resolve(module);
	}, hostOptions);
	return { host, calls, options, events };
}

const named = (calls: Call[], method: string): Call[] => calls.filter((c) => c[0] === method);

/** Runs every pending microtask, and then some. */
const settle = (): Promise<void> => new Promise((resolve) => setImmediate(resolve));

describe("startHost", () => {
	it("seeds, changes directory, then starts main", async () => {
		const { calls } = await start();
		assert.deepEqual(named(calls, "writeFile"), [["writeFile", "/workspace/a.wgsl", "x"]]);
		assert.deepEqual(calls.at(-1), ["callMain", []]);
	});

	it("hands the server each message as one string, in order", async () => {
		const { host, options } = await start();
		host.send({ id: 1 });
		host.send({ id: 2 });
		assert.equal(await options.lspNextMessage(), '{"id":1}');
		assert.equal(await options.lspNextMessage(), '{"id":2}');
	});

	it("resolves a pending read once a message is sent", async () => {
		const { host, options } = await start();
		const next = options.lspNextMessage();
		host.send({ id: 1 });
		assert.equal(await next, '{"id":1}');
	});

	it("delivers the server's messages parsed", async () => {
		const { options, events } = await start();
		options.lspOnMessage('{"id":1}');
		assert.deepEqual(events, [{ id: 1 }]);
	});

	it("delivers messages sent before main exits before onExit", async () => {
		const { options, events } = await start();
		// As emscripten/library.js does.
		queueMicrotask(() => options.lspOnMessage('{"id":1}'));
		options.onExit(0);
		assert.deepEqual(events, []);
		await settle();
		assert.deepEqual(events, [{ id: 1 }, ["exit", 0]]);
	});

	it("hands the server nothing once main has exited", async () => {
		const { host, options } = await start();
		let read = false;
		void options.lspNextMessage().then(() => {
			read = true;
		});
		options.onExit(0);
		host.send({});
		await settle();
		assert.equal(read, false);
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
