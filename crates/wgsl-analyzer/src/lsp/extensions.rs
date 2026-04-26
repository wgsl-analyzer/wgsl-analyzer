//! wgsl-analyzer extensions to the LSP.

// Note:
// When adding new resolve payloads, add a #[serde(default)] on boolean fields.
// Some clients might strip `false` values from the JSON payload due to their
// reserialization logic turning `false` into `null`.
// This will cause them to be omitted in the resolve request.
//
// See: https://github.com/rust-lang/rust-analyzer/issues/18767

#![expect(clippy::disallowed_types, reason = "serde compatibility")]

use std::{borrow::Cow, ops};

use camino::Utf8PathBuf;
use lsp_types::{
    ChangeAnnotation, ChangeAnnotationIdentifier, CodeActionKind, CodeActionParams, Command,
    DefinitionResponse, DocumentOnTypeFormattingParams, Hover, HoverRequest as LspHoverRequest,
    ImplementationParams, InsertTextFormat, LocationLink, LspNotificationMethod, LspRequestMethod,
    MessageDirection, Notification, OptionalVersionedTextDocumentIdentifier, PartialResultParams,
    Position, Range, Request, ResourceOperation, TextDocumentIdentifier,
    TextDocumentPositionParams, TextEdit, Uri, WorkDoneProgressParams, WorkspaceEdit,
    WorkspaceSymbolResponse,
};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

pub enum FullSourceRequest {}

impl Request for FullSourceRequest {
    type Params = FullSourceParameters;
    type Result = String;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspRequestMethod = LspRequestMethod::new("wgsl-analyzer/fullSource");
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FullSourceParameters {
    pub text_document: TextDocumentIdentifier,
}

pub enum AnalyzerStatusRequest {}

impl Request for AnalyzerStatusRequest {
    type Params = AnalyzerStatusParameters;
    type Result = String;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspRequestMethod = LspRequestMethod::new("wgsl-analyzer/analyzerStatus");
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzerStatusParameters {
    pub text_document: Option<TextDocumentIdentifier>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PackageInfoResult {
    pub name: Option<String>,
    pub version: Option<String>,
    pub path: Uri,
}

pub enum FetchDependencyListRequest {}

impl Request for FetchDependencyListRequest {
    type Params = FetchDependencyListParameters;
    type Result = FetchDependencyListResult;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspRequestMethod = LspRequestMethod::new("wgsl-analyzer/fetchDependencyList");
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FetchDependencyListParameters;

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct FetchDependencyListResult {
    pub packages: Vec<PackageInfoResult>,
}

pub enum MemoryUsageRequest {}

impl Request for MemoryUsageRequest {
    type Params = ();
    type Result = String;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspRequestMethod = LspRequestMethod::new("wgsl-analyzer/memoryUsage");
}

pub enum ReloadWorkspaceRequest {}

impl Request for ReloadWorkspaceRequest {
    type Params = ();
    type Result = ();

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspRequestMethod = LspRequestMethod::new("wgsl-analyzer/reloadWorkspace");
}

pub enum ViewSyntaxTreeRequest {}

impl Request for ViewSyntaxTreeRequest {
    type Params = ViewSyntaxTreeParameters;
    type Result = String;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspRequestMethod = LspRequestMethod::new("wgsl-analyzer/viewSyntaxTree");
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ViewSyntaxTreeParameters {
    pub text_document: TextDocumentIdentifier,
}

pub enum ViewWgslRequest {}

impl Request for ViewWgslRequest {
    type Params = TextDocumentPositionParams;
    type Result = String;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspRequestMethod = LspRequestMethod::new("wgsl-analyzer/viewWgsl");
}

pub enum ViewSpirvRequest {}

impl Request for ViewSpirvRequest {
    type Params = TextDocumentPositionParams;
    type Result = String;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspRequestMethod = LspRequestMethod::new("wgsl-analyzer/viewSpirv");
}

pub enum InterpretFunctionRequest {}

impl Request for InterpretFunctionRequest {
    type Params = TextDocumentPositionParams;
    type Result = String;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspRequestMethod = LspRequestMethod::new("wgsl-analyzer/interpretFunction");
}

pub enum ViewFileTextRequest {}

impl Request for ViewFileTextRequest {
    type Params = TextDocumentIdentifier;
    type Result = String;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspRequestMethod = LspRequestMethod::new("wgsl-analyzer/viewFileText");
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ViewPackageGraphParameters {
    /// Include *all* packages, not just packages in the workspace.
    pub full: bool,
}

pub enum ViewPackageGraphRequest {}

impl Request for ViewPackageGraphRequest {
    type Params = ViewPackageGraphParameters;
    type Result = String;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspRequestMethod = LspRequestMethod::new("wgsl-analyzer/viewPackageGraph");
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ViewItemTreeParameters {
    pub text_document: TextDocumentIdentifier,
}

pub enum ViewItemTreeRequest {}

impl Request for ViewItemTreeRequest {
    type Params = ViewItemTreeParameters;
    type Result = String;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspRequestMethod = LspRequestMethod::new("wgsl-analyzer/viewItemTree");
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscoverTestParameters {
    pub test_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TestItemKind {
    Package,
    Module,
    Test,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TestItem {
    pub id: String,
    pub label: String,
    pub kind: TestItemKind,
    pub can_resolve_children: bool,
    pub parent: Option<String>,
    pub text_document: Option<TextDocumentIdentifier>,
    pub range: Option<Range>,
    pub runnable: Option<Runnable>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct DiscoverTestResults {
    pub tests: Vec<TestItem>,
    pub scope: Option<Vec<String>>,
    pub scope_file: Option<Vec<TextDocumentIdentifier>>,
}

pub enum DiscoverTestRequest {}

impl Request for DiscoverTestRequest {
    type Params = DiscoverTestParameters;
    type Result = DiscoverTestResults;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspRequestMethod = LspRequestMethod::new("experimental/discoverTest");
}

pub enum DiscoveredTestsNotification {}

impl Notification for DiscoveredTestsNotification {
    type Params = DiscoverTestResults;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspNotificationMethod =
        LspNotificationMethod::new("experimental/discoveredTests");
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RunTestParameters {
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}

pub enum RunTestRequest {}

impl Request for RunTestRequest {
    type Params = RunTestParameters;
    type Result = ();

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspRequestMethod = LspRequestMethod::new("experimental/runTest");
}

pub enum EndRunTestNotification {}

impl Notification for EndRunTestNotification {
    type Params = ();

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspNotificationMethod = LspNotificationMethod::new("experimental/endRunTest");
}

pub enum AppendOutputToRunTestNotification {}

impl Notification for AppendOutputToRunTestNotification {
    type Params = String;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspNotificationMethod =
        LspNotificationMethod::new("experimental/appendOutputToRunTest");
}

pub enum AbortRunTestNotification {}

impl Notification for AbortRunTestNotification {
    type Params = ();

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspNotificationMethod = LspNotificationMethod::new("experimental/abortRunTest");
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase", tag = "tag")]
pub enum TestState {
    Passed,
    Failed { message: String },
    Skipped,
    Started,
    Enqueued,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChangeTestStateParameters {
    pub test_id: String,
    pub state: TestState,
}

pub enum ChangeTestStateNotification {}

impl Notification for ChangeTestStateNotification {
    type Params = ChangeTestStateParameters;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspNotificationMethod =
        LspNotificationMethod::new("experimental/changeTestState");
}

pub enum ViewRecursiveMemoryLayoutRequest {}

impl Request for ViewRecursiveMemoryLayoutRequest {
    type Params = TextDocumentPositionParams;
    type Result = Option<RecursiveMemoryLayout>;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspRequestMethod =
        LspRequestMethod::new("wgsl-analyzer/viewRecursiveMemoryLayout");
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecursiveMemoryLayout {
    pub nodes: Vec<MemoryLayoutNode>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MemoryLayoutNode {
    pub item_name: String,
    pub typename: String,
    pub size: u64,
    pub offset: u64,
    pub alignment: u64,
    pub parent_index: i64,
    pub children_start: i64,
    pub children_length: u64,
}

pub enum CancelFlycheckNotification {}

impl Notification for CancelFlycheckNotification {
    type Params = ();

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspNotificationMethod =
        LspNotificationMethod::new("wgsl-analyzer/cancelFlycheck");
}

pub enum RunFlycheckNotification {}

impl Notification for RunFlycheckNotification {
    type Params = RunFlycheckParameters;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspNotificationMethod = LspNotificationMethod::new("wgsl-analyzer/runFlycheck");
}

pub enum ClearFlycheckNotification {}

impl Notification for ClearFlycheckNotification {
    type Params = ();

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspNotificationMethod = LspNotificationMethod::new("wgsl-analyzer/clearFlycheck");
}

pub enum OpenServerLogsNotification {}

impl Notification for OpenServerLogsNotification {
    type Params = ();

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspNotificationMethod =
        LspNotificationMethod::new("wgsl-analyzer/openServerLogs");
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RunFlycheckParameters {
    pub text_document: Option<TextDocumentIdentifier>,
}

pub enum MatchingBraceRequest {}

impl Request for MatchingBraceRequest {
    type Params = MatchingBraceParameters;
    type Result = Vec<Position>;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspRequestMethod = LspRequestMethod::new("experimental/matchingBrace");
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatchingBraceParameters {
    pub text_document: TextDocumentIdentifier,
    pub positions: Vec<Position>,
}

pub enum ParentModuleRequest {}

impl Request for ParentModuleRequest {
    type Params = TextDocumentPositionParams;
    type Result = Option<DefinitionResponse>;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspRequestMethod = LspRequestMethod::new("experimental/parentModule");
}

pub enum JoinLinesRequest {}

impl Request for JoinLinesRequest {
    type Params = JoinLinesParameters;
    type Result = Vec<TextEdit>;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspRequestMethod = LspRequestMethod::new("experimental/joinLines");
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JoinLinesParameters {
    pub text_document: TextDocumentIdentifier,
    pub ranges: Vec<Range>,
}

pub enum OnEnterRequest {}

impl Request for OnEnterRequest {
    type Params = TextDocumentPositionParams;
    type Result = Option<Vec<SnippetTextEdit>>;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspRequestMethod = LspRequestMethod::new("experimental/onEnter");
}

pub enum RunnablesRequest {}

impl Request for RunnablesRequest {
    type Params = RunnablesParameters;
    type Result = Vec<Runnable>;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspRequestMethod = LspRequestMethod::new("experimental/runnables");
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RunnablesParameters {
    pub text_document: TextDocumentIdentifier,
    pub position: Option<Position>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Runnable {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationLink>,
    pub kind: RunnableKind,
    pub arguments: RunnableArguments,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum RunnableArguments {
    Cargo(CargoRunnableArguments),
    Shell(ShellRunnableArguments),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum RunnableKind {
    Cargo,
    Shell,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
// TODO: Clean up this cruft
pub struct CargoRunnableArguments {
    #[serde(skip_serializing_if = "FxHashMap::is_empty")]
    pub environment: FxHashMap<String, String>,
    pub cwd: Utf8PathBuf,
    /// Command to be executed instead of cargo.
    pub override_cargo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_root: Option<Utf8PathBuf>,
    // command, --package and --lib stuff
    pub cargo_arguments: Vec<String>,
    // stuff after --
    pub executable_arguments: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShellRunnableArguments {
    #[serde(skip_serializing_if = "FxHashMap::is_empty")]
    pub environment: FxHashMap<String, String>,
    pub cwd: Utf8PathBuf,
    pub program: String,
    pub arguments: Vec<String>,
}

pub enum RelatedTestsRequest {}

impl Request for RelatedTestsRequest {
    type Params = TextDocumentPositionParams;
    type Result = Vec<TestInfo>;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspRequestMethod = LspRequestMethod::new("wgsl-analyzer/relatedTests");
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TestInfo {
    pub runnable: Runnable,
}

pub enum SsrRequest {}

impl Request for SsrRequest {
    type Params = SsrParameters;
    type Result = WorkspaceEdit;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspRequestMethod = LspRequestMethod::new("experimental/ssr");
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SsrParameters {
    pub query: String,
    pub parse_only: bool,

    /// File position where SSR was invoked. Paths in `query` will be resolved relative to this
    /// position.
    #[serde(flatten)]
    pub position: TextDocumentPositionParams,

    /// Current selections. Search/replace will be restricted to these if non-empty.
    pub selections: Vec<Range>,
}

pub enum ServerStatusNotification {}

impl Notification for ServerStatusNotification {
    type Params = ServerStatusParameters;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspNotificationMethod = LspNotificationMethod::new("experimental/serverStatus");
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerStatusParameters {
    pub health: Health,
    pub quiescent: bool,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Health {
    Ok,
    Warning,
    Error,
}

impl ops::BitOrAssign for Health {
    fn bitor_assign(
        &mut self,
        rhs: Self,
    ) {
        *self = match (*self, rhs) {
            (Self::Error, _) | (_, Self::Error) => Self::Error,
            (Self::Warning, _) | (_, Self::Warning) => Self::Warning,
            _ => Self::Ok,
        }
    }
}

pub enum CodeActionRequest {}

impl Request for CodeActionRequest {
    type Params = CodeActionParams;
    type Result = Option<Vec<CodeAction>>;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspRequestMethod = LspRequestMethod::new("textDocument/codeAction");
}

pub enum CodeActionResolveRequest {}

impl Request for CodeActionResolveRequest {
    type Params = CodeAction;
    type Result = CodeAction;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspRequestMethod = LspRequestMethod::new("codeAction/resolve");
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeAction {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<CodeActionKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Command>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit: Option<SnippetWorkspaceEdit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_preferred: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CodeActionData>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeActionData {
    pub code_action_parameters: CodeActionParams,
    pub id: String,
    pub version: Option<i32>,
}

#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnippetWorkspaceEdit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes: Option<FxHashMap<Uri, Vec<TextEdit>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_changes: Option<Vec<SnippetDocumentChangeOperation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_annotations:
        Option<std::collections::HashMap<ChangeAnnotationIdentifier, ChangeAnnotation>>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(untagged, rename_all = "lowercase")]
pub enum SnippetDocumentChangeOperation {
    Op(ResourceOperation),
    Edit(SnippetTextDocumentEdit),
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnippetTextDocumentEdit {
    pub text_document: OptionalVersionedTextDocumentIdentifier,
    pub edits: Vec<SnippetTextEdit>,
}

#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnippetTextEdit {
    pub range: Range,
    pub new_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_text_format: Option<InsertTextFormat>,
    /// The annotation id, if this is an annotated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_id: Option<ChangeAnnotationIdentifier>,
}

pub enum HoverRequest {}

impl Request for HoverRequest {
    type Params = HoverParameters;
    type Result = Option<HoverResult>;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspRequestMethod = LspHoverRequest::METHOD;
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HoverParameters {
    pub text_document: TextDocumentIdentifier,
    pub position: PositionOrRange,

    #[serde(flatten)]
    pub work_done_progress_parameters: WorkDoneProgressParams,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PositionOrRange {
    Position(Position),
    Range(Range),
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct HoverResult {
    #[serde(flatten)]
    pub hover: Hover,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<CommandLinkGroup>,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
pub struct CommandLinkGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub commands: Vec<CommandLink>,
}

// LSP v3.15 Command does not have a `tooltip` field, vscode supports one.
#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
pub struct CommandLink {
    #[serde(flatten)]
    pub command: Command,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<String>,
}

pub enum ExternalDocsRequest {}

impl Request for ExternalDocsRequest {
    type Params = TextDocumentPositionParams;
    type Result = ExternalDocsResponse;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspRequestMethod = LspRequestMethod::new("experimental/externalDocs");
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ExternalDocsResponse {
    Simple(Option<Uri>),
    WithLocal(ExternalDocsPair),
}

impl Default for ExternalDocsResponse {
    fn default() -> Self {
        Self::Simple(None)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExternalDocsPair {
    pub web: Option<Uri>,
    pub local: Option<Uri>,
}

pub enum OpenCargoTomlRequest {}

impl Request for OpenCargoTomlRequest {
    type Params = OpenCargoTomlParameters;
    type Result = Option<DefinitionResponse>;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspRequestMethod = LspRequestMethod::new("experimental/openCargoToml");
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OpenCargoTomlParameters {
    pub text_document: TextDocumentIdentifier,
}

/// Information about `CodeLens`, that is to be resolved.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeLensResolveData {
    pub version: i32,
    pub kind: CodeLensResolveDataKind,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CodeLensResolveDataKind {
    Impls(ImplementationParams),
    References(TextDocumentPositionParams),
}

pub enum MoveItemRequest {}

impl Request for MoveItemRequest {
    type Params = MoveItemParameters;
    type Result = Vec<SnippetTextEdit>;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspRequestMethod = LspRequestMethod::new("experimental/moveItem");
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MoveItemParameters {
    pub direction: MoveItemDirection,
    pub text_document: TextDocumentIdentifier,
    pub range: Range,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MoveItemDirection {
    Up,
    Down,
}

#[derive(Debug)]
pub enum WorkspaceSymbolRequest {}

impl Request for WorkspaceSymbolRequest {
    type Params = WorkspaceSymbolParameters;
    type Result = Option<WorkspaceSymbolResponse>;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspRequestMethod = LspRequestMethod::new("workspace/symbol");
}

#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceSymbolParameters {
    #[serde(flatten)]
    pub partial_result_parameters: PartialResultParams,

    #[serde(flatten)]
    pub work_done_progress_parameters: WorkDoneProgressParams,

    /// A non-empty query string.
    pub query: String,

    pub search_scope: Option<WorkspaceSymbolSearchScope>,

    pub search_kind: Option<WorkspaceSymbolSearchKind>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum WorkspaceSymbolSearchScope {
    Workspace,
    WorkspaceAndDependencies,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum WorkspaceSymbolSearchKind {
    OnlyTypes,
    AllSymbols,
}

/// The document on type formatting request is sent from the client to
/// the server to format parts of the document during typing.
///
/// This is almost same as [`lsp_types::DocumentOnTypeFormattingRequest`], but the
/// result has [`SnippetTextEdit`] in it instead of [`TextEdit`].
#[derive(Debug)]
pub enum OnTypeFormattingRequest {}

impl Request for OnTypeFormattingRequest {
    type Params = DocumentOnTypeFormattingParams;
    type Result = Option<Vec<SnippetTextEdit>>;

    const MESSAGE_DIRECTION: MessageDirection = MessageDirection::ClientToServer;
    const METHOD: LspRequestMethod = LspRequestMethod::new("textDocument/onTypeFormatting");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionResolveData {
    pub position: TextDocumentPositionParams,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub imports: Vec<CompletionImport>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub version: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub trigger_character: Option<char>,
    #[serde(default)]
    pub for_ref: bool,
    pub hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InlayHintResolveData {
    pub file_id: u32,
    // This is a string instead of a u64 as javascript can't represent u64 fully
    pub hash: String,
    pub resolve_range: Range,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub version: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionImport {
    pub full_import_path: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct ClientCommandOptions {
    pub commands: Vec<String>,
}
