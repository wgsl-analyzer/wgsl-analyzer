import { ExtensionContext } from 'vscode';
import * as vscode from 'vscode';
import * as path from "path";
import * as os from "os";
import * as fs from "fs";
import * as util from "util";

const fileExists = (path: string) => util.promisify(fs.access)(path).then(s => true).catch(() => false);

import {
    LanguageClient,
} from 'vscode-languageclient/node';

import { Ctx } from './ctx';
import * as commands from './commands';
import { Config } from './config';
import { fstat } from 'fs';

let client: LanguageClient;

export async function activate(context: ExtensionContext) {
    const config = new Config(context);
    const serverPath = await getServer(config);
    if (!serverPath) {
        return;
    }

    let ctx = await Ctx.create(serverPath, context);
    ctx.registerCommand("syntaxTree", commands.syntaxTree);
    ctx.registerCommand("debugCommand", commands.debugCommand);

    vscode.workspace.onDidChangeConfiguration(_ => ctx.client.sendNotification("workspace/didChangeConfiguration", { settings: "" }), null, ctx.subscriptions);
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}

async function getServer(config: Config): Promise<string | undefined> {
    const explicitPath = config.serverPath;
    if (explicitPath) {
        if (explicitPath.startsWith("~/")) {
            return os.homedir() + explicitPath.slice("~".length);
        }
        return explicitPath;
    }

    let windows = process.platform === "win32";
    let suffix = windows ? ".exe" : "";

    const bundledPath = path.resolve(__dirname, `wgsl_analyzer${suffix}`);

    if (await fileExists(bundledPath)) {
        return bundledPath;
    }

    vscode.window.showErrorMessage("wgsl-analyzer.server.path is not specified");
    return undefined;
}