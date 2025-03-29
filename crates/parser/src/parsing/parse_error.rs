use std::fmt::{self, Write};

use rowan::TextRange;

use crate::parsing::ParserDefinition;

pub struct ParseError<P: ParserDefinition> {
    pub expected: Vec<P::TokenKind>,
    pub found: Option<P::TokenKind>,
    pub range: TextRange,
}

impl<P: ParserDefinition> ParseError<P> {
    pub fn message(&self) -> String {
        let mut message = "expected ".to_string();
        let number_expected = self.expected.len();
        let is_first = |index| index == 0;
        let is_last = |index| index == number_expected - 1;
        for (index, expected_kind) in self.expected.iter().enumerate() {
            if is_first(index) {
                let _ = write!(message, "{:?}", expected_kind);
            } else if is_last(index) && number_expected > 2 {
                let _ = write!(message, ", or {:?}", expected_kind);
            } else if is_last(index) {
                let _ = write!(message, " or {:?}", expected_kind);
            } else {
                let _ = write!(message, ", {:?}", expected_kind);
            }
        }

        if let Some(found) = self.found {
            let _ = write!(message, ", but found {:?}", found);
        }

        message
    }
}

impl<P: ParserDefinition> fmt::Debug for ParseError<P> {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct("ParseError")
            .field("expected", &self.expected)
            .field("found", &self.found)
            .field("range", &self.range)
            .finish()
    }
}

impl<P: ParserDefinition> PartialEq for ParseError<P> {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.expected == other.expected && self.found == other.found && self.range == other.range
    }
}

impl<P: ParserDefinition> fmt::Display for ParseError<P> {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "error at {}..{}: expected ",
            u32::from(self.range.start()),
            u32::from(self.range.end()),
        )?;

        let num_expected = self.expected.len();
        let is_first = |index| index == 0;
        let is_last = |index| index == num_expected - 1;

        for (index, expected_kind) in self.expected.iter().enumerate() {
            if is_first(index) {
                write!(f, "{:?}", expected_kind)?;
            } else if is_last(index) && num_expected > 2 {
                write!(f, ", or {:?}", expected_kind)?;
            } else if is_last(index) {
                write!(f, " or {:?}", expected_kind)?;
            } else {
                write!(f, ", {:?}", expected_kind)?;
            }
        }

        if let Some(found) = self.found {
            write!(f, ", but found {:?}", found)?;
        }

        Ok(())
    }
}
