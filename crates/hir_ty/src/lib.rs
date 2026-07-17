//! The type system. We currently use this to infer types for completion, hover
//! information and various assists.

pub mod builtins;
pub mod database;
pub mod function;
pub mod infer;
pub mod layout;
pub mod ty;
pub mod validate;

pub mod diagnostics;
pub mod lower;
#[cfg(test)]
mod test_db;
#[cfg(test)]
mod tests;

pub fn setup_tracing() -> Option<tracing::subscriber::DefaultGuard> {
    use std::env;
    use std::sync::LazyLock;
    use tracing_subscriber::{Registry, layer::SubscriberExt};
    use tracing_tree::HierarchicalLayer;

    let layer = HierarchicalLayer::default()
        .with_indent_lines(true)
        .with_ansi(false)
        .with_indent_amount(2)
        .with_writer(std::io::stderr);
    let subscriber = Registry::default().with(layer);
    Some(tracing::subscriber::set_default(subscriber))
}
