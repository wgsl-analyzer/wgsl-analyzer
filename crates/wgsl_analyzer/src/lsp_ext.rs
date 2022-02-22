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
