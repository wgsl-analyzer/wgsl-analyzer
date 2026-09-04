/**
 * Unit tests for the workspace seeding in `src/fs.ts`.
 *
 * These import from `dist/` rather than `src/`, so they also assert that `tsc`
 * emitted something loadable. Run `pnpm run build` first.
 */

import assert from "node:assert/strict";
import { describe, it } from "node:test";

import { makeDirectories, seedWorkspace, writeFile } from "../../dist/fs.js";

/** errno for "file exists". WASI/emscripten uses 20 here, not POSIX's 17. */
const EEXIST = 20;
/** errno for "no such file or directory", used as a stand-in for a real failure. */
const ENOENT = 44;

/** Records every call, and lets a test make `mkdir` fail on demand. */
function fakeFs({ mkdirFails } = {}) {
	const calls = [];
	return {
		calls,
		directories: () => calls.filter((c) => c[0] === "mkdir").map((c) => c[1]),
		mkdir(path) {
			calls.push(["mkdir", path]);
			const error = mkdirFails?.(path);
			if (error) throw error;
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

const errnoError = (errno) => Object.assign(new Error(`errno ${errno}`), { errno });

describe("makeDirectories", () => {
	it("creates every parent in order", () => {
		const fs = fakeFs();
		makeDirectories(fs, "/a/b/c");
		assert.deepEqual(fs.directories(), ["/a", "/a/b", "/a/b/c"]);
	});

	it("swallows EEXIST", () => {
		const fs = fakeFs({ mkdirFails: (path) => (path === "/a" ? errnoError(EEXIST) : undefined) });
		assert.doesNotThrow(() => makeDirectories(fs, "/a/b"));
		assert.deepEqual(fs.directories(), ["/a", "/a/b"]);
	});

	it("rethrows any other errno", () => {
		// 20 is EEXIST on emscripten; POSIX's 17 is a different error here and
		// must not be swallowed. This is the test that stops someone "fixing"
		// the constant to 17.
		const fs = fakeFs({ mkdirFails: () => errnoError(ENOENT) });
		assert.throws(() => makeDirectories(fs, "/a"), { errno: ENOENT });
	});

	it("rethrows an error carrying no errno", () => {
		const fs = fakeFs({ mkdirFails: () => new Error("boom") });
		assert.throws(() => makeDirectories(fs, "/a"), { message: "boom" });
	});

	it("collapses empty segments", () => {
		const fs = fakeFs();
		makeDirectories(fs, "//a//b/");
		assert.deepEqual(fs.directories(), ["/a", "/a/b"]);
	});
});

describe("writeFile", () => {
	it("creates parents before writing", () => {
		const fs = fakeFs();
		writeFile(fs, "/root/a/b.wgsl", "x");
		assert.deepEqual(fs.calls, [
			["mkdir", "/root"],
			["mkdir", "/root/a"],
			["writeFile", "/root/a/b.wgsl", "x"],
		]);
	});

	it("creates no directory for a bare filename", () => {
		// Regression: `lastIndexOf("/")` is -1 here, and the old `slice(0, -1)`
		// produced "top.wgs" — a truthy string — so this used to mkdir the
		// filename minus its last character.
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
		assert.deepEqual(fs.calls[0], ["mkdir", "/workspace"]);
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
		assert.equal(write[2], bytes);
	});
});
