use hir_def::database::{DefinitionWithBodyId, GlobalVariableId};
use hir_ty::{
    database::HirDatabase,
    ty::{ArrayType, Reference, TyKind},
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
    let inference = database.infer(DefinitionWithBodyId::GlobalVariable(var));
    let ty_kind = inference.return_type.kind(database);

    if let TyKind::Reference(Reference {
        address_space,
        access_mode,
        ..
    }) = ty_kind
    {
        hir_ty::validate::validate_address_space(
            address_space,
            access_mode,
            hir_ty::validate::Scope::Module,
            &ty_kind,
            database,
            |error| diagnostic_builder(GlobalVariableDiagnostic::AddressSpaceError(error)),
        );
    } else if !matches!(
        ty_kind,
        TyKind::Error
            | TyKind::Sampler(_)
            | TyKind::Texture(_)
            | TyKind::Array(ArrayType {
                binding_array: true,
                ..
            })
    ) {
        diagnostic_builder(GlobalVariableDiagnostic::MissingAddressSpace);
    }
}
