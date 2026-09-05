import importMetaUrlPlugin from "@codingame/esbuild-import-meta-url-plugin";
import { defineConfig } from "vite";

/**
 * wgsl-analyzer is compiled with pthreads, so it needs `SharedArrayBuffer`,
 * which browsers only expose to cross-origin isolated pages. Without these
 * headers the worker cannot start at all.
 */
const crossOriginIsolation = {
	"Cross-Origin-Opener-Policy": "same-origin",
	"Cross-Origin-Embedder-Policy": "require-corp",
};

export default defineConfig({
	server: { headers: crossOriginIsolation },
	preview: { headers: crossOriginIsolation },
	worker: { format: "es" },
	optimizeDeps: {
		// The worker and the emscripten glue are served as static assets from
		// public/wgsl-analyzer; they must not be rewritten by the bundler, because
		// the glue spawns its pthread pool relative to its own URL.
		exclude: ["wgsl-analyzer-web"],
		esbuildOptions: {
			// monaco-vscode-api loads its own workers with `new URL(..., import.meta.url)`.
			// Without this, dependency pre-bundling rewrites those URLs and the
			// editor worker 404s, falling back to running on the main thread.
			plugins: [importMetaUrlPlugin],
		},
	},
});
