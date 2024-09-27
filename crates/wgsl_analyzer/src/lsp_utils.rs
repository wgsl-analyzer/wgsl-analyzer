#![allow(dead_code)]
//! Utilities for LSP-related boilerplate code.
use std::{error::Error, ops::Range, sync::Arc};

use lsp_server::Notification;

use crate::{
    from_proto,
    global_state::GlobalState,
    line_index::{LineEndings, LineIndex, OffsetEncoding},
    LspError,
};

pub(crate) fn is_cancelled(e: &(dyn Error + 'static)) -> bool {
    e.downcast_ref::<salsa::Cancelled>().is_some()
}

pub(crate) fn invalid_params_error(message: String) -> LspError {
    LspError {
        code: lsp_server::ErrorCode::InvalidParams as i32,
        message,
    }
}

pub(crate) fn notification_is<N: lsp_types::notification::Notification>(
    notification: &Notification,
) -> bool {
    notification.method == N::METHOD
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Progress {
    Begin,
    Report,
    End,
}

impl Progress {
    pub(crate) fn fraction(done: usize, total: usize) -> f64 {
        assert!(done <= total);
        done as f64 / total.max(1) as f64
    }
}

impl GlobalState {
    pub(crate) fn show_message(&mut self, typ: lsp_types::MessageType, message: String) {
        self.send_notification::<lsp_types::notification::ShowMessage>(
            lsp_types::ShowMessageParams { typ, message },
        )
    }

    pub(crate) fn report_progress(
        &mut self,
        title: &str,
        state: Progress,
        message: Option<String>,
        fraction: Option<f64>,
    ) {
        /*if !self.config.work_done_progress() {
            return;
        }*/
        let percentage = fraction.map(|f| {
            assert!((0.0..=1.0).contains(&f));
            (f * 100.0) as u32
        });
        let token = lsp_types::ProgressToken::String(format!("rustAnalyzer/{}", title));
        let work_done_progress = match state {
            Progress::Begin => {
                self.send_request::<lsp_types::request::WorkDoneProgressCreate>(
                    lsp_types::WorkDoneProgressCreateParams {
                        token: token.clone(),
                    },
                    |_, _| (),
                );

                lsp_types::WorkDoneProgress::Begin(lsp_types::WorkDoneProgressBegin {
                    title: title.into(),
                    cancellable: None,
                    message,
                    percentage,
                })
            }
            Progress::Report => {
                lsp_types::WorkDoneProgress::Report(lsp_types::WorkDoneProgressReport {
                    cancellable: None,
                    message,
                    percentage,
                })
            }
            Progress::End => {
                lsp_types::WorkDoneProgress::End(lsp_types::WorkDoneProgressEnd { message })
            }
        };
        self.send_notification::<lsp_types::notification::Progress>(lsp_types::ProgressParams {
            token,
            value: lsp_types::ProgressParamsValue::WorkDone(work_done_progress),
        });
    }
}

pub(crate) fn apply_document_changes(
    old_text: &mut String,
    content_changes: Vec<lsp_types::TextDocumentContentChangeEvent>,
) {
    let mut line_index = LineIndex {
        index: Arc::new(base_db::line_index::LineIndex::new(old_text)),
        // We don't care about line endings or offset encoding here.
        endings: LineEndings::Unix,
        encoding: OffsetEncoding::Utf16,
    };

    // The changes we got must be applied sequentially, but can cross lines so we
    // have to keep our line index updated.
    // Some clients (e.g. Code) sort the ranges in reverse. As an optimization, we
    // remember the last valid line in the index and only rebuild it if needed.
    // The VFS will normalize the end of lines to `\n`.
    enum IndexValid {
        All,
        UpToLineExclusive(u32),
    }

    impl IndexValid {
        fn covers(&self, line: u32) -> bool {
            match *self {
                IndexValid::UpToLineExclusive(to) => to > line,
                _ => true,
            }
        }
    }

    let mut index_valid = IndexValid::All;
    for change in content_changes {
        match change.range {
            Some(range) => {
                if !index_valid.covers(range.end.line) {
                    line_index.index = Arc::new(base_db::line_index::LineIndex::new(old_text));
                }
                index_valid = IndexValid::UpToLineExclusive(range.start.line);
                if let Ok(range) = from_proto::text_range(&line_index, range) {
                    old_text.replace_range(Range::<usize>::from(range), &change.text);
                }
            }
            None => {
                *old_text = change.text;
                index_valid = IndexValid::UpToLineExclusive(0);
            }
        }
    }
}

/// Checks that the edits inside the completion and the additional edits do not overlap.
/// LSP explicitly forbids the additional edits to overlap both with the main edit and themselves.
pub(crate) fn all_edits_are_disjoint(
    completion: &lsp_types::CompletionItem,
    additional_edits: &[lsp_types::TextEdit],
) -> bool {
    let mut edit_ranges = Vec::new();
    match completion.text_edit.as_ref() {
        Some(lsp_types::CompletionTextEdit::Edit(edit)) => {
            edit_ranges.push(edit.range);
        }
        Some(lsp_types::CompletionTextEdit::InsertAndReplace(edit)) => {
            let replace = edit.replace;
            let insert = edit.insert;
            if replace.start != insert.start
                || insert.start > insert.end
                || insert.end > replace.end
            {
                // insert has to be a prefix of replace but it is not
                return false;
            }
            edit_ranges.push(replace);
        }
        None => {}
    }
    if let Some(additional_changes) = completion.additional_text_edits.as_ref() {
        edit_ranges.extend(additional_changes.iter().map(|edit| edit.range));
    };
    edit_ranges.extend(additional_edits.iter().map(|edit| edit.range));
    edit_ranges.sort_by_key(|range| (range.start, range.end));
    edit_ranges
        .iter()
        .zip(edit_ranges.iter().skip(1))
        .all(|(previous, next)| previous.end <= next.start)
}
