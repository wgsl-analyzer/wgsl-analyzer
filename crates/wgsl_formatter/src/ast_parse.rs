//! A minimal parser toolbox used by the formatter
//! to parse the AST into a structure usable for the formatter itself.

use itertools::PutBackN;
use parser::{SyntaxElementChildren, SyntaxKind, SyntaxNode, SyntaxToken};
use rowan::NodeOrToken;
use syntax::{AstNode as _, ast::AttributeList};

use crate::{
    generators::comments::read_comment,
    helpers::{LineSpacing, read_blankspace},
    reporting::FormatDocumentResult,
    trivia::{NodeTriviaItem, NodeWithTrivia, NodeWithTriviaContent},
};

type SyntaxIterInner = PutBackN<SyntaxElementChildren>;

#[cfg(not(debug_assertions))]
mod syntax_iter {
    use itertools::put_back_n;
    use parser::SyntaxNode;

    use crate::ast_parse::SyntaxIterInner;

    pub type SyntaxIter = SyntaxIterInner;
    pub fn syntax_iter(syntax: &SyntaxNode) -> SyntaxIter {
        put_back_n(syntax.children_with_tokens())
    }
}
#[cfg(not(debug_assertions))]
pub use syntax_iter::{SyntaxIter, syntax_iter};

#[cfg(debug_assertions)]
mod syntax_iter_asserting {
    use itertools::put_back_n;
    use parser::SyntaxNode;

    use crate::{
        ast_parse::SyntaxIterInner,
        reporting::{FormatDocumentError, FormatDocumentResult, UnwrapIfPreferCrash as _},
    };

    pub struct SyntaxIter {
        inner: SyntaxIterInner,

        #[cfg(debug_assertions)]
        had_end_expected: bool,
    }

    impl Iterator for SyntaxIter {
        type Item = <SyntaxIterInner as Iterator>::Item;

        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next()
        }
    }

    impl SyntaxIter {
        pub fn put_back(
            &mut self,
            item: <Self as Iterator>::Item,
        ) {
            self.inner.put_back(item);
        }

        pub fn expect_end(&mut self) -> FormatDocumentResult<()> {
            self.had_end_expected = true;

            match self.inner.next() {
                None => Ok(()),
                Some(other) => {
                    self.inner.put_back(other.clone());
                    Err(FormatDocumentError::UnexpectedNodeOrToken { received: other })
                },
            }
            .expect_if_prefer_crash()
        }
    }

    impl Drop for SyntaxIter {
        fn drop(&mut self) {
            // Come on we need linear types, please...
            if !::std::thread::panicking() {
                assert!(
                    self.had_end_expected,
                    "SyntaxIter was dropped without expect_end having been called"
                );
            }
        }
    }

    #[must_use]
    pub fn syntax_iter(syntax: &SyntaxNode) -> SyntaxIter {
        let iterator = put_back_n(syntax.children_with_tokens());

        SyntaxIter {
            inner: iterator,
            had_end_expected: false,
        }
    }
}
#[cfg(debug_assertions)]
pub use syntax_iter_asserting::{SyntaxIter, syntax_iter};

pub fn parse_end(syntax: &mut SyntaxIter) -> FormatDocumentResult<()> {
    #[cfg(debug_assertions)]
    {
        syntax.expect_end()
    }
    #[cfg(not(debug_assertions))]
    {
        Ok(())
    }
}

pub trait ParseNodePolicy {
    fn handle_preceding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction>;
    fn handle_succeeding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction>;
}

pub struct UntilEmptyLine;

impl ParseNodePolicy for UntilEmptyLine {
    fn handle_preceding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        match read_blankspace(node) {
            Some(LineSpacing::EmptyLine(_)) => Some(PolicyAction::Stop),
            _ => None,
        }
    }

    fn handle_succeeding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        self.handle_preceding(node)
    }
}

// TODO I think the default should be to ignore blankspace and *including* it should be explicit (in struct body and compound statements)
pub struct IgnoreBlankspace;
impl ParseNodePolicy for IgnoreBlankspace {
    fn handle_preceding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        (node.kind() == SyntaxKind::Blankspace).then_some(PolicyAction::Ignored)
    }

    fn handle_succeeding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        self.handle_preceding(node)
    }
}

pub struct IgnoreComma;
impl ParseNodePolicy for IgnoreComma {
    fn handle_preceding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        (node.kind() == SyntaxKind::Comma).then_some(PolicyAction::Ignored)
    }

    fn handle_succeeding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        self.handle_preceding(node)
    }
}

pub struct NoTrivia;
impl ParseNodePolicy for NoTrivia {
    fn handle_preceding(
        &self,
        _node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        Some(PolicyAction::Content)
    }

    fn handle_succeeding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        self.handle_preceding(node)
    }
}

pub struct Oneline;
impl ParseNodePolicy for Oneline {
    fn handle_preceding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        match read_blankspace(node) {
            Some(LineSpacing::EmptyLine(_) | LineSpacing::LineBreak(_)) => Some(PolicyAction::Stop),
            _ => None,
        }
    }
    fn handle_succeeding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        self.handle_preceding(node)
    }
}

pub struct Succeeding<T>(pub T)
where
    T: ParseNodePolicy;
impl<T> ParseNodePolicy for Succeeding<T>
where
    T: ParseNodePolicy,
{
    fn handle_preceding(
        &self,
        _node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        None
    }

    fn handle_succeeding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        self.0.handle_succeeding(node)
    }
}

pub struct UntilNewline;
impl ParseNodePolicy for UntilNewline {
    fn handle_preceding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        match read_blankspace(node) {
            // TODO Most IgnoreBlankspace cases should be handled this way - linebreaks and empty lines will most often stop parsing trivia
            Some(LineSpacing::LineBreak(_) | LineSpacing::EmptyLine(_)) => {
                Some(PolicyAction::IgnoreAndStop)
            },
            _ => None,
        }
    }

    fn handle_succeeding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        self.handle_preceding(node)
    }
}

#[derive(Clone)]
pub struct Filter<T>(pub T)
where
    T: Fn(&NodeOrToken<SyntaxNode, SyntaxToken>) -> Option<PolicyAction>;
impl<T> ParseNodePolicy for Filter<T>
where
    T: Fn(&NodeOrToken<SyntaxNode, SyntaxToken>) -> Option<PolicyAction>,
{
    fn handle_preceding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        self.0(node)
    }
    fn handle_succeeding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        self.handle_preceding(node)
    }
}

pub struct Chain<F, G>(pub F, pub G)
where
    F: ParseNodePolicy,
    G: ParseNodePolicy;
impl<F, G> ParseNodePolicy for Chain<F, G>
where
    F: ParseNodePolicy,
    G: ParseNodePolicy,
{
    fn handle_preceding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        if let Some(action) = self.0.handle_preceding(node) {
            return Some(action);
        }
        self.1.handle_preceding(node)
    }
    fn handle_succeeding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        if let Some(action) = self.0.handle_succeeding(node) {
            return Some(action);
        }
        self.1.handle_succeeding(node)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PolicyAction {
    Ignored,
    // TODO I think we can do without Content, as that is just a worse None in the filter
    Content,
    Stop,
    IgnoreAndStop,
}

/// Parses a node with surrounding trivia, based on the given strategy.
#[expect(clippy::needless_pass_by_value, reason = "Intended API")]
pub fn parse_node_with<TPolicy>(
    syntax: &mut SyntaxIter,
    policy: TPolicy,
) -> NodeWithTrivia
where
    TPolicy: ParseNodePolicy,
{
    let mut preceding_trivia = Vec::new();
    let mut succeeding_trivia = Vec::new();

    // TODO Remove this

    let content = loop {
        // I wish we had linear types...
        // NOTE: Make sure node is either put_back onto syntax or consumed in a meaningful way
        if let Some(node) = syntax.next() {
            let action = policy.handle_preceding(&node);
            match action {
                Some(PolicyAction::Ignored) => {},
                Some(PolicyAction::Content) => {
                    break NodeWithTriviaContent::Content(node);
                },
                Some(PolicyAction::Stop) => {
                    syntax.put_back(node);
                    break NodeWithTriviaContent::NoContent;
                },
                Some(PolicyAction::IgnoreAndStop) => {
                    break NodeWithTriviaContent::NoContent;
                },
                None => {
                    if let Some(line_spacing) = read_blankspace(&node) {
                        match line_spacing {
                            LineSpacing::OnelineBlankspace(_) => {
                                // OnelineBlankspace is *always* ignored as it never carries any formatting information
                            },
                            LineSpacing::LineBreak(_) | LineSpacing::EmptyLine(_) => {
                                preceding_trivia.push(NodeTriviaItem::LineSpacing(line_spacing));
                            },
                        }
                    } else if let Some(comment) = read_comment(&node) {
                        // TODO Think about if this can be handled in any other way...
                        // Hacky special handling to remember if a comment was followed by a newline

                        let is_newlined = if let Some(next_item) = syntax.next() {
                            let is_newlined = matches!(
                                read_blankspace(&next_item),
                                Some(LineSpacing::EmptyLine(_) | LineSpacing::LineBreak(_))
                            );
                            syntax.put_back(next_item);
                            is_newlined
                        } else {
                            false
                        };

                        if is_newlined {
                            preceding_trivia.push(NodeTriviaItem::NewlinedComment(comment));
                        } else {
                            preceding_trivia.push(NodeTriviaItem::Comment(comment));
                        }
                    } else if let NodeOrToken::Node(node) = &node
                        && let Some(attributes) = AttributeList::cast(node.clone())
                    {
                        preceding_trivia.push(NodeTriviaItem::AttributeList(attributes));
                    } else {
                        break NodeWithTriviaContent::Content(node);
                    }
                },
            }
        } else {
            break NodeWithTriviaContent::End;
        }
    };

    while let Some(node) = syntax.next() {
        let action = policy.handle_succeeding(&node);
        match action {
            Some(PolicyAction::Ignored) => {},
            Some(PolicyAction::Content) => {
                // This belongs into the "content" of the next call to parse_node_...
                syntax.put_back(node);
                break;
            },
            Some(PolicyAction::Stop) => {
                // We want to stop parsing succeeding trivia
                syntax.put_back(node);
                break;
            },
            Some(PolicyAction::IgnoreAndStop) => {
                // We want to stop parsing succeeding trivia
                break;
            },
            None => {
                if let Some(line_spacing) = read_blankspace(&node) {
                    match line_spacing {
                        LineSpacing::OnelineBlankspace(_) => {
                            // OnelineBlankspace is *always* ignored as it never carries any formatting information
                        },
                        LineSpacing::LineBreak(_) | LineSpacing::EmptyLine(_) => {
                            succeeding_trivia.push(NodeTriviaItem::LineSpacing(line_spacing));
                        },
                    }
                } else if let Some(comment) = read_comment(&node) {
                    succeeding_trivia.push(NodeTriviaItem::Comment(comment));
                } else if node.kind() == SyntaxKind::AttributeList {
                    // Attributes are always "before" the item they are attached to
                    syntax.put_back(node);
                    break;
                } else {
                    // This belongs into the "content" of the next call to parse_node_...
                    syntax.put_back(node);
                    break;
                }
            },
        }
    }

    NodeWithTrivia {
        preceding_trivia,
        node: content,
        succeeding_trivia,
    }
}
