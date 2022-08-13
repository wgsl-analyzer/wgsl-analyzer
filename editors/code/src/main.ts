import { ExtensionContext } from "vscode";
import * as vscode from "vscode";
import * as path from "path";
import * as os from "os";
import * as fs from "fs";
import * as util from "util";
import * as cp from "child_process";

const VERSION = "0.5.0";
const REV = "a992070";

const fileExists = (path: string) => util.promisify(fs.access)(path).then(s => true).catch(() => false);
const exec = util.promisify(cp.exec);

import { Ctx } from "./ctx";
import * as commands from "./commands";
import { Config } from "./config";
import { activateInlayHints } from "./inlay_hints";

let ctx: Ctx;

export async function activate(context: ExtensionContext) {
    const config = new Config(context);
    const serverPath = await getServer(config);
    if (!serverPath) {
        return;
    }
    const serverVersion = await getServerVersion(serverPath);
    if (serverVersion != VERSION) {
        const msg = `wgsl-analyzer binary version (${serverVersion}) does not match extension (${VERSION}).
If you are using a version of wgsl-analyzer without a prepackaged binary or specify a custom server path, please use the matching version: \`cargo install --git https://github.com/wgsl-analyzer/wgsl-analyzer --rev ${REV} wgsl_analyzer\`.`;
        vscode.window.showWarningMessage(msg, "Okay");
        return;
    }

    ctx = await Ctx.create(serverPath, context, config);
    ctx.registerCommand("syntaxTree", commands.syntaxTree);
    ctx.registerCommand("debugCommand", commands.debugCommand);
    ctx.registerCommand("fullSource", commands.showFullSource);

    activateInlayHints(ctx);

    vscode.workspace.onDidChangeConfiguration(_ => ctx.client.sendNotification("workspace/didChangeConfiguration", { settings: "" }), null, ctx.subscriptions);
}

export function deactivate(): Thenable<void> | undefined {
    if (!ctx.client) {
        return undefined;
    }
    return ctx.client.stop();
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

async function getServerVersion(serverPath: string): Promise<string> {
    const promise = exec(`${serverPath} --version`);
    const stdin = (promise as { child: cp.ChildProcess; }).child.stdin;
    stdin.write("\n");
    try {
        return (await promise).stdout.trim();
    } catch (e) {
        console.error(e);
        return "<unknown>";
    }
}
