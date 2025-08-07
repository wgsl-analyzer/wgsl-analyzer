use std::fmt;

use itertools::Itertools as _;
use smallvec::{SmallVec, smallvec};
use wgsl_types::syntax::{AccessMode, AddressSpace};

use crate::{database::HirDatabase, ty::TyKind};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Scope {
    Function,
    Module,
}

impl fmt::Display for Scope {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Function => write!(formatter, "function"),
            Self::Module => write!(formatter, "module"),
        }
    }
}

/// Errors which are unfulfilled expectations.
pub enum AddressSpaceError {
    AccessMode(SmallVec<[AccessMode; 2]>),
    Scope(Scope),
    Constructable,
    HostShareable,
    /// Plain type, excluding runtime-sized arrays
    WorkgroupCompatible,
    HandleOrTexture,
}

impl fmt::Display for AddressSpaceError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::AccessMode(mode) => match mode.as_slice() {
                &[mode] => write!(formatter, "expected {mode} access mode"),
                &[mode1, mode2] => write!(formatter, "expected {mode1} or {mode2} access mode"),
                other => write!(
                    formatter,
                    "expected {} access mode",
                    other.iter().format(", ")
                ),
            },
            Self::Scope(scope) => {
                write!(formatter, "address space is only valid in {scope}-scope")
            },
            Self::Constructable => formatter.write_str("type is not constructable"),
            Self::HostShareable => formatter.write_str("type is not host-shareable"),
            Self::WorkgroupCompatible => formatter.write_str(""),
            Self::HandleOrTexture => {
                formatter.write_str("address space is only valid for handle or texture types")
            },
        }
    }
}

#[expect(clippy::cognitive_complexity, reason = "TODO")]
pub fn validate_address_space<Function: FnMut(AddressSpaceError)>(
    address_space: AddressSpace,
    access_mode: AccessMode,
    scope: Scope,
    r#type: &TyKind,
    database: &dyn HirDatabase,
    mut diagnostic_builder: Function,
) {
    // We only care about the inner type here
    let r#type = r#type.unref(database);
    let ty_is_err = r#type.is_error();

    match address_space {
        AddressSpace::Function => {
            if !matches!(scope, Scope::Function) {
                diagnostic_builder(AddressSpaceError::Scope(Scope::Function));
            }
            if !matches!(access_mode, AccessMode::ReadWrite) {
                diagnostic_builder(AddressSpaceError::AccessMode(smallvec![
                    AccessMode::ReadWrite
                ]));
            }

            if !ty_is_err && !r#type.is_constructable() {
                diagnostic_builder(AddressSpaceError::Constructable);
            }
        },
        AddressSpace::Private => {
            if !matches!(scope, Scope::Module) {
                diagnostic_builder(AddressSpaceError::Scope(Scope::Module));
            }
            if !matches!(access_mode, AccessMode::ReadWrite) {
                diagnostic_builder(AddressSpaceError::AccessMode(smallvec![
                    AccessMode::ReadWrite
                ]));
            }

            if !ty_is_err && !r#type.is_constructable() {
                diagnostic_builder(AddressSpaceError::Constructable);
            }
        },
        AddressSpace::Workgroup => {
            if !matches!(scope, Scope::Module) {
                diagnostic_builder(AddressSpaceError::Scope(Scope::Module));
            }
            if !matches!(access_mode, AccessMode::ReadWrite) {
                diagnostic_builder(AddressSpaceError::AccessMode(smallvec![
                    AccessMode::ReadWrite
                ]));
            }

            if !ty_is_err && (!r#type.is_plain() || r#type.contains_runtime_sized_array(database)) {
                diagnostic_builder(AddressSpaceError::WorkgroupCompatible);
            }
        },
        AddressSpace::Uniform => {
            if !matches!(scope, Scope::Module) {
                diagnostic_builder(AddressSpaceError::Scope(Scope::Module));
            }
            if !matches!(access_mode, AccessMode::Read) {
                diagnostic_builder(AddressSpaceError::AccessMode(smallvec![
                    AccessMode::ReadWrite
                ]));
            }

            if !r#type.is_error() && !r#type.is_host_shareable(database) {
                diagnostic_builder(AddressSpaceError::HostShareable);
            }
            if !r#type.is_error() && !r#type.is_constructable() {
                diagnostic_builder(AddressSpaceError::Constructable);
            }
        },
        AddressSpace::Storage => {
            if !matches!(scope, Scope::Module) {
                diagnostic_builder(AddressSpaceError::Scope(Scope::Module));
            }
            if !matches!(access_mode, AccessMode::ReadWrite | AccessMode::Read) {
                diagnostic_builder(AddressSpaceError::AccessMode(smallvec![
                    AccessMode::ReadWrite
                ]));
            }

            if !r#type.is_error() && !r#type.is_host_shareable(database) {
                diagnostic_builder(AddressSpaceError::HostShareable);
            }
        },
        AddressSpace::Handle => {
            if !matches!(scope, Scope::Module) {
                diagnostic_builder(AddressSpaceError::Scope(Scope::Module));
            }
            if !matches!(access_mode, AccessMode::Read) {
                diagnostic_builder(AddressSpaceError::AccessMode(smallvec![
                    AccessMode::ReadWrite
                ]));
            }

            match r#type.as_ref() {
                TyKind::Sampler(_) | TyKind::Texture(_) => {},
                TyKind::Error
                | TyKind::Scalar(_)
                | TyKind::Atomic(_)
                | TyKind::Vector(_)
                | TyKind::Matrix(_)
                | TyKind::Struct(_)
                | TyKind::Array(_)
                | TyKind::Reference(_)
                | TyKind::Pointer(_)
                | TyKind::BoundVariable(_)
                | TyKind::StorageTypeOfTexelFormat(_) => {
                    diagnostic_builder(AddressSpaceError::HandleOrTexture);
                },
            }
        },
        AddressSpace::PushConstant => {
            // TODO: validate push constants
            // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/682
        },
    }
}
