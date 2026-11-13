//! Simple logger that logs either to stderr or to a file, using `tracing_subscriber`
//! filter syntax and `tracing_appender` for non blocking output.

use std::{
    fs::File,
    io::{self, Write},
    sync::{Arc, OnceLock},
};

use anyhow::Context as _;
use crossbeam_channel::Sender;
use lsp_server::Message;
use lsp_types::{LogMessageNotification, LogMessageParams, MessageType, Notification as _};
use tracing::{Level, level_filters::LevelFilter};
use tracing_subscriber::{
    Layer, Registry,
    filter::{Targets, filter_fn},
    fmt::{MakeWriter, time},
    layer::SubscriberExt as _,
};

use crate::tracing::{hprof, json};

#[derive(Debug)]
pub struct Config {
    pub writer: TracingWriter,
    pub filter: String,
    /// Filtering syntax, set in a shell:
    /// ```text
    /// env WA_PROFILE=*             // dump everything
    /// env WA_PROFILE=foo|bar|baz   // enabled only selected entries
    /// env WA_PROFILE=*@3>10        // dump everything, up to depth 3, if it takes more than 10
    /// ```
    pub profile_filter: Option<String>,

    /// Filtering syntax, set in a shell:
    /// ```text
    /// env WA_PROFILE_JSON=foo|bar|baz
    /// ```
    pub json_profile_filter: Option<String>,
}

#[derive(Debug)]
pub enum TracingWriter {
    Stderr,
    File(File),
    /// A later-initialized LSP client connection
    LspClient,
}
static LSP_CLIENT_SENDER: OnceLock<Sender<Message>> = OnceLock::new();

pub fn set_lsp_client_sender(sender: Sender<Message>) -> anyhow::Result<()> {
    LSP_CLIENT_SENDER
        .set(sender)
        .map_err(|_| anyhow::anyhow!("LSP client sender already set"))
}

impl Config {
    pub fn init(self) -> anyhow::Result<()> {
        let wa_fmt_layer = tracing_subscriber::fmt::layer()
            .with_target(false)
            .with_ansi(false);

        let wa_fmt_layer = match self.writer {
            TracingWriter::Stderr => {
                if let Ok(timer) = time::OffsetTime::local_rfc_3339() {
                    // If we can get the time offset, format logs with the timezone.
                    wa_fmt_layer
                        .with_timer(timer)
                        .with_writer(io::stderr)
                        .boxed()
                } else {
                    // Use system time if we can't get the time offset. This should
                    // never happen on Linux, but can happen on, for example, OpenBSD.
                    wa_fmt_layer.with_writer(io::stderr).boxed()
                }
            },
            TracingWriter::File(file) => {
                if let Ok(timer) = time::OffsetTime::local_rfc_3339() {
                    // If we can get the time offset, format logs with the timezone.
                    wa_fmt_layer
                        .with_timer(timer)
                        .with_writer(Arc::new(file))
                        .boxed()
                } else {
                    // Use system time if we can't get the time offset. This should
                    // never happen on Linux, but can happen on, for example, OpenBSD.
                    wa_fmt_layer.with_writer(Arc::new(file)).boxed()
                }
            },
            TracingWriter::LspClient => wa_fmt_layer
                .without_time()
                .with_level(false)
                .with_writer(LspMakeWriter)
                .boxed(),
        };

        let targets_filter: Targets = self
            .filter
            .parse()
            .with_context(|| format!("invalid log filter: `{}`", self.filter))?;
        let wa_fmt_layer = wa_fmt_layer.with_filter(targets_filter);

        // TODO: remove `.with_filter(LevelFilter::OFF)` on the `None` branch.
        let profiler_layer = match self.profile_filter {
            Some(spec) => Some(hprof::SpanTree::new_filtered(&spec)).with_filter(LevelFilter::INFO),
            None => None.with_filter(LevelFilter::OFF),
        };

        let json_profiler_layer = match self.json_profile_filter {
            Some(spec) => {
                let filter = json::JsonFilter::from_spec(&spec);
                let filter = filter_fn(move |metadata| {
                    let allowed = match &filter.allowed_names {
                        Some(names) => names.contains(metadata.name()),
                        None => true,
                    };

                    allowed && metadata.is_span()
                });
                Some(json::TimingLayer::new(io::stderr).with_filter(filter))
            },
            None => None,
        };

        let subscriber = Registry::default()
            .with(wa_fmt_layer)
            .with(json_profiler_layer)
            .with(profiler_layer);

        tracing::subscriber::set_global_default(subscriber)?;

        Ok(())
    }
}

struct LspMakeWriter;

impl<'a> MakeWriter<'a> for LspMakeWriter {
    type Writer = LspWriter;

    fn make_writer(&'a self) -> Self::Writer {
        // Messages without metadata are unexpected
        LspWriter {
            kind: MessageType::Error,
            buffer: Vec::new(),
        }
    }

    fn make_writer_for(
        &'a self,
        meta: &tracing::Metadata<'_>,
    ) -> Self::Writer {
        // Tracing levels are ordered error < warn < info < debug < trace
        let kind = if meta.level() <= &Level::ERROR {
            MessageType::Error
        } else if meta.level() <= &Level::WARN {
            MessageType::Warning
        } else if meta.level() <= &Level::INFO {
            MessageType::Info
        } else {
            MessageType::Debug
        };

        LspWriter {
            kind,
            buffer: Vec::new(),
        }
    }
}
struct LspWriter {
    kind: MessageType,
    buffer: Vec<u8>,
}

impl std::io::Write for LspWriter {
    fn write(
        &mut self,
        buf: &[u8],
    ) -> std::io::Result<usize> {
        self.buffer.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if self.buffer.is_empty() {
            return Ok(());
        }

        let Some(sender) = LSP_CLIENT_SENDER.get() else {
            return io::stderr().write_all(&self.buffer);
        };

        // TODO: maybe control more of the formatting?
        // This here is a hack
        // https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/trait.FormatEvent.html#examples ?

        let mut message = String::from_utf8_lossy_owned(std::mem::take(&mut self.buffer));
        if message.ends_with('\n') {
            message.pop();
        }

        let notification = lsp_server::Notification::new(
            LogMessageNotification::METHOD.into(),
            LogMessageParams {
                kind: self.kind,
                message,
            },
        );
        sender
            .send(Message::Notification(notification))
            .map_err(|error| {
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to send LSP message: {}", error),
                )
            })
    }
}

impl Drop for LspWriter {
    fn drop(&mut self) {
        self.flush().unwrap();
    }
}
