/**
 * Seeding the emscripten in-memory filesystem.
 *
 * This runs once the module factory has resolved, and before `callMain`,
 * so the workspace is already on disk by the time the server's VFS scans it.
 */

/**
 * The subset of emscripten's `FS` module this package relies on.
 *
 * https://emscripten.org/docs/api_reference/Filesystem-API.html#new-file-system-wasmfs
 */
export interface EmscriptenFs {
	mkdirTree(path: string): void;
	writeFile(path: string, data: string | Uint8Array): void;
	unlink(path: string): void;
	chdir(path: string): void;
	cwd(): string;
	analyzePath(path: string): { exists: boolean };
}

/** Writes one file, creating its parent directories first. */
export function writeFile(fs: EmscriptenFs, path: string, contents: string | Uint8Array): void {
	const slash = path.lastIndexOf("/");
	if (slash > 0) fs.mkdirTree(path.slice(0, slash));
	fs.writeFile(path, contents);
}

/** Populates `root` with `files`, whose keys are paths relative to it. */
export function seedWorkspace(
	fs: EmscriptenFs,
	root: string,
	files: Record<string, string | Uint8Array>,
): void {
	fs.mkdirTree(root);
	for (const [relative, contents] of Object.entries(files)) {
		writeFile(fs, `${root}/${relative.replace(/^\/+/, "")}`, contents);
	}
}
