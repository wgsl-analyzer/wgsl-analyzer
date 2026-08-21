/**
 * Monaco + wgsl-analyzer.
 *
 * `wgsl-analyzer-web` boots the server in a Web Worker and exposes its message
 * stream; monaco's own LSP client (the `monaco.lsp` namespace, bundled into
 * `monaco-editor` since 0.55) drives it over the transport in `transport.ts`.
 */

import * as monaco from "monaco-editor";
import editorWorker from "monaco-editor/editor/editor.worker?worker";
import { WgslAnalyzerServer } from "wgsl-analyzer-web";

import { WgslAnalyzerTransport } from "./transport.js";
import { ENTRY, FILES, ROOT } from "./workspace.js";

// Monaco looks its workers up here. Without this it runs them on the main
// thread, which costs the editor its background tokenization and diffing.
globalThis.MonacoEnvironment = { getWorker: () => new editorWorker() };

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
			"Not cross-origin isolated. The dev server must send "
				+ "Cross-Origin-Opener-Policy: same-origin and "
				+ "Cross-Origin-Embedder-Policy: require-corp.",
		);
		return;
	}

	// This is here so the `textDocument/didOpen` carries `languageId: "wesl"`
	// rather than `"plaintext"`.
	monaco.languages.register({
		id: LANGUAGE_ID,
		extensions: [".wesl", ".wgsl"],
		aliases: ["WESL", "WGSL"],
	});

	status("Booting wgsl-analyzer…");
	const server = await WgslAnalyzerServer.start({
		// vite's publicDir is the package's dist/assets, served at the root.
		baseUrl: "/",
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

	new monaco.lsp.MonacoLspClient(new WgslAnalyzerTransport(server));

	status("Ready. Try completion (Ctrl+Space), hover, or go-to-definition (F12).");
}

main().catch((error: unknown) => {
	status(`Failed: ${String(error)}`);
	log(String(error instanceof Error ? (error.stack ?? error.message) : error));
});
