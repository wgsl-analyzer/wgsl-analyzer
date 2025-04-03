use rowan::TextRange;

use super::lexer::Token;
use crate::SyntaxKind;

pub(crate) struct Source<'t, 'input> {
    tokens: &'t [Token<'input, SyntaxKind>],
    cursor: usize,
}

impl<'t, 'input> Source<'t, 'input> {
    pub(crate) fn new(tokens: &'t [Token<'input, SyntaxKind>]) -> Self {
        Self { tokens, cursor: 0 }
    }

    pub(crate) fn next_token(&mut self) -> Option<&'t Token<'input, SyntaxKind>> {
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
        self.peek_compound_raw().map(|(a, b)| (a.kind, b.kind))
    }

    pub(crate) fn peek_token(&mut self) -> Option<&Token<SyntaxKind>> {
        self.eat_trivia();
        self.peek_token_raw()
    }

    pub(crate) fn location(&mut self) -> impl Eq + use<> {
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

    fn peek_token_raw(&self) -> Option<&Token<SyntaxKind>> {
        self.tokens.get(self.cursor)
    }

    #[allow(clippy::type_complexity)]
    fn peek_compound_raw(&self) -> Option<(&Token<SyntaxKind>, &Token<SyntaxKind>)> {
        let a = self.tokens.get(self.cursor)?;
        let b = self.tokens.get(self.cursor + 1)?;
        Some((a, b))
    }
}
