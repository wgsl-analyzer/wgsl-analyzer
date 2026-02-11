import { InlayHint } from "vscode";
import * as lc from "vscode-languageclient";

// wgsl-analyzer overrides

export const hover = new lc.RequestType<
	HoverParameters,
	(lc.Hover & { actions: CommandLinkGroup[] }) | null,
	void
>(lc.HoverRequest.method);

export type HoverParameters = { position: lc.Position | lc.Range } & Omit<
	lc.HoverParams,
	"position"
>;

export type CommandLink = {
	/**
	 * A tooltip for the command, when represented in the UI.
	 */
	tooltip?: string;
} & lc.Command;

export type CommandLinkGroup = {
	title?: string;
	commands: CommandLink[];
};

// wgsl-analyzer extensions

export const analyzerStatus = new lc.RequestType<AnalyzerStatusParameters, string, void>(
	"wgsl-analyzer/analyzerStatus",
);
export const cancelFlycheck = new lc.NotificationType0("wgsl-analyzer/cancelFlycheck");
export const clearFlycheck = new lc.NotificationType0("wgsl-analyzer/clearFlycheck");
export const memoryUsage = new lc.RequestType0<string, void>("wgsl-analyzer/memoryUsage");
export const openServerLogs = new lc.NotificationType0("wgsl-analyzer/openServerLogs");
export const relatedTests = new lc.RequestType<lc.TextDocumentPositionParams, TestInfo[], void>(
	"wgsl-analyzer/relatedTests",
);
export const reloadWorkspace = new lc.RequestType0<null, void>("wgsl-analyzer/reloadWorkspace");

export const runFlycheck = new lc.NotificationType<{
	textDocument: lc.TextDocumentIdentifier | null;
}>("wgsl-analyzer/runFlycheck");
export const viewSyntaxTree = new lc.RequestType<ViewSyntaxTreeParameters, string, void>(
	"wgsl-analyzer/viewSyntaxTree",
);
export const viewCrateGraph = new lc.RequestType<ViewCrateGraphParameters, string, void>(
	"wgsl-analyzer/viewCrateGraph",
);
export const viewFileText = new lc.RequestType<lc.TextDocumentIdentifier, string, void>(
	"wgsl-analyzer/viewFileText",
);
export const interpretFunction = new lc.RequestType<lc.TextDocumentPositionParams, string, void>(
	"wgsl-analyzer/interpretFunction",
);
export const viewItemTree = new lc.RequestType<ViewItemTreeParameters, string, void>(
	"wgsl-analyzer/viewItemTree",
);

export type DiscoverTestParameters = { testId?: string | undefined };
export type RunTestParameters = {
	include?: string[] | undefined;
	exclude?: string[] | undefined;
};
export type TestItem = {
	id: string;
	label: string;
	kind: "package" | "module" | "test";
	canResolveChildren: boolean;
	parent?: string | undefined;
	textDocument?: lc.TextDocumentIdentifier | undefined;
	range?: lc.Range | undefined;
	runnable?: Runnable | undefined;
};
export type DiscoverTestResults = {
	tests: TestItem[];
	scope: string[] | undefined;
	scopeFile: lc.TextDocumentIdentifier[] | undefined;
};
export type TestState =
	| { tag: "failed"; message: string }
	| { tag: "passed" }
	| { tag: "started" }
	| { tag: "enqueued" }
	| { tag: "skipped" };
export type ChangeTestStateParameters = { testId: string; state: TestState };
export const discoverTest = new lc.RequestType<DiscoverTestParameters, DiscoverTestResults, void>(
	"experimental/discoverTest",
);
export const discoveredTests = new lc.NotificationType<DiscoverTestResults>(
	"experimental/discoveredTests",
);
export const runTest = new lc.RequestType<RunTestParameters, void, void>("experimental/runTest");
export const abortRunTest = new lc.NotificationType0("experimental/abortRunTest");
export const endRunTest = new lc.NotificationType0("experimental/endRunTest");
export const appendOutputToRunTest = new lc.NotificationType<string>(
	"experimental/appendOutputToRunTest",
);
export const changeTestState = new lc.NotificationType<ChangeTestStateParameters>(
	"experimental/changeTestState",
);

export type AnalyzerStatusParameters = {
	textDocument?: lc.TextDocumentIdentifier;
};

export interface FetchDependencyListParameters { }

export interface FetchDependencyListResult {
	crates: {
		name?: string;
		version?: string;
		path: string;
	}[];
}

export const fetchDependencyList = new lc.RequestType<
	FetchDependencyListParameters,
	FetchDependencyListResult,
	void
>("wgsl-analyzer/fetchDependencyList");

export interface FetchDependencyGraphParameters { }

export interface FetchDependencyGraphResult {
	crates: {
		name: string;
		version: string;
		path: string;
	}[];
}

export const fetchDependencyGraph = new lc.RequestType<
	FetchDependencyGraphParameters,
	FetchDependencyGraphResult,
	void
>("wgsl-analyzer/fetchDependencyGraph");
export type TestInfo = { runnable: Runnable };
export type SyntaxTreeParameters = {
	textDocument: lc.TextDocumentIdentifier;
	range: lc.Range | null;
};
export type ViewSyntaxTreeParameters = {
	textDocument: lc.TextDocumentIdentifier;
};
export type ViewCrateGraphParameters = { full: boolean };
export type ViewItemTreeParameters = {
	textDocument: lc.TextDocumentIdentifier;
};

// experimental extensions

export const joinLines = new lc.RequestType<JoinLinesParameters, lc.TextEdit[], void>(
	"experimental/joinLines",
);

export const matchingBrace = new lc.RequestType<MatchingBraceParameters, lc.Position[], void>(
	"experimental/matchingBrace",
);

export const moveItem = new lc.RequestType<MoveItemParameters, lc.TextEdit[], void>(
	"experimental/moveItem",
);

export const onEnter = new lc.RequestType<lc.TextDocumentPositionParams, lc.TextEdit[], void>(
	"experimental/onEnter",
);
export const openWeslToml = new lc.RequestType<OpenWeslTomlParameters, lc.Location, void>(
	"experimental/openWeslToml",
);

export interface DocsUrls {
	local?: string;
	web?: string;
}

export const openDocs = new lc.RequestType<lc.TextDocumentPositionParams, DocsUrls, void>(
	"experimental/externalDocs",
);
export const parentModule = new lc.RequestType<
	lc.TextDocumentPositionParams,
	lc.LocationLink[] | null,
	void
>("experimental/parentModule");
export const childModules = new lc.RequestType<
	lc.TextDocumentPositionParams,
	lc.LocationLink[] | null,
	void
>("experimental/childModules");

export const runnables = new lc.RequestType<RunnablesParameters, Runnable[], void>(
	"experimental/runnables",
);

export const serverStatus = new lc.NotificationType<ServerStatusParameters>(
	"experimental/serverStatus",
);

export const ssr = new lc.RequestType<SsrParameters, lc.WorkspaceEdit, void>("experimental/ssr");

export const viewRecursiveMemoryLayout = new lc.RequestType<
	lc.TextDocumentPositionParams,
	RecursiveMemoryLayout | null,
	void
>("wgsl-analyzer/viewRecursiveMemoryLayout");

export type JoinLinesParameters = {
	textDocument: lc.TextDocumentIdentifier;
	ranges: lc.Range[];
};

export type MatchingBraceParameters = {
	textDocument: lc.TextDocumentIdentifier;
	positions: lc.Position[];
};

export type MoveItemParameters = {
	textDocument: lc.TextDocumentIdentifier;
	range: lc.Range;
	direction: Direction;
};

export type Direction = "Up" | "Down";

export type OpenWeslTomlParameters = {
	textDocument: lc.TextDocumentIdentifier;
};

export type Runnable = (RunnableCargo | RunnableShell) & {
	label: string;
	location?: lc.LocationLink;
};

type RunnableCargo = {
	kind: "cargo";
	args: WeslRunnableArgs;
};

type RunnableShell = {
	kind: "shell";
	args: ShellRunnableArgs;
};

export type CommonRunnableArgs = {
	/**
	 * Environment variables to set before running the command.
	 */
	environment?: Record<string, string>;
	/**
	 * The working directory to run the command in.
	 */
	cwd: string;
};

export type ShellRunnableArgs = CommonRunnableArgs & {
	kind: string;
	program: string;
	args: string[];
};

export type WeslRunnableArgs = CommonRunnableArgs & {
	/**
	 * The workspace root directory of the cargo project.
	 */
	workspaceRoot?: string;
	/**
	 * Arguments to pass to the executable, these will be passed to the command after a `--` argument.
	 */
	executableArguments: string[];
	/**
	 * Arguments to pass to wesl-rs.
	 */
	cargoArguments: string[];
	/**
	 * Command to execute instead of `cargo`.
	 */
	// This is supplied by the user via config. We could pull this through the client config in the
	// extension directly, but that would prevent us from honoring the wgsl-analyzer.toml for it.
	overrideCargo?: string;
};

export type RunnablesParameters = {
	textDocument: lc.TextDocumentIdentifier;
	position: lc.Position | null;
};

export type ServerStatusParameters = {
	health: "ok" | "warning" | "error";
	quiescent: boolean;
	message?: string;
};
export type SsrParameters = {
	query: string;
	parseOnly: boolean;
	textDocument: lc.TextDocumentIdentifier;
	position: lc.Position;
	selections: readonly lc.Range[];
};

export type RecursiveMemoryLayoutNode = {
	item_name: string;
	typename: string;
	size: number;
	alignment: number;
	offset: number;
	parent_index: number;
	children_start: number;
	children_len: number;
};

export type RecursiveMemoryLayout = {
	nodes: RecursiveMemoryLayoutNode[];
};

export const debugCommand = new lc.RequestType<DebugCommand, string, void>(
	"wgsl-analyzer/debugCommand",
);

export interface FullSourceParameters {
	textDocument: lc.TextDocumentIdentifier;
}
export const fullSource = new lc.RequestType<FullSourceParameters, string, void>(
	"wgsl-analyzer/fullSource",
);

export const requestConfiguration = new lc.RequestType<void, unknown, void>(
	"wgsl-analyzer/requestConfiguration",
);

export interface InlayHintsParameters {
	textDocument: lc.TextDocumentIdentifier;
	range: lc.Range;
}
export const inlayHints = new lc.RequestType<InlayHintsParameters, InlayHint[], void>(
	"experimental/inlayHints",
);

export interface ImportTextDocumentParameters {
	uri: lc.DocumentUri;
}
export const importTextDocument = new lc.RequestType<ImportTextDocumentParameters, unknown, void>(
	"wgsl-analyzer/importTextDocument",
);
export interface DebugCommand {
	textDocument: lc.TextDocumentIdentifier;
	position: lc.Position;
}
