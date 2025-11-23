pub mod algo;
pub mod algorithms;
pub mod ast;
pub mod pointer;
pub mod syntax_editor;

use std::{marker::PhantomData, ops::Deref};

use either::Either;
use parser::Edition;
pub use parser::{
    Diagnostic, ParseEntryPoint, SyntaxElement, SyntaxKind, SyntaxNode, SyntaxNodeChildren,
    SyntaxToken,
};
pub use rowan::{
    Direction, GreenNode, NodeOrToken, SyntaxText, TextRange, TextSize, TokenAtOffset, WalkEvent,
    api::Preorder,
};
use smol_str::SmolStr;
use triomphe::Arc;

/// `Parse` is the result of the parsing: a syntax tree and a collection of
/// errors.
///
/// Note that we always produce a syntax tree, even for completely invalid
/// files.
#[derive(Debug)]
pub struct Parse<T> {
    green_node: GreenNode,
    errors: Option<Arc<[Diagnostic]>>,
    _ty: PhantomData<fn() -> T>,
}

impl<T> PartialEq for Parse<T> {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.green_node == other.green_node
    }
}

impl<T> Eq for Parse<T> {}

impl<T> Clone for Parse<T> {
    fn clone(&self) -> Parse<T> {
        Parse {
            green_node: self.green_node.clone(),
            errors: self.errors.clone(),
            _ty: PhantomData,
        }
    }
}

impl<T> Parse<T> {
    fn new(
        green_node: GreenNode,
        errors: Vec<Diagnostic>,
    ) -> Parse<T> {
        Parse {
            green_node,
            errors: if errors.is_empty() {
                None
            } else {
                Some(errors.into())
            },
            _ty: PhantomData,
        }
    }

    #[must_use]
    pub fn syntax_node(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }

    #[must_use]
    pub fn errors(&self) -> Vec<Diagnostic> {
        if let Some(e) = self.errors.as_deref() {
            e.to_vec()
        } else {
            vec![]
        }
    }
}

impl<T: AstNode> Parse<T> {
    /// Converts this parse result into a parse result for an untyped syntax tree.
    pub fn to_syntax(self) -> Parse<SyntaxNode> {
        Parse {
            green_node: self.green_node,
            errors: self.errors,
            _ty: PhantomData,
        }
    }

    /// Gets the parsed syntax tree as a typed ast node.
    ///
    /// # Panics
    ///
    /// Panics if the root node cannot be casted into the typed ast node
    /// (e.g. if it's an `ERROR` node).
    pub fn tree(&self) -> T {
        T::cast(self.syntax_node()).unwrap()
    }

    /// Converts from `Parse<T>` to [`Result<T, Vec<SyntaxError>>`].
    pub fn ok(self) -> Result<T, Vec<Diagnostic>> {
        match self.errors() {
            errors if !errors.is_empty() => Err(errors),
            _ => Ok(self.tree()),
        }
    }
}

impl Parse<SyntaxNode> {
    pub fn cast<N: AstNode>(self) -> Option<Parse<N>> {
        if N::cast(self.syntax_node()).is_some() {
            Some(Parse {
                green_node: self.green_node,
                errors: self.errors,
                _ty: PhantomData,
            })
        } else {
            None
        }
    }
}

impl ast::Expression {
    /// Parses an `ast::Expr` from `text`.
    ///
    /// Note that if the parsed root node is not a valid expression, [`Parse::tree`] will panic.
    /// For example:
    /// ```rust,should_panic
    /// # use syntax::{ast, Edition};
    /// ast::Expr::parse("let fail = true;", Edition::CURRENT).tree();
    /// ```
    pub fn parse(text: &str) -> Parse<ast::Expression> {
        let (green_node, errors) =
            parser::parse_entrypoint(text, ParseEntryPoint::Expression).into_parts();
        let root = SyntaxNode::new_root(green_node.clone());

        assert!(
            ast::Expression::can_cast(root.kind()) || root.kind() == SyntaxKind::Error,
            "{:?} isn't an expression",
            root.kind()
        );
        Parse::new(green_node, errors)
    }
}

/// `SourceFile` represents a parse tree for a single Rust file.
pub use crate::ast::SourceFile;

impl SourceFile {
    pub fn parse(text: &str) -> Parse<SourceFile> {
        let (green_node, errors) =
            parser::parse_entrypoint(text, ParseEntryPoint::File).into_parts();
        let root = SyntaxNode::new_root(green_node.clone());

        assert_eq!(root.kind(), SyntaxKind::SourceFile);
        Parse::new(green_node, errors)
    }
}

/// Conversion from `SyntaxNode` to typed AST
pub trait AstNode {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> &SyntaxNode;
}

pub trait AstToken {
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized;

    fn cast(syntax: SyntaxToken) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> &SyntaxToken;

    fn text(&self) -> &str {
        self.syntax().text()
    }
}

impl AstNode for SyntaxNode {
    fn can_cast(_: SyntaxKind) -> bool {
        true
    }

    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Some(syntax)
    }

    fn syntax(&self) -> &SyntaxNode {
        self
    }
}

/// An iterator over `SyntaxNode` children of a particular AST type.
#[derive(Debug, Clone)]
pub struct AstChildren<N> {
    inner: SyntaxNodeChildren,
    ph: PhantomData<N>,
}

impl<N> AstChildren<N> {
    #[must_use]
    pub fn new(parent: &SyntaxNode) -> Self {
        Self {
            inner: parent.children(),
            ph: PhantomData,
        }
    }
}

impl<N: AstNode> Iterator for AstChildren<N> {
    type Item = N;

    fn next(&mut self) -> Option<N> {
        self.inner.find_map(N::cast)
    }
}

pub enum TokenText<'borrow> {
    Borrowed(&'borrow str),
    Owned(rowan::GreenToken),
}

impl<'borrow> TokenText<'borrow> {
    #[must_use]
    pub fn as_str(&'borrow self) -> &'borrow str {
        match self {
            TokenText::Borrowed(string) => string,
            TokenText::Owned(green) => green.text(),
        }
    }
}

impl From<TokenText<'_>> for String {
    fn from(token_text: TokenText<'_>) -> Self {
        token_text.as_str().into()
    }
}

impl From<TokenText<'_>> for SmolStr {
    fn from(token_text: TokenText<'_>) -> Self {
        Self::new(token_text.as_str())
    }
}

impl Deref for TokenText<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
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
        parent.children().find(|node| node.kind() == kind)
    }

    pub(crate) fn child_token<N: AstToken>(parent: &SyntaxNode) -> Option<N> {
        parent
            .children_with_tokens()
            .filter_map(rowan::NodeOrToken::into_token)
            .find_map(N::cast)
    }

    pub(crate) fn token(
        parent: &SyntaxNode,
        kind: SyntaxKind,
    ) -> Option<SyntaxToken> {
        parent
            .children_with_tokens()
            .filter_map(rowan::NodeOrToken::into_token)
            .find(|token| token.kind() == kind)
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
                TokenText::Borrowed(first_token(green_ref).map_or("", rowan::GreenTokenData::text))
            },
            Cow::Owned(green) => first_token(&green)
                .map(ToOwned::to_owned)
                .map_or(TokenText::Borrowed(""), TokenText::Owned),
        }
    }
}

pub trait HasName: AstNode {
    fn name(&self) -> Option<ast::Name> {
        crate::support::child(self.syntax())
    }
}

pub trait HasTemplateParameters: AstNode {
    fn template_parameters(&self) -> Option<ast::TemplateList> {
        support::child(self.syntax())
    }
}

pub trait HasAttributes: AstNode {
    fn attributes(&self) -> AstChildren<ast::Attribute> {
        support::children(self.syntax())
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
        if let Some(a_node) = A::cast(syntax.clone()) {
            return Some(Self::Left(a_node));
        } else if let Some(b_node) = B::cast(syntax) {
            return Some(Self::Right(b_node));
        }
        None
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::Left(left) => left.syntax(),
            Self::Right(right) => right.syntax(),
        }
    }
}

#[must_use]
pub fn format(file: &ast::SourceFile) -> SyntaxNode {
    file.syntax().clone_for_update()
}

#[cfg(test)]
mod tests;
