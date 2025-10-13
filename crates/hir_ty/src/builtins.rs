use hir_def::module_data::Name;

use crate::{
    database::HirDatabase,
    function::{FunctionDetails, ResolvedFunctionId},
    ty::{
        ArraySize, ArrayType, AtomicType, BoundVar, MatrixType, Pointer, ScalarType, TexelFormat,
        TextureDimensionality, TextureKind, TextureType, TyKind, Type, VecSize,
    },
};
use wgsl_types::{
    syntax::{AccessMode, AddressSpace},
    ty::SamplerType,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct BuiltinId(salsa::InternId);
impl salsa::InternKey for BuiltinId {
    fn from_intern_id(
        #[expect(clippy::min_ident_chars, reason = "trait impl")] v: salsa::InternId
    ) -> Self {
        Self(v)
    }

    fn as_intern_id(&self) -> salsa::InternId {
        self.0
    }
}

impl BuiltinId {
    pub fn lookup(
        self,
        database: &dyn HirDatabase,
    ) -> Builtin {
        database.lookup_intern_builtin(self)
    }
}

impl Builtin {
    pub fn intern(
        self,
        database: &dyn HirDatabase,
    ) -> BuiltinId {
        database.intern_builtin(self)
    }
}

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
