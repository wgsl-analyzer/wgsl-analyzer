pub mod algo;
pub mod ast;
pub mod ptr;

use std::{marker::PhantomData, ops::Deref, sync::Arc};

use either::Either;
pub use rowan::Direction;
pub use wgsl_parser::{
    ParseEntryPoint, ParseError, SyntaxElement, SyntaxKind, SyntaxNode, SyntaxNodeChildren,
    SyntaxToken,
};

#[derive(Clone, Debug)]
pub struct Parse {
    green_node: rowan::GreenNode,
    errors: Arc<Vec<ParseError>>,
}
impl PartialEq for Parse {
    fn eq(&self, other: &Self) -> bool {
        self.green_node == other.green_node
    }
}
impl Eq for Parse {}
impl Parse {
    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }
    pub fn errors(&self) -> &[ParseError] {
        &self.errors
    }
    pub fn tree(&self) -> ast::SourceFile {
        ast::SourceFile::cast(self.syntax()).unwrap()
    }
}

pub fn parse(input: &str) -> Parse {
    parse_entrypoint(input, ParseEntryPoint::File)
}
pub fn parse_entrypoint(input: &str, parse_entrypoint: ParseEntryPoint) -> Parse {
    let (green_node, errors) = wgsl_parser::parse_entrypoint(input, parse_entrypoint).into_parts();
    Parse {
        green_node,
        errors: Arc::new(errors),
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
    fn can_cast(kind: SyntaxToken) -> bool
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
    pub fn new(parent: &SyntaxNode) -> Self {
        AstChildren {
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

pub enum TokenText<'a> {
    Borrowed(&'a str),
    Owned(rowan::GreenToken),
}
impl<'a> TokenText<'a> {
    pub fn as_str(&'a self) -> &'a str {
        match self {
            TokenText::Borrowed(s) => s,
            TokenText::Owned(green) => green.text(),
        }
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

    use crate::AstToken;

    use super::{AstChildren, AstNode, SyntaxKind, SyntaxNode, SyntaxToken, TokenText};

    pub(crate) fn child<N: AstNode>(parent: &SyntaxNode) -> Option<N> {
        parent.children().find_map(N::cast)
    }

    pub(crate) fn children<N: AstNode>(parent: &SyntaxNode) -> AstChildren<N> {
        AstChildren::new(parent)
    }

    pub(crate) fn child_syntax(parent: &SyntaxNode, kind: SyntaxKind) -> Option<SyntaxNode> {
        parent.children().find(|n| n.kind() == kind)
    }

    pub(crate) fn child_token<N: AstToken>(parent: &SyntaxNode) -> Option<N> {
        parent
            .children_with_tokens()
            .filter_map(|it| it.into_token())
            .find_map(N::cast)
    }

    pub(crate) fn token(parent: &SyntaxNode, kind: SyntaxKind) -> Option<SyntaxToken> {
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
                TokenText::Borrowed(first_token(green_ref).map_or("", rowan::GreenTokenData::text))
            }
            Cow::Owned(green) => first_token(&green)
                .map(ToOwned::to_owned)
                .map_or(TokenText::Borrowed(""), TokenText::Owned),
        }
    }
}

pub trait HasName: AstNode {
    fn name(&self) -> Option<ast::Name> {
        support::child(self.syntax())
    }
}
pub trait HasGenerics: AstNode {
    fn generic_arg_list(&self) -> Option<ast::GenericArgList> {
        support::child(self.syntax())
    }
}

pub trait HasAttrs: AstNode {
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
        $( ast::$ast:ident($it:ident) => $res:expr, )*
        _ => $catch_all:expr $(,)?
    }) => {{
        $( if let Some($it) = <ast::$ast as $crate::AstNode>::cast($node.clone()) { $res } else )*
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
    let node = file.syntax().clone_for_update();
    node
}
