/**
 * Seeding the emscripten in-memory filesystem.
 *
 * This runs from `Module.preRun`, which the runtime executes before `FS.init()`
 * and before `main()` exists, so the workspace is already on disk by the time
 * the server's VFS scans it.
 */

/** The subset of emscripten's `FS` module this package relies on. */
export interface EmscriptenFs {
	mkdir(path: string): void;
	writeFile(path: string, data: string | Uint8Array): void;
	unlink(path: string): void;
	chdir(path: string): void;
	cwd(): string;
	analyzePath(path: string): { exists: boolean };
}

/** errno for "file exists", the expected failure when a directory is present. */
const EEXIST = 20;

/** Creates `path` and every missing parent directory. */
export function makeDirectories(fs: EmscriptenFs, path: string): void {
	let current = "";
	for (const segment of path.split("/").filter(Boolean)) {
		current += `/${segment}`;
		try {
			fs.mkdir(current);
		} catch (error) {
			const errno = (error as { errno?: number }).errno;
			if (errno !== EEXIST) throw error;
		}
	}
}

/** Writes one file, creating its parent directories first. */
export function writeFile(fs: EmscriptenFs, path: string, contents: string | Uint8Array): void {
	// Guard on the index rather than on the sliced string: `lastIndexOf` returns
	// -1 for a bare filename, and `slice(0, -1)` would then hand `makeDirectories`
	// the filename minus its last character. Index 0 means the parent is the
	// root, which always exists.
	const slash = path.lastIndexOf("/");
	if (slash > 0) makeDirectories(fs, path.slice(0, slash));
	fs.writeFile(path, contents);
}

/** Populates `root` with `files`, whose keys are paths relative to it. */
export function seedWorkspace(
	fs: EmscriptenFs,
	root: string,
	files: Record<string, string | Uint8Array>,
): void {
	makeDirectories(fs, root);
	for (const [relative, contents] of Object.entries(files)) {
		writeFile(fs, `${root}/${relative.replace(/^\/+/, "")}`, contents);
	}
}
