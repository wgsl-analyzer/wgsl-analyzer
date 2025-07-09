//! wgsl-analyzer extensions to the LSP.

// Note:
// When adding new resolve payloads, add a #[serde(default)] on boolean fields.
// Some clients might strip `false` values from the JSON payload due to their
// reserialization logic turning `false` into `null`.
// This will cause them to be omitted in the resolve request.
//
// See: https://github.com/rust-lang/rust-analyzer/issues/18767

#![expect(clippy::disallowed_types, reason = "serde compatibility")]

use std::ops;

use camino::Utf8PathBuf;
use lsp_types::{
    CodeActionKind, DocumentOnTypeFormattingParams, PartialResultParams, Position, Range,
    TextDocumentIdentifier, Url, WorkDoneProgressParams, notification::Notification,
    request::Request,
};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

use lsp_types::TextDocumentPositionParams;

pub enum SyntaxTree {}

impl Request for SyntaxTree {
    type Params = SyntaxTreeParameters;
    type Result = String;

    const METHOD: &'static str = "wgsl-analyzer/syntaxTree";
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SyntaxTreeParameters {
    pub text_document: TextDocumentIdentifier,
    pub range: Option<Range>,
}

pub enum DebugCommand {}

impl Request for DebugCommand {
    type Params = DebugCommandParameters;
    type Result = ();

    const METHOD: &'static str = "wgsl-analyzer/debugCommand";
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DebugCommandParameters {
    #[serde(flatten)]
    pub position: TextDocumentPositionParams,
}

pub enum FullSource {}

impl Request for FullSource {
    type Params = FullSourceParameters;
    type Result = String;

    const METHOD: &'static str = "wgsl-analyzer/fullSource";
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FullSourceParameters {
    pub text_document: TextDocumentIdentifier,
}

pub enum RequestConfiguration {}

impl Request for RequestConfiguration {
    type Params = ();
    type Result = serde_json::Value;

    const METHOD: &'static str = "wgsl-analyzer/requestConfiguration";
}

pub enum InlayHints {}

impl Request for InlayHints {
    type Params = inlay_hints::InlayHintsParameters;
    type Result = Vec<inlay_hints::InlayHint>;
    const METHOD: &'static str = "experimental/inlayHints";
}

pub mod inlay_hints {
    use lsp_types::{Position, TextDocumentIdentifier};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct InlayHintsParameters {
        pub text_document: TextDocumentIdentifier,
        pub range: Option<lsp_types::Range>,
    }

    #[derive(Eq, PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct InlayHintKind(u8);

    impl InlayHintKind {
        pub const TYPE: Self = Self(1);
        pub const PARAMETER: Self = Self(2);
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct InlayHint {
        pub label: InlayHintLabel,
        pub position: Position,
        pub kind: Option<InlayHintKind>,
        pub tooltip: Option<String>,
        pub padding_left: Option<bool>,
        pub padding_right: Option<bool>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum InlayHintLabel {
        String(String),
        Parts(Vec<InlayHintLabelPart>),
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct InlayHintLabelPart {
        pub value: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tooltip: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub location: Option<lsp_types::LocationLink>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub command: Option<lsp_types::Command>,
    }
}

pub enum ImportTextDocument {}

impl Request for ImportTextDocument {
    type Params = import_text_document::ImportTextDocumentParameters;
    type Result = ();
    const METHOD: &'static str = "wgsl-analyzer/importTextDocument";
}

pub mod import_text_document {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct ImportTextDocumentParameters {
        pub uri: String,
    }
}

pub enum InternalTestingFetchConfig {}

#[derive(Deserialize, Serialize, Debug)]
pub enum InternalTestingFetchConfigOption {
    AssistEmitMustUse,
    CheckWorkspace,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub enum InternalTestingFetchConfigResponse {
    AssistEmitMustUse(bool),
    CheckWorkspace(bool),
}

impl Request for InternalTestingFetchConfig {
    type Params = InternalTestingFetchConfigParameters;
    // Option is solely to circumvent Default bound.
    type Result = Option<InternalTestingFetchConfigResponse>;
    const METHOD: &'static str = "rust-analyzer-internal/internalTestingFetchConfig";
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InternalTestingFetchConfigParameters {
    pub text_document: Option<TextDocumentIdentifier>,
    pub config: InternalTestingFetchConfigOption,
}
pub enum AnalyzerStatus {}

impl Request for AnalyzerStatus {
    type Params = AnalyzerStatusParameters;
    type Result = String;
    const METHOD: &'static str = "rust-analyzer/analyzerStatus";
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzerStatusParameters {
    pub text_document: Option<TextDocumentIdentifier>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CrateInfoResult {
    pub name: Option<String>,
    pub version: Option<String>,
    pub path: Url,
}
pub enum FetchDependencyList {}

impl Request for FetchDependencyList {
    type Params = FetchDependencyListParameters;
    type Result = FetchDependencyListResult;
    const METHOD: &'static str = "rust-analyzer/fetchDependencyList";
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FetchDependencyListParameters;

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct FetchDependencyListResult {
    pub crates: Vec<CrateInfoResult>,
}

pub enum MemoryUsage {}

impl Request for MemoryUsage {
    type Params = ();
    type Result = String;
    const METHOD: &'static str = "rust-analyzer/memoryUsage";
}

pub enum ReloadWorkspace {}

impl Request for ReloadWorkspace {
    type Params = ();
    type Result = ();
    const METHOD: &'static str = "rust-analyzer/reloadWorkspace";
}

pub enum RebuildProcMacros {}

impl Request for RebuildProcMacros {
    type Params = ();
    type Result = ();
    const METHOD: &'static str = "rust-analyzer/rebuildProcMacros";
}

pub enum ViewSyntaxTree {}

impl Request for ViewSyntaxTree {
    type Params = ViewSyntaxTreeParameters;
    type Result = String;
    const METHOD: &'static str = "rust-analyzer/viewSyntaxTree";
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ViewSyntaxTreeParameters {
    pub text_document: TextDocumentIdentifier,
}

pub enum ViewHir {}

impl Request for ViewHir {
    type Params = lsp_types::TextDocumentPositionParams;
    type Result = String;
    const METHOD: &'static str = "rust-analyzer/viewHir";
}

pub enum ViewMir {}

impl Request for ViewMir {
    type Params = lsp_types::TextDocumentPositionParams;
    type Result = String;
    const METHOD: &'static str = "rust-analyzer/viewMir";
}

pub enum InterpretFunction {}

impl Request for InterpretFunction {
    type Params = lsp_types::TextDocumentPositionParams;
    type Result = String;
    const METHOD: &'static str = "rust-analyzer/interpretFunction";
}

pub enum ViewFileText {}

impl Request for ViewFileText {
    type Params = lsp_types::TextDocumentIdentifier;
    type Result = String;
    const METHOD: &'static str = "rust-analyzer/viewFileText";
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ViewCrateGraphParameters {
    /// Include *all* crates, not just crates in the workspace.
    pub full: bool,
}

pub enum ViewCrateGraph {}

impl Request for ViewCrateGraph {
    type Params = ViewCrateGraphParameters;
    type Result = String;
    const METHOD: &'static str = "rust-analyzer/viewCrateGraph";
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ViewItemTreeParameters {
    pub text_document: TextDocumentIdentifier,
}

pub enum ViewItemTree {}

impl Request for ViewItemTree {
    type Params = ViewItemTreeParameters;
    type Result = String;
    const METHOD: &'static str = "rust-analyzer/viewItemTree";
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

pub enum DiscoverTest {}

impl Request for DiscoverTest {
    type Params = DiscoverTestParameters;
    type Result = DiscoverTestResults;
    const METHOD: &'static str = "experimental/discoverTest";
}

pub enum DiscoveredTests {}

impl Notification for DiscoveredTests {
    type Params = DiscoverTestResults;
    const METHOD: &'static str = "experimental/discoveredTests";
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RunTestParameters {
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}

pub enum RunTest {}

impl Request for RunTest {
    type Params = RunTestParameters;
    type Result = ();
    const METHOD: &'static str = "experimental/runTest";
}

pub enum EndRunTest {}

impl Notification for EndRunTest {
    type Params = ();
    const METHOD: &'static str = "experimental/endRunTest";
}

pub enum AppendOutputToRunTest {}

impl Notification for AppendOutputToRunTest {
    type Params = String;
    const METHOD: &'static str = "experimental/appendOutputToRunTest";
}

pub enum AbortRunTest {}

impl Notification for AbortRunTest {
    type Params = ();
    const METHOD: &'static str = "experimental/abortRunTest";
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

pub enum ChangeTestState {}

impl Notification for ChangeTestState {
    type Params = ChangeTestStateParameters;
    const METHOD: &'static str = "experimental/changeTestState";
}

pub enum ExpandMacro {}

impl Request for ExpandMacro {
    type Params = ExpandMacroParameters;
    type Result = Option<ExpandedMacro>;
    const METHOD: &'static str = "rust-analyzer/expandMacro";
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExpandMacroParameters {
    pub text_document: TextDocumentIdentifier,
    pub position: Position,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExpandedMacro {
    pub name: String,
    pub expansion: String,
}

pub enum ViewRecursiveMemoryLayout {}

impl Request for ViewRecursiveMemoryLayout {
    type Params = lsp_types::TextDocumentPositionParams;
    type Result = Option<RecursiveMemoryLayout>;
    const METHOD: &'static str = "rust-analyzer/viewRecursiveMemoryLayout";
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

pub enum CancelFlycheck {}

impl Notification for CancelFlycheck {
    type Params = ();
    const METHOD: &'static str = "rust-analyzer/cancelFlycheck";
}

pub enum RunFlycheck {}

impl Notification for RunFlycheck {
    type Params = RunFlycheckParameters;
    const METHOD: &'static str = "rust-analyzer/runFlycheck";
}

pub enum ClearFlycheck {}

impl Notification for ClearFlycheck {
    type Params = ();
    const METHOD: &'static str = "rust-analyzer/clearFlycheck";
}

pub enum OpenServerLogs {}

impl Notification for OpenServerLogs {
    type Params = ();
    const METHOD: &'static str = "rust-analyzer/openServerLogs";
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RunFlycheckParameters {
    pub text_document: Option<TextDocumentIdentifier>,
}

pub enum MatchingBrace {}

impl Request for MatchingBrace {
    type Params = MatchingBraceParameters;
    type Result = Vec<Position>;
    const METHOD: &'static str = "experimental/matchingBrace";
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatchingBraceParameters {
    pub text_document: TextDocumentIdentifier,
    pub positions: Vec<Position>,
}

pub enum ParentModule {}

impl Request for ParentModule {
    type Params = lsp_types::TextDocumentPositionParams;
    type Result = Option<lsp_types::GotoDefinitionResponse>;
    const METHOD: &'static str = "experimental/parentModule";
}

pub enum JoinLines {}

impl Request for JoinLines {
    type Params = JoinLinesParameters;
    type Result = Vec<lsp_types::TextEdit>;
    const METHOD: &'static str = "experimental/joinLines";
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JoinLinesParameters {
    pub text_document: TextDocumentIdentifier,
    pub ranges: Vec<Range>,
}

pub enum OnEnter {}

impl Request for OnEnter {
    type Params = lsp_types::TextDocumentPositionParams;
    type Result = Option<Vec<SnippetTextEdit>>;
    const METHOD: &'static str = "experimental/onEnter";
}

pub enum Runnables {}

impl Request for Runnables {
    type Params = RunnablesParameters;
    type Result = Vec<Runnable>;
    const METHOD: &'static str = "experimental/runnables";
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
    pub location: Option<lsp_types::LocationLink>,
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
pub struct CargoRunnableArguments {
    #[serde(skip_serializing_if = "FxHashMap::is_empty")]
    pub environment: FxHashMap<String, String>,
    pub cwd: Utf8PathBuf,
    /// Command to be executed instead of cargo
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

pub enum RelatedTests {}

impl Request for RelatedTests {
    type Params = lsp_types::TextDocumentPositionParams;
    type Result = Vec<TestInfo>;
    const METHOD: &'static str = "rust-analyzer/relatedTests";
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TestInfo {
    pub runnable: Runnable,
}

pub enum Ssr {}

impl Request for Ssr {
    type Params = SsrParameters;
    type Result = lsp_types::WorkspaceEdit;
    const METHOD: &'static str = "experimental/ssr";
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SsrParameters {
    pub query: String,
    pub parse_only: bool,

    /// File position where SSR was invoked. Paths in `query` will be resolved relative to this
    /// position.
    #[serde(flatten)]
    pub position: lsp_types::TextDocumentPositionParams,

    /// Current selections. Search/replace will be restricted to these if non-empty.
    pub selections: Vec<lsp_types::Range>,
}

pub enum ServerStatusNotification {}

impl Notification for ServerStatusNotification {
    type Params = ServerStatusParameters;
    const METHOD: &'static str = "experimental/serverStatus";
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
    type Params = lsp_types::CodeActionParams;
    type Result = Option<Vec<CodeAction>>;
    const METHOD: &'static str = "textDocument/codeAction";
}

pub enum CodeActionResolveRequest {}

impl Request for CodeActionResolveRequest {
    type Params = CodeAction;
    type Result = CodeAction;
    const METHOD: &'static str = "codeAction/resolve";
}

#[derive(Debug, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeAction {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<CodeActionKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<lsp_types::Command>,
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
    pub code_action_parameters: lsp_types::CodeActionParams,
    pub id: String,
    pub version: Option<i32>,
}

#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnippetWorkspaceEdit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes: Option<FxHashMap<lsp_types::Url, Vec<lsp_types::TextEdit>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_changes: Option<Vec<SnippetDocumentChangeOperation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_annotations: Option<
        std::collections::HashMap<
            lsp_types::ChangeAnnotationIdentifier,
            lsp_types::ChangeAnnotation,
        >,
    >,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(untagged, rename_all = "lowercase")]
pub enum SnippetDocumentChangeOperation {
    Op(lsp_types::ResourceOp),
    Edit(SnippetTextDocumentEdit),
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnippetTextDocumentEdit {
    pub text_document: lsp_types::OptionalVersionedTextDocumentIdentifier,
    pub edits: Vec<SnippetTextEdit>,
}

#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnippetTextEdit {
    pub range: Range,
    pub new_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_text_format: Option<lsp_types::InsertTextFormat>,
    /// The annotation id if this is an annotated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_id: Option<lsp_types::ChangeAnnotationIdentifier>,
}

pub enum HoverRequest {}

impl Request for HoverRequest {
    type Params = HoverParameters;
    type Result = Option<HoverResult>;
    const METHOD: &'static str = lsp_types::request::HoverRequest::METHOD;
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
    Position(lsp_types::Position),
    Range(lsp_types::Range),
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct HoverResult {
    #[serde(flatten)]
    pub hover: lsp_types::Hover,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<CommandLinkGroup>,
}

#[derive(Debug, PartialEq, Clone, Default, Deserialize, Serialize)]
pub struct CommandLinkGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub commands: Vec<CommandLink>,
}

// LSP v3.15 Command does not have a `tooltip` field, vscode supports one.
#[derive(Debug, PartialEq, Clone, Default, Deserialize, Serialize)]
pub struct CommandLink {
    #[serde(flatten)]
    pub command: lsp_types::Command,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<String>,
}

pub enum ExternalDocs {}

impl Request for ExternalDocs {
    type Params = lsp_types::TextDocumentPositionParams;
    type Result = ExternalDocsResponse;
    const METHOD: &'static str = "experimental/externalDocs";
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ExternalDocsResponse {
    Simple(Option<lsp_types::Url>),
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
    pub web: Option<lsp_types::Url>,
    pub local: Option<lsp_types::Url>,
}

pub enum OpenCargoToml {}

impl Request for OpenCargoToml {
    type Params = OpenCargoTomlParameters;
    type Result = Option<lsp_types::GotoDefinitionResponse>;
    const METHOD: &'static str = "experimental/openCargoToml";
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
    Impls(lsp_types::request::GotoImplementationParams),
    References(lsp_types::TextDocumentPositionParams),
}

pub enum MoveItem {}

impl Request for MoveItem {
    type Params = MoveItemParameters;
    type Result = Vec<SnippetTextEdit>;
    const METHOD: &'static str = "experimental/moveItem";
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
pub enum WorkspaceSymbol {}

impl Request for WorkspaceSymbol {
    type Params = WorkspaceSymbolParameters;
    type Result = Option<lsp_types::WorkspaceSymbolResponse>;
    const METHOD: &'static str = "workspace/symbol";
}

#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceSymbolParameters {
    #[serde(flatten)]
    pub partial_result_parameters: PartialResultParams,

    #[serde(flatten)]
    pub work_done_progress_parameters: WorkDoneProgressParams,

    /// A non-empty query string
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
/// This is almost same as [`lsp_types::request::OnTypeFormatting`], but the
/// result has [`SnippetTextEdit`] in it instead of [`TextEdit`].
#[derive(Debug)]
pub enum OnTypeFormatting {}

impl Request for OnTypeFormatting {
    type Params = DocumentOnTypeFormattingParams;
    type Result = Option<Vec<SnippetTextEdit>>;
    const METHOD: &'static str = "textDocument/onTypeFormatting";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionResolveData {
    pub position: lsp_types::TextDocumentPositionParams,
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
    pub resolve_range: lsp_types::Range,
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
