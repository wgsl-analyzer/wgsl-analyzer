use std::{fmt::Debug, marker::PhantomData, sync::Arc};

use base_db::{SourceDatabase, Upcast};
use salsa::InternKey;
use syntax::Parse;

use crate::{
    ast_id::AstIdMap,
    attrs::{Attr, AttrDefId, AttrsWithOwner},
    body::scope::ExprScopes,
    body::{Body, BodySourceMap},
    data::{FunctionData, GlobalConstantData, GlobalVariableData, StructData},
    hir_file_id::HirFileIdRepr,
    module_data::{
        Function, GlobalConstant, GlobalVariable, Import, ModuleInfo, ModuleItemId, Struct,
    },
    resolver::Resolver,
    type_ref::TypeRef,
    HirFileId, InFile,
};

#[salsa::query_group(DefDatabaseStorage)]
pub trait DefDatabase: InternDatabase + Upcast<dyn SourceDatabase> {
    fn parse_or_resolve(&self, file_id: HirFileId) -> Result<Parse, ()>;

    fn ast_id_map(&self, file_id: HirFileId) -> Arc<AstIdMap>;

    #[salsa::invoke(ModuleInfo::module_info_query)]
    fn module_info(&self, file_id: HirFileId) -> Arc<ModuleInfo>;

    #[salsa::invoke(Body::body_with_source_map_query)]
    fn body_with_source_map(&self, def: DefWithBodyId) -> (Arc<Body>, Arc<BodySourceMap>);

    #[salsa::invoke(Body::body_query)]
    fn body(&self, def: DefWithBodyId) -> Arc<Body>;

    #[salsa::invoke(ExprScopes::expr_scopes_query)]
    fn expr_scopes(&self, def: DefWithBodyId) -> Arc<ExprScopes>;

    #[salsa::invoke(FunctionData::fn_data_query)]
    fn fn_data(&self, def: FunctionId) -> Arc<FunctionData>;

    #[salsa::invoke(StructData::struct_data_query)]
    fn struct_data(&self, strukt: StructId) -> Arc<StructData>;

    #[salsa::invoke(GlobalVariableData::global_var_data_query)]
    fn global_var_data(&self, def: GlobalVariableId) -> Arc<GlobalVariableData>;

    #[salsa::invoke(GlobalConstantData::global_constant_data_query)]
    fn global_constant_data(&self, def: GlobalConstantId) -> Arc<GlobalConstantData>;

    #[salsa::invoke(AttrsWithOwner::attrs_query)]
    fn attrs(&self, def: AttrDefId) -> Arc<AttrsWithOwner>;
}

fn parse_or_resolve(db: &dyn DefDatabase, file_id: HirFileId) -> Result<Parse, ()> {
    match file_id.0 {
        HirFileIdRepr::FileId(file_id) => Ok(db.parse(file_id)),
        HirFileIdRepr::MacroFile(import_file) => {
            let import_loc = db.lookup_intern_import(import_file.import_id);
            let module_info = db.module_info(import_loc.file_id);
            let import: &Import = module_info.get(import_loc.value);

            match &import.value {
                crate::module_data::ImportValue::Path(_) => Err(()), // TODO: path imports
                crate::module_data::ImportValue::Custom(key) => db.parse_import(key.clone()),
            }
        }
    }
}

fn ast_id_map(db: &dyn DefDatabase, file_id: HirFileId) -> Arc<AstIdMap> {
    let map = db
        .parse_or_resolve(file_id)
        .map(|source| AstIdMap::from_source(source.tree()))
        .unwrap_or_default();
    Arc::new(map)
}

#[salsa::query_group(InternDatabaseStorage)]
pub trait InternDatabase: SourceDatabase {
    #[salsa::interned]
    fn intern_type_ref(&self, type_ref: TypeRef) -> Interned<TypeRef>;
    #[salsa::interned]
    fn intern_attr(&self, attr: Attr) -> Interned<Attr>;

    #[salsa::interned]
    fn intern_function(&self, loc: Location<Function>) -> FunctionId;
    #[salsa::interned]
    fn intern_global_variable(&self, loc: Location<GlobalVariable>) -> GlobalVariableId;
    #[salsa::interned]
    fn intern_global_constant(&self, loc: Location<GlobalConstant>) -> GlobalConstantId;
    #[salsa::interned]
    fn intern_struct(&self, loc: Location<Struct>) -> StructId;
    #[salsa::interned]
    fn intern_import(&self, loc: Location<Import>) -> ImportId;
}
pub type Location<T> = InFile<ModuleItemId<T>>;

pub struct Interned<T>(salsa::InternId, PhantomData<T>);

impl<T> std::hash::Hash for Interned<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl<T> PartialEq for Interned<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T> Eq for Interned<T> {}
impl<T> Clone for Interned<T> {
    fn clone(&self) -> Self {
        Self(self.0, PhantomData)
    }
}
impl<T> Copy for Interned<T> {}
impl<T> std::fmt::Debug for Interned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Interned").field(&self.0).finish()
    }
}
impl<T> InternKey for Interned<T> {
    fn from_intern_id(v: salsa::InternId) -> Self {
        Interned(v, PhantomData)
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
            fn lookup(&self, db: &dyn DefDatabase) -> $loc {
                db.$lookup(*self)
            }
        }
    };
}

pub trait Lookup: Sized {
    type Data;
    fn lookup(&self, db: &dyn DefDatabase) -> Self::Data;
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
intern_id!(StructId, Location<Struct>, lookup_intern_struct);
intern_id!(ImportId, Location<Import>, lookup_intern_import);

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum DefWithBodyId {
    Function(FunctionId),
    GlobalVariable(GlobalVariableId),
    GlobalConstant(GlobalConstantId),
}
impl DefWithBodyId {
    pub fn file_id(&self, db: &dyn DefDatabase) -> HirFileId {
        match self {
            DefWithBodyId::Function(id) => id.lookup(db).file_id,
            DefWithBodyId::GlobalVariable(id) => id.lookup(db).file_id,
            DefWithBodyId::GlobalConstant(id) => id.lookup(db).file_id,
        }
    }
    pub fn resolver(&self, db: &dyn DefDatabase) -> Resolver {
        let file_id = self.file_id(db);
        let module_info = db.module_info(file_id);
        Resolver::default().push_module_scope(db, file_id, module_info)
    }
}
