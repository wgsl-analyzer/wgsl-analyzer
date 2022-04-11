import * as vscode from "vscode";

export interface TraceConfig {
    extension: boolean,
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
    nagaParsing: boolean;
    nagaValidation: boolean;
}

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

    get diagnostics(): DiagnosticsConfig {
        return this.get<DiagnosticsConfig>("diagnostics");
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

    get trace(): TraceConfig {
        return this.get("trace");
    }
    get inlayHints(): InlayHintsConfig {
        return this.get("inlayHints");
    }
}
