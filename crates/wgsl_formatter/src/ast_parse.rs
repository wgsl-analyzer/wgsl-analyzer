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

pub struct IgnoreTemplateDelimiters;
impl ParseNodePolicy for IgnoreTemplateDelimiters {
    fn handle_preceding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        matches!(
            node.kind(),
            SyntaxKind::TemplateStart | SyntaxKind::TemplateEnd
        )
        .then_some(PolicyAction::Ignored)
    }

    fn handle_succeeding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        self.handle_preceding(node)
    }
}

// TODO Make a Ignore(SyntaxKind) but benchmark if that hinders inlining
pub struct IgnoreColonColon;
impl ParseNodePolicy for IgnoreColonColon {
    fn handle_preceding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        (node.kind() == SyntaxKind::ColonColon).then_some(PolicyAction::Ignored)
    }

    fn handle_succeeding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        self.handle_preceding(node)
    }
}

// TODO Make a Ignore(SyntaxKind) but benchmark if that hinders inlining
pub struct IgnoreSemicolon;
impl ParseNodePolicy for IgnoreSemicolon {
    fn handle_preceding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        (node.kind() == SyntaxKind::Semicolon).then_some(PolicyAction::Ignored)
    }

    fn handle_succeeding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        self.handle_preceding(node)
    }
}

// TODO Make a Ignore(SyntaxKind) but benchmark if that hinders inlining
pub struct IgnoreBraces;
impl ParseNodePolicy for IgnoreBraces {
    fn handle_preceding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        matches!(node.kind(), SyntaxKind::BraceLeft | SyntaxKind::BraceRight)
            .then_some(PolicyAction::Ignored)
    }

    fn handle_succeeding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        self.handle_preceding(node)
    }
}

// TODO Make a Ignore(SyntaxKind) but benchmark if that hinders inlining
pub struct IgnoreParenthesis;
impl ParseNodePolicy for IgnoreParenthesis {
    fn handle_preceding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        matches!(
            node.kind(),
            SyntaxKind::ParenthesisRight | SyntaxKind::ParenthesisLeft
        )
        .then_some(PolicyAction::Ignored)
    }

    fn handle_succeeding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        self.handle_preceding(node)
    }
}

pub struct MarkEndOnSemicolon;
impl ParseNodePolicy for MarkEndOnSemicolon {
    fn handle_preceding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        (node.kind() == SyntaxKind::Semicolon).then_some(PolicyAction::MarkEnd)
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

macro_rules! impl_tuple {
    ($($name:ident)+) => {
        #[expect(non_snake_case, reason = "Easier macro")]
        impl<$($name: ParseNodePolicy),*> ParseNodePolicy for ($($name,)+) {
            fn handle_preceding(
                &self,
                node: &NodeOrToken<SyntaxNode, SyntaxToken>,
            ) -> Option<PolicyAction> {
                let ($($name,)+) = self;
                $(
                    if let Some(action) = $name.handle_preceding(node) {
                        return Some(action);
                    }
                )*
                None
            }
            fn handle_succeeding(
                &self,
                node: &NodeOrToken<SyntaxNode, SyntaxToken>,
            ) -> Option<PolicyAction> {
                let ($($name,)+) = self;
                $(
                    if let Some(action) = $name.handle_succeeding(node) {
                        return Some(action);
                    }
                )*
                None
            }
        }
    };
}

impl<T> ParseNodePolicy for &T
where
    T: ParseNodePolicy,
{
    fn handle_preceding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        T::handle_preceding(self, node)
    }

    fn handle_succeeding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        T::handle_succeeding(self, node)
    }
}

impl_tuple!(TA TB);
impl_tuple!(TA TB TC);
impl_tuple!(TA TB TC TD);
impl_tuple!(TA TB TC TD TE);
impl_tuple!(TA TB TC TD TE TF);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PolicyAction {
    Ignored,
    Content,
    Stop,
    MarkEnd,
    IgnoreAndStop,
}

/// Parses a node with surrounding trivia, based on the given strategy.
pub fn parse_node_with<TPolicy>(
    syntax: &mut SyntaxIter,
    policy: TPolicy,
) -> NodeWithTrivia
where
    TPolicy: ParseNodePolicy,
{
    let mut preceding_trivia = Vec::new();
    let mut succeeding_trivia = Vec::new();

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
                Some(PolicyAction::MarkEnd) => {
                    syntax.put_back(node);
                    break NodeWithTriviaContent::End;
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
            Some(PolicyAction::Stop | PolicyAction::MarkEnd) => {
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

pub struct ManyNodesIterator<'syntaxiter, TPolicy: ParseNodePolicy> {
    syntax: &'syntaxiter mut SyntaxIter,
    policy: TPolicy,
    reached_end: bool,
}

impl<TPolicy: ParseNodePolicy> Iterator for ManyNodesIterator<'_, TPolicy> {
    type Item = NodeWithTrivia;

    fn next(&mut self) -> Option<Self::Item> {
        if self.reached_end {
            return None;
        }

        let item = parse_node_with(self.syntax, &self.policy);

        self.reached_end = item.is_end();
        Some(item)
    }
}

/// Create a [`ManyNodesIterator`] that parses many nodes according to the given policy until
/// it encounters the end of the [`SyntaxIter`] or a policy returns [`MarkEnd`].
///
/// If some application needs more control over parsing nodes, you can write a pretty much equivalent loop like
/// ```rust, ignore
///
/// let items = parse_many_nodes_with(
///     &mut syntax,
///     YourPolicies
/// )
/// .filter(|item| !item.is_whitespace())
/// .collect_vec();
///
/// // Is equivalent to
///
/// let mut items = Vec::new();
/// loop {
///    let item = parse_node_with(
///        &mut syntax,
///        YourPolicies
///    );
///
///    let is_end = item.is_end();
///    if !item.is_whitespace() {
///        items.push(item);
///    }
///    if is_end {
///        break;
///    }
/// }
/// ```
pub const fn parse_many_nodes_with<TPolicy>(
    syntax: &mut SyntaxIter,
    policy: TPolicy,
) -> ManyNodesIterator<'_, TPolicy>
where
    TPolicy: ParseNodePolicy,
{
    ManyNodesIterator {
        syntax,
        policy,
        reached_end: false,
    }
}
