use std::marker::PhantomData;

use crate::{AstNode, SyntaxNode, WgslLanguage};

/// A "pointer" to a [`SyntaxNode`], via location in the source code.
pub type SyntaxNodePointer = rowan::ast::SyntaxNodePtr<WgslLanguage>;

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

impl<Node: AstNode> std::hash::Hash for AstPointer<Node> {
    fn hash<H: std::hash::Hasher>(
        &self,
        state: &mut H,
    ) {
        self.raw.hash(state);
    }
}

impl<Node: AstNode> AstPointer<Node> {
    pub fn new(node: &Node) -> AstPointer<Node> {
        AstPointer {
            raw: SyntaxNodePointer::new(node.syntax()),
            _ty: PhantomData,
        }
    }

    #[track_caller]
    pub fn to_node(
        &self,
        root: &SyntaxNode,
    ) -> Node {
        let syntax_node = self.raw.to_node(root);
        Node::cast(syntax_node).unwrap()
    }

    pub fn syntax_node_pointer(&self) -> SyntaxNodePointer {
        self.raw.clone()
    }

    pub fn cast<TargetNode: AstNode>(self) -> Option<AstPointer<TargetNode>> {
        if !TargetNode::can_cast(self.raw.kind()) {
            return None;
        }
        Some(AstPointer {
            raw: self.raw,
            _ty: PhantomData,
        })
    }
}

impl<Node: AstNode> From<AstPointer<Node>> for SyntaxNodePointer {
    fn from(pointer: AstPointer<Node>) -> SyntaxNodePointer {
        pointer.raw
    }
}
