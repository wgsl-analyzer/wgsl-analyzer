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
		// Pre-bundling would move wgsl-analyzer-web's index.js, which locates its
		// worker relative to its own `import.meta.url`.
		exclude: ["wgsl-analyzer-web"],
	},
});
