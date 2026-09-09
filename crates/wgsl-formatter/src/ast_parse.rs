//! A minimal parser toolbox used by the formatter
//! to parse the AST into a structure usable for the formatter itself.

use std::option::Option;

use itertools::PutBackN;
use rowan::NodeOrToken;
use syntax::{
    AstNode as _, SyntaxElementChildren, SyntaxKind, SyntaxNode, SyntaxToken, ast::AttributeList,
};

use crate::{
    blankspace::{Blankspace, read_blankspace},
    generators::comments::read_comment,
    ignore::{is_ignored_from_within, read_ignore_next_pragma_comment},
    reporting::FormatDocumentResult,
    trivia::{NodeTriviaItem, NodeWithTrivia, NodeWithTriviaContent},
};

type SyntaxIterInner = PutBackN<SyntaxElementChildren>;

#[cfg(not(debug_assertions))]
mod syntax_iter {
    use itertools::put_back_n;
    use syntax::SyntaxNode;

    use crate::ast_parse::SyntaxIterInner;

    pub(crate) type SyntaxIter = SyntaxIterInner;
    pub(crate) fn syntax_iter(syntax: &SyntaxNode) -> SyntaxIter {
        put_back_n(syntax.children_with_tokens())
    }
}
#[cfg(not(debug_assertions))]
pub(crate) use syntax_iter::{SyntaxIter, syntax_iter};

#[cfg(debug_assertions)]
mod syntax_iter_asserting {
    use itertools::put_back_n;
    use syntax::SyntaxNode;

    use crate::{
        ast_parse::SyntaxIterInner,
        reporting::{FormatDocumentError, FormatDocumentResult, UnwrapIfPreferCrash as _},
    };

    pub(crate) struct SyntaxIter {
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
        pub(crate) fn put_back(
            &mut self,
            item: <Self as Iterator>::Item,
        ) {
            self.inner.put_back(item);
        }

        pub(crate) fn expect_end(&mut self) -> FormatDocumentResult<()> {
            self.had_end_expected = true;

            match self.inner.next() {
                None => Ok(()),
                Some(other) => {
                    self.inner.put_back(other.clone());
                    Err(FormatDocumentError::UnexpectedNodeOrToken {
                        received: Some(other),
                    })
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
    pub(crate) fn syntax_iter(syntax: &SyntaxNode) -> SyntaxIter {
        let iterator = put_back_n(syntax.children_with_tokens());

        SyntaxIter {
            inner: iterator,
            had_end_expected: false,
        }
    }
}
#[cfg(debug_assertions)]
pub(crate) use syntax_iter_asserting::{SyntaxIter, syntax_iter};

/// Expect the [`SyntaxIter`] to have ended and not have any more unhandled
/// syntax nodes.
///
/// Consequently calling this in all generator functions ensures that we don't accidentally "loose"
/// and source code that got submitted for formatting, but not consumed by a parser function.
pub(crate) fn parse_end(syntax: &mut SyntaxIter) -> FormatDocumentResult<()> {
    #[cfg(debug_assertions)]
    {
        syntax.expect_end()
    }
    #[cfg(not(debug_assertions))]
    {
        use crate::reporting::{FormatDocumentError, UnwrapIfPreferCrash as _};

        match syntax.next() {
            None => Ok(()),
            Some(other) => {
                syntax.put_back(other.clone());
                Err(FormatDocumentError::UnexpectedNodeOrToken {
                    received: Some(other),
                })
            },
        }
        .expect_if_prefer_crash()
    }
}

/// A policy that tells [`parse_node_with`] how to handle trivia or content.
pub(crate) trait ParseNodePolicy {
    fn handle_preceding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction>;
    fn handle_succeeding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction>;
}

/// A policy for [`parse_node_with`] that [discards][PolicyAction::Discard] any blankspace.
#[expect(
    non_upper_case_globals,
    reason = "Keep struct based policies and constants looking the same"
)]
pub(crate) const DiscardBlankspace: MatchKind = MatchKind(SyntaxKind::Blankspace, PolicyAction::Discard);

/// A policy for [`parse_node_with`] that [discards][PolicyAction::Discard] any comma.
#[expect(
    non_upper_case_globals,
    reason = "Keep struct based policies and constants looking the same"
)]
pub(crate) const DiscardComma: MatchKind = MatchKind(SyntaxKind::Comma, PolicyAction::Discard);

/// A policy for [`parse_node_with`] that [discards][PolicyAction::Discard] any semicolon.
#[expect(
    non_upper_case_globals,
    reason = "Keep struct based policies and constants looking the same"
)]
pub(crate) const DiscardSemicolon: MatchKind = MatchKind(SyntaxKind::Semicolon, PolicyAction::Discard);

/// A policy for [`parse_node_with`] that [discards][PolicyAction::Discard] template delimiters (`<`, `>`).
#[expect(
    non_upper_case_globals,
    reason = "Keep struct based policies and constants looking the same"
)]
pub(crate) const DiscardTemplateDelimiters: (MatchKind, MatchKind) = (
    MatchKind(SyntaxKind::TemplateStart, PolicyAction::Discard),
    MatchKind(SyntaxKind::TemplateEnd, PolicyAction::Discard),
);

/// A policy for [`parse_node_with`] that [discards][PolicyAction::Discard] braces (`{`, `}`).
#[expect(
    non_upper_case_globals,
    reason = "Keep struct based policies and constants looking the same"
)]
pub(crate) const DiscardBraces: (MatchKind, MatchKind) = (
    MatchKind(SyntaxKind::BraceLeft, PolicyAction::Discard),
    MatchKind(SyntaxKind::BraceRight, PolicyAction::Discard),
);

/// A policy for [`parse_node_with`] that [discards][PolicyAction::Discard] parentheses (`(`, `)`).
#[expect(
    non_upper_case_globals,
    reason = "Keep struct based policies and constants looking the same"
)]
pub(crate) const DiscardParenthesis: (MatchKind, MatchKind) = (
    MatchKind(SyntaxKind::ParenthesisLeft, PolicyAction::Discard),
    MatchKind(SyntaxKind::ParenthesisRight, PolicyAction::Discard),
);

/// A policy for [`parse_node_with`] that marks a semicolon as an [end][PolicyAction::Discard].
#[expect(
    non_upper_case_globals,
    reason = "Keep struct based policies and constants looking the same"
)]
pub(crate) const MarkEndOnSemicolon: MatchKind = MatchKind(SyntaxKind::Semicolon, PolicyAction::MarkEnd);

/// A policy for [`parse_node_with`] that does not admit any trivia associated with the node.
pub(crate) struct NoTrivia;
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

/// A policy modifier for [`parse_node_with`] that only activates a given policy on nodes *succeeding* the content of the parsed [`NodeWithTrivia`].
pub(crate) struct Succeeding<T>(pub(crate) T)
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

/// A policy for [`parse_node_with`] that yields a certain [`PolicyAction`] if the parsed node matches a certain `SyntaxKind`.
pub(crate) struct MatchKind(pub(crate) SyntaxKind, pub(crate) PolicyAction);
impl ParseNodePolicy for MatchKind {
    fn handle_preceding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        (node.kind() == self.0).then_some(self.1)
    }

    fn handle_succeeding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        self.handle_preceding(node)
    }
}

/// A policy for [`parse_node_with`] that [stops][PolicyAction::Stop] parsing when at least one newline is encountered.
pub(crate) struct StopAtNewline;
impl ParseNodePolicy for StopAtNewline {
    fn handle_preceding(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<PolicyAction> {
        match read_blankspace(node) {
            Some(Blankspace::LineBreak(_) | Blankspace::EmptyLine(_)) => Some(PolicyAction::Stop),
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

/// A policy for [`parse_node_with`] that runs a user provided function.
#[derive(Clone)]
pub(crate) struct Filter<T>(pub(crate) T)
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

/// Used by [Policies](ParseNodePolicy) to instruct [`parse_node_with`] on how to proceed after encountering a node.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum PolicyAction {
    /// Discard this node. It will not be part of the result and not be included in any other [`NodeWithTrivia`]s.
    Discard,
    /// Mark this node as content and continue to parse its succeeding trivia.
    Content,
    /// Stop parsing this run of trivia.
    ///
    /// If this node would have been part of the preceding trivia, try to parse this node again as succeeding trivia.
    /// If this node would have been part of the succeeding trivia, stop parsing trivia and return. This node
    /// will be parsed again by the next call to [`parse_node_with`].
    Stop,
    /// Stop parsing and return. This node will be parsed again by the next call to [`parse_node_with`].
    MarkEnd,
    /// Like [`PolicyAction::Stop`], but the node will be discarded and not be parsed by any other call to [`parse_node_with`].
    DiscardAndStop,
}

/// Parses a node with surrounding trivia, based on the given policy.
///
/// Per default this will put any blankspace, attribute and comments into the "preceding trivia" until it encounters
/// anything else.
/// After marking that other thing as the "content", continue parsing any following blankspace and comments into the "succeeding trivia".
///
/// A [`ParseNodePolicy`] can be specified to customize that behavior.
/// For example, specifying that trivia should only be associated with this node up to the next newline, after which it belongs to the next one.
/// Many common policies can be found in [`crate::ast_parse`].
///
/// This also consults [`crate::ignore`] to determine if the returned [`NodeWithTrivia`] "wants" to be exempt from being
/// formatted, due to ignore pragmas.
///
/// Example:
/// ```rust
/// # use wgsl_formatter::ast_parse::{DiscardBlankspace, parse_node_with, syntax_iter, Succeeding, StopAtNewline};
/// # pub(crate) fn foo(node: &syntax::SyntaxNode) {
///     let mut syntax = syntax_iter(node);
///     let item = parse_node_with(&mut syntax, DiscardBlankspace);
///     let item = parse_node_with(&mut syntax, (Succeeding(StopAtNewline), DiscardBlankspace));
/// # }
/// ```
///
/// Note that when composing policies using tuples, the order very much matters, as they are applied in order.
/// If you first [`DiscardBlankspace`], and then [`StopAtNewline`], the blankspaces will be discarded before they reach
/// the second policy.
#[expect(
    clippy::too_many_lines,
    reason = "Splitting this up makes it less readable than it is now."
)]
pub(crate) fn parse_node_with<TPolicy>(
    syntax: &mut SyntaxIter,
    policy: TPolicy,
) -> NodeWithTrivia
where
    TPolicy: ParseNodePolicy,
{
    let mut ignore_data = None;

    let mut preceding_trivia = Vec::new();
    let mut succeeding_trivia = Vec::new();

    let mut content = loop {
        // I wish we had linear types...
        // NOTE: Make sure node is either put_back onto syntax or consumed in a meaningful way
        if let Some(node) = syntax.next() {
            // Check if this is an ignoring directive
            if let Some(ignore) = read_ignore_next_pragma_comment(&node) {
                ignore_data = Some((ignore, std::mem::take(&mut preceding_trivia)));
                continue;
            }

            let action = policy.handle_preceding(&node);

            // When debugging tests, its useful to insert
            // eprintln!("Preceding {:?} {:?}", node, action);

            match action {
                Some(PolicyAction::Discard) => {
                    preceding_trivia.push(NodeTriviaItem::Discarded(node));
                },
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
                Some(PolicyAction::DiscardAndStop) => {
                    preceding_trivia.push(NodeTriviaItem::Discarded(node));
                    break NodeWithTriviaContent::NoContent;
                },
                None => {
                    if let Some(blankspace) = read_blankspace(&node) {
                        match blankspace {
                            Blankspace::Inline(_) => {
                                // OnelineBlankspace is *always* discarded as it never carries any formatting information
                                preceding_trivia.push(NodeTriviaItem::Discarded(node));
                            },
                            Blankspace::LineBreak(_) | Blankspace::EmptyLine(_) => {
                                preceding_trivia.push(NodeTriviaItem::LineSpacing(blankspace));
                            },
                        }
                    } else if let Some(comment) = read_comment(&node) {
                        // Hacky special handling to remember if a comment was followed by a newline

                        let is_newlined = if let Some(next_item) = syntax.next() {
                            let is_newlined = matches!(
                                read_blankspace(&next_item),
                                Some(Blankspace::EmptyLine(_) | Blankspace::LineBreak(_))
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

    // Replace with ignore
    if let Some((ignore, actual_preceding_trivia)) = ignore_data {
        let ignored_preceding_trivia =
            std::mem::replace(&mut preceding_trivia, actual_preceding_trivia);
        let ignored_content = std::mem::replace(&mut content, NodeWithTriviaContent::End);

        content = NodeWithTriviaContent::IgnoredContent {
            ignore_pragma: Some(ignore),
            ignored_preceding_trivia,
            ignored_content: Box::new(ignored_content),
        };
    }

    // Handle ignore from within
    if let NodeWithTriviaContent::Content(NodeOrToken::Node(old_content)) = &content
        && is_ignored_from_within(old_content)
    {
        let old_content = std::mem::replace(&mut content, NodeWithTriviaContent::End);

        content = NodeWithTriviaContent::IgnoredContent {
            ignore_pragma: None,
            ignored_preceding_trivia: vec![],
            ignored_content: Box::new(old_content),
        };
    }

    // Hacky special handling to make sure there is no line-spacing if attributes are immediately followed by their target
    {
        let mut items = preceding_trivia.iter().rev().skip_while(|trivia| {
            matches!(
                trivia,
                NodeTriviaItem::LineSpacing { .. } | NodeTriviaItem::Discarded { .. }
            )
        });
        if matches!(items.next(), Some(NodeTriviaItem::AttributeList { .. })) {
            for (item, syntax) in preceding_trivia
                .iter_mut()
                .rev()
                .map(|trivia| {
                    let syntax = match &trivia {
                        NodeTriviaItem::LineSpacing(content) => Some(content.syntax()),
                        NodeTriviaItem::Comment(_)
                        | NodeTriviaItem::NewlinedComment(_)
                        | NodeTriviaItem::AttributeList(_)
                        | NodeTriviaItem::Discarded(_) => None,
                    };
                    syntax.map(|syntax| (trivia, syntax))
                })
                .take_while(Option::is_some)
                .flatten()
            {
                *item = NodeTriviaItem::Discarded(syntax);
            }
        }
    }

    while let Some(node) = syntax.next() {
        let action = policy.handle_succeeding(&node);

        // When debugging tests, its useful to insert
        // eprintln!("Succeeding {:?} {:?}", node, action);

        match action {
            Some(PolicyAction::Discard) => {
                succeeding_trivia.push(NodeTriviaItem::Discarded(node));
            },
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
            Some(PolicyAction::DiscardAndStop) => {
                // We want to stop parsing succeeding trivia
                succeeding_trivia.push(NodeTriviaItem::Discarded(node));
                break;
            },
            None => {
                if let Some(blankspace) = read_blankspace(&node) {
                    match blankspace {
                        Blankspace::Inline(_) => {
                            // OnelineBlankspace is *always* discarded as it never carries any formatting information
                            succeeding_trivia.push(NodeTriviaItem::Discarded(node));
                        },
                        Blankspace::LineBreak(_) | Blankspace::EmptyLine(_) => {
                            succeeding_trivia.push(NodeTriviaItem::LineSpacing(blankspace));
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
        content,
        succeeding_trivia,
    }
}

pub(crate) struct ManyNodesIterator<'syntaxiter, TPolicy: ParseNodePolicy> {
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
/// it encounters the end.
///
/// The iterator parses until either the end of the [`SyntaxIter`] is reached or a policy returns
/// [`PolicyAction::MarkEnd`] (and thus we would yield a [`NodeWithTrivia`] that has
/// [`NodeWithTriviaContent::End`] as its content).
///
/// Example:
/// ```rust
/// # use syntax::ast;
/// # use wgsl_formatter::{ast_parse::*, reporting::*};
/// # pub(crate) fn foo(node: &syntax::SyntaxNode) -> FormatDocumentResult<()> {
///     let mut syntax = syntax_iter(node);
///     let item_arguments = parse_many_nodes_with(
///         &mut syntax,
///         (
///             Succeeding(StopAtNewline),
///             DiscardBlankspace,
///             DiscardComma,
///             DiscardParenthesis,
///         ),
///     )
///     .filter(|item| !item.is_whitespace())
///     .map(|item| item.expect_ast_node_optional::<ast::Expression>())
///     .collect::<Result<Vec<_>, _>>()?;
/// #    Ok(())
/// # }
/// ```
///
/// If some application needs more control over parsing nodes, you can write a pretty much equivalent loop like
/// ```rust, ignore
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
pub(crate) const fn parse_many_nodes_with<TPolicy>(
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

#[cfg(test)]
mod tests {
    use rowan::{GreenNodeBuilder, SyntaxNode};
    use syntax::SyntaxKind;

    use crate::{
        ast_parse::{MatchKind, parse_end, parse_node_with, syntax_iter},
        trivia::NodeTriviaItem,
    };

    use super::PolicyAction;

    #[test]
    pub(crate) fn syntax_iter_panics_if_not_parsed_end() {
        let make_node = || {
            let mut builder = GreenNodeBuilder::new();
            builder.start_node(SyntaxKind::SourceFile.into());
            builder.finish_node();
            SyntaxNode::new_root(builder.finish())
        };

        // First test that parsing end does not panic
        {
            let syntax = make_node();
            let mut iter = syntax_iter(&syntax);
            drop(parse_end(&mut iter));
            drop(iter);
        }

        assert!(
            std::panic::catch_unwind(move || {
                let syntax = make_node();
                let iter = syntax_iter(&syntax);
                drop(iter);
            })
            .is_err()
        );
    }

    #[test]
    pub(crate) fn parse_end_errors_if_not_end() {
        let mut builder = GreenNodeBuilder::new();
        builder.start_node(SyntaxKind::SourceFile.into());
        builder.token(SyntaxKind::Struct.into(), "struct");
        builder.finish_node();
        let syntax = SyntaxNode::new_root(builder.finish());

        let mut iter = syntax_iter(&syntax);
        let result = parse_end(&mut iter);

        assert!(result.is_err());
    }

    #[test]
    pub(crate) fn parse_node_with_immediate_stop() {
        let mut builder = GreenNodeBuilder::new();
        builder.start_node(SyntaxKind::SourceFile.into());
        builder.token(SyntaxKind::LineEndingComment.into(), "// Hello");
        builder.token(SyntaxKind::Struct.into(), "struct");
        builder.finish_node();
        let syntax = SyntaxNode::new_root(builder.finish());

        let mut iter = syntax_iter(&syntax);

        let immediate_stop = parse_node_with(
            &mut iter,
            MatchKind(SyntaxKind::LineEndingComment, PolicyAction::Stop),
        );

        assert!(!immediate_stop.has_content());

        let actual_node = parse_node_with(
            &mut iter,
            MatchKind(SyntaxKind::Struct, PolicyAction::Content),
        );

        assert!(actual_node.has_content());

        parse_end(&mut iter).unwrap();
    }

    #[test]
    pub(crate) fn parse_node_with_immediate_discard() {
        let mut builder = GreenNodeBuilder::new();
        builder.start_node(SyntaxKind::SourceFile.into());
        builder.token(SyntaxKind::Struct.into(), "struct");
        builder.token(SyntaxKind::Identifier.into(), "A");
        builder.finish_node();
        let syntax = SyntaxNode::new_root(builder.finish());

        let mut iter = syntax_iter(&syntax);

        let discarded_struct = parse_node_with(
            &mut iter,
            MatchKind(SyntaxKind::Struct, PolicyAction::Discard),
        );

        assert_eq!(
            discarded_struct
                .content()
                .unwrap()
                .as_token()
                .unwrap()
                .text(),
            "A"
        );

        parse_end(&mut iter).unwrap();
    }

    #[test]
    pub(crate) fn parse_node_with_immediate_discard_and_stop() {
        let mut builder = GreenNodeBuilder::new();
        builder.start_node(SyntaxKind::SourceFile.into());
        builder.token(SyntaxKind::Struct.into(), "struct");
        builder.token(SyntaxKind::Identifier.into(), "A");
        builder.finish_node();
        let syntax = SyntaxNode::new_root(builder.finish());

        let mut iter = syntax_iter(&syntax);

        let immediate_stop = parse_node_with(
            &mut iter,
            MatchKind(SyntaxKind::Struct, PolicyAction::DiscardAndStop),
        );

        assert!(!immediate_stop.has_content());

        let contains_a = parse_node_with(
            &mut iter,
            MatchKind(SyntaxKind::Identifier, PolicyAction::Content),
        );

        assert_eq!(
            contains_a.content().unwrap().as_token().unwrap().text(),
            "A"
        );

        parse_end(&mut iter).unwrap();
    }

    #[test]
    pub(crate) fn parse_node_keeps_linebreaks_around_per_default() {
        let mut builder = GreenNodeBuilder::new();
        builder.start_node(SyntaxKind::SourceFile.into());
        builder.token(SyntaxKind::Blankspace.into(), "\n\n");
        builder.token(SyntaxKind::Struct.into(), "struct");
        builder.token(SyntaxKind::Blankspace.into(), "\n\n");
        builder.finish_node();
        let syntax = SyntaxNode::new_root(builder.finish());

        let mut iter = syntax_iter(&syntax);
        let immediate_stop = parse_node_with(
            &mut iter,
            MatchKind(SyntaxKind::Struct, PolicyAction::Content),
        );
        parse_end(&mut iter).unwrap();

        assert_eq!(
            immediate_stop.content().unwrap().as_token().unwrap().text(),
            "struct"
        );

        assert!(matches!(
            &*immediate_stop.preceding_trivia,
            [NodeTriviaItem::LineSpacing(_)]
        ));

        assert!(matches!(
            &*immediate_stop.succeeding_trivia,
            [NodeTriviaItem::LineSpacing(_)]
        ));
    }
}
