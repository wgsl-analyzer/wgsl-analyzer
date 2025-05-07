use hir_def::db::{DefinitionWithBodyId, GlobalVariableId};
use hir_ty::{
    db::HirDatabase,
    ty::{ArrayType, TyKind},
    validate::AddressSpaceError,
};

pub enum GlobalVariableDiagnostic {
    MissingAddressSpace,
    AddressSpaceError(AddressSpaceError),
}

pub fn collect(
    db: &dyn HirDatabase,
    var: GlobalVariableId,
    mut f: impl FnMut(GlobalVariableDiagnostic),
) {
    let data = db.global_var_data(var);
    let infer = db.infer(DefinitionWithBodyId::GlobalVariable(var));

    let ty_kind = infer.return_type.map(|r#type| r#type.kind(db));

    if let Some(address_space) = data.address_space {
        hir_ty::validate::validate_address_space(
            address_space,
            data.access_mode
                .unwrap_or_else(|| address_space.default_access_mode()),
            hir_ty::validate::Scope::Module,
            ty_kind.unwrap_or(TyKind::Error),
            db,
            |error| f(GlobalVariableDiagnostic::AddressSpaceError(error)),
        );
    } else if let Some(r#type) = ty_kind {
        if !matches!(
            r#type,
            TyKind::Error
                | TyKind::Sampler(_)
                | TyKind::Texture(_)
                | TyKind::Array(ArrayType {
                    binding_array: true,
                    ..
                })
        ) {
            f(GlobalVariableDiagnostic::MissingAddressSpace);
        }
    }
}
