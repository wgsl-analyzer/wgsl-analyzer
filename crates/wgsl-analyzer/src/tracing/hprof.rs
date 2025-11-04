//! Consumer of `tracing` data, which prints a hierarchical profile.
//!
//! Based on <https://github.com/davidbarsky/tracing-tree>, but does less, while
//! actually printing timings for spans by default. The code here is vendored from
//! <https://github.com/matklad/tracing-span-tree>.
//!
//! Usage:
//!
//! ```ignore
//! # use tracing_subscriber::Registry;
//! let layer = hprof::SpanTree::default();
//! Registry::default().with(layer).init();
//! ```
//!
//! Example output:
//!
//! ```text
//! 8.37ms           top_level
//!   1.09ms           middle
//!     1.06ms           leaf
//!   1.06ms           middle
//!   3.12ms           middle
//!     1.06ms           leaf
//!   3.06ms           middle
//! ```
//!
//! Same data, but with `.aggregate(true)`:
//!
//! ```text
//! 8.39ms           top_level
//!  8.35ms    4      middle
//!    2.13ms    2      leaf
//! ```

use std::{
    fmt::Write as _,
    marker::PhantomData,
    mem,
    time::{Duration, Instant},
};

use rustc_hash::FxHashSet;
use tracing::{
    Event, Id, Level, Subscriber,
    field::{Field, Visit},
    span::Attributes,
};
use tracing_subscriber::{
    Layer, Registry, filter,
    layer::{Context, SubscriberExt as _},
    registry::LookupSpan,
};

#[must_use]
pub fn init(spec: &str) -> tracing::subscriber::DefaultGuard {
    let subscriber = Registry::default().with(SpanTree::new_filtered(spec));
    tracing::subscriber::set_default(subscriber)
}

#[derive(Debug)]
pub(crate) struct SpanTree<S> {
    aggregate: bool,
    write_filter: WriteFilter,
    _inner: PhantomData<fn(S)>,
}

impl<S> SpanTree<S>
where
    S: Subscriber + for<'span> tracing_subscriber::registry::LookupSpan<'span>,
{
    pub(crate) fn new_filtered(spec: &str) -> impl Layer<S> + use<S> {
        let (write_filter, allowed_names) = WriteFilter::from_spec(spec);

        // this filter the first pass for `tracing`: these are all the "profiling" spans, but things like
        // span depth or duration are not filtered here: that only occurs at write time.
        let profile_filter = filter::filter_fn(move |metadata| {
            let allowed = allowed_names
                .as_ref()
                .is_none_or(|names| names.contains(metadata.name()));

            allowed
                && metadata.is_span()
                && metadata.level() >= &Level::INFO
                && !metadata.target().starts_with("salsa")
                && metadata.name() != "compute_exhaustiveness_and_usefulness"
                && !metadata.target().starts_with("chalk")
        });

        Self {
            aggregate: true,
            write_filter,
            _inner: PhantomData,
        }
        .with_filter(profile_filter)
    }
}

struct Data {
    start: Instant,
    children: Vec<Node>,
    fields: String,
}

impl Data {
    fn new(attributes: &Attributes<'_>) -> Self {
        let mut data = Self {
            start: Instant::now(),
            children: Vec::new(),
            fields: String::new(),
        };

        let mut visitor = DataVisitor {
            string: &mut data.fields,
        };
        attributes.record(&mut visitor);
        data
    }

    fn into_node(
        self,
        name: &'static str,
    ) -> Node {
        Node {
            name,
            fields: self.fields,
            count: 1,
            duration: self.start.elapsed(),
            children: self.children,
        }
    }
}

pub struct DataVisitor<'string> {
    string: &'string mut String,
}

impl Visit for DataVisitor<'_> {
    #[expect(clippy::use_debug, reason = "intentional")]
    fn record_debug(
        &mut self,
        field: &Field,
        value: &dyn std::fmt::Debug,
    ) {
        write!(self.string, "{} = {value:?} ", field.name()).unwrap();
    }
}

#[expect(clippy::renamed_function_params, reason = "abbreviations")]
impl<S> Layer<S> for SpanTree<S>
where
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    fn on_new_span(
        &self,
        attributes: &Attributes<'_>,
        id: &Id,
        context: Context<'_, S>,
    ) {
        let span = context.span(id).unwrap();
        let data = Data::new(attributes);
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
        let span = context.span(&id).unwrap();
        let data = span.extensions_mut().remove::<Data>().unwrap();
        let mut node = data.into_node(span.name());

        if let Some(parent_span) = span.parent() {
            parent_span
                .extensions_mut()
                .get_mut::<Data>()
                .unwrap()
                .children
                .push(node);
        } else {
            if self.aggregate {
                node.aggregate();
            }
            node.print(&self.write_filter);
        }
    }
}

#[derive(Default)]
struct Node {
    name: &'static str,
    fields: String,
    count: u32,
    duration: Duration,
    children: Vec<Self>,
}

impl Node {
    fn print(
        &self,
        filter: &WriteFilter,
    ) {
        self.go(0, filter);
    }

    #[expect(clippy::print_stderr, reason = "copied from r-a")]
    fn go(
        &self,
        level: usize,
        filter: &WriteFilter,
    ) {
        if self.duration > filter.longer_than && level < filter.depth {
            let duration = Milliseconds(self.duration);
            let current_indent = level * 2;

            let mut out = String::new();
            _ = write!(out, "{:current_indent$}   {duration} {:<6}", "", self.name);

            if !self.fields.is_empty() {
                _ = write!(out, " @ {}", self.fields);
            }

            if self.count > 1 {
                _ = write!(out, " ({} calls)", self.count);
            }

            eprintln!("{out}");

            for child in &self.children {
                child.go(level + 1, filter);
            }
        }
    }

    fn aggregate(&mut self) {
        if self.children.is_empty() {
            return;
        }

        self.children.sort_by_key(|node| node.name);
        let mut index = 0;
        for inner_index in 1..self.children.len() {
            if self.children[index].name == self.children[inner_index].name {
                let child = mem::take(&mut self.children[inner_index]);
                self.children[index].duration += child.duration;
                self.children[index].count += child.count;
                self.children[index].children.extend(child.children);
            } else {
                index += 1;
                assert!(index <= inner_index);
                self.children.swap(index, inner_index);
            }
        }
        self.children.truncate(index + 1);
        for child in &mut self.children {
            child.aggregate();
        }
    }
}

#[derive(Default, Clone, Debug)]
pub(crate) struct WriteFilter {
    depth: usize,
    longer_than: Duration,
}

impl WriteFilter {
    pub(crate) fn from_spec(mut spec: &str) -> (Self, Option<FxHashSet<String>>) {
        let longer_than = spec.rfind('>').map_or_else(
            || Duration::new(0, 0),
            |index| {
                let longer_than = spec[index + 1..]
                    .parse()
                    .expect("invalid profile longer_than");
                spec = &spec[..index];
                Duration::from_millis(longer_than)
            },
        );

        let depth = spec.rfind('@').map_or(999, |index| {
            let depth: usize = spec[index + 1..].parse().expect("invalid profile depth");
            spec = &spec[..index];
            depth
        });
        let allowed = if spec == "*" {
            None
        } else {
            Some(spec.split('|').map(String::from).collect())
        };
        (Self { depth, longer_than }, allowed)
    }
}

struct Milliseconds(Duration);

impl std::fmt::Display for Milliseconds {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let milliseconds = self.0.as_millis();
        write!(formatter, "{milliseconds:5}ms")
    }
}
