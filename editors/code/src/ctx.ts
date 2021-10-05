import * as vscode from "vscode";
import { Disposable, ExtensionContext } from "vscode";
import { Executable, LanguageClient, LanguageClientOptions, ServerOptions } from "vscode-languageclient/node";
import { Config } from "./config";
import { isWgslEditor, WgslEditor } from "./util";

export class Ctx {
    private constructor(readonly ctx: ExtensionContext, readonly client: LanguageClient) { }

    static async create(serverPath: string, ctx: ExtensionContext) {
        const run: Executable = {
            command: serverPath,
        };
        const serverOptions: ServerOptions = {
            run,
            debug: run,
        };

        const initializationOptions = vscode.workspace.getConfiguration(Config.rootSection);

        const clientOptions: LanguageClientOptions = {
            documentSelector: [{ language: "wgsl" }, { scheme: 'file', pattern: '*.wgsl' }],
            initializationOptions,
        };

        let client = new LanguageClient(
            'wgsl_analyzer',
            'WGSL Language server',
            serverOptions,
            clientOptions
        );

        client.start();
        await client.onReady();

        return new Ctx(ctx, client);
    }


    registerCommand(name: string, factory: (ctx: Ctx) => Cmd) {
        const fullName = `wgsl-analyzer.${name}`;
        const cmd = factory(this);
        const dispoable = vscode.commands.registerCommand(fullName, cmd);
        this.pushCleanup(dispoable);
    }
    pushCleanup(disposable: Disposable) {
        this.ctx.subscriptions.push(disposable);
    }


    get subscriptions(): Disposable[] {
        return this.ctx.subscriptions;
    }

    get activeWgslEditor(): WgslEditor | undefined {
        const editor = vscode.window.activeTextEditor;
        return editor && isWgslEditor(editor)
            ? editor
            : undefined;
    }

}

export type Cmd = (...args: any[]) => unknown;