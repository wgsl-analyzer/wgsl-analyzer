use hir_def::type_ref::{AccessMode, StorageClass};
use itertools::Itertools;
use smallvec::{smallvec, SmallVec};

use crate::{ty::TyKind, HirDatabase};

pub enum Scope {
    Function,
    Module,
}

impl std::fmt::Debug for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Function => write!(f, "function"),
            Self::Module => write!(f, "module"),
        }
    }
}

pub enum StorageClassError {
    ExpectedAccessMode(SmallVec<[AccessMode; 2]>),
    ExpectedScope(Scope),

    ExpectedConstructable,
    ExpectedHostShareable,
    /// Plain type, excluding runtime-sized arrays
    ExpectedWorkgroupCompatible,
    ExpectedHandleOrTexture,
}

impl std::fmt::Display for StorageClassError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageClassError::ExpectedAccessMode(mode) => match mode.as_slice() {
                &[mode] => write!(f, "expected {} access mode", mode),
                &[mode1, mode2] => write!(f, "expected {} or {} access mode", mode1, mode2),
                other => write!(f, "expected {} access mode", other.iter().format(", ")),
            },
            StorageClassError::ExpectedScope(scope) => {
                write!(f, "storage class is only valid in {:?}-scope", scope)
            }
            StorageClassError::ExpectedConstructable => f.write_str("type is not constructable"),
            StorageClassError::ExpectedHostShareable => f.write_str("type is not host-shareable"),
            StorageClassError::ExpectedWorkgroupCompatible => f.write_str(""),
            StorageClassError::ExpectedHandleOrTexture => {
                f.write_str("storage class is only valid for handle or texture types")
            }
        }
    }
}

pub fn validate_storage_class(
    storage_class: StorageClass,
    access_mode: AccessMode,
    scope: Scope,
    ty: TyKind,
    db: &dyn HirDatabase,
    mut sink: impl FnMut(StorageClassError),
) {
    let ty_is_err = ty.is_err();

    match storage_class {
        StorageClass::Function => {
            if !matches!(scope, Scope::Function) {
                sink(StorageClassError::ExpectedScope(Scope::Function));
            }
            if !matches!(access_mode, AccessMode::ReadWrite) {
                sink(StorageClassError::ExpectedAccessMode(smallvec![
                    AccessMode::ReadWrite
                ]));
            }

            if !ty_is_err && !ty.is_constructable() {
                sink(StorageClassError::ExpectedConstructable);
            }
        }
        StorageClass::Private => {
            if !matches!(scope, Scope::Module) {
                sink(StorageClassError::ExpectedScope(Scope::Module));
            }
            if !matches!(access_mode, AccessMode::ReadWrite) {
                sink(StorageClassError::ExpectedAccessMode(smallvec![
                    AccessMode::ReadWrite
                ]));
            }

            if !ty_is_err && !ty.is_constructable() {
                sink(StorageClassError::ExpectedConstructable);
            }
        }
        StorageClass::Workgroup => {
            if !matches!(scope, Scope::Module) {
                sink(StorageClassError::ExpectedScope(Scope::Module));
            }
            if !matches!(access_mode, AccessMode::ReadWrite) {
                sink(StorageClassError::ExpectedAccessMode(smallvec![
                    AccessMode::ReadWrite
                ]));
            }

            if !ty_is_err && (!ty.is_plain() || ty.contains_runtime_sized_array(db)) {
                sink(StorageClassError::ExpectedWorkgroupCompatible);
            }
        }
        StorageClass::Uniform => {
            if !matches!(scope, Scope::Module) {
                sink(StorageClassError::ExpectedScope(Scope::Module));
            }
            if !matches!(access_mode, AccessMode::Read) {
                sink(StorageClassError::ExpectedAccessMode(smallvec![
                    AccessMode::ReadWrite
                ]));
            }

            if !ty.is_err() && !ty.is_host_shareable(db) {
                sink(StorageClassError::ExpectedHostShareable);
            }
            if !ty.is_err() && !ty.is_constructable() {
                sink(StorageClassError::ExpectedConstructable);
            }
        }
        StorageClass::Storage => {
            if !matches!(scope, Scope::Module) {
                sink(StorageClassError::ExpectedScope(Scope::Module));
            }
            if !matches!(access_mode, AccessMode::ReadWrite | AccessMode::Read) {
                sink(StorageClassError::ExpectedAccessMode(smallvec![
                    AccessMode::ReadWrite
                ]));
            }

            if !ty.is_err() && !ty.is_host_shareable(db) {
                sink(StorageClassError::ExpectedHostShareable);
            }
        }
        StorageClass::Handle => {
            if !matches!(scope, Scope::Module) {
                sink(StorageClassError::ExpectedScope(Scope::Module));
            }
            if !matches!(access_mode, AccessMode::Read) {
                sink(StorageClassError::ExpectedAccessMode(smallvec![
                    AccessMode::ReadWrite
                ]));
            }

            match ty {
                TyKind::Sampler(_) | TyKind::Texture(_) => {}
                _ => sink(StorageClassError::ExpectedHandleOrTexture),
            }
        }
        StorageClass::PushConstant => {
            // TODO: validate push constants
        }
    }
}
