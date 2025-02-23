use crate::{ParserDefinition, parsing::ParseError};

pub(crate) enum Event<P: ParserDefinition> {
    StartNode {
        kind: P::SyntaxKind,
        forward_parent: Option<usize>,
    },
    AddToken,
    FinishNode,
    Error(ParseError<P>),
    Placeholder,
}

impl<P: ParserDefinition> std::fmt::Debug for Event<P> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::StartNode {
                kind,
                forward_parent,
            } => f
                .debug_struct("StartNode")
                .field("kind", kind)
                .field("forward_parent", forward_parent)
                .finish(),
            Self::AddToken => write!(f, "AddToken"),
            Self::FinishNode => write!(f, "FinishNode"),
            Self::Error(arg0) => f.debug_tuple("Error").field(arg0).finish(),
            Self::Placeholder => write!(f, "Placeholder"),
        }
    }
}

impl<P: ParserDefinition> PartialEq for Event<P> {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        match (self, other) {
            (
                Self::StartNode {
                    kind: l_kind,
                    forward_parent: l_forward_parent,
                },
                Self::StartNode {
                    kind: r_kind,
                    forward_parent: r_forward_parent,
                },
            ) => l_kind == r_kind && l_forward_parent == r_forward_parent,
            (Self::Error(l0), Self::Error(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
