//! RPC API.

mod ast_id;
pub mod attributes;
pub mod body;
pub mod db;
pub mod expression;
pub mod expression_store;
pub mod item_scope;
pub mod item_tree;
pub mod mod_path;
pub mod name_resolution;
pub mod resolver;
pub mod signature;
#[cfg(test)]
mod test_db;
pub mod type_ref;
pub mod type_specifier;
pub mod visibility;
pub use ast_id::*;
use base_db::{EditionedFileId, FileRange, SourceDatabase, TextRange};
use rowan::NodeOrToken;
use syntax::{AstNode, SyntaxNode, SyntaxToken, pointer::AstPointer};

pub type FxIndexSet<T> = indexmap::IndexSet<T, rustc_hash::FxBuildHasher>;
pub type FxIndexMap<K, V> =
    indexmap::IndexMap<K, V, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>;

/// `InFile<T>` stores a value of `T` inside a particular file/syntax tree.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct InFile<T> {
    pub file_id: EditionedFileId,
    pub value: T,
}

impl<T> InFile<T> {
    pub const fn new(
        file_id: EditionedFileId,
        value: T,
    ) -> Self {
        Self { file_id, value }
    }

    // Similarly, naming here is stupid...
    pub const fn with_value<U>(
        &self,
        value: U,
    ) -> InFile<U> {
        InFile::new(self.file_id, value)
    }

    pub fn map<Function, U>(
        self,
        function: Function,
    ) -> InFile<U>
    where
        Function: FnOnce(T) -> U,
    {
        InFile::new(self.file_id, function(self.value))
    }

    pub const fn as_ref(&self) -> InFile<&T> {
        self.with_value(&self.value)
    }

    /// Get the syntax of the file.
    ///
    /// # Panics
    ///
    /// Panics if the file is not found.
    pub fn file_syntax(
        &self,
        db: &dyn base_db::SourceDatabase,
    ) -> SyntaxNode {
        self.file_id.parse(db).syntax()
    }
}

impl<Node: AstNode> InFile<Node> {
    pub fn original_file_range(
        &self,
        db: &dyn SourceDatabase,
    ) -> FileRange {
        original_file_range(db, self.file_id, self.value.syntax())
    }
}

pub trait HasTextRange {
    fn text_range(&self) -> TextRange;
}

impl<T: HasTextRange> HasTextRange for &T {
    fn text_range(&self) -> TextRange {
        (*self).text_range()
    }
}

impl HasTextRange for SyntaxToken {
    fn text_range(&self) -> TextRange {
        self.text_range()
    }
}

impl HasTextRange for SyntaxNode {
    fn text_range(&self) -> TextRange {
        self.text_range()
    }
}

impl<N: HasTextRange, T: HasTextRange> HasTextRange for NodeOrToken<N, T> {
    fn text_range(&self) -> TextRange {
        match self {
            Self::Node(node) => node.text_range(),
            Self::Token(token) => token.text_range(),
        }
    }
}

pub fn original_file_range<T>(
    db: &dyn SourceDatabase,
    file_id: EditionedFileId,
    value: &T,
) -> FileRange
where
    T: HasTextRange,
{
    FileRange {
        file_id: file_id.file_id(db),
        range: value.text_range(),
    }
}

pub trait HasSource {
    type Value: AstNode;
    fn source(
        &self,
        db: &dyn SourceDatabase,
    ) -> InFile<Self::Value> {
        let InFile { file_id, value } = self.ast_ptr(db);
        InFile::new(file_id, value.to_node(&file_id.parse(db).syntax()))
    }
    fn ast_ptr(
        &self,
        db: &dyn SourceDatabase,
    ) -> InFile<AstPointer<Self::Value>>;
}

impl<Node: AstNode> HasSource for InFile<FileAstId<Node>> {
    type Value = Node;

    fn ast_ptr(
        &self,
        db: &dyn SourceDatabase,
    ) -> InFile<AstPointer<Self::Value>> {
        let ast_id_map = AstIdMap::of(db, self.file_id);
        InFile::new(self.file_id, ast_id_map.get(self.value))
    }
}
