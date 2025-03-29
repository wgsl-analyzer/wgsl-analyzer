import * as vscode from "vscode";
import * as os from "os";
import type { Config } from "./config";
import { type Env, log, spawnAsync } from "./util";
import type { PersistentState } from "./persistent_state";
import { exec } from "child_process";

export async function bootstrap(
	context: vscode.ExtensionContext,
	config: Config,
	state: PersistentState,
): Promise<string> {
	const path = await getServer(context, config, state);
	if (!path) {
		throw new Error(
			"wgsl-analyzer Language Server is not available. " +
				"Please ensure it is [correctly installed](https://wgsl-analyzer.github.io/manual.html#installation).",
		);
	}

	log.info("Using server binary at", path);

	if (!isValidExecutable(path, config.serverExtraEnv)) {
		throw new Error(
			`Failed to execute ${path} --version.` +
				(config.serverPath
					? `\`config.server.path\` or \`config.serverPath\` has been set explicitly.\
			Consider removing this config or making a valid server binary available at that path.`
					: ""),
		);
	}

	return path;
}

async function getServer(
	context: vscode.ExtensionContext,
	config: Config,
	state: PersistentState,
): Promise<string | undefined> {
	const packageJson: {
		version: string;
		releaseTag: string | null;
		enableProposedApi: boolean | undefined;
	} = context.extension.packageJSON;

	// check if the server path is configured explicitly
	const explicitPath = process.env["__WA_LSP_SERVER_DEBUG"] ?? config.serverPath;
	if (explicitPath) {
		if (explicitPath.startsWith("~/")) {
			return os.homedir() + explicitPath.slice("~".length);
		}
		return explicitPath;
	}

	if (packageJson.releaseTag === null) {
		return "wgsl-analyzer";
	}

	// finally, use the bundled one
	const ext = process.platform === "win32" ? ".exe" : "";
	const bundled = vscode.Uri.joinPath(context.extensionUri, "server", `wgsl-analyzer${ext}`);
	const bundledExists = await fileExists(bundled);
	if (bundledExists) {
		let server = bundled;
		if (await isNixOs()) {
			server = await getNixOsServer(
				context.globalStorageUri,
				packageJson.version,
				ext,
				state,
				bundled,
				server,
			);
			await state.updateServerVersion(packageJson.version);
		}
		return server.fsPath;
	}

	await vscode.window.showErrorMessage(
		"Unfortunately we do not ship binaries for your platform yet. " +
			"You need to manually clone the wgsl-analyzer repository and " +
			"run `cargo xtask install --server` to build the language server from sources. " +
			"If you feel that your platform should be supported, please create an issue " +
			"about that [here](https://github.com/wgsl-analyzer/wgsl-analyzer/issues) and we " +
			"will consider it.",
	);
	return undefined;
}

async function fileExists(uri: vscode.Uri) {
	return await vscode.workspace.fs.stat(uri).then(
		() => true,
		() => false,
	);
}

export async function isValidExecutable(path: string, extraEnv: Env): Promise<boolean> {
	log.debug("Checking availability of a binary at", path);

	const result = await spawnAsync(path, ["--version"], {
		env: { ...process.env, ...extraEnv },
	});

	if (result.error) {
		log.warn(path, "--version:", result);
	} else {
		log.info(path, "--version:", result);
	}
	return result.status === 0;
}

async function getNixOsServer(
	globalStorageUri: vscode.Uri,
	version: string,
	ext: string,
	state: PersistentState,
	bundled: vscode.Uri,
	server: vscode.Uri,
) {
	await vscode.workspace.fs.createDirectory(globalStorageUri).then();
	const destination = vscode.Uri.joinPath(globalStorageUri, `wgsl-analyzer${ext}`);
	let exists = await vscode.workspace.fs.stat(destination).then(
		() => true,
		() => false,
	);
	if (exists && version !== state.serverVersion) {
		await vscode.workspace.fs.delete(destination);
		exists = false;
	}
	if (!exists) {
		await vscode.workspace.fs.copy(bundled, destination);
		await patchelf(destination);
	}
	server = destination;
	return server;
}

async function isNixOs(): Promise<boolean> {
	try {
		const contents = (
			await vscode.workspace.fs.readFile(vscode.Uri.file("/etc/os-release"))
		).toString();
		const idString = contents.split("\n").find((a) => a.startsWith("ID=")) || "ID=linux";
		return idString.indexOf("nixos") !== -1;
	} catch {
		return false;
	}
}

async function patchelf(destination: vscode.Uri): Promise<void> {
	await vscode.window.withProgress(
		{
			location: vscode.ProgressLocation.Notification,
			title: "Patching wgsl-analyzer for NixOS",
		},
		async (progress, _) => {
			const expression = `
            {srcStr, pkgs ? import <nixpkgs> {}}:
                pkgs.stdenv.mkDerivation {
                    name = "wgsl-analyzer";
                    src = /. + srcStr;
                    phases = [ "installPhase" "fixupPhase" ];
                    installPhase = "cp $src $out";
                    fixupPhase = ''
                    chmod 755 $out
                    patchelf --set-interpreter "$(cat $NIX_CC/nix-support/dynamic-linker)" $out
                    '';
                }
            `;
			const originalFile = vscode.Uri.file(destination.fsPath + "-orig");
			await vscode.workspace.fs.rename(destination, originalFile, { overwrite: true });
			try {
				progress.report({ message: "Patching executable", increment: 20 });
				await new Promise((resolve, reject) => {
					const handle = exec(
						`nix-build -E - --argstr srcStr '${originalFile.fsPath}' -o '${destination.fsPath}'`,
						(error, stdout, stderr) => {
							if (error != null) {
								reject(Error(stderr));
							} else {
								resolve(stdout);
							}
						},
					);
					handle.stdin?.write(expression);
					handle.stdin?.end();
				});
			} finally {
				await vscode.workspace.fs.delete(originalFile);
			}
		},
	);
}
