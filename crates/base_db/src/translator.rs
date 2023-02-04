pub use rowan::{TextRange, TextSize};
use std::collections::BTreeMap;
use syntax::TextRangeTranslator;

/// the strategy used for translating text offset before & after preprocessor.
/// - `Normal`: translate one-to-one based on the offset at beginning of the
///   block.
/// - `Import`: if translating a single position, translate it to the beginning
///   of the block. if translating a range, translate it from the beginning of
///   the block to the end of the block.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub(crate) enum TranslationStrategy {
    Normal,
    Import,
}

impl Default for TranslationStrategy {
    fn default() -> Self {
        TranslationStrategy::Normal
    }
}

/// record the text offset translation from a processed string to original
/// string. used for converting the text range for the parsed data, since it
/// will use the processed string as input.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PreProcessTranslator {
    data: BTreeMap<u32, (u32, TranslationStrategy)>,
}

impl PreProcessTranslator {
    /// create a new translation table
    pub fn new() -> Self {
        let mut res = PreProcessTranslator {
            data: BTreeMap::new(),
        };
        res.insert(0, 0);
        return res;
    }

    /// insert the offset for a block of text
    /// - `original`: the beginning position of the original text for this block
    /// - `processed`: the beginning position of the translated text for this
    ///   block
    ///
    /// the translating strategy used by default is the
    /// `TranslationStrategy::default()`.
    pub(crate) fn insert(&mut self, original: u32, processed: u32) {
        self.data.insert(processed, (original, Default::default()));
    }

    /// set the strategy of the last block.
    /// - `strategy`: the strategy for the last block
    pub(crate) fn set_last_block_strategy(&mut self, strategy: TranslationStrategy) {
        // unwrap here is safe because we initialize at least one entry in the data.
        self.data.last_entry().unwrap().get_mut().1 = strategy;
    }

    /// translate a offset from processed position to original position. please
    /// see `TranslationStrategy` for how to translate.
    /// - `position`: the processed position
    /// - `return`: the original position
    pub fn translate_start(&self, position: u32) -> u32 {
        // unwrap here is safe because we initialize at least one entry in the data.
        self.data
            .range(0..=position)
            .max_by_key(|(key, _)| *key)
            .map(|(proc, (orig, strategy))| match strategy {
                TranslationStrategy::Normal => orig + position - proc,
                TranslationStrategy::Import => *orig,
            })
            .unwrap()
            .to_owned()
    }

    /// get the end offset of original text for the current block
    /// - `position`: current offset of processed text
    /// - `return`: end offset of current block of original text
    /// # panic
    /// panic if no next block
    fn get_block_end(&self, position: u32) -> u32 {
        let (_, (orig, _)) = self
            .data
            .range((position + 1)..)
            .min_by_key(|(key, _)| *key)
            .unwrap();

        orig - 1
    }

    /// translate a offset from processed position to original position. please
    /// see `TranslationStrategy` for how to translate.
    /// - `position`: the processed position
    /// - `return`: the original position
    pub fn translate_end(&self, position: u32) -> u32 {
        // unwrap here is safe because we initialize at least one entry in the data.
        self.data
            .range(0..=position)
            .max_by_key(|(key, _)| *key)
            .map(|(proc, (orig, strategy))| match strategy {
                TranslationStrategy::Normal => orig + (position - proc),
                TranslationStrategy::Import => self.get_block_end(position),
            })
            .unwrap()
            .to_owned()
    }

    /// same as function `translate`, but use `TextSize` at parameter and return
    /// value.
    /// - `size`: the processed position
    /// - `return`: the original position
    #[allow(dead_code)]
    pub fn translate_size(&self, size: TextSize) -> TextSize {
        TextSize::from(self.translate_start(size.into()))
    }

    /// translate a range from processed position to original position. please
    /// see `TranslationStrategy` for how to translate.
    /// - `range`: the processed range
    /// - `return`: the original range
    pub fn translate_range(&self, range: TextRange) -> TextRange {
        let start = self.translate_start(range.start().into());
        let end = self.translate_end(range.end().into());
        TextRange::new(TextSize::from(start), TextSize::from(end))
    }
}

impl TextRangeTranslator for PreProcessTranslator {
    fn translate_range(&self, input: rowan::TextRange) -> rowan::TextRange {
        self.translate_range(input)
    }
}

#[cfg(test)]
mod tests {
    use super::PreProcessTranslator;
    use super::TranslationStrategy;

    #[test]
    fn translation() {
        let mut table = PreProcessTranslator::new();

        table.insert(7, 3);
        table.insert(10, 18);
        table.set_last_block_strategy(TranslationStrategy::Import);
        table.insert(14, 20);

        pretty_assertions::assert_eq!(table.translate_start(0), 0);
        pretty_assertions::assert_eq!(table.translate_start(1), 1);
        pretty_assertions::assert_eq!(table.translate_start(3), 7);
        pretty_assertions::assert_eq!(table.translate_start(4), 8);
        pretty_assertions::assert_eq!(table.translate_start(18), 10);
        pretty_assertions::assert_eq!(table.translate_start(19), 10);
        pretty_assertions::assert_eq!(table.translate_start(20), 14);

        pretty_assertions::assert_eq!(table.translate_end(0), 0);
        pretty_assertions::assert_eq!(table.translate_end(1), 1);
        pretty_assertions::assert_eq!(table.translate_end(3), 7);
        pretty_assertions::assert_eq!(table.translate_end(4), 8);
        pretty_assertions::assert_eq!(table.translate_end(18), 13);
        pretty_assertions::assert_eq!(table.translate_end(19), 13);
        pretty_assertions::assert_eq!(table.translate_end(20), 14);
    }
}
