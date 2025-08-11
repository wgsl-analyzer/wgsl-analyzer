import { strict as nativeAssert } from "assert";
import {
	type ExecOptions,
	exec,
	type SpawnOptionsWithoutStdio,
	spawn,
} from "child_process";
import { inspect } from "util";
import * as vscode from "vscode";

export function assert(
	condition: boolean,
	explanation: string,
): asserts condition {
	try {
		nativeAssert(condition, explanation);
	} catch (error) {
		log.error(`Assertion failed:`, explanation);
		throw error;
	}
}

export type Env = {
	[name: string]: string;
};

class Log {
	private readonly output = vscode.window.createOutputChannel(
		"WGSL Analyzer Client",
		{
			log: true,
		},
	);

	trace(...messages: [unknown, ...unknown[]]): void {
		this.output.trace(this.stringify(messages));
	}

	debug(...messages: [unknown, ...unknown[]]): void {
		this.output.debug(this.stringify(messages));
	}

	info(...messages: [unknown, ...unknown[]]): void {
		this.output.info(this.stringify(messages));
	}

	warn(...messages: [unknown, ...unknown[]]): void {
		this.output.warn(this.stringify(messages));
	}

	error(...messages: [unknown, ...unknown[]]): void {
		this.output.error(this.stringify(messages));
		this.output.show(true);
	}

	private stringify(messages: unknown[]): string {
		return messages
			.map((message) => {
				if (typeof message === "string") {
					return message;
				}
				if (message instanceof Error) {
					return message.stack || message.message;
				}
				return inspect(message, { depth: 6, colors: false });
			})
			.join(" ");
	}
}

export const log = new Log();

export function sleep(ms: number) {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

export type WeslDocument = vscode.TextDocument &
	({ languageId: "wesl" } | { languageId: "wgsl" });

export type WeslEditor = vscode.TextEditor & { document: WeslDocument };

export function isWeslDocument(
	document: vscode.TextDocument,
): document is WeslDocument {
	// Prevent corrupted text (particularly via inlay hints) in diff views
	// by allowing only `file` schemes.
	// Unfortunately, extensions that use diff views not always set this
	// to something different than "file".
	// See: https://github.com/rust-lang/rust-analyzer/issues/4608
	return (
		(document.languageId === "wgsl" || document.languageId === "wesl") &&
		document.uri.scheme === "file"
	);
}

export function isWeslEditor(editor: vscode.TextEditor): editor is WeslEditor {
	return isWeslDocument(editor.document);
}

export function isDocumentInWorkspace(document: WeslDocument): boolean {
	const workspaceFolders = vscode.workspace.workspaceFolders;
	if (!workspaceFolders) {
		return false;
	}
	for (const folder of workspaceFolders) {
		if (document.uri.fsPath.startsWith(folder.uri.fsPath)) {
			return true;
		}
	}
	return false;
}

/** Sets ['when'](https://code.visualstudio.com/docs/getstarted/keybindings#_when-clause-contexts) clause contexts */
// biome-ignore lint/suspicious/noExplicitAny: Signature comes from upstream
export function setContextValue(key: string, value: any): Thenable<void> {
	return vscode.commands.executeCommand("setContext", key, value);
}

/**
 * Returns a higher-order function that caches the results of invoking the
 * underlying async function.
 */
export function memoizeAsync<Ret, TThis, Parameter extends string>(
	func: (this: TThis, arg: Parameter) => Promise<Ret>,
) {
	const cache = new Map<string, Ret>();

	return async function (this: TThis, arg: Parameter) {
		const cached = cache.get(arg);
		if (cached) return cached;

		const result = await func.call(this, arg);
		cache.set(arg, result);

		return result;
	};
}

/** Awaitable wrapper around `child_process.exec` */
export function execute(
	command: string,
	options: ExecOptions,
): Promise<string> {
	log.info(`running command: ${command}`);
	return new Promise((resolve, reject) => {
		exec(command, options, (error, stdout, stderr) => {
			if (error) {
				log.error("error:", error);
				reject(error);
				return;
			}

			if (stderr) {
				reject(new Error(stderr));
				return;
			}

			resolve(stdout.trimEnd());
		});
	});
}

export class LazyOutputChannel implements vscode.OutputChannel {
	constructor(name: string) {
		this.name = name;
	}

	name: string;
	_channel: vscode.OutputChannel | undefined;

	get channel(): vscode.OutputChannel {
		if (!this._channel) {
			this._channel = vscode.window.createOutputChannel(this.name);
		}
		return this._channel;
	}

	append(value: string): void {
		this.channel.append(value);
	}

	appendLine(value: string): void {
		this.channel.appendLine(value);
	}

	replace(value: string): void {
		this.channel.replace(value);
	}

	clear(): void {
		if (this._channel) {
			this._channel.clear();
		}
	}

	show(preserveFocus?: boolean): void;
	show(column: vscode.ViewColumn, preserveFocus?: boolean): void;
	show(arg1?: boolean | vscode.ViewColumn, arg2?: boolean): void {
		let preserveFocus: boolean;
		if (typeof arg1 === "boolean") {
			preserveFocus = arg1;
		} else {
			preserveFocus = arg2 === true;
		}
		this.channel.show(preserveFocus);
	}

	hide(): void {
		if (this._channel) {
			this._channel.hide();
		}
	}

	dispose(): void {
		if (this._channel) {
			this._channel.dispose();
		}
	}
}

export type NotNull<T> = T extends null ? never : T;

export type Nullable<T> = null | T;

function isNotNull<T>(input: Nullable<T>): input is NotNull<T> {
	return input !== null;
}

function expectNotNull<T>(input: Nullable<T>, message: string): NotNull<T> {
	if (isNotNull(input)) {
		return input;
	}

	throw new TypeError(message);
}
export function unwrapNullable<T>(input: Nullable<T>): NotNull<T> {
	return expectNotNull(input, `unwrapping \`null\``);
}

export type NotUndefined<T> = T extends undefined ? never : T;
export type Undefinable<T> = T | undefined;

function isNotUndefined<T>(input: Undefinable<T>): input is NotUndefined<T> {
	return input !== undefined;
}

export function expectNotUndefined<T>(
	input: Undefinable<T>,
	message: string,
): NotUndefined<T> {
	if (isNotUndefined(input)) {
		return input;
	}

	throw new TypeError(message);
}

export function unwrapUndefinable<T>(input: Undefinable<T>): NotUndefined<T> {
	return expectNotUndefined(input, `unwrapping \`undefined\``);
}

interface SpawnAsyncReturns {
	stdout: string;
	stderr: string;
	status: number | null;
	error?: Error | undefined;
}

export async function spawnAsync(
	path: string,
	inputs?: ReadonlyArray<string>,
	options?: SpawnOptionsWithoutStdio,
): Promise<SpawnAsyncReturns> {
	const child = spawn(path, inputs, options);
	// biome-ignore lint/suspicious/noExplicitAny: Signature comes from upstream
	const stdout: Array<Buffer<any>> = [];
	// biome-ignore lint/suspicious/noExplicitAny: Signature comes from upstream
	const stderr: Array<Buffer<any>> = [];
	try {
		const result = await new Promise<{
			status: null | number;
			stderr: string;
			stdout: string;
		}>((resolve, reject) => {
			child.stdout.on("data", (chunk) => stdout.push(Buffer.from(chunk)));
			child.stderr.on("data", (chunk) => stderr.push(Buffer.from(chunk)));
			child.on("error", (error) => {
				reject({
					stdout: Buffer.concat(stdout).toString("utf8"),
					stderr: Buffer.concat(stderr).toString("utf8"),
					error,
				});
			});
			child.on("close", (status) => {
				resolve({
					stdout: Buffer.concat(stdout).toString("utf8"),
					stderr: Buffer.concat(stderr).toString("utf8"),
					status,
				});
			});
		});

		return {
			stdout: result.stdout,
			stderr: result.stderr,
			status: result.status,
		};
	} catch (exception: unknown) {
		assertIsStructuredError(exception);
		return {
			stdout: exception.stdout,
			stderr: exception.stderr,
			status: exception.status,
			error: exception.error,
		};
	}
}

type StructuredError = {
	error?: Error | undefined;
	status: null | number;
	stderr: string;
	stdout: string;
};

function assertIsStructuredError(
	object: unknown,
): asserts object is StructuredError {
	if (typeof object !== "object" || object === null || !("error" in object)) {
		throw new TypeError("Unexpected exception shape");
	}
}
