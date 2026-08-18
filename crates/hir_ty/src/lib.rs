#![warn(unused)]
//! The type system. We currently use this to infer types for completion, hover
//! information and various assists.

pub mod db;
pub mod function;
pub mod infer;
pub mod layout;
pub mod ty;
pub mod validate;

pub mod diagnostics;
pub mod lower;

pub use wgsl_types::syntax::AddressSpace;

#[cfg(test)]
mod test_db;
#[cfg(test)]
mod tests;

pub fn setup_tracing() -> tracing::subscriber::DefaultGuard {
    use tracing_subscriber::{Registry, layer::SubscriberExt as _};
    use tracing_tree::HierarchicalLayer;

    let layer = HierarchicalLayer::default()
        .with_indent_lines(true)
        .with_ansi(false)
        .with_indent_amount(2)
        .with_writer(std::io::stderr);
    let subscriber = Registry::default().with(layer);
    tracing::subscriber::set_default(subscriber)
}
