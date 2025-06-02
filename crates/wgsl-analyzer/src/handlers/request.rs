#![expect(
    clippy::needless_pass_by_value,
    reason = "handlers should have a specific signature"
)]

use base_db::{FileRange, TextRange};
use hir::diagnostics::DiagnosticsConfig;
use ide::{HoverResult, diagnostics::Severity};
use lsp_types::{
    DiagnosticRelatedInformation, DiagnosticTag, GotoDefinitionResponse, LanguageString,
    MarkedString, Position, TextDocumentIdentifier, TextDocumentPositionParams,
};
use vfs::FileId;

use crate::{
    Result,
    global_state::GlobalStateSnapshot,
    lsp::{
        self,
        extensions::{self, PositionOrRange},
        from_proto, to_proto,
    },
};

pub(crate) fn handle_goto_definition(
    snap: GlobalStateSnapshot,
    parameters: lsp_types::GotoDefinitionParams,
) -> Result<Option<GotoDefinitionResponse>> {
    let position = from_proto::file_position(&snap, &parameters.text_document_position_params)?;
    let file_id = position.file_id;
    let Some(nav_target) = snap.analysis.goto_definition(position)? else {
        return Ok(None);
    };

    let range = FileRange {
        file_id: nav_target.file_id,
        range: nav_target.focus_or_full_range(),
    };
    let location = to_proto::location(&snap, range)?;
    let response = GotoDefinitionResponse::Scalar(location);

    Ok(Some(response))
}

pub(crate) fn handle_completion(
    snap: GlobalStateSnapshot,
    parameters: lsp_types::CompletionParams,
) -> Result<Option<lsp_types::CompletionResponse>> {
    let position = from_proto::file_position(&snap, &parameters.text_document_position)?;
    let line_index = snap.file_line_index(position.file_id)?;
    let source_root = snap.analysis.source_root_id(position.file_id)?;
    let completion_config = &snap.config.completion(Some(source_root));
    let Some(items) = snap
        .analysis
        .completions(completion_config, position, None)?
    else {
        return Ok(None);
    };
    let items = to_proto::completion_items(&line_index, &parameters.text_document_position, &items);
    let list = lsp_types::CompletionList {
        is_incomplete: true,
        items,
    };
    Ok(Some(list.into()))
}

pub(crate) fn handle_formatting(
    snap: GlobalStateSnapshot,
    parameters: lsp_types::DocumentFormattingParams,
) -> Result<Option<Vec<lsp_types::TextEdit>>> {
    let file_id = from_proto::file_id(&snap, &parameters.text_document.uri)?;
    let Some(node) = snap.analysis.format(file_id, None)? else {
        return Ok(None);
    };
    let line_index = snap.file_line_index(file_id)?;

    let before = snap.analysis.file_text(file_id)?;
    let after = node.to_string();

    let diff = diff::diff(&before, &after);
    let edits = to_proto::text_edit_vec(&line_index, diff);
    Ok(Some(edits))
}

pub(crate) fn handle_hover(
    snap: GlobalStateSnapshot,
    parameters: lsp::extensions::HoverParameters,
) -> Result<Option<lsp::extensions::Hover>> {
    let position = match parameters.position {
        PositionOrRange::Position(position) => position,
        PositionOrRange::Range(range) => range.start,
    };

    let tdp = TextDocumentPositionParams {
        text_document: parameters.text_document,
        position,
    };

    let position = from_proto::file_position(&snap, &tdp)?;
    let line_index = snap.file_line_index(position.file_id)?;
    let range = TextRange::new(position.offset, position.offset);
    let file_range = FileRange {
        file_id: position.file_id,
        range,
    };

    let Some(result) = snap.analysis.hover(file_range)? else {
        return Ok(None);
    };

    let hover_content = match result.info {
        HoverResult::SourceCode(code) => MarkedString::LanguageString(lsp_types::LanguageString {
            language: "wgsl".to_owned(),
            value: code,
        }),
        HoverResult::Text(text) => MarkedString::String(text),
    };

    let inner_hover = lsp_types::Hover {
        contents: lsp_types::HoverContents::Scalar(hover_content),
        range: Some(to_proto::range(&line_index, result.range)),
    };

    let extended_hover = lsp::extensions::Hover {
        hover: inner_hover,
        actions: Vec::new(),
    };

    Ok(Some(extended_hover))
}

#[expect(
    clippy::unnecessary_wraps,
    reason = "handlers should have a specific signature"
)]
pub(crate) fn handle_shutdown(
    _snap: GlobalStateSnapshot,
    _: (),
) -> Result<()> {
    Ok(())
}

pub(crate) fn full_source(
    snap: GlobalStateSnapshot,
    parameters: extensions::FullSourceParameters,
) -> Result<String> {
    let file_id = from_proto::file_id(&snap, &parameters.text_document.uri)?;
    let source = snap
        .analysis
        .resolve_full_source(file_id)?
        .unwrap_or_else(|()| String::new()); // TODO this is weird
    Ok(source)
}

pub(crate) fn show_syntax_tree(
    snap: GlobalStateSnapshot,
    parameters: extensions::SyntaxTreeParameters,
) -> Result<String> {
    let file_id = from_proto::file_id(&snap, &parameters.text_document.uri)?;
    let line_index = snap.file_line_index(file_id)?;
    let string = snap.analysis.syntax_tree(
        file_id,
        parameters
            .range
            .and_then(|range| from_proto::text_range(&line_index, range).ok()),
    )?;
    Ok(string)
}

pub(crate) fn debug_command(
    snap: GlobalStateSnapshot,
    parameters: extensions::DebugCommandParameters,
) -> Result<()> {
    let position = from_proto::file_position(&snap, &parameters.position)?;
    snap.analysis.debug_command(position)?;

    Ok(())
}

// This is the “empty” fallback if the VFS lookup fails.
// It returns an “Unchanged” report with the same `previousResultId` the client sent.
pub(crate) fn empty_diagnostic_report() -> lsp_types::DocumentDiagnosticReportResult {
    lsp_types::DocumentDiagnosticReportResult::Report(lsp_types::DocumentDiagnosticReport::Full(
        lsp_types::RelatedFullDocumentDiagnosticReport {
            related_documents: None,
            full_document_diagnostic_report: lsp_types::FullDocumentDiagnosticReport {
                result_id: Some("wgsl-analyzer".to_owned()),
                items: vec![],
            },
        },
    ))
}

pub(crate) fn handle_document_diagnostics(
    snap: GlobalStateSnapshot,
    params: lsp_types::DocumentDiagnosticParams,
) -> anyhow::Result<lsp_types::DocumentDiagnosticReportResult> {
    let file_id = from_proto::file_id(&snap, &params.text_document.uri)?;

    let source_root = snap.analysis.source_root_id(file_id).ok();
    let config = snap.config.data().diagnostics(source_root);

    let items = publish_diagnostics(&snap, &config, file_id).unwrap();

    Ok(lsp_types::DocumentDiagnosticReportResult::Report(
        lsp_types::DocumentDiagnosticReport::Full(lsp_types::RelatedFullDocumentDiagnosticReport {
            related_documents: None,
            full_document_diagnostic_report: lsp_types::FullDocumentDiagnosticReport {
                result_id: Some(
                    params
                        .previous_result_id
                        .unwrap_or_else(|| "wgsl-analyzer".to_owned()),
                ),
                items,
            },
        }),
    ))
}

pub(crate) fn handle_inlay_hints(
    snap: GlobalStateSnapshot,
    parameters: lsp_types::InlayHintParams,
) -> Result<Option<Vec<lsp_types::InlayHint>>> {
    let document_uri = &parameters.text_document.uri;
    let file_id = from_proto::file_id(&snap, document_uri)?;
    let line_index = snap.file_line_index(file_id)?;

    let range = from_proto::file_range(
        &snap,
        &TextDocumentIdentifier::new(document_uri.to_owned()),
        parameters.range,
    );

    Ok(Some(
        snap.analysis
            .inlay_hints(&snap.config.data().inlay_hints(), file_id, range.ok())?
            .iter()
            .map(|it| to_proto::inlay_hint(true, &line_index, it))
            .collect(),
    ))
}

pub(crate) fn publish_diagnostics(
    snap: &GlobalStateSnapshot,
    config: &DiagnosticsConfig,
    file_id: FileId,
) -> Result<Vec<lsp_types::Diagnostic>> {
    let line_index = snap.file_line_index(file_id)?;
    let diagnostics = snap.analysis.diagnostics(config, file_id)?;

    diagnostics
        .into_iter()
        .map(|diagnostic| {
            let related = diagnostic
                .related
                .into_iter()
                .map(|(message, range)| {
                    Ok(DiagnosticRelatedInformation {
                        location: to_proto::location(snap, range)?,
                        message,
                    })
                })
                .collect::<Result<Vec<_>>>()?;
            Ok(lsp_types::Diagnostic {
                range: to_proto::range(&line_index, diagnostic.range),
                severity: Some(diagnostic_severity(diagnostic.severity)),
                code: None,
                code_description: None,
                source: None,
                message: diagnostic.message,
                related_information: (!related.is_empty()).then_some(related),
                tags: diagnostic.unused.then(|| vec![DiagnosticTag::UNNECESSARY]),
                data: None,
            })
        })
        .collect()
}

const fn diagnostic_severity(severity: Severity) -> lsp_types::DiagnosticSeverity {
    match severity {
        Severity::Error => lsp_types::DiagnosticSeverity::ERROR,
        Severity::WeakWarning => lsp_types::DiagnosticSeverity::HINT,
    }
}

mod diff {
    //! Generate minimal `TextEdit`s from different text versions
    use dissimilar::Chunk;
    use text_edit::{TextEdit, TextRange, TextSize};

    pub(super) fn diff(
        left: &str,
        right: &str,
    ) -> TextEdit {
        let chunks = dissimilar::diff(left, right);
        textedit_from_chunks(chunks)
    }

    fn textedit_from_chunks(chunks: Vec<dissimilar::Chunk<'_>>) -> TextEdit {
        let mut builder = TextEdit::builder();
        let mut pos = TextSize::default();

        let mut chunks = chunks.into_iter().peekable();
        while let Some(chunk) = chunks.next() {
            if let (Chunk::Delete(deleted), Some(&Chunk::Insert(inserted))) = (chunk, chunks.peek())
            {
                chunks.next().unwrap();
                let deleted_length = TextSize::of(deleted);
                builder.replace(TextRange::at(pos, deleted_length), inserted.into());
                pos += deleted_length;
                continue;
            }

            match chunk {
                Chunk::Equal(text) => {
                    pos += TextSize::of(text);
                },
                Chunk::Delete(deleted) => {
                    let deleted_length = TextSize::of(deleted);
                    builder.delete(TextRange::at(pos, deleted_length));
                    pos += deleted_length;
                },
                Chunk::Insert(inserted) => {
                    builder.insert(pos, inserted.into());
                },
            }
        }
        builder.finish()
    }
}
