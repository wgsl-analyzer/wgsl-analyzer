//! Various bare-bones error handling.
use std::{error::Error, fmt::Display};

use rowan::NodeOrToken;
use syntax::{SyntaxNode, SyntaxToken};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormatDocumentError {
    UnexpectedNodeOrToken {
        received: Option<NodeOrToken<SyntaxNode, SyntaxToken>>,
    },
    UnsupportedNodeOrToken {
        received: NodeOrToken<SyntaxNode, SyntaxToken>,
    },
    MissingNode,
}

impl Error for FormatDocumentError {}

impl Display for FormatDocumentError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        #[expect(
            clippy::use_debug,
            reason = "We want the debug information here, as this is an error we don't rely on a stable format"
        )]
        match self {
            Self::UnexpectedNodeOrToken {
                received: Some(received),
            } => {
                write!(
                    formatter,
                    "Unexpected node or token {:?} at {:?}. {received:?}",
                    received.kind(),
                    received.text_range()
                )
            },
            Self::UnexpectedNodeOrToken { received: None } => {
                write!(formatter, "Expected node or token but found None")
            },
            Self::UnsupportedNodeOrToken { received } => {
                write!(
                    formatter,
                    "Encountered unsupported Node or Token: {received:?}"
                )
            },
            Self::MissingNode => write!(formatter, "Expected to find a node but found none"),
        }
    }
}

pub(crate) type FormatDocumentResult<T> = Result<T, FormatDocumentError>;

pub(crate) trait UnwrapIfPreferCrash {
    #[must_use]
    fn expect_if_prefer_crash(self) -> Self;
}

impl<T> UnwrapIfPreferCrash for FormatDocumentResult<T> {
    #[inline]
    #[track_caller]
    fn expect_if_prefer_crash(self) -> Self {
        #[cfg(feature = "prefer-immediate-crash")]
        {
            Ok(self.expect(
                "Compiled with --features=prefer-immediate-crash, thus immediately crashing.",
            ))
        }
        #[cfg(not(feature = "prefer-immediate-crash"))]
        {
            self
        }
    }
}

#[cfg(test)]
mod tests {

    use expect_test::expect;
    use rowan::GreenNodeBuilder;
    use syntax::{SyntaxKind, SyntaxNode};

    use crate::reporting::FormatDocumentError;

    #[test]
    pub(crate) fn format_string_error_display_on_formatter_error_missing_node() {
        let error = FormatDocumentError::MissingNode;

        expect!["Expected to find a node but found none"].assert_eq(&format!("{error}"));
    }

    #[test]
    pub(crate) fn format_string_error_display_on_formatter_error_unexpected_not() {
        let mut builder = GreenNodeBuilder::new();
        builder.start_node(SyntaxKind::SourceFile.into());
        builder.finish_node();
        let syntax = SyntaxNode::new_root(builder.finish());

        let error = FormatDocumentError::UnexpectedNodeOrToken {
            received: Some(rowan::NodeOrToken::Node(syntax)),
        };

        expect!["Unexpected node or token SourceFile at 0..0. Node(SourceFile@0..0)"]
            .assert_eq(&format!("{error}"));
    }

    #[test]
    pub(crate) fn format_string_error_display_on_formatter_error_unsupported() {
        let mut builder = GreenNodeBuilder::new();
        builder.start_node(SyntaxKind::SourceFile.into());
        builder.finish_node();
        let syntax = SyntaxNode::new_root(builder.finish());

        let error = FormatDocumentError::UnsupportedNodeOrToken {
            received: rowan::NodeOrToken::Node(syntax),
        };

        expect!["Encountered unsupported Node or Token: Node(SourceFile@0..0)"]
            .assert_eq(&format!("{error}"));
    }
}
