use lsp_types::{request::Request, TextDocumentIdentifier, TextDocumentPositionParams};
use serde::{Deserialize, Serialize};

pub enum SyntaxTree {}

impl Request for SyntaxTree {
    type Params = SyntaxTreeParams;
    type Result = String;
    const METHOD: &'static str = "wgsl-analyzer/syntaxTree";
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SyntaxTreeParams {
    pub text_document: TextDocumentIdentifier,
}

pub enum DebugCommand {}

impl Request for DebugCommand {
    type Params = DebugCommandParams;
    type Result = ();
    const METHOD: &'static str = "wgsl-analyzer/debugCommand";
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DebugCommandParams {
    #[serde(flatten)]
    pub position: TextDocumentPositionParams,
}

pub enum FullSource {}

impl Request for FullSource {
    type Params = FullSourceParams;
    type Result = String;
    const METHOD: &'static str = "wgsl-analyzer/fullSource";
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FullSourceParams {
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
    type Params = inlay_hints::InlayHintsParams;
    type Result = Vec<inlay_hints::InlayHint>;
    const METHOD: &'static str = "experimental/inlayHints";
}

pub mod inlay_hints {
    use lsp_types::{Position, TextDocumentIdentifier};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct InlayHintsParams {
        pub text_document: TextDocumentIdentifier,
        pub range: Option<lsp_types::Range>,
    }

    #[derive(Eq, PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct InlayHintKind(u8);

    #[allow(dead_code)]
    impl InlayHintKind {
        pub const TYPE: InlayHintKind = InlayHintKind(1);
        pub const PARAMETER: InlayHintKind = InlayHintKind(2);
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
