use hir_def::db::{DefinitionWithBodyId, GlobalVariableId};
use hir_ty::{
    db::HirDatabase,
    infer::InferenceResult,
    ty::{ArrayType, Reference, TypeKind},
    validate::AddressSpaceError,
};

pub enum GlobalVariableDiagnostic {
    MissingAddressSpace,
    AddressSpaceError(AddressSpaceError),
}

pub fn collect<DiagnosticsBuilder>(
    db: &dyn HirDatabase,
    variable: GlobalVariableId,
    mut diagnostic_builder: DiagnosticsBuilder,
) where
    DiagnosticsBuilder: FnMut(GlobalVariableDiagnostic),
{
    let inference = InferenceResult::of(db, DefinitionWithBodyId::GlobalVariable(variable));
    let type_kind = inference.return_type().kind(db);

    if let TypeKind::Reference(Reference {
        address_space,
        access_mode,
        inner: _,
    }) = type_kind
    {
        hir_ty::validate::validate_address_space(
            address_space,
            access_mode,
            hir_ty::validate::Scope::Module,
            &type_kind,
            db,
            |error| diagnostic_builder(GlobalVariableDiagnostic::AddressSpaceError(error)),
        );
    } else if !matches!(
        type_kind,
        TypeKind::Error
            | TypeKind::Sampler(_)
            | TypeKind::Texture(_)
            | TypeKind::Array(ArrayType {
                binding_array: true,
                inner: _,
                size: _
            })
    ) {
        diagnostic_builder(GlobalVariableDiagnostic::MissingAddressSpace);
    }
}
