import * as vscode from "vscode";
import type * as lc from "vscode-languageclient/node";
import * as wa from "./lsp_ext";

import { Config, prepareVSCodeConfig } from "./config";
import { createClient } from "./client";
import { isWgslDocument, isWgslEditor, LazyOutputChannel, log, type WgslEditor } from "./util";
import type { ServerStatusParameters } from "./lsp_ext";

import { SyntaxTreeProvider, type SyntaxElement } from "./syntax_tree_provider";
import { PersistentState } from "./persistent_state";
import { bootstrap } from "./bootstrap";
import { spawn } from "node:child_process";
import { text } from "node:stream/consumers";
import type { WgslAnalyzerExtensionApi } from "./main";

import { promisify } from "util";
import { readFile } from "fs";
import { DiagnosticsConfig, InlayHintsConfig, TraceConfig } from "./config";

// We only support local folders, not eg. Live Share (`vlsl:` scheme), so do not activate if
// only those are in use. We use "Empty" to represent these scenarios.
// (w-a still somewhat works with Live Share, because commands are tunneled to the host)

export type Workspace =
	| { kind: "Empty" }
	| {
			kind: "Workspace Folder";
	  }
	| {
			kind: "Detached Files";
			files: vscode.TextDocument[];
	  };

export function fetchWorkspace(): Workspace {
	const folders = (vscode.workspace.workspaceFolders || []).filter(
		(folder) => folder.uri.scheme === "file",
	);
	const wgslDocuments = vscode.workspace.textDocuments.filter((document) =>
		isWgslDocument(document),
	);

	return folders.length === 0
		? wgslDocuments.length === 0
			? { kind: "Empty" }
			: {
					kind: "Detached Files",
					files: wgslDocuments,
				}
		: { kind: "Workspace Folder" };
}

export type CommandFactory = {
	enabled: (ctx: CtxInit) => Cmd;
	disabled?: (ctx: Ctx) => Cmd;
};

export type CtxInit = Ctx & {
	readonly client: lc.LanguageClient;
};

interface WGSLAnalyzerConfiguration {
	customImports: Record<string, string>;
	shaderDefs: [string];
	trace: TraceConfig;
	diagnostics: DiagnosticsConfig;
	inlayHints: InlayHintsConfig;
}

async function lspOptions(config: Config): Promise<WGSLAnalyzerConfiguration> {
	const start = process.hrtime();
	const customImports = await mapObjectAsync(
		config.customImports!,
		resolveImport,
		(name, _, value) => {
			vscode.window.showErrorMessage(
				`WGSL-Analyzer: failed to resolve import \`${name}\`: ${value}`,
			);
		},
	);
	const elapsed = process.hrtime(start);
	const millis = elapsed[0] * 1000 + elapsed[1] / 1_000_000;
	if (millis > 1000) {
		vscode.window.showWarningMessage(
			`WGSL-Analyzer: Took ${millis.toFixed(0)}ms to resolve imports.`,
		);
	}

	return {
		customImports,
		shaderDefs: config.shaderDefs!,
		diagnostics: config.diagnostics!,
		trace: config.trace!,
		inlayHints: config.inlayHints!,
	};
}

async function resolveImport(content: string): Promise<string> {
	let content_replaced = content;
	const folders = vscode.workspace.workspaceFolders;
	if (vscode.workspace.workspaceFolders!.length == 1) {
		content_replaced = content_replaced.replace(
			"${workspaceFolder}",
			folders![0]!.uri.toString(),
		);
	}
	const uri = vscode.Uri.parse(content_replaced);

	if (uri !== undefined) {
		if (uri.scheme == "file") {
			return promisify(readFile)(uri.fsPath, "utf-8");
		} else if (["http", "https"].includes(uri.scheme)) {
			return fetch(content).then((result) => result.text());
		} else {
			throw new Error(`unknown scheme \`${uri.scheme}\``);
		}
	} else {
		return content;
	}
}

async function mapObjectAsync<T, U>(
	object: Record<string, T>,
	f: (value: T) => Promise<U>,
	handleError?: (key: string, value: T, error: unknown) => void,
): Promise<Record<string, U>> {
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	const map = async ([key, value]: [any, any]) => {
		try {
			const mapped = await f(value);
			return [key, mapped];
		} catch (e) {
			if (handleError) {
				handleError(key, value, e);
			}
			return undefined;
		}
	};
	const entries = await Promise.all(Object.entries(object).map(map));
	return Object.fromEntries(entries.filter((entry) => entry !== undefined));
}

export class Ctx implements WgslAnalyzerExtensionApi {
	readonly statusBar: vscode.StatusBarItem;
	readonly config: Config;
	readonly workspace: Workspace;
	readonly version: string;

	private _client: lc.LanguageClient | undefined;
	private _serverPath: string | undefined;
	private traceOutputChannel: vscode.OutputChannel | undefined;
	private testController: vscode.TestController | undefined;
	private outputChannel: vscode.OutputChannel | undefined;
	private clientSubscriptions: Disposable[];
	private state: PersistentState;
	private commandFactories: Record<string, CommandFactory>;
	private commandDisposables: Disposable[];
	private unlinkedFiles: vscode.Uri[];
	private _syntaxTreeProvider: SyntaxTreeProvider | undefined;
	private _syntaxTreeView: vscode.TreeView<SyntaxElement> | undefined;
	private lastStatus: ServerStatusParameters | { health: "stopped" } = { health: "stopped" };
	private _serverVersion: string;
	private statusBarActiveEditorListener: Disposable;

	get serverPath(): string | undefined {
		return this._serverPath;
	}

	get serverVersion(): string | undefined {
		return this._serverVersion;
	}

	get client() {
		return this._client;
	}

	get syntaxTreeView() {
		return this._syntaxTreeView;
	}

	get syntaxTreeProvider() {
		return this._syntaxTreeProvider;
	}

	constructor(
		readonly extCtx: vscode.ExtensionContext,
		commandFactories: Record<string, CommandFactory>,
		workspace: Workspace,
	) {
		extCtx.subscriptions.push(this);
		this.version = extCtx.extension.packageJSON.version ?? "<unknown>";
		this._serverVersion = "<not running>";
		this.config = new Config(extCtx.subscriptions);
		this.statusBar = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left);
		this.updateStatusBarVisibility(vscode.window.activeTextEditor);
		this.statusBarActiveEditorListener = vscode.window.onDidChangeActiveTextEditor((editor) =>
			this.updateStatusBarVisibility(editor),
		);
		this.workspace = workspace;
		this.clientSubscriptions = [];
		this.commandDisposables = [];
		this.commandFactories = commandFactories;
		this.unlinkedFiles = [];
		this.state = new PersistentState(extCtx.globalState);

		this.updateCommands("disable");
		this.setServerStatus({
			health: "stopped",
		});
	}

	dispose() {
		this.config.dispose();
		this.statusBar.dispose();
		this.statusBarActiveEditorListener.dispose();
		this.testController?.dispose();
		void this.disposeClient();
		this.commandDisposables.forEach((disposable) => disposable.dispose());
	}

	async onWorkspaceFolderChanges() {
		const workspace = fetchWorkspace();
		if (workspace.kind === "Detached Files" && this.workspace.kind === "Detached Files") {
			if (workspace.files !== this.workspace.files) {
				if (this.client?.isRunning()) {
					// Ideally we would not need to tear down the server here, but currently detached files
					// are only specified at server start
					await this.stopAndDispose();
					await this.start();
				}
				return;
			}
		}
		if (workspace.kind === "Workspace Folder" && this.workspace.kind === "Workspace Folder") {
			return;
		}
		if (workspace.kind === "Empty") {
			await this.stopAndDispose();
			return;
		}
		if (this.client?.isRunning()) {
			await this.restart();
		}
	}

	private async getOrCreateClient() {
		if (this.workspace.kind === "Empty") {
			return;
		}

		if (!this.traceOutputChannel) {
			this.traceOutputChannel = new LazyOutputChannel("WGSL Analyzer Language Server Trace");
			this.pushExtCleanup(this.traceOutputChannel);
		}
		if (!this.outputChannel) {
			this.outputChannel = vscode.window.createOutputChannel("WGSL Analyzer Language Server");
			this.pushExtCleanup(this.outputChannel);
		}

		if (!this._client) {
			this._serverPath = await this.bootstrap();
			text(spawn(this._serverPath, ["--version"]).stdout.setEncoding("utf-8")).then(
				(data) => {
					const prefix = `wgsl-analyzer `;
					this._serverVersion = data
						.slice(data.startsWith(prefix) ? prefix.length : 0)
						.trim();
					this.refreshServerStatus();
				},
				(_) => {
					this._serverVersion = "<unknown>";
					this.refreshServerStatus();
				},
			);
			const newEnv = Object.assign({}, process.env, this.config.serverExtraEnv);
			const run: lc.Executable = {
				command: this._serverPath,
				options: { env: newEnv },
			};
			const serverOptions = {
				run,
				debug: run,
			};

			let rawInitializationOptions = vscode.workspace.getConfiguration("wgsl-analyzer");

			if (this.workspace.kind === "Detached Files") {
				rawInitializationOptions = {
					detachedFiles: this.workspace.files.map((file) => file.uri.fsPath),
					...rawInitializationOptions,
				};
			}

			const initializationOptions = prepareVSCodeConfig(rawInitializationOptions);

			this._client = await createClient(
				this.traceOutputChannel,
				this.outputChannel,
				initializationOptions,
				serverOptions,
				this.config,
				this.unlinkedFiles,
			);
			this.pushClientCleanup(
				this._client.onNotification(wa.serverStatus, (parameters) =>
					this.setServerStatus(parameters),
				),
			);
			this.pushClientCleanup(
				this._client.onNotification(wa.openServerLogs, () => {
					this.outputChannel!.show();
				}),
			);
		}
		return this._client;
	}

	private async bootstrap(): Promise<string> {
		return bootstrap(this.extCtx, this.config, this.state).catch((error) => {
			let message = "bootstrap error. ";

			message +=
				'See the logs in "OUTPUT > WGSL Analyzer Client" (should open automatically). ';
			message +=
				'To enable verbose logs, click the gear icon in the "OUTPUT" tab and select "Debug".';

			log.error("Bootstrap error", error);
			throw new Error(message);
		});
	}

	async start() {
		log.info("Starting language client");
		const client = await this.getOrCreateClient();
		if (!client) {
			return;
		}
		await client.start();
		this.subscriptions.push(
			client.onRequest(wa.requestConfiguration, async (_, __) => {
				const options = await lspOptions(this.config);
				return options;
			}),
			client.onRequest(wa.importTextDocument, async (parameters, __) => {
				vscode.workspace.openTextDocument(parameters.uri);
				return;
			}),
		);
		this.updateCommands();
		if (this.config.showSyntaxTree) {
			this.prepareSyntaxTreeView(client);
		}
	}

	private prepareSyntaxTreeView(client: lc.LanguageClient) {
		const ctxInit: CtxInit = {
			...this,
			client: client,
		};
		this._syntaxTreeProvider = new SyntaxTreeProvider(ctxInit);
		this._syntaxTreeView = vscode.window.createTreeView("wgslSyntaxTree", {
			treeDataProvider: this._syntaxTreeProvider,
			showCollapseAll: true,
		});

		this.pushExtCleanup(this._syntaxTreeView);

		vscode.window.onDidChangeActiveTextEditor(async () => {
			if (this.syntaxTreeView?.visible) {
				await this.syntaxTreeProvider?.refresh();
			}
		});

		vscode.workspace.onDidChangeTextDocument(async (e) => {
			if (
				vscode.window.activeTextEditor?.document !== e.document ||
				e.contentChanges.length === 0
			) {
				return;
			}

			if (this.syntaxTreeView?.visible) {
				await this.syntaxTreeProvider?.refresh();
			}
		});

		vscode.window.onDidChangeTextEditorSelection(async (e) => {
			if (!this.syntaxTreeView?.visible || !isWgslEditor(e.textEditor)) {
				return;
			}

			const selection = e.selections[0];
			if (selection === undefined) {
				return;
			}

			const result = this.syntaxTreeProvider?.getElementByRange(selection);
			if (result !== undefined) {
				await this.syntaxTreeView?.reveal(result);
			}
		});

		this._syntaxTreeView.onDidChangeVisibility(async (e) => {
			if (e.visible) {
				await this.syntaxTreeProvider?.refresh();
			}
		});
	}

	async restart() {
		// FIXME: We should re-use the client, that is ctx.deactivate() if none of the configs have changed
		await this.stopAndDispose();
		await this.start();
	}

	async stop() {
		if (!this._client) {
			return;
		}
		log.info("Stopping language client");
		this.updateCommands("disable");
		await this._client.stop();
	}

	async stopAndDispose() {
		if (!this._client) {
			return;
		}
		log.info("Disposing language client");
		this.updateCommands("disable");
		// we give the server 100ms to stop gracefully
		await this.client?.stop(100).catch((_) => {});
		await this.disposeClient();
	}

	private async disposeClient() {
		this.clientSubscriptions?.forEach((disposable) => disposable.dispose());
		this.clientSubscriptions = [];
		await this._client?.dispose();
		this._serverPath = undefined;
		this._client = undefined;
	}

	get activeWgslEditor(): WgslEditor | undefined {
		const editor = vscode.window.activeTextEditor;
		return editor && isWgslEditor(editor) ? editor : undefined;
	}

	get extensionPath(): string {
		return this.extCtx.extensionPath;
	}

	get subscriptions(): Disposable[] {
		return this.extCtx.subscriptions;
	}

	private updateCommands(forceDisable?: "disable") {
		this.commandDisposables.forEach((disposable) => disposable.dispose());
		this.commandDisposables = [];

		const clientRunning = (!forceDisable && this._client?.isRunning()) ?? false;
		const isClientRunning = function (_ctx: Ctx): _ctx is CtxInit {
			return clientRunning;
		};

		for (const [name, factory] of Object.entries(this.commandFactories)) {
			const fullName = `wgsl-analyzer.${name}`;
			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			let callback: any;
			if (isClientRunning(this)) {
				// we asserted that `client` is defined
				callback = factory.enabled(this);
			} else if (factory.disabled) {
				callback = factory.disabled(this);
			} else {
				callback = () =>
					vscode.window.showErrorMessage(
						`command ${fullName} failed: wgsl-analyzer server is not running`,
					);
			}
			this.commandDisposables.push(vscode.commands.registerCommand(fullName, callback));
		}
	}

	setServerStatus(status: ServerStatusParameters | { health: "stopped" }) {
		this.lastStatus = status;
		this.updateStatusBarItem();
	}

	refreshServerStatus() {
		this.updateStatusBarItem();
	}

	private updateStatusBarItem() {
		let icon = "";
		const status = this.lastStatus;
		const statusBar = this.statusBar;
		statusBar.tooltip = new vscode.MarkdownString("", true);
		statusBar.tooltip.isTrusted = true;
		switch (status.health) {
			case "ok":
				statusBar.color = undefined;
				statusBar.backgroundColor = undefined;
				if (this.config.statusBarClickAction === "stopServer") {
					statusBar.command = "wgsl-analyzer.stopServer";
				} else {
					statusBar.command = "wgsl-analyzer.openLogs";
				}
				void this.syntaxTreeProvider?.refresh();
				break;
			case "warning":
				statusBar.color = new vscode.ThemeColor("statusBarItem.warningForeground");
				statusBar.backgroundColor = new vscode.ThemeColor(
					"statusBarItem.warningBackground",
				);
				statusBar.command = "wgsl-analyzer.openLogs";
				icon = "$(warning) ";
				break;
			case "error":
				statusBar.color = new vscode.ThemeColor("statusBarItem.errorForeground");
				statusBar.backgroundColor = new vscode.ThemeColor("statusBarItem.errorBackground");
				statusBar.command = "wgsl-analyzer.openLogs";
				icon = "$(error) ";
				break;
			case "stopped":
				statusBar.tooltip.appendText("Server is stopped");
				statusBar.tooltip.appendMarkdown(
					"\n\n[Start server](command:wgsl-analyzer.startServer)",
				);
				statusBar.color = new vscode.ThemeColor("statusBarItem.warningForeground");
				statusBar.backgroundColor = new vscode.ThemeColor(
					"statusBarItem.warningBackground",
				);
				statusBar.command = "wgsl-analyzer.startServer";
				statusBar.text = "$(stop-circle) wgsl-analyzer";
				return;
		}
		if (status.message) {
			statusBar.tooltip.appendMarkdown(status.message);
		}
		if (statusBar.tooltip.value) {
			statusBar.tooltip.appendMarkdown("\n\n---\n\n");
		}

		const toggleCheckOnSave = this.config.checkOnSave ? "Disable" : "Enable";
		statusBar.tooltip.appendMarkdown(
			`[Extension Info](command:wgsl-analyzer.serverVersion "Show version and server binary info"): Version ${this.version}, Server Version ${this._serverVersion}` +
				"\n\n---\n\n" +
				'[$(terminal) Open Logs](command:wgsl-analyzer.openLogs "Open the server logs")' +
				"\n\n" +
				`[$(settings) ${toggleCheckOnSave} Check on Save](command:wgsl-analyzer.toggleCheckOnSave "Temporarily ${toggleCheckOnSave.toLowerCase()} check on save functionality")` +
				"\n\n" +
				'[$(stop-circle) Stop server](command:wgsl-analyzer.stopServer "Stop the server")' +
				"\n\n" +
				'[$(debug-restart) Restart server](command:wgsl-analyzer.restartServer "Restart the server")',
		);
		if (!status.quiescent) icon = "$(loading~spin) ";
		statusBar.text = `${icon}wgsl-analyzer`;
	}

	private updateStatusBarVisibility(editor: vscode.TextEditor | undefined) {
		const showStatusBar = this.config.statusBarShowStatusBar;
		if (showStatusBar == null || showStatusBar === "never") {
			this.statusBar.hide();
		} else if (showStatusBar === "always") {
			this.statusBar.show();
		} else {
			const documentSelector = showStatusBar.documentSelector;
			if (editor != null && vscode.languages.match(documentSelector, editor.document) > 0) {
				this.statusBar.show();
			} else {
				this.statusBar.hide();
			}
		}
	}

	pushExtCleanup(d: Disposable) {
		this.extCtx.subscriptions.push(d);
	}

	pushClientCleanup(d: Disposable) {
		this.clientSubscriptions.push(d);
	}
}

export interface Disposable {
	dispose(): void;
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type Cmd = (...args: any[]) => unknown;
