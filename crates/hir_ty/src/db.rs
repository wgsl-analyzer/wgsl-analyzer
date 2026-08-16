//! The home of `HirDatabase`, which is the Salsa database containing all the
//! type inference-related queries.

use base_db::{EditionedFileId, Intern as _, Lookup as _, SourceDatabase};
use hir_def::db::{Location, ModuleDefinitionId};
use hir_def::signature::{StructSignature, TypeAliasSignature};
use hir_def::{
    db::{DefinitionWithBodyId, FunctionId, StructId, TypeAliasId},
    item_scope::ItemScope,
    item_tree::ItemTree,
    resolver::Resolver,
    signature::{FieldId, FunctionSignature, LocalFieldId},
};
use la_arena::ArenaMap;
use triomphe::Arc;
use wgsl_types::syntax::AddressSpace;

use crate::infer::get_name_and_range;
use crate::{
    diagnostics::{InferenceDiagnostic, InferenceDiagnosticKind},
    function::{FunctionDetails, ResolvedFunctionId},
    infer::InferenceResult,
    lower::{TypeLoweringContext, TypeLoweringError},
    ty::{Type, TypeKind},
};

#[salsa::db]
pub trait HirDatabase: SourceDatabase + 'static {
    /// Manual implementation of upcasting from `dyn SourceDatabase` to `dyn HirDatabase`.
    ///
    /// This function is needed because Rust can't perform this upcasting automatically
    /// in the general case, as `Self` could be unsized.
    fn as_dyn(&self) -> &dyn HirDatabase;

    fn field_types(
        &self,
        key: StructId,
    ) -> Arc<(ArenaMap<LocalFieldId, Type>, Vec<InferenceDiagnostic>)> {
        field_types(self.as_dyn(), key)
    }

    fn function_type(
        &self,
        key: FunctionId,
    ) -> ResolvedFunctionId {
        function_type(self.as_dyn(), key)
    }

    fn type_alias_type(
        &self,
        key: TypeAliasId,
    ) -> Arc<(Type, Vec<InferenceDiagnostic>)> {
        type_alias_type(self.as_dyn(), key)
    }

    fn struct_is_used_in_uniform(
        &self,
        key: StructId,
        file_id: EditionedFileId,
    ) -> bool {
        struct_is_used_in_uniform(self.as_dyn(), key, file_id)
    }
}

#[salsa::db]
impl<T: SourceDatabase> HirDatabase for T {
    fn as_dyn(&self) -> &dyn HirDatabase {
        self
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct FieldInferenceDiagnostic {
    pub field: FieldId,
    pub error: TypeLoweringError,
}

#[salsa::tracked(returns(clone))]
fn field_types(
    db: &dyn HirDatabase,
    r#struct: StructId,
) -> Arc<(ArenaMap<LocalFieldId, Type>, Vec<InferenceDiagnostic>)> {
    let data = StructSignature::of(db, r#struct);

    let file_id = r#struct.lookup(db).file_id;
    let module_info = ItemScope::of(db, file_id);
    let resolver = Resolver::new(file_id, module_info);

    let mut type_context = TypeLoweringContext::new(db, &resolver, &data.store);

    let mut diagnostics = vec![];
    let mut map = ArenaMap::default();
    for (index, field) in data.fields.iter() {
        let r#type = type_context.lower_type(field.r#type);
        diagnostics.extend(
            type_context
                .diagnostics
                .drain(..)
                .map(|error| InferenceDiagnostic {
                    source: data.store.store_source,
                    kind: InferenceDiagnosticKind::InvalidType { error },
                }),
        );

        map.insert(index, r#type);
    }

    Arc::new((map, diagnostics))
}

#[salsa::tracked(returns(clone), cycle_result = type_alias_type_cycle_result)]
fn type_alias_type(
    db: &dyn HirDatabase,
    type_alias: TypeAliasId,
) -> Arc<(Type, Vec<InferenceDiagnostic>)> {
    let data = TypeAliasSignature::of(db, type_alias);

    let file_id = type_alias.lookup(db).file_id;
    let module_info = ItemScope::of(db, file_id);
    let resolver = Resolver::new(file_id, module_info);

    let mut type_context = TypeLoweringContext::new(db, &resolver, &data.store);
    let result = type_context.lower_type(data.r#type);
    let diagnostics = type_context
        .diagnostics
        .into_iter()
        .map(|error| InferenceDiagnostic {
            source: data.store.store_source,
            kind: InferenceDiagnosticKind::InvalidType { error },
        })
        .collect();

    Arc::new((result, diagnostics))
}

#[salsa::tracked(returns(clone))]
fn function_type(
    db: &dyn HirDatabase,
    function: FunctionId,
) -> ResolvedFunctionId {
    let data = FunctionSignature::of(db, function);

    let file_id = function.lookup(db).file_id;
    let module_info = ItemScope::of(db, file_id);
    let resolver = Resolver::new(file_id, module_info);

    let mut type_context = TypeLoweringContext::new(db, &resolver, &data.store);

    let return_type = data
        .return_type
        .map(|type_reference| type_context.lower_type(type_reference));

    let parameters = data
        .parameters
        .iter()
        .map(|(_, parameter)| {
            let r#type = type_context.lower_type(parameter.r#type);
            (r#type, parameter.name.clone())
        })
        .collect();

    FunctionDetails {
        name: data.name.clone(),
        return_type,
        parameters,
    }
    .intern(db)
}

#[salsa::tracked(returns(clone))]
fn struct_is_used_in_uniform(
    db: &dyn HirDatabase,
    r#struct: StructId,
    file_id: EditionedFileId,
) -> bool {
    let module_info = ItemTree::of(db, file_id);
    module_info
        .top_level_items()
        .iter()
        .any(|item| match *item {
            hir_def::item_tree::ModuleItemId::GlobalVariable(declaration) => {
                let declaration = Location::new(file_id, declaration).intern(db);
                let inference =
                    InferenceResult::of(db, DefinitionWithBodyId::GlobalVariable(declaration));
                let type_kind = inference.return_type().kind(db);

                if let TypeKind::Reference(crate::ty::Reference { address_space, .. }) = type_kind
                    && !matches!(address_space, AddressSpace::Uniform)
                {
                    return false;
                }

                inference.return_type().contains_struct(db, r#struct)
            },
            hir_def::item_tree::ModuleItemId::Function(_)
            | hir_def::item_tree::ModuleItemId::Struct(_)
            | hir_def::item_tree::ModuleItemId::GlobalConstant(_)
            | hir_def::item_tree::ModuleItemId::Override(_)
            | hir_def::item_tree::ModuleItemId::GlobalAssertStatement(_)
            | hir_def::item_tree::ModuleItemId::TypeAlias(_)
            | hir_def::item_tree::ModuleItemId::ImportStatement(_) => false,
        })
}

fn type_alias_type_cycle_result(
    db: &dyn HirDatabase,
    _: salsa::Id,
    type_alias: TypeAliasId,
) -> Arc<(Type, Vec<InferenceDiagnostic>)> {
    let data = TypeAliasSignature::of(db, type_alias);
    let (name, range) = get_name_and_range(db, ModuleDefinitionId::TypeAlias(type_alias));
    let error = InferenceDiagnostic {
        source: data.store.store_source,
        kind: InferenceDiagnosticKind::CyclicType { name, range },
    };
    Arc::new((TypeKind::Error.intern(db), vec![error]))
}
