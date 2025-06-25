import * as anser from "anser";
import * as lc from "vscode-languageclient/node";
import * as vscode from "vscode";
import * as wa from "./lsp_ext";
import * as Is from "vscode-languageclient/lib/common/utils/is";
import { assert, unwrapUndefinable } from "./util";
import * as diagnostics from "./diagnostics";
import { WorkspaceEdit } from "vscode";
import {
	type Config,
	prepareVSCodeConfig,
} from "./config";
import { sep as pathSeparator } from "path";
import { WaLanguageClient } from "./lang_client";

export async function createClient(
	traceOutputChannel: vscode.OutputChannel,
	outputChannel: vscode.OutputChannel,
	initializationOptions: vscode.WorkspaceConfiguration,
	serverOptions: lc.ServerOptions,
	config: Config,
	unlinkedFiles: vscode.Uri[],
): Promise<lc.LanguageClient> {
	const waMiddleware: lc.Middleware = {
		workspace: {
			// HACK: This is a workaround, when the client has been disposed, VSCode
			// continues to emit events to the client and the default one for this event
			// attempt to restart the client for no reason
			async didChangeWatchedFile(event, next) {
				if (client.isRunning()) {
					await next(event);
				}
			},
			async configuration(
				parameters: lc.ConfigurationParams,
				token: vscode.CancellationToken,
				next: lc.ConfigurationRequest.HandlerSignature,
			) {
				const response = await next(parameters, token);
				if (response && Array.isArray(response)) {
					return response.map((value) => {
						return prepareVSCodeConfig(value);
					});
				} else {
					return response;
				}
			},
		},
		async handleDiagnostics(
			uri: vscode.Uri,
			diagnosticList: vscode.Diagnostic[],
			next: lc.HandleDiagnosticsSignature,
		) {
			const preview = false; // todo simplify
			const errorCode = false; // todo simplify
			diagnosticList.forEach((diagnostic, index) => {
				const value =
					typeof diagnostic.code === "string" || typeof diagnostic.code === "number"
						? diagnostic.code
						: diagnostic.code?.value;
				if (
					// FIXME: We currently emit this diagnostic way too early, before we have
					// loaded the project fully
					// value === "unlinked-file" &&
					value === "temporary-disabled" &&
					!unlinkedFiles.includes(uri) &&
					(diagnostic.message === "file not included in crate hierarchy" ||
						diagnostic.message.startsWith("This file is not included in any crates"))
				) {
					const config = vscode.workspace.getConfiguration("wgsl-analyzer");
					if (config.get("showUnlinkedFileNotification")) {
						unlinkedFiles.push(uri);
						const folder = vscode.workspace.getWorkspaceFolder(uri)?.uri.fsPath;
						if (folder) {
							const parentBackslash = uri.fsPath.lastIndexOf(pathSeparator + "src");
							const parent = uri.fsPath.substring(0, parentBackslash);

							if (parent.startsWith(folder)) {
								const path = vscode.Uri.file(parent + pathSeparator + "Cargo.toml");
								void vscode.workspace.fs.stat(path).then(async () => {
									const choice = await vscode.window.showInformationMessage(
										`This wgsl file does not belong to a loaded cargo project. It looks like it might belong to the workspace at ${path.path}, do you want to add it to the linked Projects?`,
										"Yes",
										"No",
										"Don't show this again",
									);
									switch (choice) {
										case undefined:
											break;
										case "No":
											break;
										case "Yes": {
											const pathToInsert =
												"." +
												parent.substring(folder.length) +
												pathSeparator +
												"Cargo.toml";
											const value = config
												// eslint-disable-next-line @typescript-eslint/no-explicit-any
												.get<any[]>("linkedProjects")
												?.concat(pathToInsert);
											await config.update("linkedProjects", value, false);
											break;
										}
										case "Don't show this again":
											await config.update(
												"showUnlinkedFileNotification",
												false,
												false,
											);
											break;
									}
								});
							}
						}
					}
				}

				// Abuse the fact that VSCode leaks the LSP diagnostics data field through the
				// Diagnostic class, if they ever break this we are out of luck and have to go
				// back to the worst diagnostics experience ever:)

				// We encode the rendered output of a rustc diagnostic in the rendered field of
				// the data payload of the lsp diagnostic. If that field exists, overwrite the
				// diagnostic code such that clicking it opens the diagnostic in a readonly
				// text editor for easy inspection
				const rendered = (diagnostic as unknown as { data?: { rendered?: string } }).data
					?.rendered;
				if (rendered) {
					if (preview) {
						const decolorized = anser.ansiToText(rendered);
						const index = decolorized.match(/^(note|help):/m)?.index || rendered.length;
						diagnostic.message = decolorized
							.substring(0, index)
							.replace(/^ -->[^\n]+\n/m, "");
					}
					diagnostic.code = {
						target: vscode.Uri.from({
							scheme: diagnostics.URI_SCHEME,
							path: `/diagnostic message [${index.toString()}]`,
							fragment: uri.toString(),
							query: index.toString(),
						}),
						value: errorCode && value ? value : "Click for full compiler diagnostic",
					};
				}
			});
			return next(uri, diagnosticList);
		},
		async provideHover(
			document: vscode.TextDocument,
			position: vscode.Position,
			token: vscode.CancellationToken,
			_next: lc.ProvideHoverSignature,
		) {
			const editor = vscode.window.activeTextEditor;
			const positionOrRange = editor?.selection?.contains(position)
				? client.code2ProtocolConverter.asRange(editor.selection)
				: client.code2ProtocolConverter.asPosition(position);
			const parameters = {
				textDocument: client.code2ProtocolConverter.asTextDocumentIdentifier(document),
				position: positionOrRange,
			};
			return client.sendRequest(wa.hover, parameters, token).then(
				(result) => {
					if (!result) return null;
					const hover = client.protocol2CodeConverter.asHover(result);
					if (result.actions) {
						hover.contents.push(renderHoverActions(result.actions));
					}
					return hover;
				},
				(error) => {
					client.handleFailedRequest(lc.HoverRequest.type, token, error, null);
					return Promise.resolve(null);
				},
			);
		},
		// Using custom handling of CodeActions to support action groups and snippet edits.
		// Note that this means we have to re-implement lazy edit resolving ourselves as well.
		async provideCodeActions(
			document: vscode.TextDocument,
			range: vscode.Range,
			context: vscode.CodeActionContext,
			token: vscode.CancellationToken,
			_next: lc.ProvideCodeActionsSignature,
		) {
			const parameters: lc.CodeActionParams = {
				textDocument: client.code2ProtocolConverter.asTextDocumentIdentifier(document),
				range: client.code2ProtocolConverter.asRange(range),
				context: await client.code2ProtocolConverter.asCodeActionContext(context, token),
			};
			const callback = async (
				values: (lc.Command | lc.CodeAction)[] | null,
			): Promise<(vscode.Command | vscode.CodeAction)[] | undefined> => {
				if (values === null) return undefined;
				const result: (vscode.CodeAction | vscode.Command)[] = [];
				const groups = new Map<string, { index: number; items: vscode.CodeAction[] }>();
				for (const item of values) {
					// In our case we expect to get code edits only from diagnostics
					if (lc.CodeAction.is(item)) {
						assert(!item.command, "We don't expect to receive commands in CodeActions");
						const action = await client.protocol2CodeConverter.asCodeAction(
							item,
							token,
						);
						result.push(action);
						continue;
					}
					assert(
						isCodeActionWithoutEditsAndCommands(item),
						"We do not expect edits or commands here",
					);
					// eslint-disable-next-line @typescript-eslint/no-explicit-any
					const kind = client.protocol2CodeConverter.asCodeActionKind((item as any).kind);
					const action = new vscode.CodeAction(item.title, kind);
					// eslint-disable-next-line @typescript-eslint/no-explicit-any
					const group = (item as any).group;
					action.command = {
						command: "wgsl-analyzer.resolveCodeAction",
						title: item.title,
						arguments: [item],
					};

					// Set a dummy edit, so that VS Code doesn't try to resolve this.
					action.edit = new WorkspaceEdit();

					if (group) {
						let entry = groups.get(group);
						if (!entry) {
							entry = { index: result.length, items: [] };
							groups.set(group, entry);
							result.push(action);
						}
						entry.items.push(action);
					} else {
						result.push(action);
					}
				}
				for (const [group, { index, items }] of groups) {
					if (items.length === 1) {
						const item = unwrapUndefinable(items[0]);
						result[index] = item;
					} else {
						const action = new vscode.CodeAction(group);
						const item = unwrapUndefinable(items[0]);
						action.kind = item.kind;
						action.command = {
							command: "wgsl-analyzer.applyActionGroup",
							title: "",
							arguments: [
								items.map((item) => {
									return {
										label: item.title,
										arguments: item.command!.arguments![0],
									};
								}),
							],
						};

						// Set a dummy edit, so that VS Code doesn't try to resolve this.
						action.edit = new WorkspaceEdit();

						result[index] = action;
					}
				}
				return result;
			};
			return client
				.sendRequest(lc.CodeActionRequest.type, parameters, token)
				.then(callback, (_error) => undefined);
		},
	};
	const clientOptions: lc.LanguageClientOptions = {
		documentSelector: [{ scheme: "file", language: "wgsl" }],
		initializationOptions,
		diagnosticCollectionName: "wgsl",
		traceOutputChannel,
		outputChannel,
		middleware: waMiddleware,
		markdown: {
			supportHtml: true,
		},
	};

	const client = new WaLanguageClient(
		"wgsl-analyzer",
		"WGSL Analyzer Language Server",
		serverOptions,
		clientOptions,
	);

	// To turn on all proposed features use: client.registerProposedFeatures();
	client.registerFeature(new ExperimentalFeatures(config));
	client.registerFeature(new OverrideFeatures());

	return client;
}

class ExperimentalFeatures implements lc.StaticFeature {
	private readonly testExplorer: boolean;

	constructor(config: Config) {
		this.testExplorer = config.testExplorer || false;
	}

	getState(): lc.FeatureState {
		return { kind: "static" };
	}

	fillClientCapabilities(capabilities: lc.ClientCapabilities): void {
		capabilities.experimental = {
			snippetTextEdit: true,
			codeActionGroup: true,
			hoverActions: true,
			serverStatusNotification: true,
			colorDiagnosticOutput: true,
			openServerLogs: true,
			localDocs: true,
			testExplorer: this.testExplorer,
			commands: {
				commands: [
					"wgsl-analyzer.showReferences",
					"wgsl-analyzer.gotoLocation",
					"wgsl-analyzer.triggerParameterHints",
					"wgsl-analyzer.rename",
				],
			},
			...capabilities.experimental,
		};
	}

	initialize(
		_capabilities: lc.ServerCapabilities,
		_documentSelector: lc.DocumentSelector | undefined,
	): void { }

	dispose(): void { }
	clear(): void { }
}

class OverrideFeatures implements lc.StaticFeature {
	getState(): lc.FeatureState {
		return { kind: "static" };
	}

	fillClientCapabilities(capabilities: lc.ClientCapabilities): void {
		// Force disable `augmentsSyntaxTokens`, VS Code's textmate grammar is somewhat incomplete
		// making the experience generally worse
		const caps = capabilities.textDocument?.semanticTokens;
		if (caps) {
			caps.augmentsSyntaxTokens = false;
		}
	}

	initialize(
		_capabilities: lc.ServerCapabilities,
		_documentSelector: lc.DocumentSelector | undefined,
	): void { }

	dispose(): void { }
	clear(): void { }
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
function isCodeActionWithoutEditsAndCommands(value: any): boolean {
	const candidate: lc.CodeAction = value;
	return (
		candidate &&
		Is.string(candidate.title) &&
		(candidate.diagnostics === void 0 ||
			Is.typedArray(candidate.diagnostics, lc.Diagnostic.is)) &&
		(candidate.kind === void 0 || Is.string(candidate.kind)) &&
		candidate.edit === void 0 &&
		candidate.command === void 0
	);
}

// Command URIs have a form of command:command-name?arguments, where
// arguments is a percent-encoded array of data we want to pass along to
// the command function. For "Show References" this is a list of all file
// URIs with locations of every reference, and it can get quite long.
// So long in fact that it will fail rendering inside an `a` tag so we need
// to proxy around that. We store the last hover's reference command link
// here, as only one hover can be active at a time, and we do not need to
// keep a history of these.
export let HOVER_REFERENCE_COMMAND: wa.CommandLink[] = [];

function renderCommand(cmd: wa.CommandLink): string {
	HOVER_REFERENCE_COMMAND.push(cmd);
	return `[${cmd.title}](command:wgsl-analyzer.hoverRefCommandProxy?${HOVER_REFERENCE_COMMAND.length - 1} '${cmd.tooltip}')`;
}

function renderHoverActions(actions: wa.CommandLinkGroup[]): vscode.MarkdownString {
	// clean up the previous hover ref command
	HOVER_REFERENCE_COMMAND = [];
	const text = actions
		.map(
			(group) =>
				(group.title ? group.title + " " : "") +
				group.commands.map(renderCommand).join(" | "),
		)
		.join(" | ");

	const result = new vscode.MarkdownString(text);
	result.isTrusted = true;
	return result;
}
