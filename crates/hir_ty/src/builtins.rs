use base_db::{Intern as _, impl_intern_key, impl_intern_lookup};
use hir_def::item_tree::Name;
use wgsl_types::{
    syntax::{AccessMode, AddressSpace},
    ty::SamplerType,
};

use crate::{
    database::HirDatabase,
    function::{FunctionDetails, ResolvedFunctionId},
    ty::{
        ArraySize, ArrayType, AtomicType, BoundVariable, Pointer, ScalarType, TexelFormat,
        TextureDimensionality, TextureKind, TextureType, Type, TypeKind, VecSize,
    },
};

impl_intern_key!(BuiltinId, Builtin);
impl_intern_lookup!(BuiltinId, Builtin);

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum GenericArgKind {
    VecSize,
    Type,
    TexelFormat,
}

pub enum GenericArg {
    VecSize(VecSize),
    Type(Type),
    TexelFormat(TexelFormat),
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Builtin {
    name: Name,
    overloads: Vec<BuiltinOverload>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BuiltinOverloadId(usize);

impl Builtin {
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn overloads(&self) -> impl Iterator<Item = (BuiltinOverloadId, &BuiltinOverload)> {
        self.overloads
            .iter()
            .enumerate()
            .map(|(index, overload)| (BuiltinOverloadId(index), overload))
    }

    #[must_use]
    pub fn overload(
        &self,
        overload_id: BuiltinOverloadId,
    ) -> &BuiltinOverload {
        &self.overloads[overload_id.0]
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct BuiltinOverload {
    pub generics: Vec<GenericArgKind>,
    pub r#type: ResolvedFunctionId,
}

include!(concat!(env!("OUT_DIR"), "/generated/builtins.rs"));
