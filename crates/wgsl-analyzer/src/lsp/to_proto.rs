use std::ops::Not as _;

use base_db::{FileRange, TextRange, TextSize};
use ide::{
    Cancellable, Fold, FoldKind, InlayHintLabel, NavigationTarget,
    inlay_hints::{
        InlayFieldsToResolve, InlayHint as IdeInlayHint,
        InlayHintLabelPart as IdeInlayHintLabelPart, InlayKind, LazyProperty,
    },
    signature_help::SignatureHelp as IdeSignatureHelp,
};
use ide_completion::{
    CompletionFieldsToResolve,
    item::{
        CompletionItem as IdeCompletionItem, CompletionItemKind as IdeCompletionItemKind,
        CompletionRelevance,
    },
};
use ide_db::text_edit::{InsertDelete, TextEdit as IdeTextEdit};
use itertools::Itertools as _;
use lsp_types::{
    ActiveParameter, CompletionItem as LspCompletionItem,
    CompletionItemKind as LspCompletionItemKind, CompletionItemLabelDetails, CompletionItemTag,
    CompletionItemTextEdit, Definition, DefinitionResponse, Documentation, FoldingRange,
    FoldingRangeKind, InlayHint as LspInlayHint, InlayHintKind,
    InlayHintLabelPart as LspInlayHintLabelPart, InsertReplaceEdit, InsertTextFormat, Label,
    Location, LocationLink, MarkupContent, MarkupKind, ParameterInformation,
    ParameterInformationLabel, Position, Range, SignatureHelp as LspSignatureHelp,
    SignatureInformation, TextDocumentPositionParams, TextEdit as LspTextEdit, Tooltip, Uri,
};
use paths::{AbsPath, Utf8Component, Utf8Prefix};
use rustc_hash::FxHasher;
use semver::VersionReq; // spellchecker:disable-line
use serde_json::to_value;
use vfs::FileId;

use crate::{
    config::Config,
    global_state::GlobalStateSnapshot,
    line_index::{LineEndings, LineIndex, PositionEncoding},
    lsp,
};

pub(crate) fn folding_range(
    text: &str,
    line_index: &LineIndex,
    line_folding_only: bool,
    fold: &Fold,
) -> FoldingRange {
    let kind = match fold.kind {
        FoldKind::Comment => Some(FoldingRangeKind::Comment),
        FoldKind::Imports => Some(FoldingRangeKind::Imports),
        FoldKind::Region => Some(FoldingRangeKind::Region),
        FoldKind::Block
        | FoldKind::ArgList
        | FoldKind::Constants
        | FoldKind::Variables
        | FoldKind::Overrides
        | FoldKind::TypeAliases
        | FoldKind::ReturnType
        | FoldKind::Function => None,
    };

    let range = range(line_index, fold.range);

    if line_folding_only {
        // Clients with line_folding_only == true (such as VSCode) will fold the whole end line
        // even if it contains text not in the folding range. To prevent that we exclude
        // range.end.line from the folding region if there is more text after range.end
        // on the same line.
        let has_more_text_on_end_line = text[TextRange::new(fold.range.end(), TextSize::of(text))]
            .chars()
            .take_while(|item| *item != '\n')
            .any(|item| !item.is_whitespace());

        let end_line = if has_more_text_on_end_line {
            range.end.line.saturating_sub(1)
        } else {
            range.end.line
        };

        FoldingRange {
            start_line: range.start.line,
            start_character: None,
            end_line,
            end_character: None,
            kind,
            collapsed_text: None,
        }
    } else {
        FoldingRange {
            start_line: range.start.line,
            start_character: Some(range.start.character),
            end_line: range.end.line,
            end_character: Some(range.end.character),
            kind,
            collapsed_text: None,
        }
    }
}

/// Returns a [`Uri`] object from a given path, will lowercase drive letters if present.
/// This will only happen when processing windows paths.
///
/// When processing non-windows path, this is essentially the same as [`Uri::from_file_path`].
pub(crate) fn url_from_abs_path(path: &AbsPath) -> lsp_types::Uri {
    let url = lsp_types::Uri::from_file_path(path).unwrap();
    match path.components().next() {
        Some(Utf8Component::Prefix(prefix))
            if matches!(
                prefix.kind(),
                Utf8Prefix::Disk(_) | Utf8Prefix::VerbatimDisk(_)
            ) =>
        {
            // Need to lowercase driver letter
        },
        _ => return url,
    }

    let driver_letter_range = {
        let Some((scheme, drive_letter, _rest)) = url.as_str().splitn(3, ':').collect_tuple()
        else {
            return url;
        };
        let start = scheme.len() + ':'.len_utf8();
        start..(start + drive_letter.len())
    };

    // Note: lowercasing the `path` itself doesn't help, the `Url::parse`
    // machinery *also* canonicalizes the drive letter. So, just massage the
    // string in place.
    let mut url: String = url.into();
    url[driver_letter_range].make_ascii_lowercase();
    lsp_types::Uri::parse(&url).unwrap()
}

pub(crate) fn range(
    line_index: &LineIndex,
    range: TextRange,
) -> Range {
    let start = position(line_index, range.start());
    let end = position(line_index, range.end());
    Range::new(start, end)
}

pub(crate) fn position(
    line_index: &LineIndex,
    offset: TextSize,
) -> Position {
    let line_column = line_index.index.line_col(offset);
    match line_index.encoding {
        PositionEncoding::Utf8 => Position::new(line_column.line, line_column.col),
        PositionEncoding::Wide(encoding) => {
            let line_col = line_index.index.to_wide(encoding, line_column).unwrap();
            Position::new(line_col.line, line_col.col)
        },
    }
}

pub(crate) fn url(
    snap: &GlobalStateSnapshot,
    file_id: FileId,
) -> Uri {
    snap.file_id_to_url(file_id)
}

pub(crate) fn location(
    snap: &GlobalStateSnapshot,
    frange: FileRange,
) -> Cancellable<Location> {
    let url = url(snap, frange.file_id);
    let line_index = snap.file_line_index(frange.file_id)?;
    let range = range(&line_index, frange.range);
    let location = Location::new(url, range);
    Ok(location)
}

pub(crate) fn completion_items(
    config: &Config,
    fields_to_resolve: CompletionFieldsToResolve,
    line_index: &LineIndex,
    version: Option<i32>,
    tdpp: &TextDocumentPositionParams,
    completion_trigger_character: Option<char>,
    mut items: Vec<IdeCompletionItem>,
) -> Vec<LspCompletionItem> {
    if config.completion_hide_deprecated() {
        items.retain(|item| !item.deprecated);
    }

    let max_relevance = items
        .iter()
        .map(|item| item.relevance.score())
        .max()
        .unwrap_or_default();
    let mut result = Vec::with_capacity(items.len());
    for item in items {
        completion_item(
            &mut result,
            config,
            fields_to_resolve,
            line_index,
            version,
            tdpp,
            max_relevance,
            completion_trigger_character,
            &item,
        );
    }

    if let Some(limit) = config.completion(None).limit {
        result.sort_by(|item1, item2| item1.sort_text.cmp(&item2.sort_text));
        result.truncate(limit);
    }

    result
}

#[expect(clippy::too_many_arguments, reason = "TODO")]
fn completion_item(
    accumulator: &mut Vec<LspCompletionItem>,
    config: &Config,
    fields_to_resolve: CompletionFieldsToResolve,
    line_index: &LineIndex,
    version: Option<i32>,
    tdpp: &TextDocumentPositionParams,
    max_relevance: u32,
    completion_trigger_character: Option<char>,
    item: &IdeCompletionItem,
) {
    let insert_replace_support = config.insert_replace_support().then_some(tdpp.position);
    // let ref_match = item.ref_match();

    let mut additional_text_edits = Vec::new();
    let mut something_to_resolve = false;

    let filter_text = if fields_to_resolve.resolve_filter_text {
        something_to_resolve |= !item.lookup().is_empty();
        None
    } else {
        Some(item.lookup().to_owned())
    };

    let text_edit = if fields_to_resolve.resolve_text_edit {
        something_to_resolve |= true;
        None
    } else {
        // LSP does not allow arbitrary edits in completion, so we have to do a
        // non-trivial mapping here.
        let mut text_edit = None;
        let source_range = item.source_range;
        for indel in &item.text_edit {
            if indel.delete.contains_range(source_range) {
                // Extract this indel as the main edit
                text_edit = Some(if indel.delete == source_range {
                    self::completion_text_edit(line_index, insert_replace_support, indel.clone())
                } else {
                    assert!(source_range.end() == indel.delete.end());
                    let range1 = TextRange::new(indel.delete.start(), source_range.start());
                    let range2 = source_range;
                    let indel1 = InsertDelete::delete(range1);
                    let indel2 = InsertDelete::replace(range2, indel.insert.clone());
                    additional_text_edits.push(self::text_edit(line_index, indel1));
                    self::completion_text_edit(line_index, insert_replace_support, indel2)
                });
            } else {
                assert!(source_range.intersect(indel.delete).is_none());
                let text_edit = self::text_edit(line_index, indel.clone());
                additional_text_edits.push(text_edit);
            }
        }
        Some(text_edit.unwrap())
    };

    let insert_text_format = item.is_snippet.then_some(InsertTextFormat::Snippet);
    let tags = if fields_to_resolve.resolve_tags {
        something_to_resolve |= item.deprecated;
        None
    } else {
        item.deprecated.then(|| vec![CompletionItemTag::Deprecated])
    };
    // let command = if item.trigger_call_info && config.client_commands().trigger_parameter_hints {
    //     if fields_to_resolve.resolve_command {
    //         something_to_resolve |= true;
    //         None
    //     } else {
    //         Some(command::trigger_parameter_hints())
    //     }
    // } else {
    //     None
    // };

    let detail = if fields_to_resolve.resolve_detail {
        something_to_resolve |= item.detail.is_some();
        None
    } else {
        item.detail.clone()
    };

    // let documentation = if fields_to_resolve.resolve_documentation {
    //     something_to_resolve |= item.documentation.is_some();
    //     None
    // } else {
    //     item.documentation.clone().map(documentation)
    // };

    let mut lsp_item = LspCompletionItem {
        label: item.label.primary.to_string(),
        detail,
        filter_text,
        kind: Some(completion_item_kind(item.kind)),
        text_edit,
        additional_text_edits: additional_text_edits
            .is_empty()
            .not()
            .then_some(additional_text_edits),
        // documentation,
        #[expect(deprecated, reason = "we do give tags")]
        deprecated: item.deprecated.then_some(item.deprecated),
        tags,
        // command,
        insert_text_format,
        ..Default::default()
    };

    if config.completion_label_details_support() {
        let has_label_details =
            item.label.detail_left.is_some() || item.label.detail_right.is_some();
        if fields_to_resolve.resolve_label_details {
            something_to_resolve |= has_label_details;
        } else if has_label_details {
            lsp_item.label_details = Some(CompletionItemLabelDetails {
                detail: item.label.detail_left.clone(),
                description: item.label.detail_right.clone(),
            });
        }
    } else if let Some(label_detail) = &item.label.detail_left {
        lsp_item.label.push_str(label_detail.as_str());
    }

    set_score(&mut lsp_item, max_relevance, item.relevance);

    // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/914
    // let imports =
    //     if config.completion(None).enable_imports_on_the_fly && !item.import_to_add.is_empty() {
    //         item.import_to_add
    //             .clone()
    //             .into_iter()
    //             .map(|import_path| lsp::extensions::CompletionImport {
    //                 full_import_path: import_path,
    //             })
    //             .collect()
    //     } else {
    //         Vec::new()
    //     };
    // let (ref_resolve_data, resolve_data) = if something_to_resolve || !imports.is_empty() {
    //     let ref_resolve_data = if ref_match.is_some() {
    //         let ref_resolve_data = lsp::extensions::CompletionResolveData {
    //             position: tdpp.clone(),
    //             imports: Vec::new(),
    //             version,
    //             trigger_character: completion_trigger_character,
    //             for_ref: true,
    //             hash: BASE64_STANDARD.encode(completion_item_hash(&item, true)),
    //         };
    //         Some(to_value(ref_resolve_data).unwrap())
    //     } else {
    //         None
    //     };
    //     let resolve_data = lsp::extensions::CompletionResolveData {
    //         position: tdpp.clone(),
    //         imports,
    //         version,
    //         trigger_character: completion_trigger_character,
    //         for_ref: false,
    //         hash: BASE64_STANDARD.encode(completion_item_hash(&item, false)),
    //     };
    //     (ref_resolve_data, Some(to_value(resolve_data).unwrap()))
    // } else {
    //     (None, None)
    // };

    // if let Some((label, indel, relevance)) = ref_match {
    //     let mut lsp_item_with_ref = CompletionItem {
    //         label,
    //         data: ref_resolve_data,
    //         ..lsp_item.clone()
    //     };
    //     lsp_item_with_ref
    //         .additional_text_edits
    //         .get_or_insert_with(Default::default)
    //         .push(self::text_edit(line_index, indel));
    //     set_score(&mut lsp_item_with_ref, max_relevance, relevance);
    //     acc.push(lsp_item_with_ref);
    // };

    // lsp_item.data = resolve_data;
    accumulator.push(lsp_item);

    fn set_score(
        result: &mut LspCompletionItem,
        max_relevance: u32,
        relevance: CompletionRelevance,
    ) {
        if relevance.is_relevant() && relevance.score() == max_relevance {
            result.preselect = Some(true);
        }
        // The relevance needs to be inverted to come up with a sort score
        // because the client will sort ascending.
        let sort_score = relevance.score() ^ 0xff_ff_ff_ff;
        // Zero pad the string to ensure values can be properly sorted
        // by the client. Hex format is used because it is easier to
        // visually compare very large values, which the sort text
        // tends to be since it is the opposite of the score.
        result.sort_text = Some(format!("{sort_score:08x}"));
    }
}

pub(crate) const fn completion_item_kind(
    completion_item_kind: IdeCompletionItemKind
) -> LspCompletionItemKind {
    match completion_item_kind {
        IdeCompletionItemKind::Field => LspCompletionItemKind::Field,
        IdeCompletionItemKind::Function => LspCompletionItemKind::Function,
        IdeCompletionItemKind::Variable => LspCompletionItemKind::Variable,
        IdeCompletionItemKind::Keyword => LspCompletionItemKind::Keyword,
        IdeCompletionItemKind::Snippet => LspCompletionItemKind::Snippet,
        IdeCompletionItemKind::Constant => LspCompletionItemKind::Constant,
        IdeCompletionItemKind::Module => LspCompletionItemKind::Module,
        IdeCompletionItemKind::TypeAlias | IdeCompletionItemKind::Struct => {
            LspCompletionItemKind::Struct
        },
    }
}

pub(crate) fn text_edit(
    line_index: &LineIndex,
    indel: InsertDelete,
) -> LspTextEdit {
    let range = range(line_index, indel.delete);
    let new_text = match line_index.endings {
        LineEndings::Unix => indel.insert,
        LineEndings::Dos => indel.insert.replace('\n', "\r\n"),
    };
    LspTextEdit { range, new_text }
}

pub(crate) fn text_edit_vec(
    line_index: &LineIndex,
    text_edit: IdeTextEdit,
) -> Vec<LspTextEdit> {
    text_edit
        .into_iter()
        .map(|indel| self::text_edit(line_index, indel))
        .collect()
}

pub(crate) fn completion_text_edit(
    line_index: &LineIndex,
    insert_replace_support: Option<Position>,
    indel: InsertDelete,
) -> CompletionItemTextEdit {
    let text_edit = text_edit(line_index, indel);
    match insert_replace_support {
        Some(cursor_pos) => InsertReplaceEdit {
            new_text: text_edit.new_text,
            insert: Range {
                start: text_edit.range.start,
                end: cursor_pos,
            },
            replace: text_edit.range,
        }
        .into(),
        None => text_edit.into(),
    }
}

pub(crate) fn inlay_hint(
    snap: &GlobalStateSnapshot,
    fields_to_resolve: InlayFieldsToResolve,
    line_index: &LineIndex,
    file_id: FileId,
    mut inlay_hint: IdeInlayHint,
) -> Cancellable<LspInlayHint> {
    let hint_needs_resolve = |hint: &IdeInlayHint| -> Option<TextRange> {
        hint.resolve_parent.filter(|_| {
            hint.text_edit.as_ref().is_some_and(LazyProperty::is_lazy)
                || hint.label.parts.iter().any(|part| {
                    part.linked_location
                        .as_ref()
                        .is_some_and(LazyProperty::is_lazy)
                        || part.tooltip.as_ref().is_some_and(LazyProperty::is_lazy)
                })
        })
    };

    let resolve_range_and_hash = hint_needs_resolve(&inlay_hint).map(|range| {
        (
            range,
            std::hash::BuildHasher::hash_one(
                &std::hash::BuildHasherDefault::<FxHasher>::default(),
                &inlay_hint,
            ),
        )
    });

    let mut something_to_resolve = false;
    let text_edits = inlay_hint
        .text_edit
        .take()
        .and_then(|property| match property {
            LazyProperty::Computed(text_edit) => Some(text_edit),
            LazyProperty::Lazy => {
                something_to_resolve |=
                    resolve_range_and_hash.is_some() && fields_to_resolve.resolve_text_edits;
                None
            },
        })
        .map(|text_edit| text_edit_vec(line_index, text_edit));
    let (label, tooltip) = inlay_hint_label(
        snap,
        fields_to_resolve,
        &mut something_to_resolve,
        resolve_range_and_hash.is_some(),
        inlay_hint.label,
    )?;

    let data = match resolve_range_and_hash {
        Some((resolve_range, hash)) if something_to_resolve => Some(
            to_value(lsp::extensions::InlayHintResolveData {
                file_id: file_id.index(),
                hash: hash.to_string(),
                resolve_range: range(line_index, resolve_range),
                version: snap.file_version(file_id),
            })
            .unwrap(),
        ),
        _ => None,
    };

    Ok(LspInlayHint {
        position: match inlay_hint.position {
            ide::InlayHintPosition::Before => position(line_index, inlay_hint.range.start()),
            ide::InlayHintPosition::After => position(line_index, inlay_hint.range.end()),
        },
        padding_left: Some(inlay_hint.pad_left),
        padding_right: Some(inlay_hint.pad_right),
        kind: match inlay_hint.kind {
            InlayKind::Parameter => Some(InlayHintKind::Parameter),
            InlayKind::Type => Some(InlayHintKind::Type),
            InlayKind::StructLayout => None,
        },
        text_edits,
        data,
        tooltip,
        label,
    })
}

fn inlay_hint_label(
    snap: &GlobalStateSnapshot,
    fields_to_resolve: InlayFieldsToResolve,
    something_to_resolve: &mut bool,
    needs_resolve: bool,
    mut label: InlayHintLabel,
) -> Cancellable<(Label, Option<Tooltip>)> {
    let (label, tooltip) = if let [
        IdeInlayHintLabelPart {
            linked_location: None,
            ..
        },
    ] = &*label.parts
    {
        let IdeInlayHintLabelPart { text, tooltip, .. } = label.parts.pop().unwrap();
        let tooltip = tooltip.and_then(|inlay_tooltip| match inlay_tooltip {
            LazyProperty::Computed(inlay_tooltip) => Some(inlay_tooltip),
            LazyProperty::Lazy => {
                *something_to_resolve |= needs_resolve && fields_to_resolve.resolve_hint_tooltip;
                None
            },
        });
        let hint_tooltip = match tooltip {
            Some(ide::InlayTooltip::String(string)) => Some(Tooltip::String(string)),
            Some(ide::InlayTooltip::Markdown(string)) => {
                Some(Tooltip::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: string,
                }))
            },
            None => None,
        };
        (Label::String(text), hint_tooltip)
    } else {
        let parts = label
            .parts
            .into_iter()
            .map(|part| {
                let tooltip = part.tooltip.and_then(|property| match property {
                    LazyProperty::Computed(inlay_tooltip) => Some(inlay_tooltip),
                    LazyProperty::Lazy => {
                        *something_to_resolve |= fields_to_resolve.resolve_label_tooltip;
                        None
                    },
                });
                let tooltip = match tooltip {
                    Some(ide::InlayTooltip::String(string)) => Some(Tooltip::String(string)),
                    Some(ide::InlayTooltip::Markdown(source)) => {
                        Some(Tooltip::MarkupContent(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: source,
                        }))
                    },
                    None => None,
                };
                let location = part
                    .linked_location
                    .and_then(|property| match property {
                        LazyProperty::Computed(file_range) => Some(file_range),
                        LazyProperty::Lazy => {
                            *something_to_resolve |= fields_to_resolve.resolve_label_location;
                            None
                        },
                    })
                    .map(|range| location(snap, range))
                    .transpose()?;
                Ok(LspInlayHintLabelPart {
                    value: part.text,
                    tooltip,
                    location,
                    command: None,
                })
            })
            .collect::<Cancellable<_>>()?;
        (Label::InlayHintLabelPartList(parts), None)
    };
    Ok((label, tooltip))
}

pub(crate) fn location_link(
    snap: &GlobalStateSnapshot,
    source: Option<FileRange>,
    target: &NavigationTarget,
) -> Cancellable<LocationLink> {
    let origin_selection_range = match source {
        Some(source) => {
            let line_index = snap.file_line_index(source.file_id)?;
            let range = range(&line_index, source.range);
            Some(range)
        },
        None => None,
    };
    let (target_uri, target_range, target_selection_range) = location_info(snap, target)?;
    let result = LocationLink {
        origin_selection_range,
        target_uri,
        target_range,
        target_selection_range,
    };
    Ok(result)
}

fn location_info(
    snap: &GlobalStateSnapshot,
    target: &NavigationTarget,
) -> Cancellable<(Uri, Range, Range)> {
    let line_index = snap.file_line_index(target.file_id)?;

    let target_uri = url(snap, target.file_id);
    let target_range = range(&line_index, target.full_range);
    let target_selection_range = target
        .focus_range
        .map_or(target_range, |text_range| range(&line_index, text_range));
    Ok((target_uri, target_range, target_selection_range))
}

pub(crate) fn goto_definition_response(
    snap: &GlobalStateSnapshot,
    source: Option<FileRange>,
    targets: Vec<NavigationTarget>,
) -> Cancellable<DefinitionResponse> {
    if snap.config.location_link() {
        let links = targets
            .into_iter()
            .unique_by(|navigation_target| {
                (
                    navigation_target.file_id,
                    navigation_target.full_range,
                    navigation_target.focus_range,
                )
            })
            .map(|navigation_target| location_link(snap, source, &navigation_target))
            .collect::<Cancellable<Vec<_>>>()?;
        Ok(links.into())
    } else {
        let locations = targets
            .into_iter()
            .map(|navigation_target| FileRange {
                file_id: navigation_target.file_id,
                range: navigation_target.focus_or_full_range(),
            })
            .unique()
            .map(|range| location(snap, range))
            .collect::<Cancellable<Vec<_>>>()?;
        Ok(DefinitionResponse::Definition(Definition::LocationList(
            locations,
        )))
    }
}

pub(crate) fn signature_help(
    help: IdeSignatureHelp,
    // config: CallInfoConfig,
    label_offsets: bool,
    active: Option<u32>,
) -> LspSignatureHelp {
    let signatures = help
        .signatures
        .into_iter()
        .map(|call_info| {
            let parameters = if label_offsets {
                call_info
                    .parameter_ranges()
                    .iter()
                    .map(|text_range| {
                        let start = call_info.signature[..text_range.start().into()]
                            .chars()
                            .map(char::len_utf16)
                            .sum::<usize>();
                        #[expect(
                            clippy::as_conversions,
                            clippy::cast_possible_truncation,
                            reason = "a text offset does not exceed u32 in practice"
                        )]
                        let start = start as u32;
                        let offset = call_info.signature
                            [text_range.start().into()..text_range.end().into()]
                            .chars()
                            .map(char::len_utf16)
                            .sum::<usize>();
                        #[expect(
                            clippy::as_conversions,
                            clippy::cast_possible_truncation,
                            reason = "a text offset does not exceed u32 in practice"
                        )]
                        let offset = offset as u32;
                        let end = start + offset;
                        (start, end)
                    })
                    .map(|label_offsets| ParameterInformation {
                        label: ParameterInformationLabel::Tuple(label_offsets),
                        documentation: None,
                    })
                    .collect::<Vec<_>>()
            } else {
                call_info
                    .parameter_labels()
                    .map(|label| ParameterInformation {
                        label: ParameterInformationLabel::String(label.to_owned()),
                        documentation: None,
                    })
                    .collect::<Vec<_>>()
            };
            let signature_doc = call_info.documentation.map(|doc| {
                Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: doc,
                })
            });
            SignatureInformation {
                label: call_info.signature,
                documentation: signature_doc,
                parameters: Some(parameters),
                active_parameter: None,
            }
        })
        .collect();
    LspSignatureHelp {
        signatures,
        active_signature: active,
        active_parameter: help.active_parameter.map(ActiveParameter::Int), // should this be limited by `signatures[active].parameters.len()`?
    }
}

#[cfg(test)]
mod tests {
    use ParameterInformation;
    use ParameterInformationLabel;
    use base_db::FilePosition;
    use expect_test::{Expect, expect};
    use ide::Analysis;
    use test_utils::extract_offset;
    use triomphe::Arc;

    use super::*;

    #[test]
    fn signature_help_no_label_offsets() {
        // TODO: add signature help documentation to this test
        let text = r#"
fn foo() {
    bar(2, $0);
}
fn bar(x: u32, y: bool, z: bool) -> f32 { 0.0f }
fn bar(x: u32, y: bool) -> f32 { 0.0f }
fn bar() -> f32 { 0.0f }
"#;

        let (offset, text) = extract_offset(text);
        let (analysis, file_id) = Analysis::from_single_file(text);
        let help = signature_help(
            analysis
                .signature_help(FilePosition { file_id, offset })
                .unwrap()
                .unwrap(),
            // TODO: add config
            // CallInfoConfig {
            //     parameters_only: false,
            //     documentation: true,
            // },
            false,
            None,
        );
        #[expect(clippy::as_conversions, reason = "usize >= u32")]
        assert_eq!(
            help,
            LspSignatureHelp {
                signatures: vec![
                    SignatureInformation {
                        label: "fn bar(x: u32, y: bool, z: bool) -> f32".to_owned(),
                        documentation: None,
                        parameters: Some(vec![
                            ParameterInformation {
                                label: ParameterInformationLabel::String("x: u32".to_owned()),
                                documentation: None,
                            },
                            ParameterInformation {
                                label: ParameterInformationLabel::String("y: bool".to_owned()),
                                documentation: None,
                            },
                            ParameterInformation {
                                label: ParameterInformationLabel::String("z: bool".to_owned()),
                                documentation: None,
                            }
                        ]),
                        active_parameter: None,
                    },
                    SignatureInformation {
                        label: "fn bar(x: u32, y: bool) -> f32".to_owned(),
                        documentation: None,
                        parameters: Some(vec![
                            ParameterInformation {
                                label: ParameterInformationLabel::String("x: u32".to_owned()),
                                documentation: None,
                            },
                            ParameterInformation {
                                label: ParameterInformationLabel::String("y: bool".to_owned()),
                                documentation: None,
                            },
                        ]),
                        active_parameter: None,
                    }
                ],
                active_parameter: Some(ActiveParameter::Int(1)),
                active_signature: None,
            }
        );
    }

    #[test]
    fn signature_help_with_label_offsets() {
        // TODO: add signature help documentation to this test
        let text = r#"
fn foo() {
    bar($0);
}
fn bar(x: u32, y: bool) -> f32 { 0.0f }
"#;

        let (offset, text) = extract_offset(text);
        let (analysis, file_id) = Analysis::from_single_file(text);
        let help = signature_help(
            analysis
                .signature_help(FilePosition { file_id, offset })
                .unwrap()
                .unwrap(),
            // TODO: add config
            // CallInfoConfig {
            //     parameters_only: false,
            //     documentation: true,
            // },
            true,
            None,
        );
        #[expect(clippy::as_conversions, reason = "usize >= u32")]
        let found = &help.signatures[help.active_signature.unwrap_or_default() as usize];
        assert_eq!(found.label, "fn bar(x: u32, y: bool) -> f32");
        assert_eq!(
            found.parameters,
            Some(vec![
                ParameterInformation {
                    label: ParameterInformationLabel::Tuple((7, 13)),
                    documentation: None
                },
                ParameterInformation {
                    label: ParameterInformationLabel::Tuple((15, 22)),
                    documentation: None
                }
            ])
        );
    }
}
