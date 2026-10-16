import { spawn } from "node:child_process";
import { text } from "node:stream/consumers";
import type * as vscode from "vscode";
import type * as lc from "vscode-languageclient/node";
import { bootstrap } from "./bootstrap";
import type { Bootstrap } from "./context";
import * as common from "./main_common";
import { log } from "./utilities";

const nativeBootstrap: Bootstrap = async (extCtx, config, state) => {
	const serverPath = await bootstrap(extCtx, config, state).catch((exception: unknown) => {
		let message = "bootstrap error. ";

		message += 'See the logs in "OUTPUT > wgsl-analyzer Client" (should open automatically). ';
		message +=
			'To enable verbose logs, click the gear icon in the "OUTPUT" tab and select "Debug".';

		log.error("Bootstrap error", exception);
		throw new Error(message);
	});
	let serverVersion = "<not running>";
	text(spawn(serverPath, ["--version"]).stdout.setEncoding("utf-8")).then(
		(data) => {
			const prefix = `wgsl-analyzer `;
			serverVersion = data.slice(data.startsWith(prefix) ? prefix.length : 0).trim();
		},
		(exception: unknown) => {
			log.error("Failed to get language server version", exception);
			serverVersion = "<unknown>";
		},
	);
	const newEnv = Object.assign({}, process.env, config.serverExtraEnv);
	const run: lc.Executable = {
		command: serverPath,
		options: { env: newEnv },
	};
	const serverOptions = {
		run,
		debug: run,
	};
	return [serverOptions, { path: serverPath, version: serverVersion }];
};

export async function deactivate() {
	await common.deactivate();
}

export async function activate(
	context: vscode.ExtensionContext,
): Promise<common.WgslAnalyzerExtensionApi> {
	return await common.activate(context, nativeBootstrap);
}
