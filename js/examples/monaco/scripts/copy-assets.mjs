#!/usr/bin/env node
/**
 * Stages the worker and the emscripten artifacts as static assets.
 *
 * They are copied rather than imported so the bundler never rewrites their URLs:
 * `worker.js` resolves `./wgsl_analyzer.js` relative to itself, and the glue in
 * turn spawns its pthread pool with
 * `new Worker(new URL("wgsl_analyzer.js", import.meta.url))`. All three files
 * must therefore end up in one directory, under these exact names.
 */

import { copyFileSync, mkdirSync, statSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join, resolve } from "node:path";

const require = createRequire(import.meta.url);
const packageRoot = dirname(require.resolve("wgsl-analyzer-web/package.json"));
const source = join(packageRoot, "dist");
const destination = resolve(import.meta.dirname, "..", "public", "wgsl-analyzer");

const ASSETS = ["worker.js", "wgsl_analyzer.js", "wgsl_analyzer.wasm"];

mkdirSync(destination, { recursive: true });

for (const asset of ASSETS) {
	const from = join(source, asset);
	try {
		statSync(from);
	} catch {
		console.error(
			`copy-assets: ${asset} is missing from ${source}.\n` +
				"Build the package first:\n" +
				"  pnpm --filter wgsl-analyzer-web run build:wasm\n" +
				"  pnpm --filter wgsl-analyzer-web run build",
		);
		process.exit(1);
	}
	copyFileSync(from, join(destination, asset));
}

console.log(`copy-assets: staged ${ASSETS.length} files in ${destination}`);
