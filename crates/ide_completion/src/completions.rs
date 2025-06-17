use ide_db::RootDatabase;

use crate::item::{Builder, CompletionItem};

pub(crate) mod dot;
pub(crate) mod expression;
pub(crate) mod import;

/// Represents an in-progress set of completions being built.
#[derive(Debug, Default)]
pub struct Completions {
    buffer: Vec<CompletionItem>,
}

impl From<Completions> for Vec<CompletionItem> {
    fn from(value: Completions) -> Self {
        value.buffer
    }
}

impl Builder {
    /// Convenience method, which allows to add a freshly created completion into accumulator
    /// without binding it to the variable.
    pub(crate) fn add_to(
        self,
        accumulator: &mut Completions,
        database: &RootDatabase,
    ) {
        accumulator.add(self.build(database))
    }
}

impl Completions {
    fn add(
        &mut self,
        item: CompletionItem,
    ) {
        self.buffer.push(item);
    }

    fn add_opt(
        &mut self,
        item: Option<CompletionItem>,
    ) {
        if let Some(item) = item {
            self.buffer.push(item)
        }
    }

    pub(crate) fn add_all<I>(
        &mut self,
        items: I,
    ) where
        I: IntoIterator,
        I::Item: Into<CompletionItem>,
    {
        items.into_iter().for_each(|item| self.add(item.into()))
    }
}
