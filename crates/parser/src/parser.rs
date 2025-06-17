pub mod marker;
mod parse_error;

use std::marker::PhantomData;

use marker::Marker;
pub use parse_error::ParseError;

use crate::SyntaxKind;

use super::{event::Event, lexer::Token, source::Source};

pub struct Parser<'t, 'input> {
    source: Source<'t, 'input>,
    events: Vec<Event>,
    pub(crate) expected_kinds: Vec<SyntaxKind>,
    _marker: PhantomData<SyntaxKind>,
}

impl<'t, 'input> Parser<'t, 'input> {
    pub(crate) const fn new(source: Source<'t, 'input>) -> Self {
        Self {
            source,
            events: Vec::new(),
            expected_kinds: Vec::new(),
            _marker: PhantomData,
        }
    }

    pub(crate) fn parse(
        mut self,
        f: impl Fn(&mut Self),
    ) -> Vec<Event> {
        f(&mut self);
        self.events
    }

    pub fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::Placeholder);

        Marker::new(pos)
    }

    pub fn expect(
        &mut self,
        kind: SyntaxKind,
    ) {
        if self.at(kind) {
            self.bump();
        } else {
            self.error();
        }
    }

    pub fn expect_no_bump(
        &mut self,
        kind: SyntaxKind,
    ) {
        if self.at(kind) {
            self.bump();
        } else {
            self.error_no_bump(&[]);
        }
    }

    pub fn expect_recover(
        &mut self,
        kind: SyntaxKind,
        recovery: &[SyntaxKind],
    ) -> Result<(), ()> {
        if self.at(kind) {
            self.bump();
            Ok(())
        } else {
            self.error_recovery(recovery);
            Err(())
        }
    }

    pub fn eat(
        &mut self,
        kind: SyntaxKind,
    ) -> bool {
        if self.at(kind) {
            self.bump();
            true
        } else {
            false
        }
    }

    pub fn eat_set(
        &mut self,
        set: &[SyntaxKind],
    ) {
        if self.at_set(set) {
            self.bump();
        }
    }

    pub fn error(&mut self) {
        self.error_inner(None, &[], false);
    }

    pub fn error_expected(
        &mut self,
        expected: &[SyntaxKind],
    ) {
        self.error_inner(None, expected, false);
    }

    pub fn error_expected_no_bump(
        &mut self,
        expected: &[SyntaxKind],
    ) {
        self.error_inner(None, expected, true);
    }

    pub fn error_recovery(
        &mut self,
        recovery: &[SyntaxKind],
    ) {
        self.error_inner(Some(recovery), &[], false);
    }

    pub fn error_no_bump(
        &mut self,
        expected: &[SyntaxKind],
    ) {
        self.error_inner(None, expected, true);
    }

    fn error_inner(
        &mut self,
        recovery: Option<&[SyntaxKind]>,
        expected: &[SyntaxKind],
        no_bump: bool,
    ) {
        let current_token = self.source.peek_token();

        let (found, range) = if let Some(Token { kind, range, .. }) = current_token {
            (Some(*kind), *range)
        } else {
            // If we are at the end of the input we use the range of the very last token in the
            // input.
            (None, self.source.last_token_range().unwrap())
        };

        let expected = if expected.is_empty() {
            std::mem::take(&mut self.expected_kinds)
        } else {
            expected.to_vec()
        };

        self.events.push(Event::Error(ParseError {
            expected,
            found,
            range,
        }));

        let at_recovery = recovery.is_some_and(|rec| self.at_set(rec));
        if !at_recovery && !self.at_end() {
            let m = self.start();
            if !no_bump {
                self.bump();
            }
            m.complete(self, <SyntaxKind as logos::Logos>::ERROR);
        }
    }

    pub fn bump(&mut self) -> SyntaxKind {
        self.expected_kinds.clear();
        let token = self.source.next_token().unwrap();
        self.events.push(Event::AddToken);
        token.kind
    }

    pub fn bump_compound(
        &mut self,
        token: SyntaxKind,
    ) {
        self.expected_kinds.clear();
        let m = self.start();
        let _token1 = self.source.next_token().unwrap();
        self.events.push(Event::AddToken);
        let _token2 = self.source.next_token().unwrap();
        self.events.push(Event::AddToken);
        m.complete(self, token);
    }

    pub fn at(
        &mut self,
        kind: SyntaxKind,
    ) -> bool {
        if !self.expected_kinds.contains(&kind) {
            self.expected_kinds.push(kind);
        }
        self.peek() == Some(kind)
    }

    pub fn at_compound(
        &mut self,
        kind_1: SyntaxKind,
        kind_2: SyntaxKind,
    ) -> bool {
        if !self.expected_kinds.contains(&kind_1) {
            self.expected_kinds.push(kind_1);
        }
        if let Some((a, b)) = self.peek_compound() {
            a == kind_1 && b == kind_2
        } else {
            false
        }
    }

    pub fn at_or_end(
        &mut self,
        kind: SyntaxKind,
    ) -> bool {
        self.expected_kinds.push(kind);
        let token = self.peek();
        token == Some(kind) || token.is_none()
    }

    pub fn at_set(
        &mut self,
        set: &[SyntaxKind],
    ) -> bool {
        self.peek().is_some_and(|k| set.contains(&k))
    }

    pub fn at_end(&mut self) -> bool {
        self.peek().is_none()
    }

    pub fn peek(&mut self) -> Option<SyntaxKind> {
        self.source.peek_kind()
    }

    pub fn peek_compound(&mut self) -> Option<(SyntaxKind, SyntaxKind)> {
        self.source.peek_kind_compound()
    }

    pub fn set_expected(
        &mut self,
        expected: Vec<SyntaxKind>,
    ) {
        self.expected_kinds = expected;
    }

    pub fn location(&mut self) -> impl Eq + use<> {
        self.source.location()
    }
}
