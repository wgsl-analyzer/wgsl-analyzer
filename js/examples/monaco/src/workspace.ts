/**
 * The workspace seeded into the server's in-memory filesystem.
 *
 * Modeled on `crates/wgsl-analyzer/src/tests/simple_wesl`: a `wesl.toml` at the
 * root, with sources under the default `./shaders` directory.
 */

export const ROOT = "/workspace";

export const ENTRY = "shaders/main.wesl";

export const FILES: Record<string, string> = {
	"wesl.toml": 'edition = "2026_pre"\n',
	"shaders/main.wesl": `// Edit freely: diagnostics, completion and go-to-definition are served by
// wgsl-analyzer compiled to WebAssembly, running in a Web Worker.

const SCALE: f32 = 2.0;

fn double(value: f32) -> f32 {
	return value * SCALE;
}

@compute @workgroup_size(1)
fn main() {
	let doubled = double(21.0);
}
`,
};
