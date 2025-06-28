import * as vscode from "vscode";
import * as lc from "vscode-languageclient";
import * as wa from "./lsp_ext";
import * as path from "path";

import type { Ctx, Cmd, CtxInit } from "./ctx";
import {
	applySnippetWorkspaceEdit,
	applySnippetTextEdits,
	type SnippetTextDocumentEdit,
} from "./snippets";

import { isWeslDocument, sleep, isWeslEditor, unwrapUndefinable } from "./util";
import type { LanguageClient } from "vscode-languageclient/node";
import { HOVER_REFERENCE_COMMAND } from "./client";
import { log } from "./util";
import type { SyntaxElement } from "./syntax_tree_provider";

export function analyzerStatus(ctx: CtxInit): Cmd {
	const tdcp = new (class implements vscode.TextDocumentContentProvider {
		readonly uri = vscode.Uri.parse("wgsl-analyzer-status://status");
		readonly eventEmitter = new vscode.EventEmitter<vscode.Uri>();

		async provideTextDocumentContent(_uri: vscode.Uri): Promise<string> {
			if (!vscode.window.activeTextEditor) return "";
			const client = ctx.client;

			const parameters: wa.AnalyzerStatusParameters = {};
			const doc = ctx.activeWeslEditor?.document;
			if (doc != null) {
				parameters.textDocument =
					client.code2ProtocolConverter.asTextDocumentIdentifier(doc);
			}
			return await client.sendRequest(wa.analyzerStatus, parameters);
		}

		get onDidChange(): vscode.Event<vscode.Uri> {
			return this.eventEmitter.event;
		}
	})();

	ctx.pushExtCleanup(
		vscode.workspace.registerTextDocumentContentProvider("wgsl-analyzer-status", tdcp),
	);

	return async () => {
		const document = await vscode.workspace.openTextDocument(tdcp.uri);
		tdcp.eventEmitter.fire(tdcp.uri);
		void (await vscode.window.showTextDocument(document, {
			viewColumn: vscode.ViewColumn.Two,
			preserveFocus: true,
		}));
	};
}

export function memoryUsage(ctx: CtxInit): Cmd {
	const tdcp = new (class implements vscode.TextDocumentContentProvider {
		readonly uri = vscode.Uri.parse("wgsl-analyzer-memory://memory");
		readonly eventEmitter = new vscode.EventEmitter<vscode.Uri>();

		provideTextDocumentContent(_uri: vscode.Uri): vscode.ProviderResult<string> {
			if (!vscode.window.activeTextEditor) return "";

			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			return ctx.client.sendRequest(wa.memoryUsage).then((mem: any) => {
				return "Per-query memory usage:\n" + mem + "\n(note: database has been cleared)";
			});
		}

		get onDidChange(): vscode.Event<vscode.Uri> {
			return this.eventEmitter.event;
		}
	})();

	ctx.pushExtCleanup(
		vscode.workspace.registerTextDocumentContentProvider("wgsl-analyzer-memory", tdcp),
	);

	return async () => {
		tdcp.eventEmitter.fire(tdcp.uri);
		const document = await vscode.workspace.openTextDocument(tdcp.uri);
		return vscode.window.showTextDocument(document, vscode.ViewColumn.Two, true);
	};
}

export function triggerParameterHints(_: CtxInit): Cmd {
	return async () => {
		const parameterHintsEnabled = vscode.workspace
			.getConfiguration("editor")
			.get<boolean>("parameterHints.enabled");

		if (parameterHintsEnabled) {
			await vscode.commands.executeCommand("editor.action.triggerParameterHints");
		}
	};
}

export function rename(_: CtxInit): Cmd {
	return async () => {
		await vscode.commands.executeCommand("editor.action.rename");
	};
}

export function openLogs(ctx: CtxInit): Cmd {
	return async () => {
		if (ctx.client.outputChannel) {
			ctx.client.outputChannel.show();
		}
	};
}

export function matchingBrace(ctx: CtxInit): Cmd {
	return async () => {
		const editor = ctx.activeWeslEditor;
		if (!editor) return;

		const client = ctx.client;

		const response = await client.sendRequest(wa.matchingBrace, {
			textDocument: client.code2ProtocolConverter.asTextDocumentIdentifier(editor.document),
			positions: editor.selections.map((s) =>
				client.code2ProtocolConverter.asPosition(s.active),
			),
		});
		editor.selections = editor.selections.map((selection, index) => {
			const position = unwrapUndefinable(response[index]);
			const active = client.protocol2CodeConverter.asPosition(position);
			const anchor = selection.isEmpty ? active : selection.anchor;
			return new vscode.Selection(anchor, active);
		});
		editor.revealRange(editor.selection);
	};
}

export function joinLines(ctx: CtxInit): Cmd {
	return async () => {
		const editor = ctx.activeWeslEditor;
		if (!editor) return;

		const client = ctx.client;

		const items: lc.TextEdit[] = await client.sendRequest(wa.joinLines, {
			ranges: editor.selections.map((it) => client.code2ProtocolConverter.asRange(it)),
			textDocument: client.code2ProtocolConverter.asTextDocumentIdentifier(editor.document),
		});
		const textEdits = await client.protocol2CodeConverter.asTextEdits(items);
		await editor.edit((builder) => {
			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			textEdits.forEach((edit: any) => {
				builder.replace(edit.range, edit.newText);
			});
		});
	};
}

export function moveItemUp(ctx: CtxInit): Cmd {
	return moveItem(ctx, "Up");
}

export function moveItemDown(ctx: CtxInit): Cmd {
	return moveItem(ctx, "Down");
}

export function moveItem(ctx: CtxInit, direction: wa.Direction): Cmd {
	return async () => {
		const editor = ctx.activeWeslEditor;
		if (!editor) return;
		const client = ctx.client;

		const lcEdits = await client.sendRequest(wa.moveItem, {
			range: client.code2ProtocolConverter.asRange(editor.selection),
			textDocument: client.code2ProtocolConverter.asTextDocumentIdentifier(editor.document),
			direction,
		});

		if (!lcEdits) return;

		const edits = await client.protocol2CodeConverter.asTextEdits(lcEdits);
		await applySnippetTextEdits(editor, edits);
	};
}

export function onEnter(ctx: CtxInit): Cmd {
	async function handleKeypress() {
		const editor = ctx.activeWeslEditor;

		if (!editor) return false;

		const client = ctx.client;
		const lcEdits = await client
			.sendRequest(wa.onEnter, {
				textDocument: client.code2ProtocolConverter.asTextDocumentIdentifier(
					editor.document,
				),
				position: client.code2ProtocolConverter.asPosition(editor.selection.active),
			})
			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			.catch((_error: any) => {
				// client.handleFailedRequest(OnEnterRequest.type, error, null);
				return null;
			});
		if (!lcEdits) return false;

		const edits = await client.protocol2CodeConverter.asTextEdits(lcEdits);
		await applySnippetTextEdits(editor, edits);
		return true;
	}

	return async () => {
		if (await handleKeypress()) return;

		await vscode.commands.executeCommand("default:type", { text: "\n" });
	};
}

export function syntaxTreeReveal(): Cmd {
	return async (element: SyntaxElement) => {
		const activeEditor = vscode.window.activeTextEditor;

		if (activeEditor !== undefined) {
			const newSelection = new vscode.Selection(element.range.start, element.range.end);

			activeEditor.selection = newSelection;
			activeEditor.revealRange(newSelection);
		}
	};
}

function elementToString(
	activeDocument: vscode.TextDocument,
	element: SyntaxElement,
	depth: number = 0,
): string {
	let result = "  ".repeat(depth);
	const offsets = element.inner?.offsets ?? element.offsets;

	result += `${element.kind}@${offsets.start}..${offsets.end}`;

	if (element.type === "Token") {
		const text = activeDocument.getText(element.range).replace("\r\n", "\n");
		// JSON.stringify quotes and escapes the string for us.
		result += ` ${JSON.stringify(text)}\n`;
	} else {
		result += "\n";
		for (const child of element.children) {
			result += elementToString(activeDocument, child, depth + 1);
		}
	}
	return result;
}

export function syntaxTreeCopy(): Cmd {
	return async (element: SyntaxElement) => {
		const activeDocument = vscode.window.activeTextEditor?.document;
		if (!activeDocument) {
			return;
		}

		const result = elementToString(activeDocument, element);
		await vscode.env.clipboard.writeText(result);
	};
}

export function syntaxTreeHideWhitespace(ctx: CtxInit): Cmd {
	return async () => {
		if (ctx.syntaxTreeProvider !== undefined) {
			await ctx.syntaxTreeProvider.toggleWhitespace();
		}
	};
}

export function syntaxTreeShowWhitespace(ctx: CtxInit): Cmd {
	return async () => {
		if (ctx.syntaxTreeProvider !== undefined) {
			await ctx.syntaxTreeProvider.toggleWhitespace();
		}
	};
}

export function ssr(ctx: CtxInit): Cmd {
	return async () => {
		const editor = vscode.window.activeTextEditor;
		if (!editor) return;

		const client = ctx.client;

		const position = editor.selection.active;
		const selections = editor.selections;
		const textDocument = client.code2ProtocolConverter.asTextDocumentIdentifier(
			editor.document,
		);

		const options: vscode.InputBoxOptions = {
			value: "() ==>> ()",
			prompt: "Enter request, for example 'Foo($a) ==>> Foo::new($a)' ",
			validateInput: async (x: string) => {
				try {
					await client.sendRequest(wa.ssr, {
						query: x,
						parseOnly: true,
						textDocument,
						position,
						selections,
					});
				} catch (e) {
					return String(e);
				}
				return null;
			},
		};
		const request = await vscode.window.showInputBox(options);
		if (!request) return;

		await vscode.window.withProgress(
			{
				location: vscode.ProgressLocation.Notification,
				title: "Structured search replace in progress...",
				cancellable: false,
			},
			async (_progress, token) => {
				const edit = await client.sendRequest(wa.ssr, {
					query: request,
					parseOnly: false,
					textDocument,
					position,
					selections,
				});

				await vscode.workspace.applyEdit(
					await client.protocol2CodeConverter.asWorkspaceEdit(edit, token),
				);
			},
		);
	};
}

export function serverVersion(ctx: CtxInit): Cmd {
	return async () => {
		if (!ctx.serverPath) {
			void vscode.window.showWarningMessage(`wgsl-analyzer server is not running`);
			return;
		}
		void vscode.window.showInformationMessage(
			`wgsl-analyzer version: ${ctx.serverVersion} [${ctx.serverPath}]`,
		);
	};
}

export function viewFileText(ctx: CtxInit): Cmd {
	const tdcp = new (class implements vscode.TextDocumentContentProvider {
		readonly uri = vscode.Uri.parse("wgsl-analyzer-file-text://viewFileText/file.rs");
		readonly eventEmitter = new vscode.EventEmitter<vscode.Uri>();
		constructor() {
			vscode.workspace.onDidChangeTextDocument(
				this.onDidChangeTextDocument,
				this,
				ctx.subscriptions,
			);
			vscode.window.onDidChangeActiveTextEditor(
				this.onDidChangeActiveTextEditor,
				this,
				ctx.subscriptions,
			);
		}

		private onDidChangeTextDocument(event: vscode.TextDocumentChangeEvent) {
			if (isWeslDocument(event.document)) {
				// We need to order this after language server updates, but there is no API for that.
				// Hence, good old sleep().
				void sleep(10).then(() => this.eventEmitter.fire(this.uri));
			}
		}

		private onDidChangeActiveTextEditor(editor: vscode.TextEditor | undefined) {
			if (editor && isWeslEditor(editor)) {
				this.eventEmitter.fire(this.uri);
			}
		}

		async provideTextDocumentContent(
			_uri: vscode.Uri,
			ct: vscode.CancellationToken,
		): Promise<string> {
			const weslEditor = ctx.activeWeslEditor;
			if (!weslEditor) return "";
			const client = ctx.client;

			const parameters = client.code2ProtocolConverter.asTextDocumentIdentifier(
				weslEditor.document,
			);
			return client.sendRequest(wa.viewFileText, parameters, ct);
		}

		get onDidChange(): vscode.Event<vscode.Uri> {
			return this.eventEmitter.event;
		}
	})();

	ctx.pushExtCleanup(
		vscode.workspace.registerTextDocumentContentProvider("wgsl-analyzer-file-text", tdcp),
	);

	return async () => {
		const document = await vscode.workspace.openTextDocument(tdcp.uri);
		tdcp.eventEmitter.fire(tdcp.uri);
		void (await vscode.window.showTextDocument(document, {
			viewColumn: vscode.ViewColumn.Two,
			preserveFocus: true,
		}));
	};
}

export function viewItemTree(ctx: CtxInit): Cmd {
	const tdcp = new (class implements vscode.TextDocumentContentProvider {
		readonly uri = vscode.Uri.parse("wgsl-analyzer-item-tree://viewItemTree/itemtree.rs");
		readonly eventEmitter = new vscode.EventEmitter<vscode.Uri>();
		constructor() {
			vscode.workspace.onDidChangeTextDocument(
				this.onDidChangeTextDocument,
				this,
				ctx.subscriptions,
			);
			vscode.window.onDidChangeActiveTextEditor(
				this.onDidChangeActiveTextEditor,
				this,
				ctx.subscriptions,
			);
		}

		private onDidChangeTextDocument(event: vscode.TextDocumentChangeEvent) {
			if (isWeslDocument(event.document)) {
				// We need to order this after language server updates, but there is no API for that.
				// Hence, good old sleep().
				void sleep(10).then(() => this.eventEmitter.fire(this.uri));
			}
		}

		private onDidChangeActiveTextEditor(editor: vscode.TextEditor | undefined) {
			if (editor && isWeslEditor(editor)) {
				this.eventEmitter.fire(this.uri);
			}
		}

		async provideTextDocumentContent(
			_uri: vscode.Uri,
			ct: vscode.CancellationToken,
		): Promise<string> {
			const weslEditor = ctx.activeWeslEditor;
			if (!weslEditor) return "";
			const client = ctx.client;

			const parameters = {
				textDocument: client.code2ProtocolConverter.asTextDocumentIdentifier(
					weslEditor.document,
				),
			};
			return client.sendRequest(wa.viewItemTree, parameters, ct);
		}

		get onDidChange(): vscode.Event<vscode.Uri> {
			return this.eventEmitter.event;
		}
	})();

	ctx.pushExtCleanup(
		vscode.workspace.registerTextDocumentContentProvider("wgsl-analyzer-item-tree", tdcp),
	);

	return async () => {
		const document = await vscode.workspace.openTextDocument(tdcp.uri);
		tdcp.eventEmitter.fire(tdcp.uri);
		void (await vscode.window.showTextDocument(document, {
			viewColumn: vscode.ViewColumn.Two,
			preserveFocus: true,
		}));
	};
}

function crateGraph(ctx: CtxInit, full: boolean): Cmd {
	return async () => {
		const nodeModulesPath = vscode.Uri.file(path.join(ctx.extensionPath, "node_modules"));

		const panel = vscode.window.createWebviewPanel(
			"wgsl-analyzer.crate-graph",
			"wgsl-analyzer crate graph",
			vscode.ViewColumn.Two,
			{
				enableScripts: true,
				retainContextWhenHidden: true,
				localResourceRoots: [nodeModulesPath],
			},
		);
		const parameters = {
			full: full,
		};
		const client = ctx.client;
		const dot = await client.sendRequest(wa.viewCrateGraph, parameters);
		const uri = panel.webview.asWebviewUri(nodeModulesPath);

		const html = `
            <!DOCTYPE html>
            <meta charset="utf-8">
            <head>
                <style>
                    /* Fill the entire view */
                    html, body { margin:0; padding:0; overflow:hidden }
                    svg { position:fixed; top:0; left:0; height:100%; width:100% }

                    /* Disable the graphviz background and fill the polygons */
                    .graph > polygon { display:none; }
                    :is(.node,.edge) polygon { fill: white; }

                    /* Invert the line colors for dark themes */
                    body:not(.vscode-light) .edge path { stroke: white; }
                </style>
            </head>
            <body>
                <script type="text/javascript" src="${uri}/d3/dist/d3.min.js"></script>
                <script type="text/javascript" src="${uri}/@hpcc-js/wasm/dist/graphviz.umd.js"></script>
                <script type="text/javascript" src="${uri}/d3-graphviz/build/d3-graphviz.min.js"></script>
                <div id="graph"></div>
                <script>
                    let dot = \`${dot}\`;
                    let graph = d3.select("#graph")
                                  .graphviz({ useWorker: false, useSharedWorker: false })
                                  .fit(true)
                                  .zoomScaleExtent([0.1, Infinity])
                                  .renderDot(dot);

                    d3.select(window).on("click", (event) => {
                        if (event.ctrlKey) {
                            graph.resetZoom(d3.transition().duration(100));
                        }
                    });
                    d3.select(window).on("copy", (event) => {
                        event.clipboardData.setData("text/plain", dot);
                        event.preventDefault();
                    });
                </script>
            </body>
            `;

		panel.webview.html = html;
	};
}

export function viewCrateGraph(ctx: CtxInit): Cmd {
	return crateGraph(ctx, false);
}

export function viewFullCrateGraph(ctx: CtxInit): Cmd {
	return crateGraph(ctx, true);
}

export function reloadWorkspace(ctx: CtxInit): Cmd {
	return async () => ctx.client.sendRequest(wa.reloadWorkspace);
}

export function rebuildProcMacros(ctx: CtxInit): Cmd {
	return async () => ctx.client.sendRequest(wa.rebuildProcMacros);
}

async function showReferencesImpl(
	client: LanguageClient | undefined,
	uri: string,
	position: lc.Position,
	locations: lc.Location[],
) {
	if (client) {
		await vscode.commands.executeCommand(
			"editor.action.showReferences",
			vscode.Uri.parse(uri),
			client.protocol2CodeConverter.asPosition(position),
			locations.map(client.protocol2CodeConverter.asLocation),
		);
	}
}

export function showReferences(ctx: CtxInit): Cmd {
	return async (uri: string, position: lc.Position, locations: lc.Location[]) => {
		await showReferencesImpl(ctx.client, uri, position, locations);
	};
}

export function applyActionGroup(_ctx: CtxInit): Cmd {
	return async (actions: { label: string; arguments: lc.CodeAction }[]) => {
		const selectedAction = await vscode.window.showQuickPick(actions);
		if (!selectedAction) return;
		await vscode.commands.executeCommand(
			"wgsl-analyzer.resolveCodeAction",
			selectedAction.arguments,
		);
	};
}

export function gotoLocation(ctx: CtxInit): Cmd {
	return async (locationLink: lc.LocationLink) => {
		const client = ctx.client;
		const uri = client.protocol2CodeConverter.asUri(locationLink.targetUri);
		let range = client.protocol2CodeConverter.asRange(locationLink.targetSelectionRange);
		// collapse the range to a cursor position
		range = range.with({ end: range.start });

		await vscode.window.showTextDocument(uri, { selection: range });
	};
}

export function openDocs(ctx: CtxInit): Cmd {
	return async () => {
		const editor = vscode.window.activeTextEditor;
		if (!editor) {
			return;
		}
		const client = ctx.client;

		const position = editor.selection.active;
		const textDocument = { uri: editor.document.uri.toString() };

		const docLinks = await client.sendRequest(wa.openDocs, { position, textDocument });
		log.debug(docLinks);

		let fileType = vscode.FileType.Unknown;
		if (docLinks.local !== undefined) {
			try {
				fileType = (await vscode.workspace.fs.stat(vscode.Uri.parse(docLinks.local))).type;
			} catch (e) {
				log.debug("stat() threw error. Falling back to web version", e);
			}
		}

		let docLink = fileType & vscode.FileType.File ? docLinks.local : docLinks.web;
		if (docLink) {
			// instruct vscode to handle the vscode-remote link directly
			if (docLink.startsWith("vscode-remote://")) {
				docLink = docLink.replace("vscode-remote://", "vscode://vscode-remote/");
			}
			const docUri = vscode.Uri.parse(docLink);
			await vscode.env.openExternal(docUri);
		}
	};
}

export function openExternalDocs(ctx: CtxInit): Cmd {
	return async () => {
		const editor = vscode.window.activeTextEditor;
		if (!editor) {
			return;
		}
		const client = ctx.client;

		const position = editor.selection.active;
		const textDocument = { uri: editor.document.uri.toString() };

		const docLinks = await client.sendRequest(wa.openDocs, { position, textDocument });

		let docLink = docLinks.web;
		if (docLink) {
			// instruct vscode to handle the vscode-remote link directly
			if (docLink.startsWith("vscode-remote://")) {
				docLink = docLink.replace("vscode-remote://", "vscode://vscode-remote/");
			}
			const docUri = vscode.Uri.parse(docLink);
			await vscode.env.openExternal(docUri);
		}
	};
}

export function cancelFlycheck(ctx: CtxInit): Cmd {
	return async () => {
		await ctx.client.sendNotification(wa.cancelFlycheck);
	};
}

export function clearFlycheck(ctx: CtxInit): Cmd {
	return async () => {
		await ctx.client.sendNotification(wa.clearFlycheck);
	};
}

export function runFlycheck(ctx: CtxInit): Cmd {
	return async () => {
		const editor = ctx.activeWeslEditor;
		const client = ctx.client;
		const parameters = editor ? { uri: editor.document.uri.toString() } : null;

		await client.sendNotification(wa.runFlycheck, { textDocument: parameters });
	};
}

export function resolveCodeAction(ctx: CtxInit): Cmd {
	return async (parameters: lc.CodeAction) => {
		const client = ctx.client;
		parameters.command = undefined;
		const item = await client.sendRequest(lc.CodeActionResolveRequest.type, parameters);
		if (!item?.edit) {
			return;
		}
		const itemEdit = item.edit;
		// filter out all text edits and recreate the WorkspaceEdit without them so we can apply
		// snippet edits on our own
		const lcFileSystemEdit = {
			...itemEdit,
			documentChanges: itemEdit.documentChanges?.filter((change) => "kind" in change),
		};
		const fileSystemEdit =
			await client.protocol2CodeConverter.asWorkspaceEdit(lcFileSystemEdit);
		await vscode.workspace.applyEdit(fileSystemEdit);

		// replace all text edits so that we can convert snippet text edits into `vscode.SnippetTextEdit`s
		// FIXME: this is a workaround until vscode-languageclient supports doing the SnippeTextEdit conversion itself
		// also need to carry the snippetTextDocumentEdits separately, since we cannot retrieve them again using WorkspaceEdit.entries
		const [workspaceTextEdit, snippetTextDocumentEdits] = asWorkspaceSnippetEdit(ctx, itemEdit);
		await applySnippetWorkspaceEdit(workspaceTextEdit, snippetTextDocumentEdits);
		if (item.command != null) {
			await vscode.commands.executeCommand(item.command.command, item.command.arguments);
		}
	};
}

function asWorkspaceSnippetEdit(
	ctx: CtxInit,
	item: lc.WorkspaceEdit,
): [vscode.WorkspaceEdit, SnippetTextDocumentEdit[]] {
	const client = ctx.client;

	// partially borrowed from https://github.com/microsoft/vscode-languageserver-node/blob/295aaa393fda8ecce110c38880a00466b9320e63/client/src/common/protocolConverter.ts#L1060-L1101
	const result = new vscode.WorkspaceEdit();

	if (item.documentChanges) {
		const snippetTextDocumentEdits: SnippetTextDocumentEdit[] = [];

		for (const change of item.documentChanges) {
			if (lc.TextDocumentEdit.is(change)) {
				const uri = client.protocol2CodeConverter.asUri(change.textDocument.uri);
				const snippetTextEdits: (vscode.TextEdit | vscode.SnippetTextEdit)[] = [];

				for (const edit of change.edits) {
					if (
						"insertTextFormat" in edit &&
						edit.insertTextFormat === lc.InsertTextFormat.Snippet
					) {
						// is a snippet text edit
						snippetTextEdits.push(
							new vscode.SnippetTextEdit(
								client.protocol2CodeConverter.asRange(edit.range),
								new vscode.SnippetString(edit.newText),
							),
						);
					} else {
						// always as a text document edit
						snippetTextEdits.push(
							vscode.TextEdit.replace(
								client.protocol2CodeConverter.asRange(edit.range),
								edit.newText,
							),
						);
					}
				}

				snippetTextDocumentEdits.push([uri, snippetTextEdits]);
			}
		}
		return [result, snippetTextDocumentEdits];
	} else {
		// we do not handle WorkspaceEdit.changes since it is not relevant for code actions
		return [result, []];
	}
}

export function applySnippetWorkspaceEditCommand(_ctx: CtxInit): Cmd {
	return async (edit: vscode.WorkspaceEdit) => {
		await applySnippetWorkspaceEdit(edit, edit.entries());
	};
}

export function hoverRefCommandProxy(_: Ctx): Cmd {
	return async (index: number) => {
		const link = HOVER_REFERENCE_COMMAND[index];
		if (link) {
			const { command, arguments: args = [] } = link;
			await vscode.commands.executeCommand(command, ...args);
		}
	};
}

export function viewMemoryLayout(ctx: CtxInit): Cmd {
	return async () => {
		const editor = vscode.window.activeTextEditor;
		if (!editor) return;
		const client = ctx.client;

		const position = editor.selection.active;
		const expanded = await client.sendRequest(wa.viewRecursiveMemoryLayout, {
			textDocument: client.code2ProtocolConverter.asTextDocumentIdentifier(editor.document),
			position,
		});

		const document = vscode.window.createWebviewPanel(
			"memory_layout",
			"[Memory Layout]",
			vscode.ViewColumn.Two,
			{ enableScripts: true },
		);

		document.webview.html = `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
    <style>
        * {
            box-sizing: border-box;
        }

        body {
            margin: 0;
            overflow: hidden;
            min-height: 100%;
            height: 100vh;
            padding: 32px;
            position: relative;
            display: block;

            background-color: var(--vscode-editor-background);
            font-family: var(--vscode-editor-font-family);
            font-size: var(--vscode-editor-font-size);
            color: var(--vscode-editor-foreground);
        }

        .container {
            position: relative;
        }

        .trans {
            transition: all 0.2s ease-in-out;
        }

        .grid {
            height: 100%;
            position: relative;
            color: var(--vscode-commandCenter-activeBorder);
            pointer-events: none;
        }

        .grid-line {
            position: absolute;
            width: 100%;
            height: 1px;
            background-color: var(--vscode-commandCenter-activeBorder);
        }

        #tooltip {
            position: fixed;
            display: none;
            z-index: 1;
            pointer-events: none;
            padding: 4px 8px;
            z-index: 2;

            color: var(--vscode-editorHoverWidget-foreground);
            background-color: var(--vscode-editorHoverWidget-background);
            border: 1px solid var(--vscode-editorHoverWidget-border);
        }

        #tooltip b {
            color: var(--vscode-editorInlayHint-typeForeground);
        }

        #tooltip ul {
            margin-left: 0;
            padding-left: 20px;
        }

        table {
            position: absolute;
            transform: rotateZ(90deg) rotateX(180deg);
            transform-origin: top left;
            border-collapse: collapse;
            table-layout: fixed;
            left: 48px;
            top: 0;
            max-height: calc(100vw - 64px - 48px);
            z-index: 1;
        }

        td {
            border: 1px solid var(--vscode-focusBorder);
            writing-mode: vertical-rl;
            text-orientation: sideways-right;

            height: 80px;
        }

        td p {
            height: calc(100% - 16px);
            width: calc(100% - 8px);
            margin: 8px 4px;
            display: inline-block;
            transform: rotateY(180deg);
            pointer-events: none;
            overflow: hidden;
        }

        td p * {
            overflow: hidden;
            white-space: nowrap;
            text-overflow: ellipsis;
            display: inline-block;
            height: 100%;
        }

        td p b {
            color: var(--vscode-editorInlayHint-typeForeground);
        }

        td:hover {
            background-color: var(--vscode-editor-hoverHighlightBackground);
        }

        td:empty {
            visibility: hidden;
            border: 0;
        }
    </style>
</head>
<body>
    <div id="tooltip"></div>
</body>
<script>(function() {

const data = ${JSON.stringify(expanded)}

if (!(data && data.nodes.length)) {
    document.body.innerText = "Not Available"
    return
}

data.nodes.map(n => {
    n.typename = n.typename.replaceAll('&', '&amp;').replaceAll('<', '&lt;').replaceAll('>', '&gt;').replaceAll('"', ' & quot; ').replaceAll("'", '&#039;')
    return n
})

let height = window.innerHeight - 64

addEventListener("resize", e => {
    const new_height = window.innerHeight - 64
    height = new_height
    container.classList.remove("trans")
    table.classList.remove("trans")
    locate()
    setTimeout(() => { // give delay to redraw, annoying but needed
        container.classList.add("trans")
        table.classList.add("trans")
    }, 0)
})

const container = document.createElement("div")
container.classList.add("container")
container.classList.add("trans")
document.body.appendChild(container)

const tooltip = document.getElementById("tooltip")

let y = 0
let zoom = 1.0

const table = document.createElement("table")
table.classList.add("trans")
container.appendChild(table)
const rows = []

function node_t(index, depth, offset) {
    if (!rows[depth]) {
        rows[depth] = { el: document.createElement("tr"), offset: 0 }
    }

    if (rows[depth].offset < offset) {
        const pad = document.createElement("td")
        pad.colSpan = offset - rows[depth].offset
        rows[depth].el.appendChild(pad)
        rows[depth].offset += offset - rows[depth].offset
    }

    const td = document.createElement("td")
    td.innerHTML = '<p><span>' + data.nodes[index].itemName + ':</span> <b>' + data.nodes[index].typename + '</b></p>'

    td.colSpan = data.nodes[index].size

    td.addEventListener("mouseover", e => {
        const node = data.nodes[index]
        tooltip.innerHTML = node.itemName + ": <b>" + node.typename + "</b><br/>"
            + "<ul>"
            + "<li>size = " + node.size + "</li>"
            + "<li>align = " + node.alignment + "</li>"
            + "<li>field offset = " + node.offset + "</li>"
            + "</ul>"
            + "<i>double click to focus</i>"

        tooltip.style.display = "block"
    })
    td.addEventListener("mouseleave", _ => tooltip.style.display = "none")
    const total_offset = rows[depth].offset
    td.addEventListener("dblclick", e => {
        const node = data.nodes[index]
        zoom = data.nodes[0].size / node.size
        y = -(total_offset) / data.nodes[0].size * zoom
        x = 0
        locate()
    })

    rows[depth].el.appendChild(td)
    rows[depth].offset += data.nodes[index].size


    if (data.nodes[index].childrenStart != -1) {
        for (let i = 0; i < data.nodes[index].childrenLen; i++) {
            if (data.nodes[data.nodes[index].childrenStart + i].size) {
                node_t(data.nodes[index].childrenStart + i, depth + 1, offset + data.nodes[data.nodes[index].childrenStart + i].offset)
            }
        }
    }
}

node_t(0, 0, 0)

for (const row of rows) table.appendChild(row.el)

const grid = document.createElement("div")
grid.classList.add("grid")
container.appendChild(grid)

for (let i = 0; i < data.nodes[0].size / 8 + 1; i++) {
    const el = document.createElement("div")
    el.classList.add("grid-line")
    el.style.top = (i / (data.nodes[0].size / 8) * 100) + "%"
    el.innerText = i * 8
    grid.appendChild(el)
}

addEventListener("mousemove", e => {
    tooltip.style.top = e.clientY + 10 + "px"
    tooltip.style.left = e.clientX + 10 + "px"
})

function locate() {
    container.style.top = height * y + "px"
    container.style.height = (height * zoom) + "px"

    table.style.width = container.style.height
}

locate()

})()
</script>
</html>`;

		ctx.pushExtCleanup(document);
	};
}

export function toggleCheckOnSave(ctx: Ctx): Cmd {
	return async () => {
		await ctx.config.toggleCheckOnSave();
		ctx.refreshServerStatus();
	};
}

export function toggleLSPLogs(ctx: Ctx): Cmd {
	return async () => {
		const config = vscode.workspace.getConfiguration("wgsl-analyzer");
		const targetValue =
			config.get<string | undefined>("trace.server") === "verbose" ? undefined : "verbose";

		await config.update("trace.server", targetValue, vscode.ConfigurationTarget.Workspace);
		if (targetValue && ctx.client && ctx.client.traceOutputChannel) {
			ctx.client.traceOutputChannel.show();
		}
	};
}

export function openWalkthrough(_: Ctx): Cmd {
	return async () => {
		await vscode.commands.executeCommand(
			"workbench.action.openWalkthrough",
			"wgsl-analyzer.wgsl-analyzer#landing",
			false,
		);
	};
}
