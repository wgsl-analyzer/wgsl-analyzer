//! A [`tracing_subscriber::layer::Layer`] that exports new-line delinated JSON.
//!
//! Usage:
//!
//! ```ignore
//! # use tracing_subscriber::Registry;
//! let layer = json::TimingLayer::new(std::io::stderr);
//! Registry::default().with(layer).init();
//! ```

use std::{io::Write as _, marker::PhantomData, time::Instant};

use rustc_hash::FxHashSet;
use tracing::{
    Event, Subscriber,
    span::{Attributes, Id},
};
use tracing_subscriber::{Layer, fmt::MakeWriter, layer::Context, registry::LookupSpan};

struct JsonData {
    name: &'static str,
    start: std::time::Instant,
}

impl JsonData {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            start: Instant::now(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct TimingLayer<S, W> {
    writer: W,
    _inner: PhantomData<fn(S)>,
}

impl<S, W> TimingLayer<S, W> {
    pub(crate) fn new(writer: W) -> Self {
        Self {
            writer,
            _inner: PhantomData,
        }
    }
}

#[expect(clippy::renamed_function_params, reason = "abbreviations")]
impl<S, W> Layer<S> for TimingLayer<S, W>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
    W: for<'writer> MakeWriter<'writer> + Send + Sync + 'static,
{
    fn on_new_span(
        &self,
        attributes: &Attributes<'_>,
        id: &Id,
        context: Context<'_, S>,
    ) {
        let span = context.span(id).unwrap();
        let data = JsonData::new(attributes.metadata().name());
        span.extensions_mut().insert(data);
    }

    fn on_event(
        &self,
        _event: &Event<'_>,
        _context: Context<'_, S>,
    ) {
    }

    fn on_close(
        &self,
        id: Id,
        context: Context<'_, S>,
    ) {
        #[derive(serde::Serialize)]
        struct JsonDataInner {
            name: &'static str,
            elapsed_ms: u128,
        }

        let span = context.span(&id).unwrap();
        let Some(data) = span.extensions_mut().remove::<JsonData>() else {
            return;
        };

        let data = JsonDataInner {
            name: data.name,
            elapsed_ms: data.start.elapsed().as_millis(),
        };
        let mut out = serde_json::to_string(&data).expect("Unable to serialize data");
        out.push('\n');
        self.writer
            .make_writer()
            .write_all(out.as_bytes())
            .expect("Unable to write data");
    }
}

#[derive(Default, Clone, Debug)]
pub(crate) struct JsonFilter {
    pub(crate) allowed_names: Option<FxHashSet<String>>,
}

impl JsonFilter {
    pub(crate) fn from_spec(spec: &str) -> Self {
        let allowed_names = if spec == "*" {
            None
        } else {
            Some(spec.split('|').map(String::from).collect())
        };
        Self { allowed_names }
    }
}
