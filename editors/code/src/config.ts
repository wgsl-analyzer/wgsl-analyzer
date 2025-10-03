import * as os from "os";
import * as path from "path";
import type { Disposable } from "vscode";
import * as vscode from "vscode";
import * as Is from "vscode-languageclient/lib/common/utils/is";

import type { Env } from "./utilities";

import { expectNotUndefined, log, unwrapUndefinable } from "./utilities";

export type RunnableEnvCfgItem = {
	mask?: string;
	env: Record<string, string>;
	platform?: string | string[];
};
export type RunnableEnvCfg = Record<string, string> | RunnableEnvCfgItem[];
type ShowStatusBar = "always" | "never" | { documentSelector: vscode.DocumentSelector };

export interface TraceConfig {
	extension: boolean;
	server: boolean;
}

export interface InlayHintsConfig {
	enabled: boolean;
	typeHints: boolean;
	parameterHints: boolean;
	typeVerbosity: "full" | "short" | "compact";
}

export interface DiagnosticsConfig {
	typeErrors: boolean;
	nagaParsingErrors: boolean;
	nagaValidationErrors: boolean;
}

export class Config {
	readonly extensionId = "wgsl-analyzer.wgsl-analyzer";
	configureLang: vscode.Disposable | undefined;

	readonly rootSection = "wgsl-analyzer";

	private readonly requiresServerReloadOpts = ["serverPath", "server", "files", "cfg"].map(
		(opt) => `${this.rootSection}.${opt}`,
	);

	constructor(disposables: Disposable[]) {
		vscode.workspace.onDidChangeConfiguration(this.onDidChangeConfiguration, this, disposables);
		this.refreshLogging();
		this.configureLanguage();
	}

	dispose() {
		this.configureLang?.dispose();
	}

	private refreshLogging() {
		log.info(
			"Extension version:",
			vscode.extensions.getExtension(this.extensionId)?.packageJSON.version,
		);

		const cfg = Object.entries(this.cfg).filter(([_, value]) => !(value instanceof Function));
		log.info("Using configuration", Object.fromEntries(cfg));
	}

	private async onDidChangeConfiguration(event: vscode.ConfigurationChangeEvent) {
		this.refreshLogging();

		this.configureLanguage();

		const requiresServerReloadOpt = this.requiresServerReloadOpts.find((opt) =>
			event.affectsConfiguration(opt),
		);

		if (!requiresServerReloadOpt) return;

		if (this.restartServerOnConfigChange) {
			await vscode.commands.executeCommand("wgsl-analyzer.restartServer");
			return;
		}

		const message = `Changing "${requiresServerReloadOpt}" requires a server restart`;
		const userResponse = await vscode.window.showInformationMessage(message, "Restart now");

		if (userResponse) {
			const command = "wgsl-analyzer.restartServer";
			await vscode.commands.executeCommand(command);
		}
	}

	/**
	 * Sets up additional language configuration that is impossible to do via a
	 * separate language-configuration.json file. See [1] for more information.
	 *
	 * [1]: https://github.com/Microsoft/vscode/issues/11514#issuecomment-244707076
	 */
	private configureLanguage() {
		// Only need to dispose of the config if there is a change
		if (this.configureLang) {
			this.configureLang.dispose();
			this.configureLang = undefined;
		}

		let onEnterRules: vscode.OnEnterRule[] = [
			{
				// Carry indentation from the previous line
				// if it is only whitespace
				beforeText: /^\s+$/,
				action: { indentAction: vscode.IndentAction.None },
			},
			{
				// After the end of a function/field chain,
				// with the semicolon on the same line
				beforeText: /^\s+\..*;/,
				action: { indentAction: vscode.IndentAction.Outdent },
			},
			{
				// After the end of a function/field chain,
				// with semicolon detached from the rest
				beforeText: /^\s+;/,
				previousLineText: /^\s+\..*/,
				action: { indentAction: vscode.IndentAction.Outdent },
			},
		];

		if (this.typingContinueCommentsOnNewline) {
			const indentAction = vscode.IndentAction.None;

			onEnterRules = [
				...onEnterRules,
				{
					// Doc single-line comment
					// e.g. ///|
					beforeText: /^\s*\/{3}.*$/,
					action: { indentAction, appendText: "/// " },
				},
				{
					// Parent doc single-line comment
					// e.g. //!|
					beforeText: /^\s*\/{2}!.*$/,
					action: { indentAction, appendText: "//! " },
				},
				{
					// Begins an auto-closed multi-line comment (standard or parent doc)
					// e.g. /** | */ or /*! | */
					beforeText: /^\s*\/\*(\*|!)(?!\/)([^*]|\*(?!\/))*$/,
					afterText: /^\s*\*\/$/,
					action: {
						indentAction: vscode.IndentAction.IndentOutdent,
						appendText: " * ",
					},
				},
				{
					// Begins a multi-line comment (standard or parent doc)
					// e.g. /** ...| or /*! ...|
					beforeText: /^\s*\/\*(\*|!)(?!\/)([^*]|\*(?!\/))*$/,
					action: { indentAction, appendText: " * " },
				},
				{
					// Continues a multi-line comment
					// e.g.  * ...|
					beforeText: /^( {3})* \*( ([^*]|\*(?!\/))*)?$/,
					action: { indentAction, appendText: "* " },
				},
				{
					// Dedents after closing a multi-line comment
					// e.g.  */|
					beforeText: /^( {3})* \*\/\s*$/,
					action: { indentAction, removeText: 1 },
				},
			];
		}

		this.configureLang = vscode.languages.setLanguageConfiguration("wgsl", {
			onEnterRules,
		});

		this.configureLang = vscode.languages.setLanguageConfiguration("wesl", {
			onEnterRules,
		});
	}

	get previewWeslRsOutput() {
		return this.get<boolean>("diagnostics.previewWeslRsOutput");
	}

	get useWeslRsErrorCode() {
		return this.get<boolean>("diagnostics.useWeslRsErrorCode");
	}

	// We do not do runtime config validation here for simplicity. More on stackoverflow:
	// https://stackoverflow.com/questions/60135780/what-is-the-best-way-to-type-check-the-configuration-for-vscode-extension

	private get cfg(): vscode.WorkspaceConfiguration {
		return vscode.workspace.getConfiguration(this.rootSection);
	}

	/**
	 * Beware that postfix `!` operator erases both `null` and `undefined`.
	 * This is why the following does not work as expected:
	 *
	 * ```ts
	 * const nullableNum = vscode
	 *  .workspace
	 *  .getConfiguration
	 *  .getConfiguration("wgsl-analyzer")
	 *  .get<number | null>(path)!;
	 *
	 * // What happens is that type of `nullableNum` is `number` but not `null | number`:
	 * const fullFledgedNum: number = nullableNum;
	 * ```
	 * So this getter handles this quirk by not requiring the caller to use postfix `!`
	 */
	private get<T>(path: string): T | undefined {
		return prepareVSCodeConfig(this.cfg.get<T>(path));
	}

	get serverPath() {
		return this.get<null | string>("server.path") ?? this.get<null | string>("serverPath");
	}

	get serverExtraEnv(): Env {
		const extraEnv = this.get<{ [key: string]: string | number } | null>("server.extraEnv") ?? {};
		return substituteVariablesInEnv(
			Object.fromEntries(
				Object.entries(extraEnv).map(([k, v]) => [k, typeof v !== "string" ? v.toString() : v]),
			),
		);
	}

	get checkOnSave() {
		return this.get<boolean>("checkOnSave") ?? false;
	}

	async toggleCheckOnSave() {
		const config = this.cfg.inspect<boolean>("checkOnSave") ?? {
			key: "checkOnSave",
		};
		let overrideInLanguage;
		let target;
		let value;
		if (
			config.workspaceFolderValue !== undefined
			|| config.workspaceFolderLanguageValue !== undefined
		) {
			target = vscode.ConfigurationTarget.WorkspaceFolder;
			overrideInLanguage = config.workspaceFolderLanguageValue;
			value = config.workspaceFolderValue || config.workspaceFolderLanguageValue;
		} else if (config.workspaceValue !== undefined || config.workspaceLanguageValue !== undefined) {
			target = vscode.ConfigurationTarget.Workspace;
			overrideInLanguage = config.workspaceLanguageValue;
			value = config.workspaceValue || config.workspaceLanguageValue;
		} else if (config.globalValue !== undefined || config.globalLanguageValue !== undefined) {
			target = vscode.ConfigurationTarget.Global;
			overrideInLanguage = config.globalLanguageValue;
			value = config.globalValue || config.globalLanguageValue;
		} else if (config.defaultValue !== undefined || config.defaultLanguageValue !== undefined) {
			overrideInLanguage = config.defaultLanguageValue;
			value = config.defaultValue || config.defaultLanguageValue;
		}
		await this.cfg.update("checkOnSave", !(value || false), target || null, overrideInLanguage);
	}

	get problemMatcher(): string[] {
		return this.get<string[]>("runnables.problemMatcher") || [];
	}

	get testExplorer() {
		return this.get<boolean | undefined>("testExplorer");
	}

	get restartServerOnConfigChange() {
		return this.get<boolean>("restartServerOnConfigChange");
	}

	get typingContinueCommentsOnNewline() {
		return this.get<boolean>("typing.continueCommentsOnNewline");
	}

	get debug() {
		let sourceFileMap = this.get<Record<string, string> | "auto">("debug.sourceFileMap");
		if (sourceFileMap !== "auto") {
			// "/wesl/<id>" used by suggestions only.
			const { ["/wesl/<id>"]: _, ...trimmed } =
				this.get<Record<string, string>>("debug.sourceFileMap") ?? {};
			sourceFileMap = trimmed;
		}

		return {
			engine: this.get<string>("debug.engine"),
			engineSettings: this.get<object>("debug.engineSettings") ?? {},
			openDebugPane: this.get<boolean>("debug.openDebugPane"),
			buildBeforeRestart: this.get<boolean>("debug.buildBeforeRestart"),
			sourceFileMap: sourceFileMap,
		};
	}

	get hoverActions() {
		return {
			enable: this.get<boolean>("hover.actions.enable"),
			implementations: this.get<boolean>("hover.actions.implementations.enable"),
			references: this.get<boolean>("hover.actions.references.enable"),
			run: this.get<boolean>("hover.actions.run.enable"),
			debug: this.get<boolean>("hover.actions.debug.enable"),
			gotoTypeDef: this.get<boolean>("hover.actions.gotoTypeDef.enable"),
		};
	}

	get showSyntaxTree() {
		return this.get<boolean>("showSyntaxTree");
	}

	get statusBarClickAction() {
		return this.get<string>("statusBar.clickAction");
	}

	get statusBarShowStatusBar() {
		return this.get<ShowStatusBar>("statusBar.showStatusBar");
	}

	get initializeStopped() {
		return this.get<boolean>("initializeStopped");
	}

	get askBeforeUpdateTest() {
		return this.get<boolean>("runnables.askBeforeUpdateTest");
	}

	async setAskBeforeUpdateTest(value: boolean) {
		await this.cfg.update("runnables.askBeforeUpdateTest", value, true);
	}

	get diagnostics(): DiagnosticsConfig | undefined {
		return this.get<DiagnosticsConfig>("diagnostics");
	}

	get customImports(): Record<string, string> | undefined {
		return this.get("customImports");
	}

	get shaderDefs(): [string] | undefined {
		return this.get<[string]>("preprocessor.shaderDefs");
	}

	get trace(): TraceConfig | undefined {
		return this.get("trace");
	}

	get inlayHints(): InlayHintsConfig | undefined {
		return this.get("inlayHints");
	}
}

export function prepareVSCodeConfig<T>(response: T): T {
	if (Is.string(response)) {
		return substituteVSCodeVariableInString(response) as T;
		// biome-ignore lint/suspicious/noExplicitAny: Signature comes from upstream
	} else if (response && Is.array<any>(response)) {
		return response.map((value) => {
			return prepareVSCodeConfig(value);
		}) as T;
	} else if (response && typeof response === "object") {
		// biome-ignore lint/suspicious/noExplicitAny: Signature comes from upstream
		const result: { [key: string]: any } = {};
		for (const key in response) {
			const value = response[key];
			result[key] = prepareVSCodeConfig(value);
		}
		return result as T;
	}
	return response;
}

// FIXME: Merge this with `substituteVSCodeVariables` above
export function substituteVariablesInEnv(env: Env): Env {
	const missingDeps = new Set<string>();
	// vscode uses `env:ENV_NAME` for env vars resolution, and it is easier
	// to follow the same convention for our dependency tracking
	const definedEnvKeys = new Set(Object.keys(env).map((key) => `env:${key}`));
	const envWithDeps = Object.fromEntries(
		Object.entries(env).map(([key, value]) => {
			const deps = new Set<string>();
			const depRe = new RegExp(/\$\{(?<depName>.+?)\}/g);
			let match = undefined;
			while ((match = depRe.exec(value))) {
				const depName = unwrapUndefinable(match.groups?.["depName"]);
				deps.add(depName);
				// `depName` at this point can have a form of `expression` or
				// `prefix:expr`
				if (!definedEnvKeys.has(depName)) {
					missingDeps.add(depName);
				}
			}
			return [`env:${key}`, { deps: [...deps], value }];
		}),
	);

	const resolved = new Set<string>();
	for (const dep of missingDeps) {
		const match = /(?<prefix>.*?):(?<body>.+)/.exec(dep);
		if (match) {
			// biome-ignore lint/style/noNonNullAssertion: TODO
			const { prefix, body } = match.groups!;
			if (prefix === "env") {
				const envName = unwrapUndefinable(body);
				envWithDeps[dep] = {
					value: process.env[envName] ?? "",
					deps: [],
				};
				resolved.add(dep);
			} else {
				// we cannot handle other prefixes at the moment
				// leave values as is, but still mark them as resolved
				envWithDeps[dep] = {
					value: "${" + dep + "}",
					deps: [],
				};
				resolved.add(dep);
			}
		} else {
			envWithDeps[dep] = {
				value: computeVscodeVar(dep) || "${" + dep + "}",
				deps: [],
			};
		}
	}
	const toResolve = new Set(Object.keys(envWithDeps));

	let leftToResolveSize: number;
	do {
		leftToResolveSize = toResolve.size;
		for (const key of toResolve) {
			const item = unwrapUndefinable(envWithDeps[key]);
			if (item.deps.every((dep) => resolved.has(dep))) {
				item.value = item.value.replace(/\$\{(?<depName>.+?)\}/g, (_wholeMatch, depName) => {
					const item = unwrapUndefinable(envWithDeps[depName]);
					return item.value;
				});
				resolved.add(key);
				toResolve.delete(key);
			}
		}
	} while (toResolve.size > 0 && toResolve.size < leftToResolveSize);

	const resolvedEnv: Env = {};
	for (const key of Object.keys(env)) {
		const item = unwrapUndefinable(envWithDeps[`env:${key}`]);
		resolvedEnv[key] = item.value;
	}
	return resolvedEnv;
}

const VarRegex = new RegExp(/\$\{(.+?)\}/g);

function substituteVSCodeVariableInString(value: string): string {
	return value.replace(VarRegex, (substring: string, varName) => {
		if (Is.string(varName)) {
			return computeVscodeVar(varName) || substring;
		} else {
			return substring;
		}
	});
}

function computeVscodeVar(varName: string): string | null {
	const workspaceFolder = () => {
		const folders = vscode.workspace.workspaceFolders ?? [];
		const folder = folders[0];
		// TODO: support for remote workspaces?
		const fsPath: string =
			folder === undefined
				? "" // no workspace opened
				: // could use currently opened document to detect the correct
					// workspace. However, that would be determined by the document
					// user has opened on Editor startup. Could lead to
					// unpredictable workspace selection in practice.
					// It is better to pick the first one
					folder.uri.fsPath;
		return fsPath;
	};
	// https://code.visualstudio.com/docs/editor/variables-reference
	const supportedVariables: { [k: string]: () => string } = {
		workspaceFolder,

		workspaceFolderBasename: () => {
			return path.basename(workspaceFolder());
		},

		cwd: () => process.cwd(),
		userHome: () => os.homedir(),

		// see
		// https://github.com/microsoft/vscode/blob/08ac1bb67ca2459496b272d8f4a908757f24f56f/src/vs/workbench/api/common/extHostVariableResolverService.ts#L81
		// or
		// https://github.com/microsoft/vscode/blob/29eb316bb9f154b7870eb5204ec7f2e7cf649bec/src/vs/server/node/remoteTerminalChannel.ts#L56
		execPath: () => process.env["VSCODE_EXEC_PATH"] ?? process.execPath,

		pathSeparator: () => path.sep,
	};

	if (varName in supportedVariables) {
		const fn = expectNotUndefined(
			supportedVariables[varName],
			`${varName} should not be undefined here`,
		);
		return fn();
	} else {
		// return "${" + varName + "}";
		return null;
	}
}
