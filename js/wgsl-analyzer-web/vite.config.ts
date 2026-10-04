import { defineConfig } from "vite";

export default defineConfig({
	publicDir: false,
	build: {
		lib: {
			entry: "src/worker.ts",
			formats: ["es"],
			fileName: () => "worker.js",
		},
		outDir: "dist/assets",
		target: "es2024",
		minify: false,
		rolldownOptions: {
			// Resolved next to worker.js at runtime, see src/worker.ts.
			external: ["./wgsl_analyzer.js"],
		},
	},
});
