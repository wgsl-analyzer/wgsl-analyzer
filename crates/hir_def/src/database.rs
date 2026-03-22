use std::fmt::{self, Debug};

use crate::{
    FileAstId, InFile,
    ast_id::AstIdMap,
    attributes::{AttributeDefId, AttributesWithOwner},
    body::{Body, BodySourceMap, scope::ExprScopes},
    expression_store::{ExpressionSourceMap, ExpressionStore},
    item_tree::{
        Directive, Function, GlobalAssertStatement, GlobalConstant, GlobalVariable,
        ImportStatement, ItemTree, ModuleItemId, Override, Struct, TypeAlias,
    },
    resolver::Resolver,
    signature::{
        ConstantSignature, FunctionSignature, GlobalAssertStatementSignature, OverrideSignature,
        StructSignature, TypeAliasSignature, VariableSignature,
    },
};
use base_db::{
    EditionedFileId, Lookup as _, RootQueryDb, SourceDatabase, impl_intern_key, impl_intern_lookup,
};
use salsa::plumbing::AsId as _;
use syntax::{Parse, ast};
use triomphe::Arc;
use vfs::VfsPath;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExtensionsConfig {
    pub shader_int64: bool,
}

#[query_group::query_group(DefDatabaseStorage)]
pub trait DefDatabase: InternDatabase + SourceDatabase {
    /// Which language extensions are enabled.
    #[salsa::input]
    fn extensions(&self) -> ExtensionsConfig;

    fn parse_or_resolve(
        &self,
        key: EditionedFileId,
    ) -> Parse;

    fn ast_id_map(
        &self,
        key: EditionedFileId,
    ) -> Arc<AstIdMap>;

    #[salsa::invoke(ItemTree::query)]
    fn item_tree(
        &self,
        key: EditionedFileId,
    ) -> Arc<ItemTree>;

    #[salsa::invoke(Body::body_with_source_map_query)]
    fn body_with_source_map(
        &self,
        key: DefinitionWithBodyId,
    ) -> (Arc<Body>, Arc<BodySourceMap>);

    #[salsa::invoke(Body::body_query)]
    fn body(
        &self,
        key: DefinitionWithBodyId,
    ) -> Arc<Body>;

    #[salsa::invoke(ExprScopes::expression_scopes_query)]
    fn expression_scopes(
        &self,
        key: DefinitionWithBodyId,
    ) -> Arc<ExprScopes>;

    #[salsa::invoke(signature_with_source_map)]
    fn signature_with_source_map(
        &self,
        key: DefinitionWithBodyId,
    ) -> (Arc<ExpressionStore>, Arc<ExpressionSourceMap>);

    #[salsa::invoke(FunctionSignature::query)]
    fn function_data(
        &self,
        key: FunctionId,
    ) -> (Arc<FunctionSignature>, Arc<ExpressionSourceMap>);

    #[salsa::invoke(StructSignature::query)]
    fn struct_data(
        &self,
        key: StructId,
    ) -> (Arc<StructSignature>, Arc<ExpressionSourceMap>);

    #[salsa::invoke(TypeAliasSignature::query)]
    fn type_alias_data(
        &self,
        key: TypeAliasId,
    ) -> (Arc<TypeAliasSignature>, Arc<ExpressionSourceMap>);

    #[salsa::invoke(VariableSignature::query)]
    fn global_var_data(
        &self,
        key: GlobalVariableId,
    ) -> (Arc<VariableSignature>, Arc<ExpressionSourceMap>);

    #[salsa::invoke(ConstantSignature::query)]
    fn global_constant_data(
        &self,
        key: GlobalConstantId,
    ) -> (Arc<ConstantSignature>, Arc<ExpressionSourceMap>);

    #[salsa::invoke(GlobalAssertStatementSignature::query)]
    fn global_assert_statement_data(
        &self,
        key: GlobalAssertStatementId,
    ) -> (
        Arc<GlobalAssertStatementSignature>,
        Arc<ExpressionSourceMap>,
    );

    #[salsa::invoke(OverrideSignature::query)]
    fn override_data(
        &self,
        key: OverrideId,
    ) -> (Arc<OverrideSignature>, Arc<ExpressionSourceMap>);

    #[salsa::invoke(AttributesWithOwner::attrs_query)]
    fn attrs(
        &self,
        key: AttributeDefId,
    ) -> (Arc<AttributesWithOwner>, Arc<ExpressionSourceMap>);
}

fn signature_with_source_map(
    database: &dyn DefDatabase,
    key: DefinitionWithBodyId,
) -> (Arc<ExpressionStore>, Arc<ExpressionSourceMap>) {
    match key {
        DefinitionWithBodyId::Function(id) => {
            let (data, source_map) = database.function_data(id);
            (data.store.clone(), source_map)
        },
        DefinitionWithBodyId::GlobalVariable(id) => {
            let (data, source_map) = database.global_var_data(id);
            (data.store.clone(), source_map)
        },
        DefinitionWithBodyId::GlobalConstant(id) => {
            let (data, source_map) = database.global_constant_data(id);
            (data.store.clone(), source_map)
        },
        DefinitionWithBodyId::Override(id) => {
            let (data, source_map) = database.override_data(id);
            (data.store.clone(), source_map)
        },
        DefinitionWithBodyId::GlobalAssertStatement(id) => {
            let (data, source_map) = database.global_assert_statement_data(id);
            (data.store.clone(), source_map)
        },
    }
}

fn parse_or_resolve(
    database: &dyn DefDatabase,
    file_id: EditionedFileId,
) -> Parse {
    database.parse(file_id)
}

fn ast_id_map(
    database: &dyn DefDatabase,
    file_id: EditionedFileId,
) -> Arc<AstIdMap> {
    let parsed = database.parse_or_resolve(file_id);
    let map = AstIdMap::from_source(&parsed.tree());
    Arc::new(map)
}

#[query_group::query_group(InternDatabaseStorage)]
pub trait InternDatabase: RootQueryDb {
    #[salsa::interned]
    fn intern_import(
        &self,
        location: Location<ImportStatement>,
    ) -> ImportId;
    #[salsa::interned]
    fn intern_directive(
        &self,
        location: Location<Directive>,
    ) -> DirectiveId;
    #[salsa::interned]
    fn intern_function(
        &self,
        location: Location<Function>,
    ) -> FunctionId;
    #[salsa::interned]
    fn intern_global_variable(
        &self,
        location: Location<GlobalVariable>,
    ) -> GlobalVariableId;
    #[salsa::interned]
    fn intern_global_constant(
        &self,
        location: Location<GlobalConstant>,
    ) -> GlobalConstantId;
    #[salsa::interned]
    fn intern_override(
        &self,
        location: Location<Override>,
    ) -> OverrideId;
    #[salsa::interned]
    fn intern_struct(
        &self,
        location: Location<Struct>,
    ) -> StructId;
    #[salsa::interned]
    fn intern_type_alias(
        &self,
        location: Location<TypeAlias>,
    ) -> TypeAliasId;
    #[salsa::interned]
    fn intern_global_assert_statement(
        &self,
        location: Location<GlobalAssertStatement>,
    ) -> GlobalAssertStatementId;
}

pub type Location<T> = InFile<ModuleItemId<T>>;

macro_rules! impl_intern {
    ($id:ident, $loc:ty, $intern:ident, $lookup:ident) => {
        impl_intern_key!($id, $loc);
        impl_intern_lookup!(DefDatabase, $id, $loc, $intern, $lookup);
    };
}

impl_intern!(
    ImportId,
    Location<ImportStatement>,
    intern_import,
    lookup_intern_import
);
impl_intern!(
    DirectiveId,
    Location<Directive>,
    intern_directive,
    lookup_intern_directive
);
impl_intern!(
    FunctionId,
    Location<Function>,
    intern_function,
    lookup_intern_function
);
impl_intern!(
    GlobalVariableId,
    Location<GlobalVariable>,
    intern_global_variable,
    lookup_intern_global_variable
);
impl_intern!(
    GlobalConstantId,
    Location<GlobalConstant>,
    intern_global_constant,
    lookup_intern_global_constant
);
impl_intern!(
    OverrideId,
    Location<Override>,
    intern_override,
    lookup_intern_override
);
impl_intern!(
    StructId,
    Location<Struct>,
    intern_struct,
    lookup_intern_struct
);
impl_intern!(
    TypeAliasId,
    Location<TypeAlias>,
    intern_type_alias,
    lookup_intern_type_alias
);
impl_intern!(
    GlobalAssertStatementId,
    Location<GlobalAssertStatement>,
    intern_global_assert_statement,
    lookup_intern_global_assert_statement
);

/// Module items with a body.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, salsa_macros::Supertype)]
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
        database: &dyn DefDatabase,
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
        database: &dyn DefDatabase,
    ) -> Resolver {
        let file_id = self.file_id(database);
        let module_info = database.item_tree(file_id);
        Resolver::default().push_module_scope(file_id, module_info)
    }
}

/// All module items.
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
        database: &dyn DefDatabase,
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
        database: &dyn DefDatabase,
    ) -> Resolver {
        let file_id = self.file_id(database);
        let module_info = database.item_tree(file_id);
        Resolver::default().push_module_scope(file_id, module_info)
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
