use std::fmt::Debug;

use base_db::{EditionedFileId, Lookup as _, SourceDatabase, impl_intern_key, impl_intern_lookup};
use salsa::plumbing::AsId as _;
use syntax::{ExtensionsConfig, Parse, ast};
use triomphe::Arc;
use vfs::VfsPath;

use crate::{
    FileAstId, InFile,
    ast_id::AstIdMap,
    attributes::{AttributeDefId, AttributesWithOwner},
    body::{Body, BodySourceMap, scope::ExprScopes},
    expression_store::{ExpressionSourceMap, ExpressionStore},
    item_scope::ItemScope,
    item_tree::{
        Directive, Function, GlobalAssertStatement, GlobalConstant, GlobalVariable,
        ImportStatement, ItemTree, ModuleItemId, Override, Struct, TypeAlias,
    },
    resolver::Resolver,
    signature::{
        AssertStatementSignature, ConstantSignature, FunctionSignature, OverrideSignature,
        StructSignature, TypeAliasSignature, VariableSignature,
    },
};

/// `Location` points to an AST node in any file. Corresponds to `AstId` in Rust-Analyzer.
///
/// It is stable across reparses, and can be used as salsa key/value.
pub type Location<T> = InFile<FileAstId<T>>;

macro_rules! impl_intern {
    ($id:ident, $loc:ty) => {
        impl_intern_key!($id, $loc);
        impl_intern_lookup!($id, $loc);
    };
}

impl_intern!(ImportId, Location<ast::ImportStatement>);
impl_intern!(DirectiveId, Location<ast::Directive>);
impl_intern!(FunctionId, Location<ast::FunctionDeclaration>);
impl_intern!(GlobalVariableId, Location<ast::VariableDeclaration>);
impl_intern!(GlobalConstantId, Location<ast::ConstantDeclaration>);
impl_intern!(OverrideId, Location<ast::OverrideDeclaration>);
impl_intern!(StructId, Location<ast::StructDeclaration>);
impl_intern!(TypeAliasId, Location<ast::TypeAliasDeclaration>);
impl_intern!(GlobalAssertStatementId, Location<ast::AssertStatement>);

/// Module items with a body.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, salsa_macros::Supertype)]
pub enum DefinitionWithBodyId {
    Function(FunctionId),
    GlobalVariable(GlobalVariableId),
    GlobalConstant(GlobalConstantId),
    GlobalAssertStatement(GlobalAssertStatementId),
    Override(OverrideId),
}

impl DefinitionWithBodyId {
    pub fn file_id(
        self,
        database: &dyn SourceDatabase,
    ) -> EditionedFileId {
        match self {
            Self::Function(id) => id.lookup(database).file_id,
            Self::GlobalVariable(id) => id.lookup(database).file_id,
            Self::GlobalConstant(id) => id.lookup(database).file_id,
            Self::GlobalAssertStatement(id) => id.lookup(database).file_id,
            Self::Override(id) => id.lookup(database).file_id,
        }
    }

    pub fn resolver(
        self,
        database: &dyn SourceDatabase,
    ) -> Resolver<'_> {
        let file_id = self.file_id(database);
        let module_info = ItemScope::of(database, file_id);
        Resolver::new(file_id, module_info)
    }
}

/// The definitions which are visible in the module.
///
/// Does not include import statements, since its the items of the import statement that are visible.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, salsa_macros::Supertype)]
pub enum ModuleDefinitionId {
    Function(FunctionId),
    GlobalVariable(GlobalVariableId),
    GlobalConstant(GlobalConstantId),
    GlobalAssertStatement(GlobalAssertStatementId),
    Override(OverrideId),
    Struct(StructId),
    TypeAlias(TypeAliasId),
}

impl ModuleDefinitionId {
    pub fn file_id(
        self,
        database: &dyn SourceDatabase,
    ) -> EditionedFileId {
        match self {
            Self::Function(id) => id.lookup(database).file_id,
            Self::GlobalVariable(id) => id.lookup(database).file_id,
            Self::GlobalConstant(id) => id.lookup(database).file_id,
            Self::GlobalAssertStatement(id) => id.lookup(database).file_id,
            Self::Override(id) => id.lookup(database).file_id,
            Self::Struct(id) => id.lookup(database).file_id,
            Self::TypeAlias(id) => id.lookup(database).file_id,
        }
    }

    pub fn resolver(
        self,
        database: &dyn SourceDatabase,
    ) -> Resolver<'_> {
        let file_id = self.file_id(database);
        let module_info = ItemScope::of(database, file_id);
        Resolver::new(file_id, module_info)
    }

    #[must_use]
    pub const fn with_body(self) -> Option<DefinitionWithBodyId> {
        match self {
            Self::Function(id) => Some(DefinitionWithBodyId::Function(id)),
            Self::GlobalVariable(id) => Some(DefinitionWithBodyId::GlobalVariable(id)),
            Self::GlobalConstant(id) => Some(DefinitionWithBodyId::GlobalConstant(id)),
            Self::GlobalAssertStatement(id) => {
                Some(DefinitionWithBodyId::GlobalAssertStatement(id))
            },
            Self::Override(id) => Some(DefinitionWithBodyId::Override(id)),
            Self::Struct(_) | Self::TypeAlias(_) => None,
        }
    }
}

impl From<DefinitionWithBodyId> for ModuleDefinitionId {
    fn from(value: DefinitionWithBodyId) -> Self {
        match value {
            DefinitionWithBodyId::Function(id) => Self::Function(id),
            DefinitionWithBodyId::GlobalVariable(id) => Self::GlobalVariable(id),
            DefinitionWithBodyId::GlobalConstant(id) => Self::GlobalConstant(id),
            DefinitionWithBodyId::Override(id) => Self::Override(id),
            DefinitionWithBodyId::GlobalAssertStatement(id) => Self::GlobalAssertStatement(id),
        }
    }
}
