import { argv } from "node:process";
import * as esbuild from "esbuild";
import { nodeModulesPolyfillPlugin } from "esbuild-plugins-node-modules-polyfill";

await esbuild.build({
	absWorkingDir: import.meta.dirname,
	entryPoints: ["src/web/main.ts"],
	outfile: "out/web/main.js",
	bundle: true,
	external: ["vscode"],
	format: "cjs",
	platform: "browser",
	target: "es2022",
	// `vscode-languageclient` exports `./node` only under this condition.
	conditions: ["node"],
	define: { global: "globalThis" },
	// vscode-jsonrpc's Node runtime calls the global `setImmediate`.
	inject: ["node:timers"],
	plugins: [
		nodeModulesPolyfillPlugin({
			globals: { Buffer: true, process: true },
			modules: {
				assert: true,
				buffer: true,
				os: true,
				path: true,
				process: true,
				timers: true,
				util: true,
				// Imported by the Node language client, but never called on the web.
				child_process: "empty",
				crypto: "empty",
				fs: "empty",
				net: "empty",
				readline: "empty",
			},
		}),
	],
	minify: argv.includes("--minify"),
	sourcemap: argv.includes("--sourcemap"),
	logLevel: "info",
});
