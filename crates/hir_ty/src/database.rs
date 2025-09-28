//! The home of `HirDatabase`, which is the Salsa database containing all the
//! type inference-related queries.

use std::fmt;

use crate::builtins::{Builtin, BuiltinId};
use crate::function::{FunctionDetails, ResolvedFunctionId};
use crate::infer::{InferenceContext, InferenceResult, TyLoweringContext, TypeContainer};
use crate::ty::{TyKind, Type};
use hir_def::data::FieldId;
use hir_def::database::{DefinitionWithBodyId, GlobalVariableId};
use hir_def::{
    HirFileId, InFile,
    data::LocalFieldId,
    database::{DefDatabase, DefinitionId, FunctionId, Lookup as _, StructId},
    resolver::Resolver,
};
use la_arena::ArenaMap;
use triomphe::Arc;
use wgsl_types::syntax::AddressSpace;

#[salsa::query_group(HirDatabaseStorage)]
pub trait HirDatabase: DefDatabase + fmt::Debug {
    #[salsa::invoke(crate::infer::infer_query)]
    #[salsa::cycle(crate::infer::infer_cycle_result)]
    fn infer(
        &self,
        key: DefinitionWithBodyId,
    ) -> Arc<InferenceResult>;

    fn field_types(
        &self,
        key: StructId,
    ) -> Arc<ArenaMap<LocalFieldId, Type>>;
    fn function_type(
        &self,
        key: FunctionId,
    ) -> ResolvedFunctionId;
    fn struct_is_used_in_uniform(
        &self,
        key: StructId,
        file_id: HirFileId,
    ) -> bool;

    #[salsa::interned]
    fn intern_ty(
        &self,
        r#type: TyKind,
    ) -> Type;

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
    database: &dyn HirDatabase,
    r#struct: StructId,
) -> Arc<ArenaMap<LocalFieldId, Type>> {
    let data = database.struct_data(r#struct).0;

    let file_id = r#struct.lookup(database).file_id;
    let module_info = database.module_info(file_id);
    let resolver = Resolver::default().push_module_scope(file_id, module_info);

    let mut ty_ctx = TyLoweringContext::new(database, &resolver, &data.store);

    let mut map = ArenaMap::default();
    for (index, field) in data.fields.iter() {
        let r#type = ty_ctx.lower_ty(&field.r#type);

        map.insert(index, r#type);
    }

    Arc::new(map)
}

fn function_type(
    database: &dyn HirDatabase,
    function: FunctionId,
) -> ResolvedFunctionId {
    let data = database.fn_data(function).0;

    let file_id = function.lookup(database).file_id;
    let module_info = database.module_info(file_id);
    let resolver = Resolver::default().push_module_scope(file_id, module_info);

    let mut ty_ctx = TyLoweringContext::new(database, &resolver, &data.store);

    let return_type = data
        .return_type
        .as_ref()
        .map(|type_ref| ty_ctx.lower_ty(&type_ref));

    let parameters = data
        .parameters
        .iter()
        .map(|(_, param)| {
            let r#type = ty_ctx.lower_ty(&param.r#type);
            (r#type, param.name.clone())
        })
        .collect();

    FunctionDetails {
        return_type,
        parameters,
    }
    .intern(database)
}

fn struct_is_used_in_uniform(
    database: &dyn HirDatabase,
    r#struct: StructId,
    file_id: HirFileId,
) -> bool {
    let module_info = database.module_info(file_id);
    module_info.items().iter().any(|item| match *item {
        hir_def::module_data::ModuleItem::GlobalVariable(decl) => {
            let decl = database.intern_global_variable(InFile::new(file_id, decl));
            let inference = database.infer(DefinitionWithBodyId::GlobalVariable(decl));
            let ty_kind = inference.return_type.kind(database);

            if let TyKind::Reference(crate::ty::Reference { address_space, .. }) = ty_kind
                && !matches!(address_space, AddressSpace::Uniform)
            {
                return false;
            }

            inference.return_type.contains_struct(database, r#struct)
        },
        hir_def::module_data::ModuleItem::Function(_)
        | hir_def::module_data::ModuleItem::Struct(_)
        | hir_def::module_data::ModuleItem::GlobalConstant(_)
        | hir_def::module_data::ModuleItem::Override(_)
        | hir_def::module_data::ModuleItem::TypeAlias(_) => false,
    })
}
