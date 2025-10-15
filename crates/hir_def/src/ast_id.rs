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
    pub fn ast_id<N: AstNode>(
        &self,
        item: &N,
    ) -> FileAstId<N> {
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
    pub fn try_ast_id<N: AstNode>(
        &self,
        item: &N,
    ) -> Option<FileAstId<N>> {
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
    /// Panics if `N` cannot be cast to the [`SyntaxKind`].
    #[must_use]
    pub fn get<N: AstNode>(
        &self,
        id: FileAstId<N>,
    ) -> AstPointer<N> {
        self.arena[id.id].clone().cast::<N>().unwrap()
    }

    fn alloc(
        &mut self,
        item: &SyntaxNode,
    ) -> Idx<SyntaxNodePointer> {
        self.arena.alloc(SyntaxNodePointer::new(item))
    }
}

/// `AstId` points to an AST node in a specific file.
pub struct FileAstId<N: AstNode> {
    id: Idx<SyntaxNodePointer>,
    _marker: PhantomData<fn() -> N>,
}

impl<N: AstNode> PartialEq for FileAstId<N> {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.id == other.id
    }
}

impl<N: AstNode> Eq for FileAstId<N> {}

impl<N: AstNode> Clone for FileAstId<N> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<N: AstNode> Copy for FileAstId<N> {}

impl<N: AstNode> fmt::Debug for FileAstId<N> {
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
