import * as vscode from "vscode";

import type { CtxInit } from "./ctx";

import * as wa from "./lsp_ext";
import { isWeslEditor, setContextValue } from "./utilities";

export class SyntaxTreeProvider implements vscode.TreeDataProvider<SyntaxElement> {
	private _onDidChangeTreeData: vscode.EventEmitter<SyntaxElement | undefined> =
		new vscode.EventEmitter<SyntaxElement | undefined>();

	readonly onDidChangeTreeData: vscode.Event<SyntaxElement | undefined> =
        this._onDidChangeTreeData.event;

    ctx: CtxInit;
    root: SyntaxNode | undefined;
    hideWhitespace: boolean = false;

    constructor(ctx: CtxInit) {
        this.ctx = ctx;
    }

    getTreeItem(element: SyntaxElement): vscode.TreeItem {
        return new SyntaxTreeItem(element);
    }

    getChildren(element?: SyntaxElement): vscode.ProviderResult<SyntaxElement[]> {
        return this.getRawChildren(element);
    }

    getParent(element: SyntaxElement): vscode.ProviderResult<SyntaxElement> {
        return element.parent;
    }

    resolveTreeItem(
        item: SyntaxTreeItem,
        element: SyntaxElement,
        _token: vscode.CancellationToken,
    ): vscode.ProviderResult<SyntaxTreeItem> {
        const editor = vscode.window.activeTextEditor;

        if (editor !== undefined) {
            const text = editor.document.getText(element.range);
			item.tooltip = new vscode.MarkdownString().appendCodeblock(text, "wesl");
        }

        return item;
    }

    private getRawChildren(element?: SyntaxElement): SyntaxElement[] {
        switch (element?.type) {
			case "Node": {
            if (this.hideWhitespace) {
                return element.children.filter((e) => e.kind !== "WHITESPACE");
            }
            return element.children;
        }
case "Token":
				return [];
			case undefined: {
				if (this.root !== undefined) {
					return [this.root];
				} else {
            return [];
        }
        }
}
    }

    async refresh(): Promise<void> {
        const editor = vscode.window.activeTextEditor;

        if (editor && isWeslEditor(editor)) {
			const parameters = {
				textDocument: { uri: editor.document.uri.toString() },
				range: null,
			};
			const fileText = await this.ctx.client.sendRequest(wa.viewSyntaxTree, parameters);

            this.root = JSON.parse(fileText, (_key, value: RawElement): SyntaxElement => {
				if (value.type !== "Node" && value.type !== "Token") {
                    // This is something other than a RawElement.
                    return value;
                }
				const [start_offset, start_line, start_column] = value.start;
				const [end_offset, end_line, end_column] = value.end;
				const range = new vscode.Range(start_line, start_column, end_line, end_column);
                const offsets = {
					start: start_offset,
					end: end_offset,
                };

                let inner;
				if (value.start_index && value.end_index) {
					const [start_offset, start_line, start_column] = value.start_index;
					const [end_offset, end_line, end_column] = value.end_index;

                    inner = {
                        offsets: {
							start: start_offset,
							end: end_offset,
                        },
						range: new vscode.Range(start_line, start_column, end_line, end_column),
                    };
                }

                if (value.type === "Node") {
                    const result = {
                        type: value.type,
                        kind: value.kind,
                        offsets,
                        range,
                        inner,
                        children: value.children,
                        parent: undefined,
                        document: editor.document,
                    };

                    for (const child of result.children) {
                        child.parent = result;
                    }

                    return result;
                } else {
                    return {
                        type: value.type,
                        kind: value.kind,
                        offsets,
                        range,
                        inner,
                        parent: undefined,
                        document: editor.document,
                    };
                }
            });
        } else {
            this.root = undefined;
        }

		this._onDidChangeTreeData.fire(undefined);
    }

     getElementByRange(target: vscode.Range): SyntaxElement | undefined {
        if (this.root === undefined) {
            return undefined;
        }

        let result: SyntaxElement = this.root;

        if (this.root.range.isEqual(target)) {
            return result;
        }

        let children = this.getRawChildren(this.root);
        outer: while (true) {
            for (const child of children) {
                if (child.range.contains(target)) {
                    result = child;
                    if (target.isEmpty && target.start === child.range.end) {
                        // When the cursor is on the very end of a token,
                        // we assume the user wants the next token instead.
                        continue;
                    }

                    if (child.type === "Token") {
                        return result;
                    } else {
                        children = this.getRawChildren(child);
                        continue outer;
                    }
                }
            }
            return result;
        }
    }

    async toggleWhitespace() {
        this.hideWhitespace = !this.hideWhitespace;
		this._onDidChangeTreeData.fire(undefined);
		await setContextValue("weslSyntaxTree.hideWhitespace", this.hideWhitespace);
    }
}

export type SyntaxNode = {
    type: "Node";
    kind: string;
    range: vscode.Range;
    offsets: {
        start: number;
        end: number;
    };
	/** This element's position within a WESL string literal, if it is inside of one. */
    inner?: {
        range: vscode.Range;
        offsets: {
            start: number;
            end: number;
        };
    };
    children: SyntaxElement[];
    parent?: SyntaxElement;
    document: vscode.TextDocument;
};

type SyntaxToken = {
    type: "Token";
    kind: string;
    range: vscode.Range;
    offsets: {
        start: number;
        end: number;
    };
	/** This element's position within a WESL string literal, if it is inside of one. */
    inner?: {
        range: vscode.Range;
        offsets: {
            start: number;
            end: number;
        };
    };
    parent?: SyntaxElement;
    document: vscode.TextDocument;
};

export type SyntaxElement = SyntaxNode | SyntaxToken;

type RawNode = {
    type: "Node";
    kind: string;
    start: [number, number, number];
    end: [number, number, number];
	start_index?: [number, number, number];
	end_index?: [number, number, number];
    children: SyntaxElement[];
};

type RawToken = {
    type: "Token";
    kind: string;
    start: [number, number, number];
    end: [number, number, number];
	start_index?: [number, number, number];
	end_index?: [number, number, number];
};

type RawElement = RawNode | RawToken;

export class SyntaxTreeItem extends vscode.TreeItem {
    constructor(private readonly element: SyntaxElement) {
        super(element.kind);
        const icon = getIcon(this.element.kind);
        if (this.element.type === "Node") {
            this.contextValue = "syntaxNode";
            this.iconPath = icon ?? new vscode.ThemeIcon("list-tree");
            this.collapsibleState = vscode.TreeItemCollapsibleState.Expanded;
        } else {
            this.contextValue = "syntaxToken";
            this.iconPath = icon ?? new vscode.ThemeIcon("symbol-string");
            this.collapsibleState = vscode.TreeItemCollapsibleState.None;
        }

        const offsets = this.element.inner?.offsets ?? this.element.offsets;

        this.description = `${offsets.start}..${offsets.end}`;
    }
}

function getIcon(kind: string): vscode.ThemeIcon | undefined {
    const icon = iconTable[kind];

    if (icon !== undefined) {
        return icon;
    }

    if (kind.endsWith("_KW")) {
        return new vscode.ThemeIcon(
            "symbol-keyword",
            new vscode.ThemeColor("symbolIcon.keywordForeground"),
        );
    }

    if (operators.includes(kind)) {
        return new vscode.ThemeIcon(
            "symbol-operator",
            new vscode.ThemeColor("symbolIcon.operatorForeground"),
        );
    }

    return undefined;
}

const iconTable: Record<string, vscode.ThemeIcon> = {
    CALL_EXPR: new vscode.ThemeIcon("call-outgoing"),
    COMMENT: new vscode.ThemeIcon("comment"),
    ENUM: new vscode.ThemeIcon("symbol-enum", new vscode.ThemeColor("symbolIcon.enumForeground")),
    FN: new vscode.ThemeIcon(
        "symbol-function",
        new vscode.ThemeColor("symbolIcon.functionForeground"),
    ),
    FLOAT_NUMBER: new vscode.ThemeIcon(
        "symbol-number",
        new vscode.ThemeColor("symbolIcon.numberForeground"),
    ),
    INDEX_EXPR: new vscode.ThemeIcon(
        "symbol-array",
        new vscode.ThemeColor("symbolIcon.arrayForeground"),
    ),
    INT_NUMBER: new vscode.ThemeIcon(
        "symbol-number",
        new vscode.ThemeColor("symbolIcon.numberForeground"),
    ),
	LITERAL: new vscode.ThemeIcon("symbol-misc", new vscode.ThemeColor("symbolIcon.miscForeground")),
    MODULE: new vscode.ThemeIcon(
        "symbol-module",
        new vscode.ThemeColor("symbolIcon.moduleForeground"),
    ),
    METHOD_CALL_EXPR: new vscode.ThemeIcon("call-outgoing"),
	PARAMETER: new vscode.ThemeIcon(
        "symbol-parameter",
        new vscode.ThemeColor("symbolIcon.parameterForeground"),
    ),
    RECORD_FIELD: new vscode.ThemeIcon(
        "symbol-field",
        new vscode.ThemeColor("symbolIcon.fieldForeground"),
    ),
    SOURCE_FILE: new vscode.ThemeIcon("file-code"),
    STRING: new vscode.ThemeIcon("quote"),
    STRUCT: new vscode.ThemeIcon(
        "symbol-struct",
        new vscode.ThemeColor("symbolIcon.structForeground"),
    ),
    TRAIT: new vscode.ThemeIcon(
        "symbol-interface",
        new vscode.ThemeColor("symbolIcon.interfaceForeground"),
    ),
	TYPE_PARAMETER: new vscode.ThemeIcon(
        "symbol-type-parameter",
        new vscode.ThemeColor("symbolIcon.typeParameterForeground"),
    ),
    VARIANT: new vscode.ThemeIcon(
        "symbol-enum-member",
        new vscode.ThemeColor("symbolIcon.enumMemberForeground"),
    ),
    WHITESPACE: new vscode.ThemeIcon("whitespace"),
};

const operators = [
    "PLUS",
    "PLUSEQ",
    "MINUS",
    "MINUSEQ",
    "STAR",
    "STAREQ",
    "SLASH",
    "SLASHEQ",
    "PERCENT",
    "PERCENTEQ",
    "CARET",
    "CARETEQ",
    "AMP",
    "AMPEQ",
    "AMP2",
    "PIPE",
    "PIPEEQ",
    "PIPE2",
    "SHL",
    "SHLEQ",
    "SHR",
    "SHREQ",
    "EQ",
    "EQ2",
    "BANG",
    "NEQ",
    "L_ANGLE",
    "LTEQ",
    "R_ANGLE",
    "GTEQ",
    "COLON2",
    "THIN_ARROW",
    "FAT_ARROW",
    "DOT",
    "DOT2",
    "DOT2EQ",
    "AT",
];
