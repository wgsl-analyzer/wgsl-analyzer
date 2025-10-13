use std::{
    fmt::{self, Debug},
    hash,
    marker::PhantomData,
};

use base_db::{FileId, SourceDatabase, TextRange, TextSize};
use salsa::InternKey;
use syntax::{
    AstNode as _, Parse,
    ast::{self, Item},
};
use triomphe::Arc;
use vfs::VfsPath;

use crate::{
    HirFileId, InFile,
    ast_id::AstIdMap,
    attributes::{Attribute, AttributeDefId, AttributesWithOwner},
    body::{Body, BodySourceMap, scope::ExprScopes},
    data::{
        FunctionData, GlobalConstantData, GlobalVariableData, OverrideData, StructData,
        TypeAliasData,
    },
    expression_store::{ExpressionSourceMap, ExpressionStore},
    hir_file_id::{HirFileIdRepr, relative_file},
    module_data::{
        Function, GlobalConstant, GlobalVariable, ModuleInfo, ModuleItemId, Override, Struct,
        TypeAlias,
    },
    resolver::Resolver,
};

#[salsa::query_group(DefDatabaseStorage)]
pub trait DefDatabase: InternDatabase + SourceDatabase {
    fn parse_or_resolve(
        &self,
        key: HirFileId,
    ) -> Result<Parse, ()>;

    fn get_path(
        &self,
        key: HirFileId,
    ) -> Result<VfsPath, ()>;

    fn get_file_id(
        &self,
        key: VfsPath,
    ) -> Result<FileId, ()>;

    fn ast_id_map(
        &self,
        key: HirFileId,
    ) -> Arc<AstIdMap>;

    fn resolve_full_source(
        &self,
        key: HirFileId,
    ) -> Result<String, ()>;

    #[salsa::invoke(ModuleInfo::module_info_query)]
    fn module_info(
        &self,
        key: HirFileId,
    ) -> Arc<ModuleInfo>;

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

    #[salsa::invoke(FunctionData::query)]
    fn fn_data(
        &self,
        key: FunctionId,
    ) -> (Arc<FunctionData>, Arc<ExpressionSourceMap>);

    #[salsa::invoke(StructData::query)]
    fn struct_data(
        &self,
        key: StructId,
    ) -> (Arc<StructData>, Arc<ExpressionSourceMap>);

    #[salsa::invoke(TypeAliasData::type_alias_data_query)]
    fn type_alias_data(
        &self,
        key: TypeAliasId,
    ) -> (Arc<TypeAliasData>, Arc<ExpressionSourceMap>);

    #[salsa::invoke(GlobalVariableData::global_var_data_query)]
    fn global_var_data(
        &self,
        key: GlobalVariableId,
    ) -> (Arc<GlobalVariableData>, Arc<ExpressionSourceMap>);

    #[salsa::invoke(GlobalConstantData::global_constant_data_query)]
    fn global_constant_data(
        &self,
        key: GlobalConstantId,
    ) -> (Arc<GlobalConstantData>, Arc<ExpressionSourceMap>);

    #[salsa::invoke(OverrideData::override_data_query)]
    fn override_data(
        &self,
        key: OverrideId,
    ) -> (Arc<OverrideData>, Arc<ExpressionSourceMap>);

    #[salsa::invoke(AttributesWithOwner::attrs_query)]
    fn attrs(
        &self,
        key: AttributeDefId,
    ) -> (Arc<AttributesWithOwner>, Arc<ExpressionSourceMap>);
}

fn get_path(
    database: &dyn DefDatabase,
    file_id: HirFileId,
) -> Result<VfsPath, ()> {
    match file_id.0 {
        HirFileIdRepr::FileId(file_id) => Ok(database.file_path(file_id)),
    }
}

#[expect(clippy::unnecessary_wraps, reason = "Needed for salsa")]
fn get_file_id(
    database: &dyn DefDatabase,
    path: VfsPath,
) -> Result<FileId, ()> {
    Ok(database.file_id(path))
}

fn parse_or_resolve(
    database: &dyn DefDatabase,
    file_id: HirFileId,
) -> Result<Parse, ()> {
    match file_id.0 {
        HirFileIdRepr::FileId(file_id) => Ok(database.parse(file_id)),
    }
}

fn resolve_full_source(
    database: &dyn DefDatabase,
    file_id: HirFileId,
) -> Result<String, ()> {
    let parse = database.parse_or_resolve(file_id)?;
    let root = ast::SourceFile::cast(parse.syntax().clone_for_update()).unwrap();
    Ok(root.syntax().to_string())
}

fn ast_id_map(
    database: &dyn DefDatabase,
    file_id: HirFileId,
) -> Arc<AstIdMap> {
    let map = database
        .parse_or_resolve(file_id)
        .map(|source| AstIdMap::from_source(&source.tree()))
        .unwrap_or_default();
    Arc::new(map)
}

#[salsa::query_group(InternDatabaseStorage)]
pub trait InternDatabase: SourceDatabase {
    #[salsa::interned]
    fn intern_function(
        &self,
        loc: Location<Function>,
    ) -> FunctionId;
    #[salsa::interned]
    fn intern_global_variable(
        &self,
        location: Location<GlobalVariable>,
    ) -> GlobalVariableId;
    #[salsa::interned]
    fn intern_global_constant(
        &self,
        loc: Location<GlobalConstant>,
    ) -> GlobalConstantId;
    #[salsa::interned]
    fn intern_override(
        &self,
        loc: Location<Override>,
    ) -> OverrideId;
    #[salsa::interned]
    fn intern_struct(
        &self,
        loc: Location<Struct>,
    ) -> StructId;
    #[salsa::interned]
    fn intern_type_alias(
        &self,
        loc: Location<TypeAlias>,
    ) -> TypeAliasId;
}

pub type Location<T> = InFile<ModuleItemId<T>>;

pub struct Interned<T>(salsa::InternId, PhantomData<T>);

impl<T> hash::Hash for Interned<T> {
    fn hash<H: hash::Hasher>(
        &self,
        state: &mut H,
    ) {
        self.0.hash(state);
    }
}

impl<T> PartialEq for Interned<T> {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Interned<T> {}

impl<T> Clone for Interned<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Interned<T> {}

impl<T> fmt::Debug for Interned<T> {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.debug_tuple("Interned").field(&self.0).finish()
    }
}

impl<T> InternKey for Interned<T> {
    fn from_intern_id(
        #[expect(clippy::min_ident_chars, reason = "trait impl")] v: salsa::InternId
    ) -> Self {
        Self(v, PhantomData)
    }

    fn as_intern_id(&self) -> salsa::InternId {
        self.0
    }
}

macro_rules! intern_id {
    ($id:ident, $loc:ty, $lookup:ident) => {
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
        pub struct $id(salsa::InternId);
        impl InternKey for $id {
            fn from_intern_id(
                #[expect(clippy::min_ident_chars, reason = "trait impl")] v: salsa::InternId
            ) -> Self {
                $id(v)
            }

            fn as_intern_id(&self) -> salsa::InternId {
                self.0
            }
        }

        impl Lookup for $id {
            type Data = $loc;

            fn lookup(
                &self,
                database: &dyn DefDatabase,
            ) -> $loc {
                database.$lookup(*self)
            }
        }
    };
}

pub trait Lookup: Sized {
    type Data;
    fn lookup(
        &self,
        database: &dyn DefDatabase,
    ) -> Self::Data;
}

intern_id!(FunctionId, Location<Function>, lookup_intern_function);
intern_id!(
    GlobalVariableId,
    Location<GlobalVariable>,
    lookup_intern_global_variable
);
intern_id!(
    GlobalConstantId,
    Location<GlobalConstant>,
    lookup_intern_global_constant
);
intern_id!(OverrideId, Location<Override>, lookup_intern_override);
intern_id!(StructId, Location<Struct>, lookup_intern_struct);
intern_id!(TypeAliasId, Location<TypeAlias>, lookup_intern_type_alias);

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum DefinitionWithBodyId {
    Function(FunctionId),
    GlobalVariable(GlobalVariableId),
    GlobalConstant(GlobalConstantId),
    Override(OverrideId),
}

impl DefinitionWithBodyId {
    pub fn file_id(
        self,
        database: &dyn DefDatabase,
    ) -> HirFileId {
        match self {
            Self::Function(id) => id.lookup(database).file_id,
            Self::GlobalVariable(id) => id.lookup(database).file_id,
            Self::GlobalConstant(id) => id.lookup(database).file_id,
            Self::Override(id) => id.lookup(database).file_id,
        }
    }

    pub fn resolver(
        self,
        database: &dyn DefDatabase,
    ) -> Resolver {
        let file_id = self.file_id(database);
        let module_info = database.module_info(file_id);
        Resolver::default().push_module_scope(file_id, module_info)
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum DefinitionId {
    Function(FunctionId),
    GlobalVariable(GlobalVariableId),
    GlobalConstant(GlobalConstantId),
    Override(OverrideId),
    Struct(StructId),
    TypeAlias(TypeAliasId),
}

impl DefinitionId {
    pub fn file_id(
        self,
        database: &dyn DefDatabase,
    ) -> HirFileId {
        match self {
            Self::Function(id) => id.lookup(database).file_id,
            Self::GlobalVariable(id) => id.lookup(database).file_id,
            Self::GlobalConstant(id) => id.lookup(database).file_id,
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
        let module_info = database.module_info(file_id);
        Resolver::default().push_module_scope(file_id, module_info)
    }
}
