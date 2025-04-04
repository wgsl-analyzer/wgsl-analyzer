use crate::item::{Builder, CompletionItem};

pub(crate) mod dot;
pub(crate) mod expression;
pub(crate) mod import;

#[derive(Debug, Default)]
pub struct Completions {
    buf: Vec<CompletionItem>,
}

impl From<Completions> for Vec<CompletionItem> {
    fn from(value: Completions) -> Self {
        value.buf
    }
}

impl Builder {
    /// Convenience method, which allows to add a freshly created completion into accumulator
    /// without binding it to the variable.
    pub(crate) fn add_to(
        self,
        accumulator: &mut Completions,
    ) {
        accumulator.add(self.build())
    }
}

impl Completions {
    fn add(
        &mut self,
        item: CompletionItem,
    ) {
        self.buf.push(item);
    }

    fn add_opt(
        &mut self,
        item: Option<CompletionItem>,
    ) {
        if let Some(item) = item {
            self.buf.push(item)
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
