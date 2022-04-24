import { Cmd, Ctx } from "./ctx";
import * as vscode from "vscode";
import { isWgslDocument, isWgslEditor, sleep } from "./util";
import * as lsp_ext from "./lsp_ext";

export function syntaxTree(ctx: Ctx): Cmd {
  const tdcp = new (class implements vscode.TextDocumentContentProvider {
    readonly uri = vscode.Uri.parse("wgsl-analyzer://syntaxtree/tree.wgslst");
    readonly eventEmitter = new vscode.EventEmitter<vscode.Uri>();
    constructor() {
      vscode.workspace.onDidChangeTextDocument(this.onDidChangeTextDocument, this, ctx.subscriptions);
      vscode.window.onDidChangeActiveTextEditor(this.onDidChangeActiveTextEditor, this, ctx.subscriptions);
    }

    private onDidChangeTextDocument(event: vscode.TextDocumentChangeEvent) {
      if (isWgslDocument(event.document)) {
        // We need to order this after language server updates, but there's no API for that.
        // Hence, good old sleep().
        void sleep(10).then(() => this.eventEmitter.fire(this.uri));
      }
    }
    private onDidChangeActiveTextEditor(editor: vscode.TextEditor | undefined) {
      if (editor && isWgslEditor(editor)) {
        this.eventEmitter.fire(this.uri);
      }
    }

    provideTextDocumentContent(uri: vscode.Uri, ct: vscode.CancellationToken): vscode.ProviderResult<string> {
      const wgslEditor = ctx.activeWgslEditor;
      if (!wgslEditor) return "";
      let selection = vscode.window.activeTextEditor.selection;
      const params = {
        textDocument: {
          uri: wgslEditor.document.uri.toString(),
        },
        range: { start: selection.start, end: selection.end },
      };
      return ctx.client.sendRequest(lsp_ext.syntaxTree, params, ct);
    }

    get onDidChange(): vscode.Event<vscode.Uri> {
      return this.eventEmitter.event;
    }
  })();

  ctx.pushCleanup(vscode.workspace.registerTextDocumentContentProvider("wgsl-analyzer", tdcp));
  ctx.pushCleanup(
    vscode.languages.setLanguageConfiguration("wgsl_syntax_tree", {
      brackets: [["[", ")"]],
    })
  );

  return async () => {
    const uri = tdcp.uri;

    let document = await vscode.workspace.openTextDocument(uri);

    tdcp.eventEmitter.fire(uri);

    await vscode.window.showTextDocument(document, {
      viewColumn: vscode.ViewColumn.Two,
      preserveFocus: true,
    });
  };
}

export function debugCommand(ctx: Ctx): Cmd {
  return () => {
    const wgslEditor = ctx.activeWgslEditor;
    if (!wgslEditor) return;

    let position = wgslEditor.selection.active;
    const params = { textDocument: { uri: wgslEditor.document.uri.toString() }, position };
    ctx.client.sendRequest(lsp_ext.debugCommand, params);
  };
}

export function showFullSource(ctx: Ctx): Cmd {
  const tdcp = new (class implements vscode.TextDocumentContentProvider {
    readonly uri = vscode.Uri.parse("wgsl-analyzer:///fullSource.wgsl");
    readonly eventEmitter = new vscode.EventEmitter<vscode.Uri>();
    constructor() {
      vscode.workspace.onDidChangeTextDocument(this.onDidChangeTextDocument, this, ctx.subscriptions);
      vscode.window.onDidChangeActiveTextEditor(this.onDidChangeActiveTextEditor, this, ctx.subscriptions);
    }

    private onDidChangeTextDocument(event: vscode.TextDocumentChangeEvent) {
      if (isWgslDocument(event.document)) {
        // We need to order this after language server updates, but there's no API for that.
        // Hence, good old sleep().
        void sleep(10).then(() => this.eventEmitter.fire(this.uri));
      }
    }
    private onDidChangeActiveTextEditor(editor: vscode.TextEditor | undefined) {
      if (editor && isWgslEditor(editor)) {
        this.eventEmitter.fire(this.uri);
      }
    }

    provideTextDocumentContent(uri: vscode.Uri, ct: vscode.CancellationToken): vscode.ProviderResult<string> {
      const wgslEditor = ctx.activeWgslEditor;
      if (!wgslEditor) return "";

      const params = { textDocument: { uri: wgslEditor.document.uri.toString() } };
      return ctx.client.sendRequest(lsp_ext.fullSource, params, ct);
    }

    get onDidChange(): vscode.Event<vscode.Uri> {
      return this.eventEmitter.event;
    }
  })();

  ctx.pushCleanup(vscode.workspace.registerTextDocumentContentProvider("wgsl-analyzer", tdcp));

  return async () => {
    const wgslEditor = ctx.activeWgslEditor;
    if (!wgslEditor) return;

    const uri = tdcp.uri;
    const document = await vscode.workspace.openTextDocument(uri);
    vscode.languages.setTextDocumentLanguage(document, "wgsl");

    await vscode.window.showTextDocument(document, {
      viewColumn: vscode.ViewColumn.Two,
      preserveFocus: true,
    });
  };
}
