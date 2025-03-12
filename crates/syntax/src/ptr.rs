use std::marker::PhantomData;

use rowan::TextRange;
use wgsl_parser::{SyntaxKind, SyntaxNode};

use crate::AstNode;

/// A pointer to a syntax node inside a file. It can be used to remember a
/// specific node across reparses of the same file.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SyntaxNodePointer {
    // Do not expose this field further. At some point, we might want to replace
    // range with node id.
    pub(crate) range: TextRange,
    kind: SyntaxKind,
}

impl SyntaxNodePointer {
    pub fn new(node: &SyntaxNode) -> SyntaxNodePointer {
        SyntaxNodePointer {
            range: node.text_range(),
            kind: node.kind(),
        }
    }

    /// "Dereference" the pointer to get the node it points to.
    ///
    /// Panics if node is not found, so make sure that `root` syntax tree is
    /// equivalent (is build from the same text) to the tree which was
    /// originally used to get this [`SyntaxNodePointer`].
    ///
    /// The complexity is linear in the depth of the tree and logarithmic in
    /// tree width. Because most trees are shallow, thinking about this as
    /// `O(log(N))` in the size of the tree is not too wrong!
    #[track_caller]
    pub fn to_node(
        &self,
        root: &SyntaxNode,
    ) -> SyntaxNode {
        assert!(root.parent().is_none());
        std::iter::successors(Some(root.clone()), |node| {
            node.child_or_token_at_range(self.range)
                .and_then(|it| it.into_node())
        })
        .find(|it| it.text_range() == self.range && it.kind() == self.kind)
        .ok_or_else(|| format!("cannot resolve local pointer to SyntaxNode: {:?}", self))
        .unwrap()
    }

    pub fn cast<N: AstNode>(self) -> Option<AstPointer<N>> {
        if !N::can_cast(self.kind) {
            return None;
        }
        Some(AstPointer {
            raw: self,
            _ty: PhantomData,
        })
    }
}

/// Like `SyntaxNodePointer`, but remembers the type of node
pub struct AstPointer<N: AstNode> {
    raw: SyntaxNodePointer,
    _ty: PhantomData<fn() -> N>,
}

impl<N: AstNode> std::fmt::Debug for AstPointer<N> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.debug_struct("AstPointer")
            .field("raw", &self.raw)
            .finish()
    }
}

impl<N: AstNode> Clone for AstPointer<N> {
    fn clone(&self) -> AstPointer<N> {
        AstPointer {
            raw: self.raw.clone(),
            _ty: PhantomData,
        }
    }
}

impl<N: AstNode> Eq for AstPointer<N> {}

impl<N: AstNode> PartialEq for AstPointer<N> {
    fn eq(
        &self,
        other: &AstPointer<N>,
    ) -> bool {
        self.raw == other.raw
    }
}

impl<N: AstNode> std::hash::Hash for AstPointer<N> {
    fn hash<H: std::hash::Hasher>(
        &self,
        state: &mut H,
    ) {
        self.raw.hash(state);
    }
}

impl<N: AstNode> AstPointer<N> {
    pub fn new(node: &N) -> AstPointer<N> {
        AstPointer {
            raw: SyntaxNodePointer::new(node.syntax()),
            _ty: PhantomData,
        }
    }

    #[track_caller]
    pub fn to_node(
        &self,
        root: &SyntaxNode,
    ) -> N {
        let syntax_node = self.raw.to_node(root);
        N::cast(syntax_node).unwrap()
    }

    pub fn syntax_node_pointer(&self) -> SyntaxNodePointer {
        self.raw.clone()
    }

    pub fn cast<U: AstNode>(self) -> Option<AstPointer<U>> {
        if !U::can_cast(self.raw.kind) {
            return None;
        }
        Some(AstPointer {
            raw: self.raw,
            _ty: PhantomData,
        })
    }
}

impl<N: AstNode> From<AstPointer<N>> for SyntaxNodePointer {
    fn from(pointer: AstPointer<N>) -> SyntaxNodePointer {
        pointer.raw
    }
}
