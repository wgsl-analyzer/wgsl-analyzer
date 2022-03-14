use base_db::{line_index::LineIndex, FileRange, TextRange, TextSize};
use ide_completion::item::{CompletionItem, CompletionItemKind, CompletionRelevance};
use itertools::Itertools;
use paths::AbsPath;
use std::path;
use text_edit::{Indel, TextEdit};
use vfs::FileId;

use crate::{global_state::GlobalStateSnapshot, Result};

/// Returns a `Url` object from a given path, will lowercase drive letters if present.
/// This will only happen when processing windows paths.
///
/// When processing non-windows path, this is essentially the same as `Url::from_file_path`.
pub(crate) fn url_from_abs_path(path: &AbsPath) -> lsp_types::Url {
    let url = lsp_types::Url::from_file_path(path).unwrap();
    match path.as_ref().components().next() {
        Some(path::Component::Prefix(prefix))
            if matches!(
                prefix.kind(),
                path::Prefix::Disk(_) | path::Prefix::VerbatimDisk(_)
            ) =>
        {
            // Need to lowercase driver letter
        }
        _ => return url,
    }

    let driver_letter_range = {
        let (scheme, drive_letter, _rest) = match url.as_str().splitn(3, ':').collect_tuple() {
            Some(it) => it,
            None => return url,
        };
        let start = scheme.len() + ':'.len_utf8();
        start..(start + drive_letter.len())
    };

    // Note: lowercasing the `path` itself doesn't help, the `Url::parse`
    // machinery *also* canonicalizes the drive letter. So, just massage the
    // string in place.
    let mut url: String = url.into();
    url[driver_letter_range].make_ascii_lowercase();
    lsp_types::Url::parse(&url).unwrap()
}

pub(crate) fn range(line_index: &LineIndex, range: TextRange) -> lsp_types::Range {
    let start = position(line_index, range.start());
    let end = position(line_index, range.end());
    lsp_types::Range::new(start, end)
}

pub(crate) fn position(line_index: &LineIndex, offset: TextSize) -> lsp_types::Position {
    let line_col = line_index.line_col(offset);
    lsp_types::Position::new(line_col.line, line_col.col)
}

pub(crate) fn url(snap: &GlobalStateSnapshot, file_id: FileId) -> lsp_types::Url {
    snap.file_id_to_url(file_id)
}

pub(crate) fn location(
    snap: &GlobalStateSnapshot,
    frange: FileRange,
) -> Result<lsp_types::Location> {
    let url = url(snap, frange.file_id);
    let line_index = snap.analysis.line_index(frange.file_id)?;
    let range = range(&line_index, frange.range);
    let loc = lsp_types::Location::new(url, range);
    Ok(loc)
}

pub(crate) fn completion_items(
    // config: &Config,
    line_index: &LineIndex,
    tdpp: lsp_types::TextDocumentPositionParams,
    items: Vec<CompletionItem>,
) -> Vec<lsp_types::CompletionItem> {
    let max_relevance = items
        .iter()
        .map(|it| it.relevance().score())
        .min()
        .unwrap_or_default();
    let mut res = Vec::with_capacity(items.len());
    for item in items {
        completion_item(&mut res, line_index, &tdpp, max_relevance, item)
    }
    res
}

fn completion_item(
    acc: &mut Vec<lsp_types::CompletionItem>,
    // config: &Config,
    line_index: &LineIndex,
    _tdpp: &lsp_types::TextDocumentPositionParams,
    max_relevance: u32,
    item: CompletionItem,
) {
    let mut additional_text_edits = Vec::new();

    // LSP does not allow arbitrary edits in completion, so we have to do a
    // non-trivial mapping here.
    let text_edit = {
        let mut text_edit = None;
        let source_range = item.source_range();
        for indel in item.text_edit().iter() {
            if indel.delete.contains_range(source_range) {
                // let insert_replace_support = config.insert_replace_support().then(|| tdpp.position);
                let insert_replace_support = None;
                text_edit = Some(if indel.delete == source_range {
                    self::completion_text_edit(line_index, insert_replace_support, indel.clone())
                } else {
                    assert!(source_range.end() == indel.delete.end());
                    let range1 = TextRange::new(indel.delete.start(), source_range.start());
                    let range2 = source_range;
                    let indel1 = Indel::replace(range1, String::new());
                    let indel2 = Indel::replace(range2, indel.insert.clone());
                    additional_text_edits.push(self::text_edit(line_index, indel1));
                    self::completion_text_edit(line_index, insert_replace_support, indel2)
                })
            } else {
                assert!(source_range.intersect(indel.delete).is_none());
                let text_edit = self::text_edit(line_index, indel.clone());
                additional_text_edits.push(text_edit);
            }
        }
        text_edit.unwrap()
    };

    let mut lsp_item = lsp_types::CompletionItem {
        label: item.label().to_string(),
        detail: item.detail().map(|it| it.to_string()),
        filter_text: Some(item.lookup().to_string()),
        kind: Some(completion_item_kind(item.kind())),
        text_edit: Some(text_edit),
        additional_text_edits: Some(additional_text_edits),
        // documentation: item.documentation().map(documentation),
        // deprecated: Some(item.deprecated()),
        documentation: None,
        deprecated: Some(false),
        ..Default::default()
    };

    set_score(&mut lsp_item, max_relevance, item.relevance());

    // if item.deprecated() {
    //     lsp_item.tags = Some(vec![lsp_types::CompletionItemTag::DEPRECATED])
    // }

    // if item.trigger_call_info() && config.client_commands().trigger_parameter_hints {
    //     lsp_item.command = Some(command::trigger_parameter_hints());
    // }

    if item.is_snippet() {
        lsp_item.insert_text_format = Some(lsp_types::InsertTextFormat::SNIPPET);
    }
    /*if config.completion().enable_imports_on_the_fly {
        if let imports @ [_, ..] = item.imports_to_add() {
            let imports: Vec<_> = imports
                .iter()
                .filter_map(|import_edit| {
                    let import_path = &import_edit.import.import_path;
                    let import_name = import_path.segments().last()?;
                    Some(lsp_ext::CompletionImport {
                        full_import_path: import_path.to_string(),
                        imported_name: import_name.to_string(),
                    })
                })
                .collect();
            if !imports.is_empty() {
                let data = lsp_ext::CompletionResolveData {
                    position: tdpp.clone(),
                    imports,
                };
                lsp_item.data = Some(to_value(data).unwrap());
            }
        }
    }*/

    /*if let Some((mutability, relevance)) = item.ref_match() {
        let mut lsp_item_with_ref = lsp_item.clone();
        set_score(&mut lsp_item_with_ref, max_relevance, relevance);
        lsp_item_with_ref.label = format!(
            "&{}{}",
            mutability.as_keyword_for_ref(),
            lsp_item_with_ref.label
        );
        if let Some(it) = &mut lsp_item_with_ref.text_edit {
            let new_text = match it {
                lsp_types::CompletionTextEdit::Edit(it) => &mut it.new_text,
                lsp_types::CompletionTextEdit::InsertAndReplace(it) => &mut it.new_text,
            };
            *new_text = format!("&{}{}", mutability.as_keyword_for_ref(), new_text);
        }

        acc.push(lsp_item_with_ref);
    };*/

    acc.push(lsp_item);

    fn set_score(
        res: &mut lsp_types::CompletionItem,
        max_relevance: u32,
        relevance: CompletionRelevance,
    ) {
        if relevance.score() == max_relevance {
            res.preselect = Some(true);
        }
        // Zero pad the string to ensure values can be properly sorted
        // by the client. Hex format is used because it is easier to
        // visually compare very large values.
        res.sort_text = Some(format!("{:08x}", relevance.score()));
    }
}

pub(crate) fn completion_item_kind(
    completion_item_kind: CompletionItemKind,
) -> lsp_types::CompletionItemKind {
    match completion_item_kind {
        CompletionItemKind::Field => lsp_types::CompletionItemKind::FIELD,
        CompletionItemKind::Function => lsp_types::CompletionItemKind::FUNCTION,
        CompletionItemKind::Variable => lsp_types::CompletionItemKind::VARIABLE,
        CompletionItemKind::Keyword => lsp_types::CompletionItemKind::KEYWORD,
        CompletionItemKind::Snippet => lsp_types::CompletionItemKind::SNIPPET,
        CompletionItemKind::Constant => lsp_types::CompletionItemKind::CONSTANT,
        CompletionItemKind::Struct => lsp_types::CompletionItemKind::STRUCT,
        CompletionItemKind::Module => lsp_types::CompletionItemKind::MODULE,
        CompletionItemKind::TypeAlias => lsp_types::CompletionItemKind::STRUCT,
    }
}

pub(crate) fn text_edit(line_index: &LineIndex, indel: Indel) -> lsp_types::TextEdit {
    let range = range(line_index, indel.delete);
    lsp_types::TextEdit {
        range,
        new_text: indel.insert,
    }
}

pub(crate) fn text_edit_vec(
    line_index: &LineIndex,
    text_edit: TextEdit,
) -> Vec<lsp_types::TextEdit> {
    text_edit
        .into_iter()
        .map(|indel| self::text_edit(line_index, indel))
        .collect()
}

pub(crate) fn completion_text_edit(
    line_index: &LineIndex,
    insert_replace_support: Option<lsp_types::Position>,
    indel: Indel,
) -> lsp_types::CompletionTextEdit {
    let text_edit = text_edit(line_index, indel);
    match insert_replace_support {
        Some(cursor_pos) => lsp_types::InsertReplaceEdit {
            new_text: text_edit.new_text,
            insert: lsp_types::Range {
                start: text_edit.range.start,
                end: cursor_pos,
            },
            replace: text_edit.range,
        }
        .into(),
        None => text_edit.into(),
    }
}
