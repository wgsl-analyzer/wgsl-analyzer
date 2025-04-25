pub mod algorithms;
pub mod ast;
pub mod parsing;
pub mod pointer;
mod syntax_error;
mod syntax_node;

use std::{marker::PhantomData, ops::Range, sync::Arc};
mod token_text;

use ast::{AstChildren, SourceFile};
use either::Either;
pub use parser::Edition;
pub use parser::{ParseEntryPoint, ParseError, SyntaxKind};

pub use crate::{
    ast::{AstNode, AstToken},
    pointer::{AstPointer, SyntaxNodePointer},
    syntax_error::SyntaxError,
    syntax_node::{
        PreorderWithTokens, SyntaxElement, SyntaxElementChildren, SyntaxNode, SyntaxNodeChildren,
        SyntaxToken, SyntaxTreeBuilder, WgslLanguage,
    },
    token_text::TokenText,
};
pub use lexer::unescape;
pub use rowan::{
    Direction, GreenNode, NodeOrToken, SyntaxText, TextRange, TextSize, TokenAtOffset, WalkEvent,
    api::Preorder,
};
pub use smol_str::{SmolStr, SmolStrBuilder, ToSmolStr, format_smolstr};

/// `Parse` is the result of the parsing: a syntax tree and a collection of
/// errors.
///
/// Note that we always produce a syntax tree, even for completely invalid
/// files.
#[derive(Debug, PartialEq, Eq)]
pub struct Parse<T> {
    green: GreenNode,
    errors: Option<Arc<[SyntaxError]>>,
    _ty: PhantomData<fn() -> T>,
}

impl<T> Clone for Parse<T> {
    fn clone(&self) -> Parse<T> {
        Parse {
            green: self.green.clone(),
            errors: self.errors.clone(),
            _ty: PhantomData,
        }
    }
}

impl<T> Parse<T> {
    fn new(
        green: GreenNode,
        errors: Vec<SyntaxError>,
    ) -> Parse<T> {
        Parse {
            green,
            errors: if errors.is_empty() {
                None
            } else {
                Some(errors.into())
            },
            _ty: PhantomData,
        }
    }

    pub fn syntax_node(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.clone())
    }

    pub fn errors(&self) -> Vec<SyntaxError> {
        let errors = if let Some(e) = self.errors.as_deref() {
            e.to_vec()
        } else {
            vec![]
        };
        // validation::validate(&self.syntax_node(), &mut errors);
        errors
    }
}

impl Parse<SourceFile> {
    pub fn reparse(
        &self,
        delete: TextRange,
        insert: &str,
        edition: Edition,
    ) -> Parse<SourceFile> {
        self.incremental_reparse(delete, insert, edition)
            .unwrap_or_else(|| self.full_reparse(delete, insert, edition))
    }

    fn incremental_reparse(
        &self,
        delete: TextRange,
        insert: &str,
        edition: Edition,
    ) -> Option<Parse<SourceFile>> {
        // FIXME: validation errors are not handled here
        parsing::incremental_reparse(
            self.tree().syntax(),
            delete,
            insert,
            self.errors.as_deref().unwrap_or_default().iter().cloned(),
            edition,
        )
        .map(|(green_node, errors, _reparsed_range)| Parse {
            green: green_node,
            errors: if errors.is_empty() {
                None
            } else {
                Some(errors.into())
            },
            _ty: PhantomData,
        })
    }

    fn full_reparse(
        &self,
        delete: TextRange,
        insert: &str,
        edition: Edition,
    ) -> Parse<SourceFile> {
        let mut text = self.tree().syntax().text().to_string();
        text.replace_range(Range::<usize>::from(delete), insert);
        SourceFile::parse(&text, edition)
    }
}

pub fn parse_entrypoint(
    input: &str,
    parse_entrypoint: ParseEntryPoint,
) -> Parse<SourceFile> {
    let (green, errors) =
        parser::parse_entrypoint::<WgslLanguage>(input, parse_entrypoint).into_parts();
    Parse::new(green, errors)
}

mod support {
    use std::borrow::Cow;

    use super::{AstChildren, AstNode, SyntaxKind, SyntaxNode, SyntaxToken, TokenText};
    use crate::AstToken;

    pub(crate) fn child<N: AstNode>(parent: &SyntaxNode) -> Option<N> {
        parent.children().find_map(N::cast)
    }

    pub(crate) fn children<N: AstNode>(parent: &SyntaxNode) -> AstChildren<N> {
        AstChildren::new(parent)
    }

    pub(crate) fn child_syntax(
        parent: &SyntaxNode,
        kind: SyntaxKind,
    ) -> Option<SyntaxNode> {
        parent.children().find(|n| n.kind() == kind)
    }

    pub(crate) fn child_token<N: AstToken>(parent: &SyntaxNode) -> Option<N> {
        parent
            .children_with_tokens()
            .filter_map(|it| it.into_token())
            .find_map(N::cast)
    }

    pub(crate) fn token(
        parent: &SyntaxNode,
        kind: SyntaxKind,
    ) -> Option<SyntaxToken> {
        parent
            .children_with_tokens()
            .filter_map(|it| it.into_token())
            .find(|it| it.kind() == kind)
    }

    pub(crate) fn text_of_first_token(node: &SyntaxNode) -> TokenText<'_> {
        fn first_token(green_ref: &rowan::GreenNodeData) -> Option<&rowan::GreenTokenData> {
            green_ref
                .children()
                .next()
                .and_then(rowan::NodeOrToken::into_token)
        }

        match node.green() {
            Cow::Borrowed(green_ref) => {
                TokenText::borrowed(first_token(green_ref).map_or("", rowan::GreenTokenData::text))
            },
            Cow::Owned(green) => first_token(&green)
                .map(ToOwned::to_owned)
                .map_or(TokenText::borrowed(""), TokenText::owned),
        }
    }
}

pub trait HasName: AstNode {
    fn name(&self) -> Option<ast::Name> {
        support::child(self.syntax())
    }
}

pub trait HasGenerics: AstNode {
    fn generic_arg_list(&self) -> Option<ast::GenericArgumentList> {
        support::child(self.syntax())
    }
}

pub trait HasAttributes: AstNode {
    fn attribute_list(&self) -> Option<ast::AttributeList> {
        support::child(self.syntax())
    }
    fn attributes(&self) -> Either<AstChildren<ast::Attribute>, std::iter::Empty<ast::Attribute>> {
        match self.attribute_list() {
            Some(list) => Either::Left(list.attributes()),
            None => Either::Right(std::iter::empty()),
        }
    }
}

#[macro_export]
macro_rules! match_ast {
    (match $node:ident { $($tt:tt)* }) => { match_ast!(match ($node) { $($tt)* }) };

    (match ($node:expr) {
        $( ast::$ast:ident($it:ident) => $result:expr, )*
        _ => $catch_all:expr $(,)?
    }) => {{
        $( if let Some($it) = <ast::$ast as $crate::AstNode>::cast($node.clone()) { $result } else )*
        { $catch_all }
    }};
}

impl<A: AstNode, B: AstNode> AstNode for Either<A, B> {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        A::can_cast(kind) || B::can_cast(kind)
    }

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let Some(a) = A::cast(syntax.clone()) {
            return Some(Either::Left(a));
        } else if let Some(b) = B::cast(syntax) {
            return Some(Either::Right(b));
        }
        None
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            Either::Left(l) => l.syntax(),
            Either::Right(r) => r.syntax(),
        }
    }
}

pub fn format(file: &ast::SourceFile) -> SyntaxNode {
    file.syntax().clone_for_update()
}

#[cfg(test)]
mod tests;

#[cfg(test)]
fn check_entrypoint(
    input: &str,
    entry_point: ParseEntryPoint,
    expected_tree: expect_test::Expect,
) {
    let parse = parse_entrypoint(input, entry_point);
    expected_tree.assert_eq(&parse.debug_tree());
}

#[cfg(test)]
use expect_test::Expect;

#[cfg(test)]
fn check_expression(
    input: &str,
    expected_tree: Expect,
) {
    check_entrypoint(input, ParseEntryPoint::Expression, expected_tree);
}
