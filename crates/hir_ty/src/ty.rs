pub mod pretty;

use std::{borrow::Cow, fmt, hash, num::NonZeroU32};

use base_db::{Intern as _, Lookup as _, impl_intern_key, impl_intern_lookup};
use hir_def::db::StructId;
use wgsl_types::{
    syntax::{AccessMode, AddressSpace, TexelFormat},
    tplt::AccelerationStructureTags,
    ty::SamplerType,
};

use crate::db::HirDatabase;

impl_intern_key!(Type, TypeKind);
impl_intern_lookup!(Type, TypeKind);

impl Type {
    pub fn kind(
        self,
        db: &dyn HirDatabase,
    ) -> TypeKind {
        self.lookup(db).clone()
    }

    pub fn is_err(
        self,
        db: &dyn HirDatabase,
    ) -> bool {
        match self.lookup(db) {
            TypeKind::Error => true,
            TypeKind::Scalar(_)
            | TypeKind::Struct(_)
            | TypeKind::BuiltinStruct(_)
            | TypeKind::Texture(_)
            | TypeKind::AccelerationStructure(_)
            | TypeKind::Sampler(_) => false,
            TypeKind::Atomic(atomic_type) => atomic_type.inner.is_err(db),
            TypeKind::Vector(vector_type) => vector_type.component_type.is_err(db),
            TypeKind::Matrix(matrix_type) => matrix_type.inner.is_err(db),
            TypeKind::Array(array_type) => array_type.inner.is_err(db),
            TypeKind::Reference(reference) => reference.inner.is_err(db),
            TypeKind::Pointer(pointer) => pointer.inner.is_err(db),
        }
    }

    #[expect(clippy::doc_paragraphs_missing_punctuation, reason = "false positive")]
    /// `T` -> `T`, `vecN<T>` -> `T`
    #[must_use]
    pub fn this_or_vec_inner(
        self,
        db: &dyn HirDatabase,
    ) -> Self {
        match self.kind(db) {
            TypeKind::Vector(vector) => vector.component_type,
            TypeKind::Reference(reference) => reference.inner.this_or_vec_inner(db),
            TypeKind::Error
            | TypeKind::Scalar(_)
            | TypeKind::Atomic(_)
            | TypeKind::Matrix(_)
            | TypeKind::Struct(_)
            | TypeKind::BuiltinStruct(_)
            | TypeKind::Array(_)
            | TypeKind::Texture(_)
            | TypeKind::AccelerationStructure(_)
            | TypeKind::Sampler(_)
            | TypeKind::Pointer(_) => self,
        }
    }

    pub fn is_convertible_to(
        self,
        r#type: Self,
        db: &dyn HirDatabase,
    ) -> bool {
        self.kind(db).is_convertible_to(&r#type.kind(db), db)
    }

    #[expect(clippy::doc_paragraphs_missing_punctuation, reason = "false positive")]
    /// The type T is the concretization of type S if:
    /// - T is concrete, and
    /// - T is not a reference type, and
    /// - ConversionRank(S, T) is finite, and
    /// - For any other non-reference type T2, ConversionRank(S, T2) > ConversionRank(S, T).
    ///
    /// The concretization of a value e of type T is the value resulting from applying, to e, the
    /// feasible conversion that maps T to the concretization of T.
    ///
    /// Reference: <https://www.w3.org/TR/WGSL/#concretization>
    #[must_use]
    pub fn concretize(
        self,
        db: &dyn HirDatabase,
    ) -> Self {
        match self.kind(db).concretize(db) {
            Some(type_kind) => type_kind.intern(db),
            None => self,
        }
    }

    #[expect(clippy::doc_paragraphs_missing_punctuation, reason = "false positive")]
    /// Apply the load rule.
    ///
    /// Reference: <https://www.w3.org/TR/WGSL/#load-rule>
    #[must_use]
    pub fn loaded(
        self,
        db: &dyn HirDatabase,
    ) -> Self {
        if let TypeKind::Reference(Reference {
            address_space: _,
            inner,
            access_mode: _,
        }) = self.kind(db)
        {
            debug_assert!(!matches!(inner.kind(db), TypeKind::Reference(_)));
            inner
        } else {
            self
        }
    }

    pub fn contains_struct(
        self,
        db: &dyn HirDatabase,
        r#struct: StructId,
    ) -> bool {
        self.kind(db).contains_struct(db, r#struct)
    }
}

#[salsa::tracked]
impl Type {
    /// Apply the load rule.
    ///
    /// Reference: <https://www.w3.org/TR/WGSL/#load-rule>
    #[salsa::tracked(cycle_result = |_, _, _| false)]
    pub fn is_constructible(
        self,
        db: &dyn HirDatabase,
    ) -> bool {
        match self.kind(db) {
            TypeKind::Error | TypeKind::Scalar(_) | TypeKind::Vector(_) | TypeKind::Matrix(_) => {
                true
            },
            TypeKind::Struct(struct_id) => db
                .field_types(struct_id)
                .0
                .iter()
                .all(|(_, field_type)| *field_type.is_constructible(db)),
            TypeKind::BuiltinStruct(builtin_struct) => builtin_struct
                .fields
                .iter()
                .all(|(_, field_type)| *field_type.is_constructible(db)),
            TypeKind::Array(array_type) => array_type.is_constructible(db),
            TypeKind::Atomic(_)
            | TypeKind::Texture(_)
            | TypeKind::Sampler(_)
            | TypeKind::Reference(_)
            | TypeKind::AccelerationStructure(_)
            | TypeKind::Pointer(_) => false,
        }
    }
}

/// A struct type returned by builtin functions.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BuiltinStruct {
    pub name: String,
    pub fields: Vec<(String, Type)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeKind {
    Error,
    Scalar(ScalarType),
    Atomic(AtomicType),
    #[expect(clippy::doc_paragraphs_missing_punctuation, reason = "false positive")]
    /// <https://www.w3.org/TR/WGSL/#vector-types>
    Vector(VectorType),
    Matrix(MatrixType),
    Struct(StructId),
    BuiltinStruct(BuiltinStruct),
    Array(ArrayType),
    Texture(TextureType),
    Sampler(SamplerType),
    Reference(Reference),
    Pointer(Pointer),
    AccelerationStructure(Option<AccelerationStructureTags>),
}

impl hash::Hash for TypeKind {
    fn hash<Hasher>(
        &self,
        state: &mut Hasher,
    ) where
        Hasher: hash::Hasher,
    {
        core::mem::discriminant(self).hash(state);
    }
}

impl TypeKind {
    pub fn is_convertible_to(
        &self,
        r#type: &Self,
        db: &dyn HirDatabase,
    ) -> bool {
        conversion_rank(self, r#type, db).is_some()
    }

    pub fn unref(
        &self,
        db: &dyn HirDatabase,
    ) -> Cow<'_, Self> {
        match self {
            Self::Reference(reference) => Cow::Owned(reference.inner.kind(db)),
            Self::Error
            | Self::Scalar(_)
            | Self::Atomic(_)
            | Self::Vector(_)
            | Self::Matrix(_)
            | Self::Struct(_)
            | Self::BuiltinStruct(_)
            | Self::Array(_)
            | Self::Texture(_)
            | Self::AccelerationStructure(_)
            | Self::Sampler(_)
            | Self::Pointer(_) => Cow::Borrowed(self),
        }
    }

    /// Abstract types will be mapped to the corresponding default concrete type.
    pub fn concretize(
        &self,
        db: &dyn HirDatabase,
    ) -> Option<Self> {
        Some(match self {
            Self::Scalar(ScalarType::AbstractInt) => Self::Scalar(ScalarType::I32),
            Self::Scalar(ScalarType::AbstractFloat) => Self::Scalar(ScalarType::F32),
            Self::Array(ArrayType {
                inner,
                binding_array,
                size,
            }) => Self::Array(ArrayType {
                inner: inner.kind(db).concretize(db)?.intern(db),
                binding_array: *binding_array,
                size: size.clone(),
            }),
            Self::Vector(VectorType {
                size,
                component_type,
            }) => Self::Vector(VectorType {
                size: *size,
                component_type: component_type.kind(db).concretize(db)?.intern(db),
            }),
            Self::Matrix(MatrixType {
                columns,
                rows,
                inner,
            }) => Self::Matrix(MatrixType {
                columns: *columns,
                rows: *rows,
                inner: inner.kind(db).concretize(db)?.intern(db),
            }),
            Self::Error
            | Self::Scalar(_)
            | Self::Atomic(_)
            | Self::Struct(_)
            | Self::BuiltinStruct(_)
            | Self::Texture(_)
            | Self::Sampler(_)
            | Self::Reference(_)
            | Self::AccelerationStructure(_)
            | Self::Pointer(_) => return None,
        })
    }

    #[must_use]
    pub const fn is_numeric_scalar(&self) -> bool {
        match self {
            Self::Scalar(scalar) => scalar.is_numeric(),
            Self::Error
            | Self::Atomic(_)
            | Self::Vector(_)
            | Self::Matrix(_)
            | Self::Struct(_)
            | Self::BuiltinStruct(_)
            | Self::Array(_)
            | Self::Texture(_)
            | Self::Sampler(_)
            | Self::Reference(_)
            | Self::AccelerationStructure(_)
            | Self::Pointer(_) => false,
        }
    }

    #[must_use]
    pub const fn is_index(&self) -> bool {
        match self {
            Self::Scalar(scalar) => scalar.is_index(),
            Self::Error
            | Self::Atomic(_)
            | Self::BuiltinStruct(_)
            | Self::Vector(_)
            | Self::Matrix(_)
            | Self::Struct(_)
            | Self::Array(_)
            | Self::Texture(_)
            | Self::Sampler(_)
            | Self::Reference(_)
            | Self::AccelerationStructure(_)
            | Self::Pointer(_) => false,
        }
    }

    #[must_use]
    pub fn is_abstract(
        &self,
        db: &dyn HirDatabase,
    ) -> bool {
        match self {
            Self::Scalar(ScalarType::AbstractInt | ScalarType::AbstractFloat) => true,
            Self::Array(ArrayType {
                inner,
                binding_array: _,
                size: _,
            })
            | Self::Vector(VectorType {
                component_type: inner,
                size: _,
            })
            | Self::Matrix(MatrixType {
                inner,
                columns: _,
                rows: _,
            }) => inner.kind(db).is_abstract(db),
            Self::Scalar(_)
            | Self::Error
            | Self::Atomic(_)
            | Self::Struct(_)
            | Self::BuiltinStruct(_)
            | Self::Texture(_)
            | Self::Sampler(_)
            | Self::Reference(_)
            | Self::AccelerationStructure(_)
            | Self::Pointer(_) => false,
        }
    }

    #[must_use]
    pub const fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    #[must_use]
    pub const fn is_plain(&self) -> bool {
        matches!(
            self,
            Self::Scalar(_)
                | Self::Vector(_)
                | Self::Matrix(_)
                | Self::Atomic(_)
                | Self::Array(_)
                | Self::Struct(_)
                | Self::BuiltinStruct(_)
        )
    }

    #[must_use]
    pub const fn is_constructable(&self) -> bool {
        matches!(
            self,
            Self::Scalar(_)
                | Self::Vector(_)
                | Self::Matrix(_)
                | Self::Array(ArrayType {
                    size: ArraySize::Constant(_),
                    inner: _,
                    binding_array: _
                })
                | Self::Struct(_)
        )
    }

    #[must_use]
    pub const fn is_storable(&self) -> bool {
        matches!(
            self,
            Self::Scalar(_)
                | Self::Vector(_)
                | Self::Matrix(_)
                | Self::Atomic(_)
                | Self::Array(_)
                | Self::Struct(_)
                | Self::Texture(_)
                | Self::Sampler(_)
        )
    }

    pub fn is_host_shareable(
        &self,
        db: &dyn HirDatabase,
    ) -> bool {
        match self {
            Self::Scalar(scalar) => scalar.is_numeric(),
            Self::Vector(vec) => vec.component_type.kind(db).is_numeric_scalar(),
            // Error types are treated as optimistically compatible to avoid
            // irrelevant diagnostics (for example, when a struct is not yet defined).
            Self::Matrix(_) | Self::Atomic(_) | Self::Error => true,
            Self::Array(array) => array.inner.kind(db).is_host_shareable(db),
            Self::Struct(r#struct) => db
                .field_types(*r#struct)
                .0
                .iter()
                .all(|(_, r#type)| r#type.kind(db).is_host_shareable(db)),
            Self::BuiltinStruct(_)
            | Self::Texture(_)
            | Self::Sampler(_)
            | Self::Reference(_)
            | Self::AccelerationStructure(_)
            | Self::Pointer(_) => false,
        }
    }

    pub fn contains_runtime_sized_array(
        &self,
        db: &dyn HirDatabase,
    ) -> bool {
        match self {
            Self::Array(ArrayType {
                size: ArraySize::Dynamic,
                inner: _,
                binding_array: _,
            }) => true,
            Self::Struct(r#struct) => db
                .field_types(*r#struct)
                .0
                .iter()
                .any(|(_, r#type)| r#type.kind(db).contains_runtime_sized_array(db)),
            Self::Error
            | Self::Scalar(_)
            | Self::Atomic(_)
            | Self::Vector(_)
            | Self::Matrix(_)
            | Self::Array(_)
            | Self::BuiltinStruct(_)
            | Self::Texture(_)
            | Self::Sampler(_)
            | Self::Reference(_)
            | Self::AccelerationStructure(_)
            | Self::Pointer(_) => false,
        }
    }

    pub fn contains_struct(
        &self,
        db: &dyn HirDatabase,
        r#struct: StructId,
    ) -> bool {
        match self {
            Self::Atomic(atomic) => atomic.inner.contains_struct(db, r#struct),
            Self::Struct(id) => {
                if *id == r#struct {
                    return true;
                }
                db.field_types(*id)
                    .0
                    .values()
                    .any(|r#type| r#type.contains_struct(db, r#struct))
            },
            Self::Array(array) => array.inner.contains_struct(db, r#struct),
            Self::Reference(reference) => reference.inner.contains_struct(db, r#struct),
            Self::Pointer(pointer) => pointer.inner.contains_struct(db, r#struct),
            Self::Error
            | Self::Scalar(_)
            | Self::Vector(_)
            | Self::Matrix(_)
            | Self::BuiltinStruct(_)
            | Self::Texture(_)
            | Self::AccelerationStructure(_)
            | Self::Sampler(_) => false,
        }
    }
}

/// Implements the [conversion rank algorithm](https://www.w3.org/TR/WGSL/#conversion-rank)
/// Taken from wesl-rs.
fn conversion_rank(
    ty1: &TypeKind,
    ty2: &TypeKind,
    db: &dyn HirDatabase,
) -> Option<u32> {
    // reference: <https://www.w3.org/TR/WGSL/#conversion-rank>
    match (ty1, ty2) {
        (_, _) if ty1 == ty2 => Some(0),
        (
            TypeKind::Reference(Reference {
                inner: ty1,
                access_mode: AccessMode::Read | AccessMode::ReadWrite,
                address_space: _,
            }),
            ty2,
        ) if &ty1.kind(db) == ty2 => Some(0),
        (
            TypeKind::Scalar(ScalarType::AbstractInt),
            TypeKind::Scalar(ScalarType::AbstractFloat),
        ) => Some(5),
        (TypeKind::Scalar(ScalarType::AbstractInt), TypeKind::Scalar(ScalarType::I32)) => Some(3),
        (TypeKind::Scalar(ScalarType::AbstractInt), TypeKind::Scalar(ScalarType::U32)) => Some(4),
        (TypeKind::Scalar(ScalarType::AbstractInt), TypeKind::Scalar(ScalarType::F32)) => Some(6),
        (TypeKind::Scalar(ScalarType::AbstractInt), TypeKind::Scalar(ScalarType::F16)) => Some(7),
        (TypeKind::Scalar(ScalarType::AbstractFloat), TypeKind::Scalar(ScalarType::F32)) => Some(1),
        (TypeKind::Scalar(ScalarType::AbstractFloat), TypeKind::Scalar(ScalarType::F16)) => Some(2),
        (TypeKind::Struct(_), TypeKind::Struct(_)) => {
            // TODO: special consideration for frexp and modf for correctness
            // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/680
            // https://github.com/wgsl-tooling-wg/wesl-rs/blob/fea56c869ba2ee8825b7b06e4d9d0d2876b2bc77/crates/wgsl-types/src/conv.rs#L312
            None
        },
        (
            TypeKind::Array(ArrayType {
                inner: ty1,
                size: n1,
                binding_array: _,
            }),
            TypeKind::Array(ArrayType {
                inner: ty2,
                size: n2,
                binding_array: _,
            }),
        ) if n1 == n2 => conversion_rank(&ty1.kind(db), &ty2.kind(db), db),
        (
            TypeKind::Vector(VectorType {
                size: n1,
                component_type: ty1,
            }),
            TypeKind::Vector(VectorType {
                size: n2,
                component_type: ty2,
            }),
        ) if n1 == n2 => conversion_rank(&ty1.kind(db), &ty2.kind(db), db),
        (
            TypeKind::Matrix(MatrixType {
                columns: c1,
                rows: r1,
                inner: ty1,
            }),
            TypeKind::Matrix(MatrixType {
                columns: c2,
                rows: r2,
                inner: ty2,
            }),
        ) if c1 == c2 && r1 == r2 => conversion_rank(&ty1.kind(db), &ty2.kind(db), db),
        // optimistically assume that whatever went wrong, the intention was for it to work
        // prevents extra diagnostics from being emitted
        (TypeKind::Error, _) | (_, TypeKind::Error) => Some(0),
        _ => None,
    }
}

/// The scalar types are [`bool`], [`AbstractInt`], [`AbstractFloat`], [`i32`], [`u32`], [`f32`], and [`f16`].
///
/// <https://www.w3.org/TR/WGSL/#scalar-types>
///
/// [`bool`]: <https://www.w3.org/TR/WGSL/#bool>
/// [`AbstractInt`]: <https://www.w3.org/TR/WGSL/#abstractint>
/// [`AbstractFloat`]: <https://www.w3.org/TR/WGSL/#abstractfloat>
/// [`i32`]: <https://www.w3.org/TR/WGSL/#i32>
/// [`u32`]: <https://www.w3.org/TR/WGSL/#u32>
/// [`f32`]: <https://www.w3.org/TR/WGSL/#f32>
/// [`f16`]: <https://www.w3.org/TR/WGSL/#f16>
#[expect(clippy::doc_paragraphs_missing_punctuation, reason = "false positive")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ScalarType {
    /// <https://www.w3.org/TR/WGSL/#bool>
    Bool,
    /// <https://www.w3.org/TR/WGSL/#abstractint>
    AbstractInt,
    /// <https://www.w3.org/TR/WGSL/#abstractfloat>
    AbstractFloat,
    /// <https://www.w3.org/TR/WGSL/#i32>
    I32,
    /// <https://www.w3.org/TR/WGSL/#u32>
    U32,
    /// <https://www.w3.org/TR/WGSL/#f32>
    F32,
    /// <https://www.w3.org/TR/WGSL/#f16>
    F16,
    // SHADER_INT64
    I64,
    U64,
}

impl ScalarType {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Bool => "bool",
            Self::AbstractInt => "__abstract_int",
            Self::AbstractFloat => "__abstract_float",
            Self::I32 => "i32",
            Self::U32 => "u32",
            Self::F32 => "f32",
            Self::F16 => "f16",
            Self::I64 => "i64",
            Self::U64 => "u64",
        }
    }

    #[must_use]
    #[expect(clippy::doc_paragraphs_missing_punctuation, reason = "false positive")]
    /// The numeric scalar types are [`AbstractInt`], [`AbstractFloat`], [`i32`], [`u32`], [`f32`], and [`f16`].
    ///
    /// Reference: <https://www.w3.org/TR/WGSL/#numeric-scalar>
    ///
    /// [`AbstractInt`]: <https://www.w3.org/TR/WGSL/#abstractint>
    /// [`AbstractFloat`]: <https://www.w3.org/TR/WGSL/#abstractfloat>
    /// [`i32`]: <https://www.w3.org/TR/WGSL/#i32>
    /// [`u32`]: <https://www.w3.org/TR/WGSL/#u32>
    /// [`f32`]: <https://www.w3.org/TR/WGSL/#f32>
    /// [`f16`]: <https://www.w3.org/TR/WGSL/#f16>
    pub const fn is_numeric(self) -> bool {
        matches!(
            self,
            Self::AbstractInt | Self::AbstractFloat | Self::I32 | Self::U32 | Self::F32 | Self::F16
        )
    }

    #[must_use]
    #[expect(clippy::doc_paragraphs_missing_punctuation, reason = "false positive")]
    /// The integer scalar types are [`AbstractInt`], [`i32`], and [`u32`].
    ///
    /// Reference: <https://www.w3.org/TR/WGSL/#integer-scalar>
    ///
    /// [`AbstractInt`]: <https://www.w3.org/TR/WGSL/#abstractint>
    /// [`i32`]: <https://www.w3.org/TR/WGSL/#i32>
    /// [`u32`]: <https://www.w3.org/TR/WGSL/#u32>
    pub const fn is_integer(self) -> bool {
        matches!(self, Self::AbstractInt | Self::I32 | Self::U32)
    }

    #[must_use]
    #[expect(clippy::doc_paragraphs_missing_punctuation, reason = "false positive")]
    /// The collection index types are [`AbstractInt`], [`i32`], and [`u32`].
    ///
    /// Reference: <https://www.w3.org/TR/WGSL/#vector-single-component>
    ///
    /// [`AbstractInt`]: <https://www.w3.org/TR/WGSL/#abstractint>
    /// [`i32`]: <https://www.w3.org/TR/WGSL/#i32>
    /// [`u32`]: <https://www.w3.org/TR/WGSL/#u32>
    pub const fn is_index(self) -> bool {
        matches!(self, Self::AbstractInt | Self::I32 | Self::U32)
    }
}

#[expect(clippy::doc_paragraphs_missing_punctuation, reason = "false positive")]
/// N must be in {2, 3, 4}.
///
/// <https://www.w3.org/TR/WGSL/#vector-types>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VecSize {
    Two,
    Three,
    Four,
}

impl TryFrom<u8> for VecSize {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            _ => return Err(()),
        })
    }
}

impl fmt::Display for VecSize {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Two => formatter.write_str("2"),
            Self::Three => formatter.write_str("3"),
            Self::Four => formatter.write_str("4"),
        }
    }
}

impl VecSize {
    /// Get the dimensionality of the vector (can be `2`, `3`, or `4`) as a [`u8`].
    ///
    /// # Panics
    ///
    /// Panics if self is the [`BoundVariable`] variant.
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        match self {
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
        }
    }
}

#[expect(clippy::doc_paragraphs_missing_punctuation, reason = "false positive")]
/// A vector is a grouped sequence of 2, 3, or 4 [scalar](https://www.w3.org/TR/WGSL/#scalar) components.
///
/// Reference: [6.2.6. Vector Types](https://www.w3.org/TR/WGSL/#vector-types)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VectorType {
    /// N must be in {2, 3, 4}
    pub size: VecSize,
    /// T must be one of the [scalar types](https://www.w3.org/TR/WGSL/#scalar).
    pub component_type: Type,
}

impl VectorType {
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self.size {
            VecSize::Two => "vec2",
            VecSize::Three => "vec3",
            VecSize::Four => "vec4",
        }
    }
}

#[expect(clippy::doc_paragraphs_missing_punctuation, reason = "false positive")]
/// A matrix is a grouped sequence of 2, 3, or 4 floating point vectors.
///
/// Reference: [6.2.7. Matrix Types](https://www.w3.org/TR/WGSL/#matrix-types)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatrixType {
    pub columns: VecSize,
    pub rows: VecSize,
    /// Must be [`f32`], [`f16`], or [`AbstractFloat`]
    ///
    /// [`f32`]: <https://www.w3.org/TR/WGSL/#f32>
    /// [`f16`]: <https://www.w3.org/TR/WGSL/#f16>
    /// [`AbstractFloat`]: <https://www.w3.org/TR/WGSL/#abstractfloat>
    pub inner: Type,
}

impl MatrixType {
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match (self.columns, self.rows) {
            (VecSize::Two, VecSize::Two) => "mat2x2",
            (VecSize::Two, VecSize::Three) => "mat2x3",
            (VecSize::Two, VecSize::Four) => "mat2x4",
            (VecSize::Three, VecSize::Two) => "mat3x2",
            (VecSize::Three, VecSize::Three) => "mat3x3",
            (VecSize::Three, VecSize::Four) => "mat3x4",
            (VecSize::Four, VecSize::Two) => "mat4x2",
            (VecSize::Four, VecSize::Three) => "mat4x3",
            (VecSize::Four, VecSize::Four) => "mat4x4",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AtomicType {
    pub inner: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayType {
    pub inner: Type,
    pub binding_array: bool,
    pub size: ArraySize,
}

impl ArrayType {
    #[expect(clippy::unused_self, reason = "intended API")]
    #[must_use]
    pub const fn name(&self) -> &'static str {
        "array"
    }

    pub fn is_constructible(
        &self,
        db: &dyn HirDatabase,
    ) -> bool {
        self.size != ArraySize::Dynamic && *self.inner.is_constructible(db)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ArraySize {
    Constant(NonZeroU32),
    Dynamic,
}

impl ArraySize {
    pub const MAX: NonZeroU32 = NonZeroU32::MAX;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pointer {
    pub address_space: AddressSpace,
    pub inner: Type,
    pub access_mode: AccessMode,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Reference {
    pub address_space: AddressSpace,
    pub inner: Type,
    pub access_mode: AccessMode,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TextureType {
    pub kind: TextureKind,
    pub dimension: TextureDimensionality,
    pub arrayed: bool,
    pub multisampled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextureKind {
    Sampled(Type),
    Storage(TexelFormat, AccessMode),
    Depth,
    External,
}

impl TextureKind {
    pub fn from_sampled(
        sampled: wgsl_types::syntax::SampledType,
        db: &dyn HirDatabase,
    ) -> Self {
        match sampled {
            wgsl_types::syntax::SampledType::I32 => {
                Self::Sampled(TypeKind::Scalar(ScalarType::I32).intern(db))
            },
            wgsl_types::syntax::SampledType::U32 => {
                Self::Sampled(TypeKind::Scalar(ScalarType::U32).intern(db))
            },
            wgsl_types::syntax::SampledType::F32 => {
                Self::Sampled(TypeKind::Scalar(ScalarType::F32).intern(db))
            },
            wgsl_types::syntax::SampledType::U64 => {
                Self::Sampled(TypeKind::Scalar(ScalarType::U64).intern(db))
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextureDimensionality {
    D1,
    D2,
    D3,
    Cube,
}

impl fmt::Display for TextureDimensionality {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::D1 => formatter.write_str("1d"),
            Self::D2 => formatter.write_str("2d"),
            Self::D3 => formatter.write_str("3d"),
            Self::Cube => formatter.write_str("cube"),
        }
    }
}
