use logos::Logos;
use rowan::{TextRange, TextSize};

#[derive(Debug, PartialEq)]
pub struct Token<'a, TokenKind> {
    pub kind: TokenKind,
    pub text: &'a str,
    pub range: TextRange,
}

pub struct Lexer<'a, TokenKind: Logos<'a>> {
    inner: logos::Lexer<'a, TokenKind>,
}

impl<'a, TokenKind: Logos<'a, Source = str>> Lexer<'a, TokenKind>
where
    TokenKind::Extras: Default,
{
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: TokenKind::lexer(input),
        }
    }
}

impl<'a, TokenKind: Logos<'a, Source = str>> Iterator for Lexer<'a, TokenKind> {
    type Item = Token<'a, TokenKind>;

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
