pub mod marker;
mod parse_error;

pub use parse_error::ParseError;

use super::event::Event;
use super::lexer::Token;
use super::source::Source;
use super::ParserDefinition;
use marker::Marker;
use std::marker::PhantomData;

pub struct Parser<'t, 'input, P: ParserDefinition> {
    source: Source<'t, 'input, P>,
    events: Vec<Event<P>>,
    pub(crate) expected_kinds: Vec<P::TokenKind>,
    _marker: PhantomData<P::SyntaxKind>,
}

impl<'t, 'input, P: ParserDefinition> Parser<'t, 'input, P> {
    pub(crate) fn new(source: Source<'t, 'input, P>) -> Self {
        Self {
            source,
            events: Vec::new(),
            expected_kinds: Vec::new(),
            _marker: PhantomData,
        }
    }

    pub(crate) fn parse(mut self, f: impl Fn(&mut Self)) -> Vec<Event<P>> {
        f(&mut self);
        self.events
    }

    pub fn start(&mut self) -> Marker<P> {
        let pos = self.events.len();
        self.events.push(Event::Placeholder);

        Marker::new(pos)
    }

    pub fn expect(&mut self, kind: P::TokenKind) {
        if self.at(kind) {
            self.bump();
        } else {
            self.error();
        }
    }
    pub fn expect_no_bump(&mut self, kind: P::TokenKind) {
        if self.at(kind) {
            self.bump();
        } else {
            self.error_no_bump(&[]);
        }
    }
    pub fn expect_recover(
        &mut self,
        kind: P::TokenKind,
        recovery: &[P::TokenKind],
    ) -> Result<(), ()> {
        if self.at(kind) {
            self.bump();
            Ok(())
        } else {
            self.error_recovery(recovery);
            Err(())
        }
    }

    pub fn eat(&mut self, kind: P::TokenKind) -> bool {
        if self.at(kind) {
            self.bump();
            true
        } else {
            false
        }
    }
    pub fn eat_set(&mut self, set: &[P::TokenKind]) {
        if self.at_set(set) {
            self.bump();
        }
    }

    pub fn error(&mut self) {
        self.error_inner(None, &[], false)
    }
    pub fn error_expected(&mut self, expected: &[P::TokenKind]) {
        self.error_inner(None, expected, false)
    }
    pub fn error_expected_no_bump(&mut self, expected: &[P::TokenKind]) {
        self.error_inner(None, expected, true)
    }
    pub fn error_recovery(&mut self, recovery: &[P::TokenKind]) {
        self.error_inner(Some(recovery), &[], false)
    }
    pub fn error_no_bump(&mut self, expected: &[P::TokenKind]) {
        self.error_inner(None, expected, true)
    }

    fn error_inner(
        &mut self,
        recovery: Option<&[P::TokenKind]>,
        expected: &[P::TokenKind],
        no_bump: bool,
    ) {
        let current_token = self.source.peek_token();

        let (found, range) = if let Some(Token { kind, range, .. }) = current_token {
            (Some(*kind), *range)
        } else {
            // If we’re at the end of the input we use the range of the very last token in the
            // input.
            (None, self.source.last_token_range().unwrap())
        };

        let expected = match expected.is_empty() {
            true => std::mem::take(&mut self.expected_kinds),
            false => expected.to_vec(),
        };

        self.events.push(Event::Error(ParseError {
            expected,
            found,
            range,
        }));

        let at_recovery = recovery.map_or(false, |rec| self.at_set(rec));
        if !at_recovery && !self.at_end() {
            let m = self.start();
            if !no_bump {
                self.bump();
            }
            m.complete(self, <P::TokenKind as logos::Logos>::ERROR.into());
        }
    }

    pub fn bump(&mut self) -> P::TokenKind {
        self.expected_kinds.clear();
        let token = self.source.next_token().unwrap();
        self.events.push(Event::AddToken);
        token.kind
    }

    pub fn bump_compound(&mut self, token: P::SyntaxKind) {
        self.expected_kinds.clear();
        let m = self.start();
        let _token1 = self.source.next_token().unwrap();
        self.events.push(Event::AddToken);
        let _token2 = self.source.next_token().unwrap();
        self.events.push(Event::AddToken);
        m.complete(self, token);
    }

    pub fn at(&mut self, kind: P::TokenKind) -> bool {
        if !self.expected_kinds.contains(&kind) {
            self.expected_kinds.push(kind);
        }
        self.peek() == Some(kind)
    }

    pub fn at_compound(&mut self, kind_1: P::TokenKind, kind_2: P::TokenKind) -> bool {
        if !self.expected_kinds.contains(&kind_1) {
            self.expected_kinds.push(kind_1);
        }
        if let Some((a, b)) = self.peek_compound() {
            a == kind_1 && b == kind_2
        } else {
            false
        }
    }

    pub fn at_or_end(&mut self, kind: P::TokenKind) -> bool {
        self.expected_kinds.push(kind);
        let token = self.peek();
        token == Some(kind) || token.is_none()
    }

    pub fn at_set(&mut self, set: &[P::TokenKind]) -> bool {
        self.peek().map_or(false, |k| set.contains(&k))
    }

    pub fn at_end(&mut self) -> bool {
        self.peek().is_none()
    }

    pub fn peek(&mut self) -> Option<P::TokenKind> {
        self.source.peek_kind()
    }
    pub fn peek_compound(&mut self) -> Option<(P::TokenKind, P::TokenKind)> {
        self.source.peek_kind_compound()
    }

    pub fn set_expected(&mut self, expected: Vec<P::TokenKind>) {
        self.expected_kinds = expected;
    }

    pub fn location(&mut self) -> impl Eq {
        self.source.location()
    }
}
