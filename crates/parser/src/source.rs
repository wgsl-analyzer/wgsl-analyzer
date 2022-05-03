use super::lexer::Token;
use crate::TokenKind;
use rowan::TextRange;

use super::ParserDefinition;

pub(crate) struct Source<'t, 'input, P: ParserDefinition> {
    tokens: &'t [Token<'input, P::TokenKind>],
    cursor: usize,
}

impl<'t, 'input, P: ParserDefinition> Source<'t, 'input, P> {
    pub(crate) fn new(tokens: &'t [Token<'input, P::TokenKind>]) -> Self {
        Self { tokens, cursor: 0 }
    }

    pub(crate) fn next_token(&mut self) -> Option<&'t Token<'input, P::TokenKind>> {
        self.eat_trivia();

        let token = self.tokens.get(self.cursor)?;
        self.cursor += 1;

        Some(token)
    }

    pub(crate) fn peek_kind(&mut self) -> Option<P::TokenKind> {
        self.eat_trivia();
        self.peek_kind_raw()
    }

    pub(crate) fn peek_kind_compound(&mut self) -> Option<(P::TokenKind, P::TokenKind)> {
        self.eat_trivia();
        self.peek_compound_raw().map(|(a, b)| (a.kind, b.kind))
    }

    pub(crate) fn peek_token(&mut self) -> Option<&Token<P::TokenKind>> {
        self.eat_trivia();
        self.peek_token_raw()
    }

    pub fn location(&mut self) -> impl Eq {
        self.cursor
    }

    fn eat_trivia(&mut self) {
        while self.at_trivia() {
            self.cursor += 1;
        }
    }

    fn at_trivia(&self) -> bool {
        self.peek_kind_raw().map_or(false, P::TokenKind::is_trivia)
    }

    pub(crate) fn last_token_range(&self) -> Option<TextRange> {
        self.tokens.last().map(|Token { range, .. }| *range)
    }

    fn peek_kind_raw(&self) -> Option<P::TokenKind> {
        self.peek_token_raw().map(|Token { kind, .. }| *kind)
    }

    fn peek_token_raw(&self) -> Option<&Token<P::TokenKind>> {
        self.tokens.get(self.cursor)
    }

    fn peek_compound_raw(&self) -> Option<(&Token<P::TokenKind>, &Token<P::TokenKind>)> {
        let a = self.tokens.get(self.cursor)?;
        let b = self.tokens.get(self.cursor + 1)?;
        Some((a, b))
    }
}
