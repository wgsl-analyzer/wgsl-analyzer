use logos::Logos;
use rowan::{TextRange, TextSize};

#[derive(Debug, PartialEq)]
pub(crate) struct Token<'text, SyntaxKind> {
    pub kind: SyntaxKind,
    pub text: &'text str,
    pub range: TextRange,
}

pub(crate) struct Lexer<'source, SyntaxKind: Logos<'source>> {
    inner: logos::Lexer<'source, SyntaxKind>,
}

impl<'source, SyntaxKind: Logos<'source, Source = str>> Lexer<'source, SyntaxKind>
where
    SyntaxKind::Extras: Default,
{
    pub(crate) fn new(input: &'source str) -> Self {
        Self {
            inner: SyntaxKind::lexer(input),
        }
    }
}

impl<'source, SyntaxKind: Logos<'source, Source = str>> Iterator for Lexer<'source, SyntaxKind> {
    type Item = Token<'source, SyntaxKind>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice();

        let range = {
            let std::ops::Range { start, end } = self.inner.span();
            let start = TextSize::try_from(start).unwrap();
            let end = TextSize::try_from(end).unwrap();

            TextRange::new(start, end)
        };

        Some(Self::Item { kind, text, range })
    }
}
