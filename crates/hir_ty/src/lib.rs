use std::sync::Arc;

use base_db::Upcast;
use builtins::{Builtin, BuiltinId};
use function::{FunctionDetails, ResolvedFunctionId};
use hir_def::{
    data::LocalFieldId,
    db::{DefDatabase, DefWithBodyId, FunctionId, Lookup, StructId},
    hir_file_id::ImportFile,
    resolver::Resolver,
    type_ref::StorageClass,
    HirFileId, InFile,
};
use infer::{InferenceResult, TyLoweringContext};
use la_arena::ArenaMap;
use ty::{Ty, TyKind};

pub mod builtins;
mod function;
pub mod infer;
pub mod layout;
pub mod ty;
pub mod validate;

#[salsa::query_group(HirDatabaseStorage)]
pub trait HirDatabase: DefDatabase + Upcast<dyn DefDatabase> {
    #[salsa::invoke(infer::infer_query)]
    fn infer(&self, def: DefWithBodyId) -> Arc<InferenceResult>;

    fn field_types(&self, strukt: StructId) -> Arc<ArenaMap<LocalFieldId, Ty>>;
    fn function_type(&self, function: FunctionId) -> ResolvedFunctionId;

    fn struct_is_used_in_uniform(&self, strukt: StructId, file_id: HirFileId) -> bool;

    #[salsa::interned]
    fn intern_ty(&self, ty: TyKind) -> Ty;

    #[salsa::interned]
    fn intern_builtin(&self, builtin: Builtin) -> BuiltinId;

    #[salsa::interned]
    fn intern_resolved_function(&self, builtin: FunctionDetails) -> ResolvedFunctionId;
}

fn field_types(db: &dyn HirDatabase, strukt: StructId) -> Arc<ArenaMap<LocalFieldId, Ty>> {
    let data = db.struct_data(strukt);

    let file_id = strukt.lookup(db.upcast()).file_id;
    let module_info = db.module_info(file_id);
    let resolver = Resolver::default().push_module_scope(db.upcast(), file_id, module_info);

    let mut ty_ctx = TyLoweringContext::new(db, &resolver);

    let mut map = ArenaMap::default();
    for (idx, field) in data.fields.iter() {
        let ty = db.lookup_intern_type_ref(field.ty);
        let ty = ty_ctx.lower_ty(&ty);

        map.insert(idx, ty);
    }

    Arc::new(map)
}

fn function_type(db: &dyn HirDatabase, function: FunctionId) -> ResolvedFunctionId {
    let data = db.fn_data(function);

    let file_id = function.lookup(db.upcast()).file_id;
    let module_info = db.module_info(file_id);
    let resolver = Resolver::default().push_module_scope(db.upcast(), file_id, module_info);

    let mut ty_ctx = TyLoweringContext::new(db, &resolver);

    let return_type = data
        .return_type
        .map(|type_ref| ty_ctx.lower_ty(&db.lookup_intern_type_ref(type_ref)));
    let parameters = data
        .params
        .iter()
        .map(|(type_ref, name)| {
            let ty = ty_ctx.lower_ty(&db.lookup_intern_type_ref(*type_ref));
            (ty, name.clone())
        })
        .collect();

    db.intern_resolved_function(FunctionDetails {
        return_type,
        parameters,
    })
}

fn struct_is_used_in_uniform(db: &dyn HirDatabase, strukt: StructId, file_id: HirFileId) -> bool {
    let module_info = db.module_info(file_id);
    module_info.items().iter().any(|item| match *item {
        hir_def::module_data::ModuleItem::Import(import) => {
            let import_id = db.intern_import(InFile::new(file_id, import));
            let file_id = ImportFile { import_id };
            db.struct_is_used_in_uniform(strukt, file_id.into())
        }
        hir_def::module_data::ModuleItem::GlobalVariable(decl) => {
            let decl = db.intern_global_variable(InFile::new(file_id, decl));
            let data = db.global_var_data(decl);

            if !matches!(data.storage_class, Some(StorageClass::Uniform)) {
                return false;
            }

            let inference = db.infer(DefWithBodyId::GlobalVariable(decl));
            let ty = match inference.return_type {
                Some(ty) => ty,
                None => return false,
            };

            ty.contains_struct(db, strukt)
        }
        _ => false,
    })
}
