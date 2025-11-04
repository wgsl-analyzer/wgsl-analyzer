#![expect(
    clippy::needless_pass_by_value,
    reason = "handlers should have a specific signature"
)]

use base_db::{FilePosition, FileRange, TextRange};
use hir::diagnostics::DiagnosticsConfig;
use ide::{Cancellable, HoverAction, HoverGotoTypeData, diagnostics::Severity};
use lsp_types::{
    DiagnosticRelatedInformation, DiagnosticTag, GotoDefinitionResponse, HoverContents, InlayHint,
    InlayHintParams, MarkupContent, MarkupKind, Range, TextDocumentIdentifier,
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
    try_default,
};

pub(crate) fn handle_goto_definition(
    snap: GlobalStateSnapshot,
    parameters: lsp_types::GotoDefinitionParams,
) -> anyhow::Result<Option<lsp_types::GotoDefinitionResponse>> {
    let _p = tracing::info_span!("handle_goto_definition").entered();
    let position = try_default!(from_proto::file_position(
        &snap,
        &parameters.text_document_position_params
    )?);
    let Some(navigation_info) = snap.analysis.goto_definition(position)? else {
        return Ok(None);
    };
    let source = FileRange {
        file_id: position.file_id,
        range: navigation_info.focus_or_full_range(),
    };
    let location = to_proto::location(&snap, source)?;
    Ok(Some(GotoDefinitionResponse::Scalar(location)))
    // let result = to_proto::goto_definition_response(&snap, Some(source), vec![navigation_info])?;
    // Ok(Some(result))
}

pub(crate) fn handle_completion(
    snap: GlobalStateSnapshot,
    lsp_types::CompletionParams {
        text_document_position,
        context,
        ..
    }: lsp_types::CompletionParams,
) -> anyhow::Result<Option<lsp_types::CompletionResponse>> {
    let _p = tracing::info_span!("handle_completion").entered();
    let mut position = try_default!(from_proto::file_position(&snap, &text_document_position)?);
    let line_index = snap.file_line_index(position.file_id)?;
    let completion_trigger_character = context
        .and_then(|context| context.trigger_character)
        .and_then(|string| string.chars().next());

    let source_root = snap.analysis.source_root_id(position.file_id)?;
    let completion_config = &snap.config.completion(Some(source_root));
    // FIXME: We should fix up the position when retrying the cancelled request instead
    position.offset = position.offset.min(line_index.index.len());
    let Some(items) =
        snap.analysis
            .completions(completion_config, position, completion_trigger_character)?
    else {
        return Ok(None);
    };

    let items = to_proto::completion_items(
        &snap.config,
        completion_config.fields_to_resolve,
        &line_index,
        snap.file_version(position.file_id),
        &text_document_position,
        completion_trigger_character,
        items,
    );

    let completion_list = lsp_types::CompletionList {
        is_incomplete: true,
        items,
    };
    Ok(Some(completion_list.into()))
}

pub(crate) fn handle_formatting(
    snap: GlobalStateSnapshot,
    parameters: lsp_types::DocumentFormattingParams,
) -> Result<Option<Vec<lsp_types::TextEdit>>> {
    let Some(file_id) = from_proto::file_id(&snap, &parameters.text_document.uri)? else {
        return Ok(None);
    };
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
) -> Result<Option<lsp::extensions::HoverResult>> {
    let _p = tracing::info_span!("handle_hover").entered();
    let range = match parameters.position {
        PositionOrRange::Position(position) => Range::new(position, position),
        PositionOrRange::Range(range) => range,
    };
    let file_range = try_default!(from_proto::file_range(
        &snap,
        &parameters.text_document,
        range
    )?);

    let hover = snap.config.hover();
    let Some(info) = snap.analysis.hover(&hover, file_range)? else {
        return Ok(None);
    };

    let line_index = snap.file_line_index(file_range.file_id)?;
    let range = to_proto::range(&line_index, info.range);
    let markup_kind = hover.format;
    let hover = lsp::extensions::HoverResult {
        hover: lsp_types::Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: match markup_kind {
                    ide::HoverDocFormat::Markdown => MarkupKind::Markdown,
                    ide::HoverDocFormat::PlainText => MarkupKind::PlainText,
                },
                value: info.info.markup.to_string(),
            }),
            range: Some(range),
        },
        actions: if snap.config.hover_actions().none() {
            Vec::new()
        } else {
            prepare_hover_actions(&snap, &info.info.actions)
        },
    };

    Ok(Some(hover))
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
    let file_id = try_default!(from_proto::file_id(&snap, &parameters.text_document.uri)?);
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
    let file_id = try_default!(from_proto::file_id(&snap, &parameters.text_document.uri)?);
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
    let Some(position) = from_proto::file_position(&snap, &parameters.position)? else {
        return Ok(());
    };
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
    parameters: lsp_types::DocumentDiagnosticParams,
) -> anyhow::Result<lsp_types::DocumentDiagnosticReportResult> {
    let Some(file_id) = from_proto::file_id(&snap, &parameters.text_document.uri)? else {
        return Ok(empty_diagnostic_report());
    };
    let source_root = snap.analysis.source_root_id(file_id).ok();
    let config = snap.config.diagnostics(source_root);

    if !config.enabled {
        return Ok(empty_diagnostic_report());
    }

    let items = publish_diagnostics(&snap, &config, file_id).unwrap();

    Ok(lsp_types::DocumentDiagnosticReportResult::Report(
        lsp_types::DocumentDiagnosticReport::Full(lsp_types::RelatedFullDocumentDiagnosticReport {
            related_documents: None,
            full_document_diagnostic_report: lsp_types::FullDocumentDiagnosticReport {
                result_id: Some(
                    parameters
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
    parameters: InlayHintParams,
) -> anyhow::Result<Option<Vec<InlayHint>>> {
    let _p = tracing::info_span!("handle_inlay_hints").entered();
    let document_uri = &parameters.text_document.uri;
    let FileRange { file_id, range } = try_default!(from_proto::file_range(
        &snap,
        &TextDocumentIdentifier::new(document_uri.to_owned()),
        parameters.range,
    )?);
    let line_index = snap.file_line_index(file_id)?;
    let range = TextRange::new(
        range.start().min(line_index.index.len()),
        range.end().min(line_index.index.len()),
    );

    let inlay_hints_config = snap.config.inlay_hints();
    Ok(Some(
        snap.analysis
            .inlay_hints(&inlay_hints_config, file_id, Some(range))?
            .into_iter()
            .map(|inlay_hint| {
                to_proto::inlay_hint(
                    &snap,
                    inlay_hints_config.fields_to_resolve,
                    &line_index,
                    file_id,
                    inlay_hint,
                )
            })
            .collect::<Cancellable<Vec<_>>>()?,
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

fn prepare_hover_actions(
    snap: &GlobalStateSnapshot,
    actions: &[HoverAction],
) -> Vec<lsp::extensions::CommandLinkGroup> {
    actions
        .iter()
        .filter_map(|hover_action| match hover_action {
            HoverAction::Implementation(position) => show_impl_command_link(snap, *position),
            HoverAction::Reference(position) => show_ref_command_link(snap, *position),
            HoverAction::GoToType(targets) => goto_type_action_links(snap, targets),
        })
        .collect()
}

const fn show_impl_command_link(
    snap: &GlobalStateSnapshot,
    position: FilePosition,
) -> Option<lsp::extensions::CommandLinkGroup> {
    None
}

const fn show_ref_command_link(
    snap: &GlobalStateSnapshot,
    position: FilePosition,
) -> Option<lsp::extensions::CommandLinkGroup> {
    None
}

const fn goto_type_action_links(
    snap: &GlobalStateSnapshot,
    targets: &[HoverGotoTypeData],
) -> Option<lsp::extensions::CommandLinkGroup> {
    None
}

mod diff {
    //! Generate minimal `TextEdit`s from different text versions
    use base_db::{TextRange, TextSize};
    use dissimilar::Chunk;
    use ide_db::text_edit::TextEdit;

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
