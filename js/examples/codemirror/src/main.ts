/**
 * CodeMirror + wgsl-analyzer.
 *
 * `wgsl-analyzer-web` boots the server in a Web Worker;
 * `@marimo-team/codemirror-languageserver` drives it through the `Transport`
 * the package's `codemirror` entry point supplies.
 */

import { LanguageServerClient, languageServerWithClient } from "@marimo-team/codemirror-languageserver";
import { EditorView, basicSetup } from "codemirror";
import { WgslAnalyzerServer } from "wgsl-analyzer-web";
import { WgslAnalyzerTransport } from "wgsl-analyzer-web/codemirror";

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

	const documentUri = server.uriOf(ENTRY);
	const client = new LanguageServerClient({
		rootUri: server.rootUri,
		workspaceFolders: [{ uri: server.rootUri, name: "workspace" }],
		transport: new WgslAnalyzerTransport(server),
	});

	new EditorView({
		doc: FILES[ENTRY] as string,
		parent: document.getElementById("editor") as HTMLElement,
		extensions: [
			basicSetup,
			languageServerWithClient({
				client,
				documentUri,
				languageId: LANGUAGE_ID,
				allowHTMLContent: false,
			}),
		],
	});

	status("Ready. Try completion (Ctrl+Space), hover, or go-to-definition.");
}

main().catch((error: unknown) => {
	status(`Failed: ${String(error)}`);
	log(String(error instanceof Error ? (error.stack ?? error.message) : error));
});
