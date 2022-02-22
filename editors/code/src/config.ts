import * as vscode from "vscode";

export class Config {
    public static readonly rootSection = "wgsl-analyzer";

    constructor(ctx: vscode.ExtensionContext) {
    }

    private get cfg(): vscode.WorkspaceConfiguration {
        return vscode.workspace.getConfiguration(Config.rootSection);
    }

    private get<T>(path: string): T {
        return this.cfg.get<T>(path)!;
    }

    get showTypeErrors(): boolean {
        return this.get<boolean>("showTypeErrors");
    }

    get serverPath(): string | null {
        return this.get<string | null>("server.path");
    }

    get customImports(): Record<string, string> {
        return this.get("customImports");
    }

    get shaderDefs(): [string] {
        return this.get<[string]>("preprocessor.shaderDefs");
    }

    get trace(): { extension: boolean, server: boolean; } {
        return this.get("trace");
    }
}