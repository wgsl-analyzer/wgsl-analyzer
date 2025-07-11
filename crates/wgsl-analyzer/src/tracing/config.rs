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
use tracing_tree::HierarchicalLayer;

use crate::tracing::hprof;
use crate::tracing::json;

#[derive(Debug)]
pub struct Config<T> {
    pub writer: T,
    pub filter: String,
    /// The meaning of `CHALK_DEBUG` is to tell chalk crates
    /// (i.e. chalk-solve, chalk-ir, chalk-recursive) how to filter tracing
    /// logs. But now we can only have just one filter, which means we have to
    /// merge chalk filter to our main filter (from `WA_LOG` env).
    ///
    /// The acceptable syntax of `CHALK_DEBUG` is `target[span{field=value}]=level`.
    /// As the value should only affect chalk crates, we'd better manually
    /// specify the target. And for simplicity, `CHALK_DEBUG` only accept the value
    /// that specify level.
    pub chalk_filter: Option<String>,

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
                // never happen on Linux, but can happen on e.g. OpenBSD.
                wa_fmt_layer.boxed()
            },
        }
        .with_filter(targets_filter);

        let chalk_layer = match self.chalk_filter {
            Some(chalk_filter) => {
                let level: LevelFilter = chalk_filter
                    .parse()
                    .with_context(|| "invalid chalk log filter")?;

                let chalk_filter = Targets::new()
                    .with_target("chalk_solve", level)
                    .with_target("chalk_ir", level)
                    .with_target("chalk_recursive", level);
                // TODO: remove `.with_filter(LevelFilter::OFF)` on the `None` branch.
                HierarchicalLayer::default()
                    .with_indent_lines(true)
                    .with_ansi(false)
                    .with_indent_amount(2)
                    .with_writer(io::stderr)
                    .with_filter(chalk_filter)
                    .boxed()
            },
            None => None::<HierarchicalLayer>
                .with_filter(LevelFilter::OFF)
                .boxed(),
        };
        // TODO: remove `.with_filter(LevelFilter::OFF)` on the `None` branch.
        let profiler_layer = self.profile_filter.map_or_else(
            || None.with_filter(LevelFilter::OFF),
            |spec| Some(hprof::SpanTree::new_filtered(&spec)).with_filter(LevelFilter::INFO),
        );
        let json_profiler_layer = self.json_profile_filter.map_or_else(
            || None,
            |spec| {
                let filter = json::JsonFilter::from_spec(&spec);
                let filter = filter_fn(move |metadata| {
                    let allowed = filter
                        .allowed_names
                        .as_ref()
                        .is_none_or(|names| names.contains(metadata.name()));
                    allowed && metadata.is_span()
                });
                Some(json::TimingLayer::new(std::io::stderr).with_filter(filter))
            },
        );
        let subscriber = Registry::default()
            .with(wa_fmt_layer)
            .with(json_profiler_layer)
            .with(profiler_layer)
            .with(chalk_layer);
        tracing::subscriber::set_global_default(subscriber)?;
        Ok(())
    }
}
