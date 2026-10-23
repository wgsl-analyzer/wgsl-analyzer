import { type ProcessOptions, Wasm } from "@vscode/wasm-wasi/v1";
import { createStdioOptions, createUriConverters, startServer } from "@vscode/wasm-wasi-lsp";
import * as vscode from "vscode";
import type * as lc from "vscode-languageclient/node";
import type { Bootstrap } from "../context";
import * as common from "../main_common";
import { setServedScheme } from "../utilities";

const serverStderr = vscode.window.createOutputChannel("wgsl-analyzer Server stderr");

const wasiBootstrap: Bootstrap = async (extCtx) => {
	const wasm: Wasm = await Wasm.load();

	const serverOptions: lc.ServerOptions = async () => {
		const options: ProcessOptions = {
			stdio: createStdioOptions(),
			mountPoints: [{ kind: "workspaceFolder" }],
		};
		const filename = vscode.Uri.joinPath(extCtx.extensionUri, "server", "wgsl-analyzer.wasm");
		const bits = (await vscode.workspace.fs.readFile(filename)) as Uint8Array<ArrayBuffer>;
		const module = await WebAssembly.compile(bits);
		// In 64 KiB pages: 512 MiB initial, 4 GiB maximum. Must match the link args in
		// `.cargo/config.toml`.
		const process = await wasm.createProcess(
			"wgsl-analyzer",
			module,
			{ initial: 8192, maximum: 65536, shared: true },
			options,
		);

		const decoder = new TextDecoder("utf-8");
		process.stderr?.onData((data) => {
			serverStderr.append(decoder.decode(data));
		});

		return startServer(process);
	};
	// TODO: find good defaults for version and path on web
	return [serverOptions, { path: "", version: "" }, createUriConverters()];
};

export async function deactivate() {
	await common.deactivate();
}

export async function activate(
	context: vscode.ExtensionContext,
): Promise<common.WgslAnalyzerExtensionApi> {
	// `vscode-vfs://github/<owner>/<repo>` on vscode.dev
	// `vscode-test-web://mount/` under @vscode/test-web
	setServedScheme(vscode.workspace.workspaceFolders?.[0]?.uri.scheme ?? "file");
	return await common.activate(context, wasiBootstrap);
}
