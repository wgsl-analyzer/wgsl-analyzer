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
            #[expect(clippy::print_stderr, reason = "This is only active in debug builds")]
            if self.had_end_expected {
                eprintln!("SyntaxIter was dropped without expect_end having been called");
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

pub trait UntilFilter {
    fn filter(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<FilterAction>;
}

pub struct UntilEmptyLine;

impl UntilFilter for UntilEmptyLine {
    fn filter(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<FilterAction> {
        match read_blankspace(node) {
            Some(LineSpacing::EmptyLine(_)) => Some(FilterAction::Stop),
            _ => None,
        }
    }
}

pub struct UntilSyntaxKind(pub SyntaxKind);
impl UntilFilter for UntilSyntaxKind {
    fn filter(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<FilterAction> {
        (node.kind() == self.0).then_some(FilterAction::Stop)
    }
}

pub struct BareSyntaxKind(pub SyntaxKind);
impl UntilFilter for BareSyntaxKind {
    fn filter(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<FilterAction> {
        (node.kind() != self.0).then_some(FilterAction::Stop)
    }
}

// TODO I think the default should be to ignore blankspace and *including* it should be explicit (in struct body and compound statements)
pub struct IgnoreBlankspace;
impl UntilFilter for IgnoreBlankspace {
    fn filter(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<FilterAction> {
        (node.kind() == SyntaxKind::Blankspace).then_some(FilterAction::Ignored)
    }
}

pub struct NoTrivia;
impl UntilFilter for NoTrivia {
    fn filter(
        &self,
        _node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<FilterAction> {
        Some(FilterAction::Content)
    }
}

pub struct Oneline;
impl UntilFilter for Oneline {
    fn filter(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<FilterAction> {
        match read_blankspace(node) {
            Some(LineSpacing::EmptyLine(_) | LineSpacing::LineBreak(_)) => Some(FilterAction::Stop),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct Filter<T>(pub T)
where
    T: Fn(&NodeOrToken<SyntaxNode, SyntaxToken>) -> Option<FilterAction>;
impl<T> UntilFilter for Filter<T>
where
    T: Fn(&NodeOrToken<SyntaxNode, SyntaxToken>) -> Option<FilterAction>,
{
    fn filter(
        &self,
        node: &NodeOrToken<SyntaxNode, SyntaxToken>,
    ) -> Option<FilterAction> {
        self.0(node)
    }
}

/// Parses a node with surrounding trivia, based on the given strategy.
pub fn parse_node_with<F>(
    syntax: &mut SyntaxIter,
    until: F,
) -> NodeWithTrivia
where
    F: UntilFilter,
{
    parse_node_with_trivia_filter(syntax, |node| until.filter(node))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FilterAction {
    Ignored,
    // TODO I think we can do without Content, as that is just a worse None in the filter
    Content,
    Stop,
    IgnoreAndStop,
}

#[deprecated]
pub fn parse_node_with_trivia_filter<F>(
    syntax: &mut SyntaxIter,
    filter: F,
) -> NodeWithTrivia
where
    F: Fn(&NodeOrToken<SyntaxNode, SyntaxToken>) -> Option<FilterAction>,
{
    parse_node_with_trivia_filter_2(syntax, &filter, &filter)
}

// TODO Rename this... .... WIP api naming is hard
pub fn parse_node_with_trivia_filter_2<FPre, FPost>(
    syntax: &mut SyntaxIter,
    filter_pre: FPre,
    filter_post: FPost,
) -> NodeWithTrivia
where
    FPre: Fn(&NodeOrToken<SyntaxNode, SyntaxToken>) -> Option<FilterAction>,
    FPost: Fn(&NodeOrToken<SyntaxNode, SyntaxToken>) -> Option<FilterAction>,
{
    let mut preceding_trivia = Vec::new();
    let mut succeeding_trivia = Vec::new();

    // TODO Remove this

    let content = loop {
        // I wish we had linear types...
        // NOTE: Make sure node is either put_back onto syntax or consumed in a meaningful way
        if let Some(node) = syntax.next() {
            let action = filter_pre(&node);
            println!("Pre {node:?} {action:?}");
            match action {
                Some(FilterAction::Ignored) => {},
                Some(FilterAction::Content) => {
                    break NodeWithTriviaContent::Content(node);
                },
                Some(FilterAction::Stop) => {
                    syntax.put_back(node);
                    break NodeWithTriviaContent::NoContent;
                },
                Some(FilterAction::IgnoreAndStop) => {
                    break NodeWithTriviaContent::NoContent;
                },
                None => {
                    if let Some(line_spacing) = read_blankspace(&node) {
                        preceding_trivia.push(NodeTriviaItem::LineSpacing(line_spacing));
                    } else if let Some(comment) = read_comment(&node) {
                        preceding_trivia.push(NodeTriviaItem::Comment(comment));
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
        let action = filter_post(&node);
        println!("Post {node:?} {action:?}");
        match action {
            Some(FilterAction::Ignored) => {},
            Some(FilterAction::Content) => {
                // This belongs into the "content" of the next call to parse_node_...
                syntax.put_back(node);
                break;
            },
            Some(FilterAction::Stop) => {
                // We want to stop parsing succeeding trivia
                syntax.put_back(node);
                break;
            },
            Some(FilterAction::IgnoreAndStop) => {
                // We want to stop parsing succeeding trivia
                break;
            },
            None => {
                if let Some(line_spacing) = read_blankspace(&node) {
                    succeeding_trivia.push(NodeTriviaItem::LineSpacing(line_spacing));
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
