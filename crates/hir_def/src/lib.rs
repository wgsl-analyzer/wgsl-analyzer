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

use crate::{database::ImportId, module_data::Import};

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

    pub fn map<F: FnOnce(T) -> U, U>(
        self,
        f: F,
    ) -> InFile<U> {
        InFile::new(self.file_id, f(self.value))
    }

    pub const fn as_ref(&self) -> InFile<&T> {
        self.with_value(&self.value)
    }

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
            Self::Node(n) => n.text_range(),
            Self::Token(t) => t.text_range(),
        }
    }
}

pub fn original_file_range<T: HasTextRange>(
    database: &dyn DefDatabase,
    file_id: HirFileId,
    value: &T,
) -> FileRange {
    original_file_range_inner(database, file_id, value.text_range())
}

fn original_file_range_inner(
    database: &dyn DefDatabase,
    file_id: HirFileId,
    range: TextRange,
) -> FileRange {
    match file_id.0 {
        HirFileIdRepr::FileId(file_id) => FileRange { file_id, range },
        HirFileIdRepr::MacroFile(import) => {
            let loc = import_location(database, import.import_id);
            original_file_range_inner(database, loc.file_id, loc.value)
        },
    }
}

fn import_location(
    database: &dyn DefDatabase,
    import_id: ImportId,
) -> InFile<TextRange> {
    let import_loc = database.lookup_intern_import(import_id);
    let module_info = database.module_info(import_loc.file_id);
    let def_map = database.ast_id_map(import_loc.file_id);
    let root = database
        .parse_or_resolve(import_loc.file_id)
        .unwrap()
        .syntax();
    let import: &Import = module_info.get(import_loc.value);
    let pointer = def_map.get(import.ast_id);
    let node = pointer.to_node(&root);

    import_loc.with_value(node.syntax().text_range())
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
