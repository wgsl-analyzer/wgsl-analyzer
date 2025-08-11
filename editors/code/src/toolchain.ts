import * as os from "os";
import * as path from "path";
import * as vscode from "vscode";

import { log, memoizeAsync } from "./utilities";

interface CompilationArtifact {
	fileName: string;
	name: string;
	kind: string;
	isTest: boolean;
}

export interface ArtifactSpec {
	cargoArgs: string[];
	filter?: (artifacts: CompilationArtifact[]) => CompilationArtifact[];
}

// FIXME: The server should provide this
export function weslPath(env?: Record<string, string>): Promise<string> {
	if (env?.["WESLRS_TOOLCHAIN"]) {
		return Promise.resolve("wesl");
	}
	return getPathForExecutable("wesl");
}

/** Mirrors `toolchain::get_path_for_executable()` implementation */
const getPathForExecutable = memoizeAsync(
	// We apply caching to decrease file-system interactions
	async (executableName: "wesl"): Promise<string> => {
		{
			const envVar = process.env[executableName.toUpperCase()];
			if (envVar) return envVar;
		}

		if (await lookupInPath(executableName)) return executableName;

		const cargoHome = getCargoHome();
		if (cargoHome) {
			const standardPath = vscode.Uri.joinPath(
				cargoHome,
				"bin",
				executableName,
			);
			if (await isFileAtUri(standardPath)) return standardPath.fsPath;
		}
		return executableName;
	},
);

async function lookupInPath(exec: string): Promise<boolean> {
	const paths = process.env["PATH"] ?? "";

	const candidates = paths.split(path.delimiter).flatMap((directoryInPath) => {
		const candidate = path.join(directoryInPath, exec);
		return os.type() === "Windows_NT"
			? [candidate, `${candidate}.exe`]
			: [candidate];
	});

	for (const candidate of candidates) {
		if (await isFileAtPath(candidate)) {
			return true;
		}
	}
	return false;
}

function getCargoHome(): vscode.Uri | null {
	const envVar = process.env["CARGO_HOME"];
	if (envVar) return vscode.Uri.file(envVar);
	try {
		// hmm, `os.homedir()` seems to be infallible
		// it is not mentioned in docs and cannot be inferred by the type signature...
		return vscode.Uri.joinPath(vscode.Uri.file(os.homedir()), ".cargo");
	} catch (error) {
		log.error("Failed to read the fs info", error);
	}
	return null;
}

async function isFileAtPath(path: string): Promise<boolean> {
	return isFileAtUri(vscode.Uri.file(path));
}

async function isFileAtUri(uri: vscode.Uri): Promise<boolean> {
	try {
		return (
			((await vscode.workspace.fs.stat(uri)).type & vscode.FileType.File) !== 0
		);
	} catch {
		return false;
	}
}
