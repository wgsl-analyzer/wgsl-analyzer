/**
 * Monaco + wgsl-analyzer.
 *
 * `wgsl-analyzer-web` boots the server in a Web Worker and exposes its message
 * stream; `monaco-languageclient` drives it as an ordinary language client over
 * the vscode-jsonrpc transports the package supplies.
 */

import { LogLevel } from "@codingame/monaco-vscode-api";
// Aliased to @codingame/monaco-vscode-editor-api in package.json, which is what
// monaco-languageclient requires in place of stock monaco-editor.
import * as monaco from "monaco-editor";
import { MonacoLanguageClient } from "monaco-languageclient";
import { MonacoVscodeApiWrapper } from "monaco-languageclient/vscodeApiWrapper";
import { configureDefaultWorkerFactory } from "monaco-languageclient/workerFactory";
import { WgslAnalyzerServer } from "wgsl-analyzer-web";
import { createMessageTransports } from "wgsl-analyzer-web/jsonrpc";

import { ENTRY, FILES, ROOT } from "./workspace.js";

const LANGUAGE_ID = "wesl";

const statusElement = document.getElementById("status") as HTMLElement;
const logElement = document.getElementById("log") as HTMLElement;

function status(text: string): void {
	statusElement.textContent = text;
}

function log(line: string): void {
	logElement.textContent += `${line}\n`;
	logElement.scrollTop = logElement.scrollHeight;
}

async function main(): Promise<void> {
	if (!globalThis.crossOriginIsolated) {
		status(
			"Not cross-origin isolated. The dev server must send " +
				"Cross-Origin-Opener-Policy: same-origin and " +
				"Cross-Origin-Embedder-Policy: require-corp.",
		);
		return;
	}

	status("Starting the vscode API…");
	const apiWrapper = new MonacoVscodeApiWrapper({
		$type: "classic",
		viewsConfig: { $type: "EditorService" },
		logLevel: LogLevel.Warning,
		// The wrapper's fallback registers a worker factory with no loaders, so
		// every lookup misses and monaco runs its workers on the main thread.
		// This registers the real defaults.
		monacoWorkerFactory: configureDefaultWorkerFactory,
	});
	await apiWrapper.start();

	// Register the language directly rather than through an extension manifest:
	// in "classic" mode the extension host that would process `contributes` is
	// not loaded, so a manifest would leave the model as plaintext and the
	// client's document selector would never match.
	monaco.languages.register({
		id: LANGUAGE_ID,
		extensions: [".wesl", ".wgsl"],
		aliases: ["WESL", "WGSL"],
	});

	status("Booting wgsl-analyzer…");
	const server = await WgslAnalyzerServer.start({
		baseUrl: "/wgsl-analyzer/",
		root: ROOT,
		files: FILES,
		onStderr: (line) => log(line),
		onExit: (code) => {
			status(`Server exited with code ${code}.`);
			log(`server exited with code ${code}`);
		},
	});

	const documentUri = monaco.Uri.parse(server.uriOf(ENTRY));
	const model = monaco.editor.createModel(FILES[ENTRY] as string, LANGUAGE_ID, documentUri);
	monaco.editor.create(document.getElementById("editor") as HTMLElement, {
		model,
		automaticLayout: true,
		theme: "vs-dark",
		minimap: { enabled: false },
	});

	const client = new MonacoLanguageClient({
		name: "wgsl-analyzer",
		clientOptions: {
			documentSelector: [{ language: LANGUAGE_ID }],
			workspaceFolder: {
				index: 0,
				name: "workspace",
				uri: monaco.Uri.parse(server.rootUri),
			},
			// The server's own diagnostics are the point of the example; do not let
			// the client silently swallow a crash.
			errorHandler: {
				error: () => ({ action: 1 }),
				closed: () => ({ action: 1 }),
			},
		},
		messageTransports: createMessageTransports(server),
	});

	await client.start();
	status("Ready. Try completion (Ctrl+Space), hover, or go-to-definition (F12).");
}

main().catch((error: unknown) => {
	status(`Failed: ${String(error)}`);
	log(String(error instanceof Error ? (error.stack ?? error.message) : error));
});
