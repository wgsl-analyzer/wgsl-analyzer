use base_db::{FileRange, TextRange};
use hir::diagnostics::DiagnosticsConfig;
use ide::HoverResult;
use lsp_types::{GotoDefinitionResponse, LanguageString, MarkedString};
use std::process::exit;
use vfs::FileId;

use crate::global_state::GlobalStateSnapshot;
use crate::{from_proto, lsp_ext, to_proto, Result};

pub fn handle_goto_definition(
    snap: GlobalStateSnapshot,
    params: lsp_types::GotoDefinitionParams,
) -> Result<Option<GotoDefinitionResponse>> {
    let position = from_proto::file_position(&snap, params.text_document_position_params)?;
    let file_id = position.file_id;
    let nav_target = match snap.analysis.goto_definition(position)? {
        Some(nav_target) => nav_target,
        None => return Ok(None),
    };

    let range = FileRange {
        file_id: file_id,
        range: nav_target.focus_or_full_range(),
    };
    let location = to_proto::location(&snap, range)?;
    let response = GotoDefinitionResponse::Scalar(location);

    Ok(Some(response))
}

pub fn handle_completion(
    snap: GlobalStateSnapshot,
    params: lsp_types::CompletionParams,
) -> Result<Option<lsp_types::CompletionResponse>> {
    let position = from_proto::file_position(&snap, params.text_document_position.clone())?;
    let line_index = snap.analysis.line_index(position.file_id)?;
    let items = match snap.analysis.completions(position)? {
        Some(items) => items,
        None => return Ok(None),
    };
    let items = to_proto::completion_items(&line_index, params.text_document_position, items);
    let list = lsp_types::CompletionList {
        is_incomplete: true,
        items,
    };
    Ok(Some(list.into()))
}

pub fn handle_formatting(
    snap: GlobalStateSnapshot,
    params: lsp_types::DocumentFormattingParams,
) -> Result<Option<Vec<lsp_types::TextEdit>>> {
    let file_id = from_proto::file_id(&snap, &params.text_document.uri)?;
    let node = match snap.analysis.format(file_id, None)? {
        Some(node) => node,
        None => return Ok(None),
    };
    let line_index = snap.analysis.line_index(file_id)?;

    let before = snap.analysis.file_text(file_id)?;
    let after = node.to_string();

    let diff = diff::diff(&before, &after);
    let edits = to_proto::text_edit_vec(&line_index, diff);
    Ok(Some(edits))
}

pub fn handle_hover(
    snap: GlobalStateSnapshot,
    params: lsp_types::HoverParams,
) -> Result<Option<lsp_types::Hover>> {
    let position = from_proto::file_position(&snap, params.text_document_position_params.clone())?;
    let line_index = snap.analysis.line_index(position.file_id)?;
    let range = TextRange::new(position.offset, position.offset);
    let file_range = FileRange {
        file_id: position.file_id,
        range,
    };

    let result = match snap.analysis.hover(file_range)? {
        Some(hover) => hover,
        None => return Ok(None),
    };

    let hover_content = match result.info {
        HoverResult::SourceCode(code) => MarkedString::LanguageString(LanguageString {
            language: "wgsl".to_string(),
            value: code,
        }),
        HoverResult::Text(text) => MarkedString::String(text),
    };
    let hover = lsp_types::Hover {
        contents: lsp_types::HoverContents::Scalar(hover_content),
        range: Some(to_proto::range(&line_index, result.range)),
    };

    Ok(Some(hover))
}

pub fn handle_shutdown(
    _snap: GlobalStateSnapshot,
    _: (),
) -> Result<()> {
    exit(0);
}

pub fn show_syntax_tree(
    snap: GlobalStateSnapshot,
    params: lsp_ext::SyntaxTreeParams,
) -> Result<String> {
    let file_id = from_proto::file_id(&snap, &params.text_document.uri)?;
    let string = snap.analysis.syntax_tree(file_id)?;
    Ok(string)
}

pub fn debug_command(snap: GlobalStateSnapshot, params: lsp_ext::DebugCommandParams) -> Result<()> {
    let position = from_proto::file_position(&snap, params.position)?;
    snap.analysis.debug_command(position)?;

    Ok(())
}

pub fn publish_diagnostics(
    snap: &GlobalStateSnapshot,
    config: &DiagnosticsConfig,
    file_id: FileId,
) -> Result<Vec<lsp_types::Diagnostic>> {
    let line_index = snap.analysis.line_index(file_id)?;
    let diagnostics = snap.analysis.diagnostics(config, file_id)?;
    let lsp_diagnostics = diagnostics
        .into_iter()
        .map(|diagnostic| lsp_types::Diagnostic {
            range: to_proto::range(&*line_index, diagnostic.range),
            severity: Some(lsp_types::DiagnosticSeverity::Error),
            code: None,
            code_description: None,
            source: None,
            message: diagnostic.message,
            related_information: None,
            tags: None,
            data: None,
        })
        .collect();

    Ok(lsp_diagnostics)
}

mod diff {
    //! Generate minimal `TextEdit`s from different text versions
    use dissimilar::Chunk;
    use text_edit::{TextEdit, TextRange, TextSize};

    pub fn diff(left: &str, right: &str) -> TextEdit {
        let chunks = dissimilar::diff(left, right);
        textedit_from_chunks(chunks)
    }

    fn textedit_from_chunks(chunks: Vec<dissimilar::Chunk>) -> TextEdit {
        let mut builder = TextEdit::builder();
        let mut pos = TextSize::default();

        let mut chunks = chunks.into_iter().peekable();
        while let Some(chunk) = chunks.next() {
            if let (Chunk::Delete(deleted), Some(&Chunk::Insert(inserted))) = (chunk, chunks.peek())
            {
                chunks.next().unwrap();
                let deleted_len = TextSize::of(deleted);
                builder.replace(TextRange::at(pos, deleted_len), inserted.into());
                pos += deleted_len;
                continue;
            }

            match chunk {
                Chunk::Equal(text) => {
                    pos += TextSize::of(text);
                }
                Chunk::Delete(deleted) => {
                    let deleted_len = TextSize::of(deleted);
                    builder.delete(TextRange::at(pos, deleted_len));
                    pos += deleted_len;
                }
                Chunk::Insert(inserted) => {
                    builder.insert(pos, inserted.into());
                }
            }
        }
        builder.finish()
    }
}
