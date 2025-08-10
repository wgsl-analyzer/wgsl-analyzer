mod ast_id;
pub mod attributes;
pub mod body;
pub mod data;
pub mod database;
pub mod expression;
pub mod hir_file_id;
pub mod module_data;
pub mod resolver;
pub mod type_ref;

pub use ast_id::*;
use base_db::{FileRange, TextRange};
use database::DefDatabase;
pub use hir_file_id::HirFileId;
use hir_file_id::HirFileIdRepr;
use module_data::{ModuleDataNode, ModuleItemId};
use rowan::NodeOrToken;
use syntax::{AstNode, SyntaxNode, SyntaxToken};

/// `InFile<T>` stores a value of `T` inside a particular file/syntax tree.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct InFile<T> {
    pub file_id: HirFileId,
    pub value: T,
}

impl<T> InFile<T> {
    pub const fn new(
        file_id: HirFileId,
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

    pub fn map<Function: FnOnce(T) -> U, U>(
        self,
        function: Function,
    ) -> InFile<U> {
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
        database: &dyn database::DefDatabase,
    ) -> SyntaxNode {
        database
            .parse_or_resolve(self.file_id)
            .expect("source created from invalid file")
            .syntax()
    }
}

impl<N: AstNode> InFile<N> {
    pub fn original_file_range(
        &self,
        database: &dyn DefDatabase,
    ) -> FileRange {
        original_file_range(database, self.file_id, self.value.syntax())
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

pub fn original_file_range<T: HasTextRange>(
    database: &dyn DefDatabase,
    file_id: HirFileId,
    value: &T,
) -> FileRange {
    match file_id.0 {
        HirFileIdRepr::FileId(file_id) => FileRange {
            file_id,
            range: value.text_range(),
        },
    }
}

pub trait HasSource {
    type Value;
    fn source(
        &self,
        database: &dyn DefDatabase,
    ) -> InFile<Self::Value>;
}

impl<N: ModuleDataNode> HasSource for InFile<ModuleItemId<N>> {
    type Value = N::Source;

    fn source(
        &self,
        database: &dyn DefDatabase,
    ) -> InFile<N::Source> {
        let module_info = database.module_info(self.file_id);
        let ast_id_map = database.ast_id_map(self.file_id);
        let root = database.parse_or_resolve(self.file_id);
        let node = N::lookup(&module_info.data, self.value.index);

        InFile::new(
            self.file_id,
            ast_id_map
                .get(node.ast_id())
                .to_node(&root.unwrap().syntax()),
        )
    }
}
