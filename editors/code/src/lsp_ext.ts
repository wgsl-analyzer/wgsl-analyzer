import * as lc from "vscode-languageclient";
import { InlayHint } from "vscode";

export interface SyntaxTreeParams {
    textDocument: lc.TextDocumentIdentifier;
    range: lc.Range | null;
}
export const syntaxTree = new lc.RequestType<SyntaxTreeParams, string, void>("wgsl-analyzer/syntaxTree");

export interface DebugCommand {
    textDocument: lc.TextDocumentIdentifier;
    position: lc.Position;
}
export const debugCommand = new lc.RequestType<DebugCommand, string, void>("wgsl-analyzer/debugCommand");


export interface FullSourceParams {
    textDocument: lc.TextDocumentIdentifier;
}
export const fullSource = new lc.RequestType<FullSourceParams, string, void>("wgsl-analyzer/fullSource");

export const requestConfiguration = new lc.RequestType<void, unknown, void>("wgsl-analyzer/requestConfiguration");


export interface InlayHintsParams {
    textDocument: lc.TextDocumentIdentifier;
    range: lc.Range;
}
export const inlayHints = new lc.RequestType<InlayHintsParams, InlayHint[], void>("experimental/inlayHints");