use std::marker::PhantomData;

use crate::{event::Event, ParserDefinition};

use super::Parser;
use drop_bomb::DropBomb;

pub struct Marker<P: ParserDefinition> {
    pos: usize,
    bomb: DropBomb,
    _marker: PhantomData<(P::TokenKind, P::SyntaxKind)>,
}

impl<P: ParserDefinition> Marker<P> {
    pub(crate) fn new(pos: usize) -> Self {
        Self {
            pos,
            bomb: DropBomb::new("Markers need to be completed"),
            _marker: PhantomData,
        }
    }

    pub fn complete(mut self, p: &mut Parser<P>, kind: P::SyntaxKind) -> CompletedMarker<P> {
        self.bomb.defuse();

        let event_at_pos = &mut p.events[self.pos];
        assert_eq!(*event_at_pos, Event::Placeholder);

        *event_at_pos = Event::StartNode {
            kind,
            forward_parent: None,
        };

        p.events.push(Event::FinishNode);

        CompletedMarker {
            pos: self.pos,
            _marker: PhantomData,
        }
    }
}

pub struct CompletedMarker<P: ParserDefinition> {
    pos: usize,
    _marker: PhantomData<(P::TokenKind, P::SyntaxKind)>,
}

impl<P: ParserDefinition> CompletedMarker<P> {
    pub fn precede(self, p: &mut Parser<P>) -> Marker<P> {
        let new_m = p.start();

        if let Event::StartNode {
            ref mut forward_parent,
            ..
        } = p.events[self.pos]
        {
            *forward_parent = Some(new_m.pos - self.pos);
        } else {
            unreachable!();
        }

        new_m
    }
}
