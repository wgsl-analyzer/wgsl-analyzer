//! The home of `HirDatabase`, which is the Salsa database containing all the
//! type inference-related queries.

use std::sync::Arc;

use crate::builtins::{Builtin, BuiltinId};
use crate::function::{FunctionDetails, ResolvedFunctionId};
use crate::infer::{InferenceResult, TyLoweringContext};
use crate::ty::{Ty, TyKind};
use base_db::Upcast;
use hir_def::{
    HirFileId, InFile,
    data::LocalFieldId,
    db::{DefDatabase, DefinitionWithBodyId, FunctionId, Lookup, StructId},
    hir_file_id::ImportFile,
    resolver::Resolver,
    type_ref::StorageClass,
};
use la_arena::ArenaMap;

#[salsa::query_group(HirDatabaseStorage)]
pub trait HirDatabase: DefDatabase + Upcast<dyn DefDatabase> {
    #[salsa::invoke(crate::infer::infer_query)]
    fn infer(
        &self,
        def: DefinitionWithBodyId,
    ) -> Arc<InferenceResult>;

    fn field_types(
        &self,
        r#struct: StructId,
    ) -> Arc<ArenaMap<LocalFieldId, Ty>>;
    fn function_type(
        &self,
        function: FunctionId,
    ) -> ResolvedFunctionId;

    fn struct_is_used_in_uniform(
        &self,
        r#struct: StructId,
        file_id: HirFileId,
    ) -> bool;

    #[salsa::interned]
    fn intern_ty(
        &self,
        r#type: TyKind,
    ) -> Ty;

    #[salsa::interned]
    fn intern_builtin(
        &self,
        builtin: Builtin,
    ) -> BuiltinId;

    #[salsa::interned]
    fn intern_resolved_function(
        &self,
        builtin: Arc<FunctionDetails>,
    ) -> ResolvedFunctionId;
}

fn field_types(
    db: &dyn HirDatabase,
    r#struct: StructId,
) -> Arc<ArenaMap<LocalFieldId, Ty>> {
    let data = db.struct_data(r#struct);

    let file_id = r#struct.lookup(db.upcast()).file_id;
    let module_info = db.module_info(file_id);
    let resolver = Resolver::default().push_module_scope(db.upcast(), file_id, module_info);

    let mut ty_ctx = TyLoweringContext::new(db, &resolver);

    let mut map = ArenaMap::default();
    for (index, field) in data.fields.iter() {
        let r#type = db.lookup_intern_type_ref(field.r#type);
        let r#type = ty_ctx.lower_ty(&r#type);

        map.insert(index, r#type);
    }

    Arc::new(map)
}

fn function_type(
    db: &dyn HirDatabase,
    function: FunctionId,
) -> ResolvedFunctionId {
    let data = db.fn_data(function);

    let file_id = function.lookup(db.upcast()).file_id;
    let module_info = db.module_info(file_id);
    let resolver = Resolver::default().push_module_scope(db.upcast(), file_id, module_info);

    let mut ty_ctx = TyLoweringContext::new(db, &resolver);

    let return_type = data
        .return_type
        .map(|type_ref| ty_ctx.lower_ty(&db.lookup_intern_type_ref(type_ref)));
    let parameters = data
        .parameters
        .iter()
        .map(|(type_ref, name)| {
            let r#type = ty_ctx.lower_ty(&db.lookup_intern_type_ref(*type_ref));
            (r#type, name.clone())
        })
        .collect();

    FunctionDetails {
        return_type,
        parameters,
    }
    .intern(db)
}

fn struct_is_used_in_uniform(
    db: &dyn HirDatabase,
    r#struct: StructId,
    file_id: HirFileId,
) -> bool {
    let module_info = db.module_info(file_id);
    module_info.items().iter().any(|item| match *item {
        hir_def::module_data::ModuleItem::Import(import) => {
            let import_id = db.intern_import(InFile::new(file_id, import));
            let file_id = ImportFile { import_id };
            db.struct_is_used_in_uniform(r#struct, file_id.into())
        },
        hir_def::module_data::ModuleItem::GlobalVariable(decl) => {
            let decl = db.intern_global_variable(InFile::new(file_id, decl));
            let data = db.global_var_data(decl);

            if !matches!(data.storage_class, Some(StorageClass::Uniform)) {
                return false;
            }

            let inference = db.infer(DefinitionWithBodyId::GlobalVariable(decl));
            let r#type = match inference.return_type {
                Some(r#type) => r#type,
                None => return false,
            };

            r#type.contains_struct(db, r#struct)
        },
        _ => false,
    })
}
