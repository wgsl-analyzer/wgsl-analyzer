use ide_completion::CompletionFieldsToResolve;
use line_index::WideEncoding;
use lsp_types::{
    CallHierarchyServerCapability, CodeActionKind, CodeActionOptions, CodeActionProviderCapability,
    CodeLensOptions, CompletionOptions, CompletionOptionsCompletionItem, DeclarationCapability,
    DiagnosticOptions, DocumentOnTypeFormattingOptions, FileOperationFilter, FileOperationPattern,
    FileOperationPatternKind, FileOperationRegistrationOptions, FoldingRangeProviderCapability,
    HoverProviderCapability, ImplementationProviderCapability, InlayHintOptions,
    InlayHintServerCapabilities, OneOf, PositionEncodingKind, RenameOptions, SaveOptions,
    SelectionRangeProviderCapability, SemanticTokensFullOptions, SemanticTokensLegend,
    SemanticTokensOptions, ServerCapabilities, SignatureHelpOptions, TextDocumentSyncCapability,
    TextDocumentSyncKind, TextDocumentSyncOptions, TypeDefinitionProviderCapability,
    WorkDoneProgressOptions, WorkspaceFileOperationsServerCapabilities,
    WorkspaceFoldersServerCapabilities, WorkspaceServerCapabilities,
};
use rustc_hash::FxHashSet;
use serde_json::json;

use crate::{
    config::{Config, WgslfmtConfig},
    line_index::PositionEncoding,
    lsp::{extensions, semantic_tokens},
};

#[must_use]
pub fn server_capabilities(config: &Config) -> ServerCapabilities {
    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::INCREMENTAL,
        )),
        definition_provider: Some(OneOf::Left(true)),
        completion_provider: Some(CompletionOptions {
            completion_item: None,
            resolve_provider: None,
            trigger_characters: Some(vec![".".to_owned()]),
            all_commit_characters: None,
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: None,
            },
        }),
        document_formatting_provider: Some(OneOf::Left(true)),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        // rename_provider: Some(OneOf::Left(true)),
        // definition_provider: Some(OneOf::Left(true)),
        inlay_hint_provider: Some(OneOf::Left(true)),
        experimental: Some(json!({ "inlayHints": true })),
        diagnostic_provider: Some(lsp_types::DiagnosticServerCapabilities::Options(
            DiagnosticOptions {
                identifier: None,
                inter_file_dependencies: false,
                workspace_diagnostics: false,
                work_done_progress_options: WorkDoneProgressOptions {
                    work_done_progress: Some(false),
                },
            },
        )),
        ..Default::default()
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct ClientCapabilities(lsp_types::ClientCapabilities);

impl ClientCapabilities {
    pub const fn new(caps: lsp_types::ClientCapabilities) -> Self {
        Self(caps)
    }

    fn completions_resolve_provider(&self) -> bool {
        let client_capabilities = self.completion_resolve_support_properties();
        let fields_to_resolve =
            CompletionFieldsToResolve::from_client_capabilities(&client_capabilities);
        fields_to_resolve != CompletionFieldsToResolve::empty()
    }

    fn inlay_hints_resolve_provider(&self) -> bool {
        let client_capabilities = self.inlay_hint_resolve_support_properties();
        // let fields_to_resolve =
        //     InlayFieldsToResolve::from_client_capabilities(&client_capabilities);
        // fields_to_resolve != InlayFieldsToResolve::empty()
        false
    }

    fn experimental_bool(
        &self,
        index: &'static str,
    ) -> bool {
        || -> _ { self.0.experimental.as_ref()?.get(index)?.as_bool() }().unwrap_or_default()
    }

    fn experimental<T: serde::de::DeserializeOwned>(
        &self,
        index: &'static str,
    ) -> Option<T> {
        serde_json::from_value(self.0.experimental.as_ref()?.get(index)?.clone()).ok()
    }

    /// Parses client capabilities and returns all completion resolve capabilities rust-analyzer supports.
    pub fn completion_item_edit_resolve(&self) -> bool {
        (|| {
            Some(
                self.0
                    .text_document
                    .as_ref()?
                    .completion
                    .as_ref()?
                    .completion_item
                    .as_ref()?
                    .resolve_support
                    .as_ref()?
                    .properties
                    .iter()
                    .any(|cap_string| cap_string.as_str() == "additionalTextEdits"),
            )
        })() == Some(true)
    }

    pub fn completion_label_details_support(&self) -> bool {
        (|| -> _ {
            self.0
                .text_document
                .as_ref()?
                .completion
                .as_ref()?
                .completion_item
                .as_ref()?
                .label_details_support
        })() == Some(true)
    }

    fn completion_item(&self) -> CompletionOptionsCompletionItem {
        CompletionOptionsCompletionItem {
            label_details_support: Some(self.completion_label_details_support()),
        }
    }

    #[expect(
        clippy::return_and_then,
        reason = "https://github.com/rust-lang/rust-clippy/pull/14950"
    )]
    fn code_action_capabilities(&self) -> CodeActionProviderCapability {
        self.0
            .text_document
            .as_ref()
            .and_then(|capabilities| capabilities.code_action.as_ref())
            .and_then(|capabilities| capabilities.code_action_literal_support.as_ref())
            .map_or(CodeActionProviderCapability::Simple(true), |_| {
                CodeActionProviderCapability::Options(CodeActionOptions {
                    // Advertise support for all built-in CodeActionKinds.
                    // Ideally we would base this off of the client capabilities
                    // but the client is supposed to fall back gracefully for unknown values.
                    code_action_kinds: Some(vec![
                        CodeActionKind::EMPTY,
                        CodeActionKind::QUICKFIX,
                        CodeActionKind::REFACTOR,
                        CodeActionKind::REFACTOR_EXTRACT,
                        CodeActionKind::REFACTOR_INLINE,
                        CodeActionKind::REFACTOR_REWRITE,
                    ]),
                    resolve_provider: Some(true),
                    work_done_progress_options: WorkDoneProgressOptions::default(),
                })
            })
    }

    pub(crate) fn negotiated_encoding(&self) -> PositionEncoding {
        let client_encodings = self.0.general.as_ref().map_or([].as_slice(), |general| {
            general.position_encodings.as_deref().unwrap_or_default()
        });

        for enc in client_encodings {
            if enc == &PositionEncodingKind::UTF8 {
                return PositionEncoding::Utf8;
            } else if enc == &PositionEncodingKind::UTF32 {
                return PositionEncoding::Wide(WideEncoding::Utf32);
            }
            // NB: intentionally prefer just about anything else to utf-16.
        }

        PositionEncoding::Wide(WideEncoding::Utf16)
    }

    pub fn workspace_edit_resource_operations(
        &self
    ) -> Option<&[lsp_types::ResourceOperationKind]> {
        self.0
            .workspace
            .as_ref()?
            .workspace_edit
            .as_ref()?
            .resource_operations
            .as_deref()
    }

    pub fn semantics_tokens_augments_syntax_tokens(&self) -> bool {
        (|| -> _ {
            self.0
                .text_document
                .as_ref()?
                .semantic_tokens
                .as_ref()?
                .augments_syntax_tokens
        })()
        .unwrap_or(false)
    }

    pub fn did_save_text_document_dynamic_registration(&self) -> bool {
        let caps = (|| -> _ { self.0.text_document.as_ref()?.synchronization.clone() })()
            .unwrap_or_default();
        caps.did_save == Some(true) && caps.dynamic_registration == Some(true)
    }

    pub fn did_change_watched_files_dynamic_registration(&self) -> bool {
        (|| -> _ {
            self.0
                .workspace
                .as_ref()?
                .did_change_watched_files
                .as_ref()?
                .dynamic_registration
        })()
        .unwrap_or_default()
    }

    pub fn did_change_watched_files_relative_pattern_support(&self) -> bool {
        (|| -> _ {
            self.0
                .workspace
                .as_ref()?
                .did_change_watched_files
                .as_ref()?
                .relative_pattern_support
        })()
        .unwrap_or_default()
    }

    pub fn location_link(&self) -> bool {
        (|| -> _ { self.0.text_document.as_ref()?.definition?.link_support })().unwrap_or_default()
    }

    pub fn line_folding_only(&self) -> bool {
        (|| -> _ {
            self.0
                .text_document
                .as_ref()?
                .folding_range
                .as_ref()?
                .line_folding_only
        })()
        .unwrap_or_default()
    }

    pub fn hierarchical_symbols(&self) -> bool {
        (|| -> _ {
            self.0
                .text_document
                .as_ref()?
                .document_symbol
                .as_ref()?
                .hierarchical_document_symbol_support
        })()
        .unwrap_or_default()
    }

    pub fn code_action_literals(&self) -> bool {
        (|| -> _ {
            self.0
                .text_document
                .as_ref()?
                .code_action
                .as_ref()?
                .code_action_literal_support
                .as_ref()
        })()
        .is_some()
    }

    pub fn work_done_progress(&self) -> bool {
        (|| -> _ { self.0.window.as_ref()?.work_done_progress })().unwrap_or_default()
    }

    pub fn will_rename(&self) -> bool {
        (|| -> _ {
            self.0
                .workspace
                .as_ref()?
                .file_operations
                .as_ref()?
                .will_rename
        })()
        .unwrap_or_default()
    }

    pub fn change_annotation_support(&self) -> bool {
        (|| -> _ {
            self.0
                .workspace
                .as_ref()?
                .workspace_edit
                .as_ref()?
                .change_annotation_support
                .as_ref()
        })()
        .is_some()
    }

    pub fn code_action_resolve(&self) -> bool {
        (|| -> _ {
            Some(
                self.0
                    .text_document
                    .as_ref()?
                    .code_action
                    .as_ref()?
                    .resolve_support
                    .as_ref()?
                    .properties
                    .as_slice(),
            )
        })()
        .unwrap_or_default()
        .iter()
        .any(|property| property == "edit")
    }

    pub fn signature_help_label_offsets(&self) -> bool {
        (|| -> _ {
            self.0
                .text_document
                .as_ref()?
                .signature_help
                .as_ref()?
                .signature_information
                .as_ref()?
                .parameter_information
                .as_ref()?
                .label_offset_support
        })()
        .unwrap_or_default()
    }

    pub fn text_document_diagnostic(&self) -> bool {
        (|| -> _ { self.0.text_document.as_ref()?.diagnostic.as_ref() })().is_some()
    }

    pub fn text_document_diagnostic_related_document_support(&self) -> bool {
        (|| -> _ {
            self.0
                .text_document
                .as_ref()?
                .diagnostic
                .as_ref()?
                .related_document_support
        })() == Some(true)
    }

    pub fn code_action_group(&self) -> bool {
        self.experimental_bool("codeActionGroup")
    }

    pub fn commands(&self) -> Option<extensions::ClientCommandOptions> {
        self.experimental("commands")
    }

    pub fn local_docs(&self) -> bool {
        self.experimental_bool("localDocs")
    }

    pub fn open_server_logs(&self) -> bool {
        self.experimental_bool("openServerLogs")
    }

    pub fn server_status_notification(&self) -> bool {
        self.experimental_bool("serverStatusNotification")
    }

    pub fn snippet_text_edit(&self) -> bool {
        self.experimental_bool("snippetTextEdit")
    }

    pub fn hover_actions(&self) -> bool {
        self.experimental_bool("hoverActions")
    }

    /// Whether the client supports colored output for full diagnostics from `checkOnSave`.
    pub fn color_diagnostic_output(&self) -> bool {
        self.experimental_bool("colorDiagnosticOutput")
    }

    pub fn test_explorer(&self) -> bool {
        self.experimental_bool("testExplorer")
    }

    pub fn completion_snippet(&self) -> bool {
        (|| -> _ {
            self.0
                .text_document
                .as_ref()?
                .completion
                .as_ref()?
                .completion_item
                .as_ref()?
                .snippet_support
        })()
        .unwrap_or_default()
    }

    pub fn semantic_tokens_refresh(&self) -> bool {
        (|| -> _ {
            self.0
                .workspace
                .as_ref()?
                .semantic_tokens
                .as_ref()?
                .refresh_support
        })()
        .unwrap_or_default()
    }

    pub fn code_lens_refresh(&self) -> bool {
        (|| -> _ {
            self.0
                .workspace
                .as_ref()?
                .code_lens
                .as_ref()?
                .refresh_support
        })()
        .unwrap_or_default()
    }

    pub fn inlay_hints_refresh(&self) -> bool {
        (|| -> _ {
            self.0
                .workspace
                .as_ref()?
                .inlay_hint
                .as_ref()?
                .refresh_support
        })()
        .unwrap_or_default()
    }

    pub fn diagnostics_refresh(&self) -> bool {
        (|| -> _ {
            self.0
                .workspace
                .as_ref()?
                .diagnostic
                .as_ref()?
                .refresh_support
        })()
        .unwrap_or_default()
    }

    #[expect(
        clippy::return_and_then,
        reason = "https://github.com/rust-lang/rust-clippy/pull/14950"
    )]
    pub fn inlay_hint_resolve_support_properties(&self) -> FxHashSet<&str> {
        self.0
            .text_document
            .as_ref()
            .and_then(|text| text.inlay_hint.as_ref())
            .and_then(|inlay_hint_caps| inlay_hint_caps.resolve_support.as_ref())
            .map(|inlay_resolve| inlay_resolve.properties.iter())
            .into_iter()
            .flatten()
            .map(std::string::String::as_str)
            .collect()
    }

    #[expect(
        clippy::return_and_then,
        reason = "https://github.com/rust-lang/rust-clippy/pull/14950"
    )]
    pub fn completion_resolve_support_properties(&self) -> FxHashSet<&str> {
        self.0
            .text_document
            .as_ref()
            .and_then(|text| text.completion.as_ref())
            .and_then(|completion_caps| completion_caps.completion_item.as_ref())
            .and_then(|completion_item_caps| completion_item_caps.resolve_support.as_ref())
            .map(|resolve_support| resolve_support.properties.iter())
            .into_iter()
            .flatten()
            .map(std::string::String::as_str)
            .collect()
    }

    pub fn hover_markdown_support(&self) -> bool {
        (|| -> _ {
            Some(
                self.0
                    .text_document
                    .as_ref()?
                    .hover
                    .as_ref()?
                    .content_format
                    .as_ref()?
                    .as_slice(),
            )
        })()
        .unwrap_or_default()
        .contains(&lsp_types::MarkupKind::Markdown)
    }

    pub fn insert_replace_support(&self) -> bool {
        (|| -> _ {
            self.0
                .text_document
                .as_ref()?
                .completion
                .as_ref()?
                .completion_item
                .as_ref()?
                .insert_replace_support
        })()
        .unwrap_or_default()
    }
}
