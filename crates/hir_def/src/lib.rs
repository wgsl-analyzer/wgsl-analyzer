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
use rowan::NodeOrToken;
use syntax::{AstNode, SyntaxNode, SyntaxToken};

use crate::{db::ImportId, module_data::Import};

/// `InFile<T>` stores a value of `T` inside a particular file/syntax tree.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct InFile<T> {
    pub file_id: HirFileId,
    pub value: T,
}

impl<T> InFile<T> {
    pub fn new(
        file_id: HirFileId,
        value: T,
    ) -> InFile<T> {
        InFile { file_id, value }
    }

    // Similarly, naming here is stupid...
    pub fn with_value<U>(
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

    pub fn as_ref(&self) -> InFile<&T> {
        self.with_value(&self.value)
    }

    pub fn file_syntax(
        &self,
        db: &dyn db::DefDatabase,
    ) -> SyntaxNode {
        db.parse_or_resolve(self.file_id)
            .expect("source created from invalid file")
            .syntax()
    }
}

impl<N: AstNode> InFile<N> {
    pub fn original_file_range(
        &self,
        db: &dyn DefDatabase,
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
            NodeOrToken::Node(n) => n.text_range(),
            NodeOrToken::Token(t) => t.text_range(),
        }
    }
}

pub fn original_file_range<T: HasTextRange>(
    db: &dyn DefDatabase,
    file_id: HirFileId,
    val: &T,
) -> FileRange {
    original_file_range_inner(db, file_id, val.text_range())
}

fn original_file_range_inner(
    db: &dyn DefDatabase,
    file_id: HirFileId,
    range: TextRange,
) -> FileRange {
    match file_id.0 {
        HirFileIdRepr::FileId(file_id) => FileRange { file_id, range },
        HirFileIdRepr::MacroFile(import) => {
            let loc = import_location(db, import.import_id);
            original_file_range_inner(db, loc.file_id, loc.value)
        },
    }
}

fn import_location(
    db: &dyn DefDatabase,
    import_id: ImportId,
) -> InFile<TextRange> {
    let import_loc = db.lookup_intern_import(import_id);
    let module_info = db.module_info(import_loc.file_id);
    let def_map = db.ast_id_map(import_loc.file_id);
    let root = db.parse_or_resolve(import_loc.file_id).unwrap().syntax();
    let import: &Import = module_info.get(import_loc.value);
    let ptr = def_map.get(import.ast_id);
    let node = ptr.to_node(&root);

    import_loc.with_value(node.syntax().text_range())
}

pub trait HasSource {
    type Value;
    fn source(
        &self,
        db: &dyn DefDatabase,
    ) -> InFile<Self::Value>;
}

impl<N: ModuleDataNode> HasSource for InFile<ModuleItemId<N>> {
    type Value = N::Source;

    fn source(
        &self,
        db: &dyn DefDatabase,
    ) -> InFile<N::Source> {
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
