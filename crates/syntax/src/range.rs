use super::{ParseError, SyntaxNode, SyntaxToken, TextRangeTranslator};
use rowan::{NodeOrToken, TextRange};

trait HasTextRange {
    fn text_range(&self) -> TextRange;
}

impl<T: HasTextRange> HasTextRange for &T {
    fn text_range(&self) -> TextRange {
        (*self).text_range()
    }
}

impl HasTextRange for SyntaxToken {
    fn text_range(&self) -> TextRange {
        self.text_range()
    }
}
impl HasTextRange for SyntaxNode {
    fn text_range(&self) -> TextRange {
        self.text_range()
    }
}
impl<N: HasTextRange, T: HasTextRange> HasTextRange for NodeOrToken<N, T> {
    fn text_range(&self) -> TextRange {
        match self {
            NodeOrToken::Node(n) => n.text_range(),
            NodeOrToken::Token(t) => t.text_range(),
        }
    }
}

impl HasTextRange for ParseError {
    fn text_range(&self) -> TextRange {
        self.range
    }
}

pub trait HasTranslatableTextRange {
    fn translated_range<T: TextRangeTranslator + ?Sized>(&self, translator: &T)
        -> rowan::TextRange;
}

impl<U: HasTextRange> HasTranslatableTextRange for U {
    fn translated_range<T: TextRangeTranslator + ?Sized>(
        &self,
        translator: &T,
    ) -> rowan::TextRange {
        translator.translate_range(self.text_range())
    }
}
