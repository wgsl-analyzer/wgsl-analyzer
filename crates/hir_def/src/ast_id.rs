use std::{fmt, marker::PhantomData};

use la_arena::{Arena, Idx};
use syntax::{
    AstNode, SyntaxNode,
    ast::{self, SourceFile},
    pointer::{AstPointer, SyntaxNodePointer},
};

/// Maps items' `SyntaxNode`s to `FileAstId`s and back.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct AstIdMap {
    arena: Arena<SyntaxNodePointer>,
}

impl AstIdMap {
    pub fn from_source(source: &SourceFile) -> Self {
        let mut map = Self::default();

        source
            .syntax()
            .children()
            .filter_map(ast::Item::cast)
            .for_each(|item| {
                map.alloc(item.syntax());
            });
        map
    }

    /// Returns an `AstId` for the given item.
    ///
    /// # Panics
    ///
    /// Panics if the item is not found in the map.
    pub fn ast_id<Node>(
        &self,
        item: &Node,
    ) -> FileAstId<Node>
    where
        Node: AstNode,
    {
        self.try_ast_id(item).unwrap_or_else(|| {
            panic!(
                "Cannot find {:?} in AstIdMap:\n{:?}",
                item.syntax(),
                self.arena
                    .iter()
                    .map(|(_id, node)| node)
                    .collect::<Vec<_>>(),
            )
        })
    }

    /// Returns an `AstId` for the given item.
    pub fn try_ast_id<Node>(
        &self,
        item: &Node,
    ) -> Option<FileAstId<Node>>
    where
        Node: AstNode,
    {
        let pointer = SyntaxNodePointer::new(item.syntax());
        let (id, _) = self.arena.iter().find(|(_id, node)| **node == pointer)?;

        Some(FileAstId {
            id,
            _marker: PhantomData,
        })
    }

    /// Convert an id to a pointer to the AST.
    ///
    /// # Panics
    ///
    /// Panics if `N` cannot be cast to the [`syntax::SyntaxKind`].
    #[must_use]
    pub fn get<Node>(
        &self,
        id: FileAstId<Node>,
    ) -> AstPointer<Node>
    where
        Node: AstNode,
    {
        self.arena[id.id].clone().cast::<Node>().unwrap()
    }

    fn alloc(
        &mut self,
        item: &SyntaxNode,
    ) -> Idx<SyntaxNodePointer> {
        self.arena.alloc(SyntaxNodePointer::new(item))
    }
}

/// `AstId` points to an AST node in a specific file.
pub struct FileAstId<Node: AstNode> {
    id: Idx<SyntaxNodePointer>,
    _marker: PhantomData<fn() -> Node>,
}

impl<Node: AstNode> PartialEq for FileAstId<Node> {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.id == other.id
    }
}

impl<Node: AstNode> Eq for FileAstId<Node> {}
impl<Node: AstNode> std::hash::Hash for FileAstId<Node> {
    fn hash<Hasher>(
        &self,
        state: &mut Hasher,
    ) where
        Hasher: std::hash::Hasher,
    {
        self.id.hash(state);
    }
}

impl<Node: AstNode> Clone for FileAstId<Node> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Node: AstNode> Copy for FileAstId<Node> {}

impl<Node: AstNode> fmt::Debug for FileAstId<Node> {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("FileAstId")
            .field("id", &self.id)
            .finish()
    }
}

impl<SourceNode: AstNode> FileAstId<SourceNode> {
    // Can't make this a From implementation because of coherence
    #[inline]
    #[must_use]
    pub fn upcast<TargetNode>(self) -> FileAstId<TargetNode>
    where
        SourceNode: Into<TargetNode>,
        TargetNode: AstNode,
    {
        FileAstId {
            id: self.id,
            _marker: PhantomData,
        }
    }
}
