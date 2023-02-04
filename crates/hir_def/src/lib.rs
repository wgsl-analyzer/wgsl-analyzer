mod ast_id;
pub mod attrs;
pub mod body;
pub mod data;
pub mod db;
pub mod expr;
pub mod hir_file_id;
pub mod module_data;
pub mod resolver;
pub mod type_ref;

pub use ast_id::*;
use base_db::{FileRange, TextRange};
use db::DefDatabase;
pub use hir_file_id::HirFileId;
use hir_file_id::HirFileIdRepr;
use module_data::{ModuleDataNode, ModuleItemId};
use syntax::{AstNode, HasTranslatableTextRange, SyntaxNode, SyntaxToken};

use crate::{db::ImportId, module_data::Import};

/// `InFile<T>` stores a value of `T` inside a particular file/syntax tree.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct InFile<T> {
    pub file_id: HirFileId,
    pub value: T,
}

impl<T> InFile<T> {
    pub fn new(file_id: HirFileId, value: T) -> InFile<T> {
        InFile { file_id, value }
    }

    // Similarly, naming here is stupid...
    pub fn with_value<U>(&self, value: U) -> InFile<U> {
        InFile::new(self.file_id, value)
    }

    pub fn map<F: FnOnce(T) -> U, U>(self, f: F) -> InFile<U> {
        InFile::new(self.file_id, f(self.value))
    }
    pub fn as_ref(&self) -> InFile<&T> {
        self.with_value(&self.value)
    }
    pub fn file_syntax(&self, db: &dyn db::DefDatabase) -> SyntaxNode {
        db.parse_or_resolve(self.file_id)
            .expect("source created from invalid file")
            .syntax()
    }
}

impl<N: AstNode> InFile<N> {
    pub fn original_file_range(&self, db: &dyn DefDatabase) -> FileRange {
        todo!()
    }
}

pub trait HasSource {
    type Value;
    fn source(&self, db: &dyn DefDatabase) -> InFile<Self::Value>;
}

impl<N: ModuleDataNode> HasSource for InFile<ModuleItemId<N>> {
    type Value = N::Source;

    fn source(&self, db: &dyn DefDatabase) -> InFile<N::Source> {
        let module_info = db.module_info(self.file_id);
        let ast_id_map = db.ast_id_map(self.file_id);
        let root = db.parse_or_resolve(self.file_id);
        let node = N::lookup(&module_info.data, self.value.index);

        InFile::new(
            self.file_id,
            ast_id_map
                .get(node.ast_id())
                .to_node(&root.unwrap().syntax()),
        )
    }
}
