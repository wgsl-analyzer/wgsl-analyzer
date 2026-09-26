/**
 * Unit tests for the workspace seeding in `src/fs.ts`.
 *
 * These import from `dist/` rather than `src/`, so they also assert that `tsc`
 * emitted something loadable. Run `pnpm run build` first.
 */

import assert from "node:assert/strict";
import { describe, it } from "node:test";

import { type EmscriptenFs, seedWorkspace, writeFile } from "../../dist/fs.js";

/** One recorded call: the method name followed by the arguments it received. */
type Call = [method: string, ...args: unknown[]];

interface FakeFs extends EmscriptenFs {
	readonly calls: Call[];
	/** Just the paths passed to `mkdirTree`, in call order. */
	directories(): unknown[];
}

/** Records every call. */
function fakeFs(): FakeFs {
	const calls: Call[] = [];
	// The return annotation contextually types the methods below, so their
	// parameters do not need repeating here.
	return {
		calls,
		directories: () => calls.filter((c) => c[0] === "mkdirTree").map((c) => c[1]),
		mkdirTree(path) {
			calls.push(["mkdirTree", path]);
		},
		writeFile(path, data) {
			calls.push(["writeFile", path, data]);
		},
		unlink(path) {
			calls.push(["unlink", path]);
		},
		chdir(path) {
			calls.push(["chdir", path]);
		},
		cwd() {
			return "/";
		},
		analyzePath(path) {
			return { exists: calls.some((c) => c[0] === "writeFile" && c[1] === path) };
		},
	};
}

describe("writeFile", () => {
	it("creates parents before writing", () => {
		const fs = fakeFs();
		writeFile(fs, "/root/a/b.wgsl", "x");
		assert.deepEqual(fs.calls, [
			["mkdirTree", "/root/a"],
			["writeFile", "/root/a/b.wgsl", "x"],
		]);
	});

	it("creates no directory for a bare filename", () => {
		const fs = fakeFs();
		writeFile(fs, "top.wgsl", "x");
		assert.deepEqual(fs.directories(), []);
		assert.deepEqual(fs.calls, [["writeFile", "top.wgsl", "x"]]);
	});

	it("creates no directory when the parent is the root", () => {
		const fs = fakeFs();
		writeFile(fs, "/top.wgsl", "x");
		assert.deepEqual(fs.directories(), []);
	});
});

describe("seedWorkspace", () => {
	it("creates the root before any file", () => {
		const fs = fakeFs();
		seedWorkspace(fs, "/workspace", { "a.wgsl": "x" });
		assert.deepEqual(fs.calls[0], ["mkdirTree", "/workspace"]);
	});

	it("strips leading slashes from relative paths", () => {
		const fs = fakeFs();
		seedWorkspace(fs, "/workspace", {
			"/shaders/a.wgsl": "x",
			"///shaders/b.wgsl": "y",
		});
		const written = fs.calls.filter((c) => c[0] === "writeFile").map((c) => c[1]);
		assert.deepEqual(written, ["/workspace/shaders/a.wgsl", "/workspace/shaders/b.wgsl"]);
	});

	it("passes binary contents through by identity", () => {
		const fs = fakeFs();
		const bytes = new Uint8Array([1, 2, 3]);
		seedWorkspace(fs, "/workspace", { "a.bin": bytes });
		const write = fs.calls.find((c) => c[0] === "writeFile");
		assert.ok(write, "seedWorkspace recorded no writeFile call");
		assert.equal(write[2], bytes);
	});
});
