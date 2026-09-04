#!/usr/bin/env node
/**
 * Builds the wgsl-analyzer binary for wasm32-unknown-emscripten and stages the
 * artifacts in `dist/`.
 *
 * Usage: node scripts/build-wasm.mjs [--debug]
 */

import { spawnSync } from "node:child_process";
import { copyFileSync, mkdirSync, statSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

const PACKAGE_ROOT = resolve(import.meta.dirname, "..");
const WORKSPACE_ROOT = resolve(PACKAGE_ROOT, "../..");
const TARGET = "wasm32-unknown-emscripten";

const debug = process.argv.includes("--debug");
const profile = debug ? "debug" : "release";

function fail(message) {
	console.error(`\nbuild-wasm: ${message}\n`);
	process.exit(1);
}

// emcc is not on PATH by default; the emsdk has to be sourced first.
if (spawnSync("emcc", ["--version"], { stdio: "ignore" }).status !== 0) {
	fail(
		"emcc was not found on PATH.\n" +
			"Activate the emscripten SDK first, for example:\n" +
			"  source /path/to/emsdk/emsdk_env.sh",
	);
}

const cargoArguments = [
	"+nightly",
	"build",
	// The shipped rust-std for this target is built without the wasm `atomics`
	// feature, so it cannot be linked with -pthread. std has to be rebuilt.
	"-Zbuild-std=std,panic_unwind",
	"--package",
	"wgsl-analyzer",
	"--bin",
	"wgsl-analyzer",
	"--target",
	TARGET,
];
if (!debug) cargoArguments.push("--release");

console.log(`build-wasm: cargo ${cargoArguments.join(" ")}`);
const build = spawnSync("cargo", cargoArguments, {
	cwd: WORKSPACE_ROOT,
	stdio: "inherit",
});
if (build.status !== 0) {
	fail(`cargo exited with status ${build.status ?? "unknown"}`);
}

const outputDirectory = join(WORKSPACE_ROOT, "target", TARGET, profile);
const distDirectory = join(PACKAGE_ROOT, "dist");
mkdirSync(distDirectory, { recursive: true });

// The glue is renamed from the bin name back to the crate name, and this is
// load-bearing rather than cosmetic: emcc emits pthread bootstrap code that does
// `new Worker(new URL("wgsl_analyzer.js", import.meta.url))`. Cargo renames the
// artifact to the bin name, so shipping it that way 404s every pthread.
const artifacts = [
	["wgsl-analyzer.js", "wgsl_analyzer.js"],
	["wgsl_analyzer.wasm", "wgsl_analyzer.wasm"],
];

for (const [from, to] of artifacts) {
	const source = join(outputDirectory, from);
	try {
		statSync(source);
	} catch {
		fail(`expected artifact is missing: ${source}`);
	}
	const destination = join(distDirectory, to);
	mkdirSync(dirname(destination), { recursive: true });
	copyFileSync(source, destination);
	const megabytes = (statSync(destination).size / 1024 / 1024).toFixed(1);
	console.log(`build-wasm: ${to} (${megabytes} MB)`);
}

console.log(`build-wasm: staged ${profile} artifacts in ${distDirectory}`);
