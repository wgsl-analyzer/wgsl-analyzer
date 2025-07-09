use std::fmt;

use crate::{SyntaxKind, parser::ParseError};

pub(crate) enum Event {
    StartNode {
        kind: SyntaxKind,
        forward_parent: Option<usize>,
    },
    AddToken,
    FinishNode,
    Error(ParseError),
    Placeholder,
}

impl fmt::Debug for Event {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::StartNode {
                kind,
                forward_parent,
            } => formatter
                .debug_struct("StartNode")
                .field("kind", kind)
                .field("forward_parent", forward_parent)
                .finish(),
            Self::AddToken => write!(formatter, "AddToken"),
            Self::FinishNode => write!(formatter, "FinishNode"),
            Self::Error(arg0) => formatter.debug_tuple("Error").field(arg0).finish(),
            Self::Placeholder => write!(formatter, "Placeholder"),
        }
    }
}

impl PartialEq for Event {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        match (self, other) {
            (
                Self::StartNode {
                    kind: left_kind,
                    forward_parent: left_forward_parent,
                },
                Self::StartNode {
                    kind: right_kind,
                    forward_parent: right_forward_parent,
                },
            ) => left_kind == right_kind && left_forward_parent == right_forward_parent,
            (Self::Error(l0), Self::Error(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
