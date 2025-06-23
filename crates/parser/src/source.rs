use rowan::TextRange;

use super::lexer::Token;
use crate::SyntaxKind;

pub(crate) struct Source<'tokens, 'input> {
    tokens: &'tokens [Token<'input, SyntaxKind>],
    cursor: usize,
}

impl<'tokens, 'input> Source<'tokens, 'input> {
    pub(crate) const fn new(tokens: &'tokens [Token<'input, SyntaxKind>]) -> Self {
        Self { tokens, cursor: 0 }
    }

    pub(crate) fn next_token(&mut self) -> Option<&'tokens Token<'input, SyntaxKind>> {
        self.eat_trivia();

        let token = self.tokens.get(self.cursor)?;
        self.cursor += 1;

        Some(token)
    }

    pub(crate) fn peek_kind(&mut self) -> Option<SyntaxKind> {
        self.eat_trivia();
        self.peek_kind_raw()
    }

    pub(crate) fn peek_kind_compound(&mut self) -> Option<(SyntaxKind, SyntaxKind)> {
        self.eat_trivia();
        self.peek_compound_raw()
            .map(|(current, peek)| (current.kind, peek.kind))
    }

    pub(crate) fn peek_token(&mut self) -> Option<&Token<'_, SyntaxKind>> {
        self.eat_trivia();
        self.peek_token_raw()
    }

    pub(crate) fn location(&self) -> impl Eq + use<> {
        self.cursor
    }

    fn eat_trivia(&mut self) {
        while self.at_trivia() {
            self.cursor += 1;
        }
    }

    fn at_trivia(&self) -> bool {
        self.peek_kind_raw().is_some_and(SyntaxKind::is_trivia)
    }

    pub(crate) fn last_token_range(&self) -> Option<TextRange> {
        self.tokens.last().map(|Token { range, .. }| *range)
    }

    fn peek_kind_raw(&self) -> Option<SyntaxKind> {
        self.peek_token_raw().map(|Token { kind, .. }| *kind)
    }

    fn peek_token_raw(&self) -> Option<&Token<'_, SyntaxKind>> {
        self.tokens.get(self.cursor)
    }

    fn peek_compound_raw(&self) -> Option<(&Token<'_, SyntaxKind>, &Token<'_, SyntaxKind>)> {
        let current = self.tokens.get(self.cursor)?;
        let peek = self.tokens.get(self.cursor + 1)?;
        Some((current, peek))
    }
}
