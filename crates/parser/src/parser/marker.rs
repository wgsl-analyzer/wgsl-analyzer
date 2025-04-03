use std::marker::PhantomData;

use drop_bomb::DropBomb;

use super::Parser;
use crate::{SyntaxKind, event::Event};

pub struct Marker {
    pos: usize,
    bomb: DropBomb,
    _marker: PhantomData<SyntaxKind>,
}

impl Marker {
    pub(crate) fn new(pos: usize) -> Self {
        Self {
            pos,
            bomb: DropBomb::new("Markers need to be completed"),
            _marker: PhantomData,
        }
    }

    pub fn complete(
        mut self,
        p: &mut Parser,
        kind: SyntaxKind,
    ) -> CompletedMarker {
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

pub struct CompletedMarker {
    pos: usize,
    _marker: PhantomData<SyntaxKind>,
}

impl CompletedMarker {
    pub fn precede(
        self,
        p: &mut Parser,
    ) -> Marker {
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
