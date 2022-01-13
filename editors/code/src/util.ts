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