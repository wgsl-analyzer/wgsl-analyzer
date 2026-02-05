//! Simple logger that logs either to stderr or to a file, using `tracing_subscriber`
//! filter syntax and `tracing_appender` for non blocking output.

use std::io;

use anyhow::Context as _;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
    Layer as _, Registry,
    filter::{Targets, filter_fn},
    fmt::{MakeWriter, time},
    layer::SubscriberExt as _,
};

use crate::tracing::{hprof, json};

#[derive(Debug)]
pub struct Config<T> {
    pub writer: T,
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

impl<T> Config<T>
where
    T: for<'writer> MakeWriter<'writer> + Send + Sync + 'static,
{
    pub fn init(self) -> anyhow::Result<()> {
        let targets_filter: Targets = self
            .filter
            .parse()
            .with_context(|| format!("invalid log filter: `{}`", self.filter))?;

        let writer = self.writer;

        let wa_fmt_layer = tracing_subscriber::fmt::layer()
            .with_target(false)
            .with_ansi(false)
            .with_writer(writer);

        let wa_fmt_layer = match time::OffsetTime::local_rfc_3339() {
            Ok(timer) => {
                // If we can get the time offset, format logs with the timezone.
                wa_fmt_layer.with_timer(timer).boxed()
            },
            Err(_) => {
                // Use system time if we can't get the time offset. This should
                // never happen on Linux, but can happen on, for example, OpenBSD.
                wa_fmt_layer.boxed()
            },
        }
        .with_filter(targets_filter);

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
