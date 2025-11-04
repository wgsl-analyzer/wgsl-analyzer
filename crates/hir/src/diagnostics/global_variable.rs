use hir_def::database::{DefinitionWithBodyId, GlobalVariableId};
use hir_ty::{
    database::HirDatabase,
    ty::{ArrayType, Reference, TypeKind},
    validate::AddressSpaceError,
};

pub enum GlobalVariableDiagnostic {
    MissingAddressSpace,
    AddressSpaceError(AddressSpaceError),
}

pub fn collect<Function: FnMut(GlobalVariableDiagnostic)>(
    database: &dyn HirDatabase,
    variable: GlobalVariableId,
    mut diagnostic_builder: Function,
) {
    let inference = database.infer(DefinitionWithBodyId::GlobalVariable(variable));
    let type_kind = inference.return_type().kind(database);

    if let TypeKind::Reference(Reference {
        address_space,
        access_mode,
        ..
    }) = type_kind
    {
        hir_ty::validate::validate_address_space(
            address_space,
            access_mode,
            hir_ty::validate::Scope::Module,
            &type_kind,
            database,
            |error| diagnostic_builder(GlobalVariableDiagnostic::AddressSpaceError(error)),
        );
    } else if !matches!(
        type_kind,
        TypeKind::Error
            | TypeKind::Sampler(_)
            | TypeKind::Texture(_)
            | TypeKind::Array(ArrayType {
                binding_array: true,
                ..
            })
    ) {
        diagnostic_builder(GlobalVariableDiagnostic::MissingAddressSpace);
    }
}
