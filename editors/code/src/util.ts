import * as lc from "vscode-languageclient/node";
import * as vscode from "vscode";

export function sleep(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

export type WgslDocument = vscode.TextDocument & { languageId: "wgsl"; };
export type WgslEditor = vscode.TextEditor & { document: WgslDocument; };

export function isWgslDocument(document: vscode.TextDocument): document is WgslDocument {
    return document.languageId === "wgsl";
}

export function isWgslEditor(editor: vscode.TextEditor): editor is WgslEditor {
    return isWgslDocument(editor.document);
}

export async function sendRequestWithRetry<TParam, TRet>(
    client: lc.LanguageClient,
    reqType: lc.RequestType<TParam, TRet, unknown>,
    param: TParam,
    token?: vscode.CancellationToken,
): Promise<TRet> {
    // The sequence is `10 * (2 ** (2 * n))` where n is 1, 2, 3...
    for (const delay of [40, 160, 640, 2560, 10240, null]) {
        try {
            return await (token
                ? client.sendRequest(reqType, param, token)
                : client.sendRequest(reqType, param)
            );
        } catch (error) {
            if (delay === null) {
                vscode.window.showWarningMessage("WGSL-Analyzer: LSP request timed out: " + JSON.stringify({ method: reqType.method, param, error }));
                throw error;
            }
            if (error.code === lc.LSPErrorCodes.RequestCancelled) {
                throw error;
            }

            if (error.code !== lc.LSPErrorCodes.ContentModified) {
                vscode.window.showWarningMessage("WGSL-Analyzer: LSP request failed" + JSON.stringify({ method: reqType.method, param, error }));
                throw error;
            }
            await sleep(delay);
        }
    }
    throw 'unreachable';
}