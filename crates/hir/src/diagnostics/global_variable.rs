use hir_def::db::{DefinitionWithBodyId, GlobalVariableId};
use hir_ty::{
    db::HirDatabase,
    ty::{ArrayType, TyKind},
    validate::StorageClassError,
};

pub enum GlobalVariableDiagnostic {
    MissingStorageClass,
    StorageClassError(StorageClassError),
}

pub fn collect(
    db: &dyn HirDatabase,
    var: GlobalVariableId,
    mut f: impl FnMut(GlobalVariableDiagnostic),
) {
    let data = db.global_var_data(var);
    let infer = db.infer(DefinitionWithBodyId::GlobalVariable(var));

    let ty_kind = infer.return_type.map(|ty| ty.kind(db));

    if let Some(storage_class) = data.storage_class {
        hir_ty::validate::validate_storage_class(
            storage_class,
            data.access_mode
                .unwrap_or_else(|| storage_class.default_access_mode()),
            hir_ty::validate::Scope::Module,
            ty_kind.unwrap_or(TyKind::Error),
            db,
            |error| f(GlobalVariableDiagnostic::StorageClassError(error)),
        );
    } else if let Some(ty) = ty_kind {
        if !matches!(
            ty,
            TyKind::Error
                | TyKind::Sampler(_)
                | TyKind::Texture(_)
                | TyKind::Array(ArrayType {
                    binding_array: true,
                    ..
                })
        ) {
            f(GlobalVariableDiagnostic::MissingStorageClass);
        }
    }
}
