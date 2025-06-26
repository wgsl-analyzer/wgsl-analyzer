use std::mem;

use rowan::{GreenNodeBuilder, Language as _};

use crate::WeslLanguage;

use super::{Parse, SyntaxKind, event::Event, lexer::Token, parser::ParseError};

pub(crate) struct Sink<'tokens, 'input> {
    builder: GreenNodeBuilder<'static>,
    tokens: &'tokens [Token<'input, SyntaxKind>],
    cursor: usize,
    events: Vec<Event>,
    errors: Vec<ParseError>,
}

impl<'tokens, 'input> Sink<'tokens, 'input> {
    pub(crate) fn new(
        tokens: &'tokens [Token<'input, SyntaxKind>],
        events: Vec<Event>,
    ) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            tokens,
            cursor: 0,
            events,
            errors: Vec::new(),
        }
    }

    pub(crate) fn finish(mut self) -> Parse {
        for index in 0..self.events.len() {
            match mem::replace(&mut self.events[index], Event::Placeholder) {
                Event::StartNode {
                    kind: starting_kind,
                    forward_parent: starting_forward_parent,
                } => {
                    let mut kinds = vec![starting_kind];

                    let mut inner_index = index;
                    let mut forward_parent = starting_forward_parent;

                    // Walk through the forward parent of the forward parent and the forward parent
                    // of that, and of that, etc. until we reach a StartNode event without a forward
                    // parent.
                    #[expect(clippy::unreachable, reason = "TODO")]
                    while let Some(fp) = forward_parent {
                        inner_index += fp;

                        forward_parent = if let Event::StartNode {
                            kind,
                            forward_parent,
                        } =
                            mem::replace(&mut self.events[inner_index], Event::Placeholder)
                        {
                            kinds.push(kind);
                            forward_parent
                        } else {
                            unreachable!()
                        };
                    }

                    for kind in kinds.into_iter().rev() {
                        self.builder.start_node(WeslLanguage::kind_to_raw(kind));
                    }
                },
                Event::AddToken => self.token(),
                Event::FinishNode => self.builder.finish_node(),
                Event::Error(error) => self.errors.push(error),
                Event::Placeholder => {},
            }

            self.eat_trivia();
        }

        Parse {
            green_node: self.builder.finish(),
            errors: self.errors,
        }
    }

    fn eat_trivia(&mut self) {
        while let Some(token) = self.tokens.get(self.cursor) {
            if !token.kind.is_trivia() {
                break;
            }

            self.token();
        }
    }

    fn token(&mut self) {
        let Token { kind, text, .. } = self.tokens[self.cursor];
        self.builder.token(WeslLanguage::kind_to_raw(kind), text);
        self.cursor += 1;
    }
}
