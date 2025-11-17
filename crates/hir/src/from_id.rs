//! Utility module for converting between hir_def ids and code_model wrappers.
//!
//! It's unclear if we need this long-term, but it's definitely useful while we
//! are splitting the hir.

use hir_def::{data::FieldId, database::ModuleDefinitionId};

use crate::{Field, Local, ModuleDef};

macro_rules! from_id {
    ($(($id:path, $ty:path)),* $(,)?) => {$(
        impl From<$id> for $ty {
            fn from(id: $id) -> $ty {
                $ty { id }
            }
        }
        impl From<$ty> for $id {
            fn from(ty: $ty) -> $id {
                ty.id
            }
        }
    )*}
}

from_id![
    (hir_def::database::FunctionId, crate::Function),
    (hir_def::database::GlobalVariableId, crate::GlobalVariable),
    (hir_def::database::GlobalConstantId, crate::GlobalConstant),
    (hir_def::database::OverrideId, crate::Override),
    (hir_def::database::StructId, crate::Struct),
    (hir_def::database::TypeAliasId, crate::TypeAlias),
    (hir_def::data::FieldId, crate::Field),
];

impl From<ModuleDefinitionId> for ModuleDef {
    fn from(id: ModuleDefinitionId) -> Self {
        match id {
            ModuleDefinitionId::Function(it) => Self::Function(it.into()),
            ModuleDefinitionId::GlobalVariable(it) => Self::GlobalVariable(it.into()),
            ModuleDefinitionId::GlobalConstant(it) => Self::GlobalConstant(it.into()),
            ModuleDefinitionId::Override(it) => Self::Override(it.into()),
            ModuleDefinitionId::Struct(it) => Self::Struct(it.into()),
            ModuleDefinitionId::TypeAlias(it) => Self::TypeAlias(it.into()),
        }
    }
}

impl From<ModuleDef> for ModuleDefinitionId {
    fn from(id: ModuleDef) -> Self {
        match id {
            ModuleDef::Function(it) => Self::Function(it.into()),
            ModuleDef::GlobalVariable(it) => Self::GlobalVariable(it.into()),
            ModuleDef::GlobalConstant(it) => Self::GlobalConstant(it.into()),
            ModuleDef::Override(it) => Self::Override(it.into()),
            ModuleDef::Struct(it) => Self::Struct(it.into()),
            ModuleDef::TypeAlias(it) => Self::TypeAlias(it.into()),
        }
    }
}
