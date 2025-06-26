use hir_def::database::{DefinitionWithBodyId, GlobalVariableId};
use hir_ty::{
    database::HirDatabase,
    ty::{ArrayType, TyKind},
    validate::AddressSpaceError,
};

pub enum GlobalVariableDiagnostic {
    MissingAddressSpace,
    AddressSpaceError(AddressSpaceError),
}

pub fn collect<Function: FnMut(GlobalVariableDiagnostic)>(
    database: &dyn HirDatabase,
    var: GlobalVariableId,
    mut diagnostic_builder: Function,
) {
    let data = database.global_var_data(var);
    let infer = database.infer(DefinitionWithBodyId::GlobalVariable(var));

    let ty_kind = infer.return_type.map(|r#type| r#type.kind(database));

    if let Some(address_space) = data.address_space {
        hir_ty::validate::validate_address_space(
            address_space,
            data.access_mode
                .unwrap_or_else(|| address_space.default_access_mode()),
            hir_ty::validate::Scope::Module,
            &ty_kind.unwrap_or(TyKind::Error),
            database,
            |error| diagnostic_builder(GlobalVariableDiagnostic::AddressSpaceError(error)),
        );
    } else if let Some(r#type) = ty_kind
        && !matches!(
            r#type,
            TyKind::Error
                | TyKind::Sampler(_)
                | TyKind::Texture(_)
                | TyKind::Array(ArrayType {
                    binding_array: true,
                    ..
                })
        )
    {
        diagnostic_builder(GlobalVariableDiagnostic::MissingAddressSpace);
    }
}
