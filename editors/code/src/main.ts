import * as assert from "node:assert";
import * as vscode from "vscode";
import * as lc from "vscode-languageclient/node";
import * as commands from "./commands";
import { type CommandFactory, Ctx, fetchWorkspace } from "./ctx";
import * as diagnostics from "./diagnostics";
import { setContextValue } from "./utilities";

const WESL_PROJECT_CONTEXT_NAME = "inWeslProject";

export interface WgslAnalyzerExtensionApi {
	// FIXME: this should be non-optional
	readonly client?: lc.LanguageClient;
}

export async function deactivate() {
	await setContextValue(WESL_PROJECT_CONTEXT_NAME, undefined);
}

export async function activate(
	context: vscode.ExtensionContext,
): Promise<WgslAnalyzerExtensionApi> {
	checkConflictingExtensions();

	const ctx = new Ctx(context, createCommands(), fetchWorkspace());

	// VS Code does not show a notification when an extension fails to activate
	// so we do it ourselves.
	const api = await activateServer(ctx).catch((error: unknown) => {
		assert.ok(error instanceof Error);
		void vscode.window.showErrorMessage(
			`Cannot activate wgsl-analyzer extension: ${error.message}`,
		);
		throw error;
	});
	await setContextValue(WESL_PROJECT_CONTEXT_NAME, true);
	return api;
}

async function activateServer(ctx: Ctx): Promise<WgslAnalyzerExtensionApi> {
	const diagnosticProvider = new diagnostics.TextDocumentProvider(ctx);
	ctx.pushExtCleanup(
		vscode.workspace.registerTextDocumentContentProvider(
			diagnostics.URI_SCHEME,
			diagnosticProvider,
		),
	);

	const decorationProvider = new diagnostics.AnsiDecorationProvider(ctx);
	ctx.pushExtCleanup(decorationProvider);

	async function decorateVisibleEditors(document: vscode.TextDocument) {
		for (const editor of vscode.window.visibleTextEditors) {
			if (document === editor.document) {
				await decorationProvider.provideDecorations(editor);
			}
		}
	}

	vscode.workspace.onDidChangeTextDocument(
		async (event) => {
			await decorateVisibleEditors(event.document);
		},
		null,
		ctx.subscriptions,
	);
	vscode.workspace.onDidOpenTextDocument(decorateVisibleEditors, null, ctx.subscriptions);
	vscode.window.onDidChangeActiveTextEditor(
		async (editor) => {
			if (editor) {
				diagnosticProvider.triggerUpdate(editor.document.uri);
				await decorateVisibleEditors(editor.document);
			}
		},
		null,
		ctx.subscriptions,
	);
	vscode.window.onDidChangeVisibleTextEditors(
		async (visibleEditors) => {
			for (const editor of visibleEditors) {
				diagnosticProvider.triggerUpdate(editor.document.uri);
				await decorationProvider.provideDecorations(editor);
			}
		},
		null,
		ctx.subscriptions,
	);

	vscode.workspace.onDidChangeWorkspaceFolders(
		async (_) => ctx.onWorkspaceFolderChanges(),
		null,
		ctx.subscriptions,
	);
	vscode.workspace.onDidChangeConfiguration(
		async (_) => {
			await ctx.client?.sendNotification(lc.DidChangeConfigurationNotification.type, {
				settings: "",
			});
		},
		null,
		ctx.subscriptions,
	);

	if (ctx.config.initializeStopped) {
		ctx.setServerStatus({
			health: "stopped",
		});
	} else {
		await ctx.start();
	}

	return ctx;
}

function createCommands(): Record<string, CommandFactory> {
	return {
		onEnter: {
			enabled: commands.onEnter,
			disabled: (_) => () => vscode.commands.executeCommand("default:type", { text: "\n" }),
		},
		restartServer: {
			enabled: (ctx) => async () => {
				await ctx.restart();
			},
			disabled: (ctx) => async () => {
				await ctx.start();
			},
		},
		startServer: {
			enabled: (ctx) => async () => {
				await ctx.start();
			},
			disabled: (ctx) => async () => {
				await ctx.start();
			},
		},
		stopServer: {
			enabled: (ctx) => async () => {
				// FIXME: We should re-use the client, that is ctx.deactivate() if none of the configs have changed
				await ctx.stopAndDispose();
				ctx.setServerStatus({
					health: "stopped",
				});
			},
			disabled: (_) => async () => {
				// idempotent
			},
		},

		analyzerStatus: { enabled: commands.analyzerStatus },

		memoryUsage: { enabled: commands.memoryUsage },
		reloadWorkspace: { enabled: commands.reloadWorkspace },
		matchingBrace: { enabled: commands.matchingBrace },
		joinLines: { enabled: commands.joinLines },
		viewFileText: { enabled: commands.viewFileText },
		viewItemTree: { enabled: commands.viewItemTree },
		viewDependencyGraph: { enabled: commands.viewCrateGraph },
		viewFullDependencyGraph: { enabled: commands.viewFullDependencyGraph },
		openDocs: { enabled: commands.openDocs },
		openExternalDocs: { enabled: commands.openExternalDocs },
		moveItemUp: { enabled: commands.moveItemUp },
		moveItemDown: { enabled: commands.moveItemDown },
		cancelFlycheck: { enabled: commands.cancelFlycheck },
		clearFlycheck: { enabled: commands.clearFlycheck },
		runFlycheck: { enabled: commands.runFlycheck },
		ssr: { enabled: commands.ssr },
		serverVersion: { enabled: commands.serverVersion },
		viewMemoryLayout: { enabled: commands.viewMemoryLayout },
		// toggleCheckOnSave: { enabled: commands.toggleCheckOnSave },
		// toggleLSPLogs: { enabled: commands.toggleLSPLogs },
		// openWalkthrough: { enabled: commands.openWalkthrough },
		// Internal commands which are invoked by the server.
		applyActionGroup: { enabled: commands.applyActionGroup },
		applySnippetWorkspaceEdit: {
			enabled: commands.applySnippetWorkspaceEditCommand,
		},
		gotoLocation: { enabled: commands.gotoLocation },
		hoverRefCommandProxy: { enabled: commands.hoverRefCommandProxy },
		resolveCodeAction: { enabled: commands.resolveCodeAction },
		showReferences: { enabled: commands.showReferences },
		triggerParameterHints: { enabled: commands.triggerParameterHints },
		rename: { enabled: commands.rename },
		openLogs: { enabled: commands.openLogs },
		syntaxTreeReveal: { enabled: commands.syntaxTreeReveal },
		syntaxTreeCopy: { enabled: commands.syntaxTreeCopy },
		syntaxTreeHideWhitespace: { enabled: commands.syntaxTreeHideWhitespace },
		syntaxTreeShowWhitespace: { enabled: commands.syntaxTreeShowWhitespace },
	};
}

function checkConflictingExtensions() {
	if (vscode.extensions.getExtension("polymeilex.wgsl")) {
		vscode.window
			.showWarningMessage(
				"You have both the wgsl-analyzer (wgsl-analyzer.wgsl-analyzer) and WGSL (polymeilex.wgsl) "
					+ "plugins enabled. These are known to conflict and cause various functions of "
					+ "both plugins to not work correctly. You should disable one of them.",
				"Got it",
			)
			.then(
				() => {
					// no action needed
				},
				// biome-ignore lint/suspicious/noConsole: nothing else we can do here
				console.error,
			);
	}
}
