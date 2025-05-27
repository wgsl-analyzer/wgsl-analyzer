use hir_def::type_ref::{AccessMode, AddressSpace};
use itertools::Itertools;
use smallvec::{SmallVec, smallvec};

use crate::{db::HirDatabase, ty::TyKind};

pub enum Scope {
    Function,
    Module,
}

impl std::fmt::Debug for Scope {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Function => write!(f, "function"),
            Self::Module => write!(f, "module"),
        }
    }
}

pub enum AddressSpaceError {
    ExpectedAccessMode(SmallVec<[AccessMode; 2]>),
    ExpectedScope(Scope),

    ExpectedConstructable,
    ExpectedHostShareable,
    /// Plain type, excluding runtime-sized arrays
    ExpectedWorkgroupCompatible,
    ExpectedHandleOrTexture,
}

impl std::fmt::Display for AddressSpaceError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            AddressSpaceError::ExpectedAccessMode(mode) => match mode.as_slice() {
                &[mode] => write!(f, "expected {mode} access mode"),
                &[mode1, mode2] => write!(f, "expected {mode1} or {mode2} access mode"),
                other => write!(f, "expected {} access mode", other.iter().format(", ")),
            },
            AddressSpaceError::ExpectedScope(scope) => {
                write!(f, "address space is only valid in {scope:?}-scope")
            },
            AddressSpaceError::ExpectedConstructable => f.write_str("type is not constructable"),
            AddressSpaceError::ExpectedHostShareable => f.write_str("type is not host-shareable"),
            AddressSpaceError::ExpectedWorkgroupCompatible => f.write_str(""),
            AddressSpaceError::ExpectedHandleOrTexture => {
                f.write_str("address space is only valid for handle or texture types")
            },
        }
    }
}

pub fn validate_address_space(
    address_space: AddressSpace,
    access_mode: AccessMode,
    scope: Scope,
    r#type: TyKind,
    db: &dyn HirDatabase,
    mut sink: impl FnMut(AddressSpaceError),
) {
    let ty_is_err = r#type.is_error();

    match address_space {
        AddressSpace::Function => {
            if !matches!(scope, Scope::Function) {
                sink(AddressSpaceError::ExpectedScope(Scope::Function));
            }
            if !matches!(access_mode, AccessMode::ReadWrite) {
                sink(AddressSpaceError::ExpectedAccessMode(smallvec![
                    AccessMode::ReadWrite
                ]));
            }

            if !ty_is_err && !r#type.is_constructable() {
                sink(AddressSpaceError::ExpectedConstructable);
            }
        },
        AddressSpace::Private => {
            if !matches!(scope, Scope::Module) {
                sink(AddressSpaceError::ExpectedScope(Scope::Module));
            }
            if !matches!(access_mode, AccessMode::ReadWrite) {
                sink(AddressSpaceError::ExpectedAccessMode(smallvec![
                    AccessMode::ReadWrite
                ]));
            }

            if !ty_is_err && !r#type.is_constructable() {
                sink(AddressSpaceError::ExpectedConstructable);
            }
        },
        AddressSpace::Workgroup => {
            if !matches!(scope, Scope::Module) {
                sink(AddressSpaceError::ExpectedScope(Scope::Module));
            }
            if !matches!(access_mode, AccessMode::ReadWrite) {
                sink(AddressSpaceError::ExpectedAccessMode(smallvec![
                    AccessMode::ReadWrite
                ]));
            }

            if !ty_is_err && (!r#type.is_plain() || r#type.contains_runtime_sized_array(db)) {
                sink(AddressSpaceError::ExpectedWorkgroupCompatible);
            }
        },
        AddressSpace::Uniform => {
            if !matches!(scope, Scope::Module) {
                sink(AddressSpaceError::ExpectedScope(Scope::Module));
            }
            if !matches!(access_mode, AccessMode::Read) {
                sink(AddressSpaceError::ExpectedAccessMode(smallvec![
                    AccessMode::ReadWrite
                ]));
            }

            if !r#type.is_error() && !r#type.is_host_shareable(db) {
                sink(AddressSpaceError::ExpectedHostShareable);
            }
            if !r#type.is_error() && !r#type.is_constructable() {
                sink(AddressSpaceError::ExpectedConstructable);
            }
        },
        AddressSpace::Storage => {
            if !matches!(scope, Scope::Module) {
                sink(AddressSpaceError::ExpectedScope(Scope::Module));
            }
            if !matches!(access_mode, AccessMode::ReadWrite | AccessMode::Read) {
                sink(AddressSpaceError::ExpectedAccessMode(smallvec![
                    AccessMode::ReadWrite
                ]));
            }

            if !r#type.is_error() && !r#type.is_host_shareable(db) {
                sink(AddressSpaceError::ExpectedHostShareable);
            }
        },
        AddressSpace::Handle => {
            if !matches!(scope, Scope::Module) {
                sink(AddressSpaceError::ExpectedScope(Scope::Module));
            }
            if !matches!(access_mode, AccessMode::Read) {
                sink(AddressSpaceError::ExpectedAccessMode(smallvec![
                    AccessMode::ReadWrite
                ]));
            }

            match r#type {
                TyKind::Sampler(_) | TyKind::Texture(_) => {},
                _ => sink(AddressSpaceError::ExpectedHandleOrTexture),
            }
        },
        AddressSpace::PushConstant => {
            // TODO: validate push constants
        },
    }
}
