//! Utilities for LSP-related boilerplate code.
use crate::{
    LspError,
    global_state::GlobalState,
    line_index::{LineEndings, LineIndex},
};
use crate::{line_index::PositionEncoding, lsp::from_proto};
use lsp_server::Notification;
use lsp_types::request::Request as _;
use std::{error::Error, ops::Range};
use triomphe::Arc;

pub(crate) fn is_cancelled(error: &(dyn Error + 'static)) -> bool {
    error.downcast_ref::<salsa::Cancelled>().is_some()
}

#[expect(clippy::as_conversions, reason = "valid according to JSON RPC")]
pub(crate) const fn invalid_params_error(message: String) -> LspError {
    LspError {
        code: lsp_server::ErrorCode::InvalidParams as i32,
        message,
    }
}

pub(crate) fn notification_is<N: lsp_types::notification::Notification>(
    notification: &Notification
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
    #[expect(clippy::as_conversions, reason = "necessary to obtain a decimal value")]
    pub(crate) fn fraction(
        done: usize,
        total: usize,
    ) -> f64 {
        assert!(done <= total);
        done as f64 / total.max(1) as f64
    }
}

impl GlobalState {
    pub(crate) fn show_message(
        &mut self,
        r#type: lsp_types::MessageType,
        message: String,
        show_open_log_button: bool,
    ) {
        if self.config.open_server_logs() && show_open_log_button {
            self.send_request::<lsp_types::request::ShowMessageRequest>(
                lsp_types::ShowMessageRequestParams {
                    typ: r#type,
                    message,
                    actions: Some(vec![lsp_types::MessageActionItem {
                        title: "Open server logs".to_owned(),
                        properties: <_>::default(),
                    }]),
                },
                |this, response| {
                    let lsp_server::Response {
                        error: None,
                        result: Some(result),
                        ..
                    } = response
                    else {
                        return;
                    };
                    if let Ok(Some(_item)) = crate::from_json::<
						<lsp_types::request::ShowMessageRequest as lsp_types::request::Request>::Result
					>(
						lsp_types::request::ShowMessageRequest::METHOD, &result) {
						this.send_notification::<super::extensions::OpenServerLogs>(());
					}
                },
            );
        } else {
            self.send_notification::<lsp_types::notification::ShowMessage>(
                lsp_types::ShowMessageParams {
                    typ: r#type,
                    message,
                },
            );
        }
    }

    /// If `additional_info` is [`Some`], appends a note to the notification telling to check the logs.
    /// This will always log `message` + `additional_info` to the server's error log.
    pub(crate) fn show_and_log_error(
        &mut self,
        message: String,
        additional_info: Option<String>,
    ) {
        if let Some(additional_info) = additional_info {
            tracing::error!("{message}:\n{additional_info}");
            self.show_message(
                lsp_types::MessageType::ERROR,
                message,
                tracing::enabled!(tracing::Level::ERROR),
            );
        } else {
            tracing::error!("{message}");
            self.send_notification::<lsp_types::notification::ShowMessage>(
                lsp_types::ShowMessageParams {
                    typ: lsp_types::MessageType::ERROR,
                    message,
                },
            );
        }
    }

    /// rust-analyzer is resilient -- if it fails, this doesn't usually affect
    /// the user experience. Part of that is that we deliberately hide panics
    /// from the user.
    ///
    /// We do however want to pester rust-analyzer developers with panics and
    /// other "you really gotta fix that" messages. The current strategy is to
    /// be noisy for "from source" builds or when profiling is enabled.
    ///
    /// It's unclear if making from source `cargo xtask install` builds more
    /// panicky is a good idea, let's see if we can keep our awesome bleeding
    /// edge users from being upset!
    pub(crate) fn poke_wgsl_analyzer_developer(
        &mut self,
        message: String,
    ) {
        let from_source_build = option_env!("POKE_WA_DEVS").is_some();
        let profiling_enabled = std::env::var("WA_PROFILE").is_ok();
        if from_source_build || profiling_enabled {
            self.show_and_log_error(message, None);
        }
    }

    pub(crate) fn report_progress(
        &mut self,
        title: &str,
        state: &Progress,
        message: Option<String>,
        fraction: Option<f64>,
        cancel_token: Option<String>,
    ) {
        if !self.config.work_done_progress() {
            return;
        }
        #[expect(clippy::as_conversions, reason = "no better helper method")]
        #[expect(clippy::cast_sign_loss, reason = "asserted to be in-range")]
        #[expect(clippy::cast_possible_truncation, reason = "asserted to be in-range")]
        let percentage = fraction.map(|fraction| {
            assert!((0.0..=1.0).contains(&fraction));
            (fraction * 100.0) as u32
        });
        let cancellable = Some(cancel_token.is_some());
        let token = lsp_types::ProgressToken::String(
            cancel_token.unwrap_or_else(|| format!("rustAnalyzer/{title}")),
        );
        tracing::debug!(?token, ?state, "report_progress {message:?}");
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
                    cancellable,
                    message,
                    percentage,
                })
            },
            Progress::Report => {
                lsp_types::WorkDoneProgress::Report(lsp_types::WorkDoneProgressReport {
                    cancellable,
                    message,
                    percentage,
                })
            },
            Progress::End => {
                lsp_types::WorkDoneProgress::End(lsp_types::WorkDoneProgressEnd { message })
            },
        };
        self.send_notification::<lsp_types::notification::Progress>(lsp_types::ProgressParams {
            token,
            value: lsp_types::ProgressParamsValue::WorkDone(work_done_progress),
        });
    }
}

pub(crate) fn apply_document_changes(
    encoding: PositionEncoding,
    file_contents: &str,
    mut content_changes: Vec<lsp_types::TextDocumentContentChangeEvent>,
) -> String {
    // If at least one of the changes is a full document change, use the last
    // of them as the starting point and ignore all previous changes.
    let (mut text, content_changes) = match content_changes
        .iter()
        .rposition(|change| change.range.is_none())
    {
        Some(index) => {
            let text = std::mem::take(&mut content_changes[index].text);
            (text, &content_changes[index + 1..])
        },
        None => (file_contents.to_owned(), (&*content_changes)),
    };
    if content_changes.is_empty() {
        return text;
    }

    let mut line_index = LineIndex {
        // the index will be overwritten in the bottom loop's first iteration
        index: Arc::new(line_index::LineIndex::new(&text)),
        // We don't care about line endings here.
        endings: LineEndings::Unix,
        encoding,
    };

    // The changes we got must be applied sequentially, but can cross lines so we
    // have to keep our line index updated.
    // Some clients (e.g. Code) sort the ranges in reverse. As an optimization, we
    // remember the last valid line in the index and only rebuild it if needed.
    // The VFS will normalize the end of lines to `\n`.
    let mut index_valid = !0_u32;
    for change in content_changes {
        // The None case can't happen as we have handled it above already
        if let Some(range) = change.range {
            if index_valid <= range.end.line {
                *Arc::make_mut(&mut line_index.index) = ide::LineIndex::new(&text);
            }
            index_valid = range.start.line;
            if let Ok(range) = from_proto::text_range(&line_index, range) {
                text.replace_range(Range::<usize>::from(range), &change.text);
            }
        }
    }
    text
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
        },
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
        },
        None => {},
    }
    if let Some(additional_changes) = completion.additional_text_edits.as_ref() {
        edit_ranges.extend(additional_changes.iter().map(|edit| edit.range));
    }
    edit_ranges.extend(additional_edits.iter().map(|edit| edit.range));
    edit_ranges.sort_by_key(|range| (range.start, range.end));
    edit_ranges
        .iter()
        .zip(edit_ranges.iter().skip(1))
        .all(|(previous, next)| previous.end <= next.start)
}
