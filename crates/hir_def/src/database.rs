use std::{
    fmt::{self, Debug},
    hash,
    marker::PhantomData,
};

use base_db::{EditionedFileId, FileId, PackageId, SourceDatabase};
use salsa::InternKey;
use syntax::{Edition, Parse, ast};
use triomphe::Arc;
use vfs::VfsPath;

use crate::{
    HirFileId, InFile,
    ast_id::AstIdMap,
    attributes::{AttributeDefId, AttributesWithOwner},
    body::{Body, BodySourceMap, scope::ExprScopes},
    expression_store::{ExpressionSourceMap, ExpressionStore},
    hir_file_id::HirFileIdRepr,
    item_tree::{
        Directive, Function, GlobalAssertStatement, GlobalConstant, GlobalVariable,
        ImportStatement, ItemTree, ModuleItemId, Override, Struct, TypeAlias,
    },
    nameres::DefMap,
    resolver::Resolver,
    signature::{
        ConstantSignature, FunctionSignature, GlobalAssertStatementSignature, OverrideSignature,
        StructSignature, TypeAliasSignature, VariableSignature,
    },
};

#[salsa::query_group(DefDatabaseStorage)]
pub trait DefDatabase: InternDatabase + SourceDatabase {
    fn parse_or_resolve(
        &self,
        key: HirFileId,
    ) -> Parse;

    fn editioned_file_id(
        &self,
        key: FileId,
    ) -> EditionedFileId;

    fn get_path(
        &self,
        key: HirFileId,
    ) -> VfsPath;

    fn get_file_id(
        &self,
        key: VfsPath,
    ) -> Result<FileId, ()>;

    fn ast_id_map(
        &self,
        key: HirFileId,
    ) -> Arc<AstIdMap>;

    #[salsa::invoke(ItemTree::query)]
    fn item_tree(
        &self,
        key: HirFileId,
    ) -> Arc<ItemTree>;

    #[salsa::invoke(DefMap::package_def_map_query)]
    fn package_def_map_query(
        &self,
        package: PackageId,
    ) -> Arc<DefMap>;

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

fn get_path(
    database: &dyn DefDatabase,
    file_id: HirFileId,
) -> VfsPath {
    match file_id.0 {
        HirFileIdRepr::FileId(file_id) => database.file_path(file_id.file_id),
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
) -> Parse {
    match file_id.0 {
        HirFileIdRepr::FileId(file_id) => database.parse(file_id),
    }
}

fn editioned_file_id(
    database: &dyn DefDatabase,
    file_id: FileId,
) -> EditionedFileId {
    let edition =
        if let Some((_, Some(extension))) = database.file_path(file_id).name_and_extension() {
            if extension.eq_ignore_ascii_case("wesl") {
                Edition::LATEST
            } else if extension.eq_ignore_ascii_case("wgsl") {
                Edition::Wgsl
            } else {
                Edition::CURRENT
            }
        } else {
            Edition::CURRENT
        };

    EditionedFileId { file_id, edition }
}

fn ast_id_map(
    database: &dyn DefDatabase,
    file_id: HirFileId,
) -> Arc<AstIdMap> {
    let parsed = database.parse_or_resolve(file_id);
    let map = AstIdMap::from_source(&parsed.tree());
    Arc::new(map)
}

#[salsa::query_group(InternDatabaseStorage)]
pub trait InternDatabase: SourceDatabase {
    #[salsa::interned]
    fn intern_import(
        &self,
        location: Location<ImportStatement>,
    ) -> ImportId;
    /* #[salsa::interned]
    fn intern_directive(
        &self,
        location: Location<Directive>,
    ) -> DirectiveId;*/
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
    fn from_intern_id(v: salsa::InternId) -> Self {
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
            fn from_intern_id(v: salsa::InternId) -> Self {
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
intern_id!(ImportId, Location<ImportStatement>, lookup_intern_import);
// intern_id!(DirectiveId, Location<Directive>, lookup_intern_directive);
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
intern_id!(
    GlobalAssertStatementId,
    Location<GlobalAssertStatement>,
    lookup_intern_global_assert_statement
);

/// Module items with a body.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
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
    ) -> HirFileId {
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

/// The defs which can be visible in the module.
/// Does not include things like import statements.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum ModuleDefinitionId {
    /// Modules can be *visible* inside of a module,
    /// most notably when using a `import foo::somemodule` statement.
    Module(EditionedFileId),
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
    ) -> HirFileId {
        match self {
            Self::Module(id) => HirFileId::from(id),
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
