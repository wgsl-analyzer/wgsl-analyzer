use std::marker::PhantomData;

use la_arena::{Arena, Idx};
use syntax::{
    AstNode, SyntaxNode,
    ast::{self, SourceFile},
    pointer::{AstPointer, SyntaxNodePointer},
};

/// Maps items' `SyntaxNode`s to `ErasedFileAstId`s and back.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct AstIdMap {
    arena: Arena<SyntaxNodePointer>,
}

impl AstIdMap {
    pub fn from_source(source: SourceFile) -> AstIdMap {
        let mut map = AstIdMap::default();

        source
            .syntax()
            .children()
            .flat_map(ast::Item::cast)
            .for_each(|item| {
                map.alloc(item.syntax());

                if let ast::Item::Function(function) = item {
                    if let Some(parameters) = function.parameter_list() {
                        for import in parameters
                            .parameters()
                            .filter_map(|parameter| parameter.import())
                        {
                            map.alloc(import.syntax());
                        }
                    }
                }
            });
        map
    }

    pub fn ast_id<N: AstNode>(
        &self,
        item: &N,
    ) -> FileAstId<N> {
        let pointer = SyntaxNodePointer::new(item.syntax());
        let id = match self.arena.iter().find(|(_id, node)| **node == pointer) {
            Some((it, _)) => it,
            None => panic!(
                "Cannot find {:?} in AstIdMap:\n{:?}",
                item.syntax(),
                self.arena
                    .iter()
                    .map(|(_id, node)| node)
                    .collect::<Vec<_>>(),
            ),
        };

        FileAstId {
            id,
            _marker: PhantomData,
        }
    }

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

impl<N: AstNode> std::fmt::Debug for FileAstId<N> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.debug_struct("FileAstId").field("id", &self.id).finish()
    }
}
