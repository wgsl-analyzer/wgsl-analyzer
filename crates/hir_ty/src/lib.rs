use std::sync::Arc;

use base_db::Upcast;
use builtins::{Builtin, BuiltinId};
use hir_def::{
    data::LocalFieldId,
    db::{DefDatabase, DefWithBodyId, FunctionId, Lookup, StructId},
    resolver::Resolver,
};
use infer::{InferenceResult, TyLoweringContext};
use la_arena::ArenaMap;
use ty::{FunctionType, Ty, TyKind};

pub mod builtins;
pub mod infer;
pub mod ty;
pub mod validate;

#[salsa::query_group(HirDatabaseStorage)]
pub trait HirDatabase: DefDatabase + Upcast<dyn DefDatabase> {
    #[salsa::invoke(infer::infer_query)]
    fn infer(&self, def: DefWithBodyId) -> Arc<InferenceResult>;

    fn field_types(&self, strukt: StructId) -> Arc<ArenaMap<LocalFieldId, Ty>>;
    fn function_type(&self, function: FunctionId) -> Ty;

    #[salsa::interned]
    fn intern_ty(&self, ty: TyKind) -> Ty;

    #[salsa::interned]
    fn intern_builtin(&self, builtin: Builtin) -> BuiltinId;
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

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct FunctionTypes {
    pub function_type: Ty,
}
fn function_type(db: &dyn HirDatabase, function: FunctionId) -> Ty {
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
        .map(|type_ref| ty_ctx.lower_ty(&db.lookup_intern_type_ref(type_ref.clone())))
        .collect();

    let ty = TyKind::Function(FunctionType {
        return_type,
        parameters,
    });
    db.intern_ty(ty)
}
