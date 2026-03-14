use hir_def::item_tree::{EnableExtension, Name};
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

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct BuiltinId(salsa::InternId);
impl salsa::InternKey for BuiltinId {
    fn from_intern_id(v: salsa::InternId) -> Self {
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
    required_extension: Option<EnableExtension>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BuiltinOverloadId(usize);

impl Builtin {
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    #[must_use]
    pub const fn required_extension(&self) -> Option<EnableExtension> {
        self.required_extension
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

impl Builtin {
    /// Counts how many leading parameters of `overload` unify with `argument_types`.
    /// Returns `None` if the overload has fewer parameters than arguments.
    fn overload_match_score(
        database: &dyn HirDatabase,
        overload: &BuiltinOverload,
        argument_types: &[crate::ty::Type],
    ) -> Option<usize> {
        let function = overload.r#type.lookup(database);
        if function.parameters.len() < argument_types.len() {
            return None;
        }
        let mut table = crate::infer::unify::UnificationTable::default();
        let mut score = 0_usize;
        for (expected, &found) in function.parameters().zip(argument_types.iter()) {
            let found = found.unref(database);
            if crate::infer::unify::unify(database, &mut table, expected, found).is_ok() {
                score += 1;
            } else {
                break;
            }
        }
        Some(score)
    }

    /// Returns matching overloads sorted by match quality (best first).
    /// Falls back to all overloads when none match or `argument_types` is empty.
    pub fn matching_overloads<'overloads>(
        &'overloads self,
        database: &'overloads dyn HirDatabase,
        argument_types: &'overloads [crate::ty::Type],
    ) -> Vec<(BuiltinOverloadId, &'overloads BuiltinOverload)> {
        if argument_types.is_empty() {
            return self.overloads().collect();
        }

        let mut scored: Vec<_> = self
            .overloads()
            .filter_map(|(id, overload)| {
                let score = Self::overload_match_score(database, overload, argument_types)?;
                // Only include overloads that matched at least the first argument
                (score > 0).then_some((score, id, overload))
            })
            .collect();

        if scored.is_empty() {
            return self.overloads().collect();
        }

        // Sort by score descending (best match first)
        scored.sort_by(|lhs, rhs| rhs.0.cmp(&lhs.0));
        scored
            .into_iter()
            .map(|(_, id, overload)| (id, overload))
            .collect()
    }

    /// Returns the single overload that is an exact match (all parameters
    /// accounted for and unified), or `None` if zero or multiple overloads
    /// match exactly.  Used for hover where we want a single best result.
    pub fn exact_overload<'overloads>(
        &'overloads self,
        database: &'overloads dyn HirDatabase,
        argument_types: &'overloads [crate::ty::Type],
    ) -> Option<&'overloads BuiltinOverload> {
        if argument_types.is_empty() {
            return None;
        }

        let exact: Vec<_> = self
            .overloads()
            .filter(|(_, overload)| {
                let function = overload.r#type.lookup(database);
                // Must have exactly the same number of params as arguments
                if function.parameters.len() != argument_types.len() {
                    return false;
                }
                let mut table = crate::infer::unify::UnificationTable::default();
                function
                    .parameters()
                    .zip(argument_types.iter())
                    .all(|(expected, &found)| {
                        let found = found.unref(database);
                        crate::infer::unify::unify(database, &mut table, expected, found).is_ok()
                    })
            })
            .collect();

        (exact.len() == 1).then(|| exact[0].1)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct BuiltinOverload {
    pub generics: Vec<GenericArgKind>,
    pub r#type: ResolvedFunctionId,
}

include!(concat!(env!("OUT_DIR"), "/generated/builtins.rs"));
