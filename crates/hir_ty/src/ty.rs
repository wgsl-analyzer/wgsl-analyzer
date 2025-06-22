pub mod pretty;

use std::{
    borrow::Cow,
    fmt::{self, Write as _},
    str::FromStr,
};

pub use hir_def::type_ref::{AccessMode, AddressSpace};
use hir_def::{database::StructId, type_ref::VecDimensionality};
use salsa::InternKey;

use crate::database::HirDatabase;

// TODO:
// [ ] nesting depth
// [ ] constructable
// [ ] io-shareable
// [ ] host-shareable
// [ ] storable

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Type {
    r#type: salsa::InternId,
}

impl InternKey for Type {
    fn from_intern_id(
        #[expect(clippy::min_ident_chars, reason = "trait impl")] v: salsa::InternId
    ) -> Self {
        Self { r#type: v }
    }

    fn as_intern_id(&self) -> salsa::InternId {
        self.r#type
    }
}

impl Type {
    pub fn kind(
        self,
        database: &dyn HirDatabase,
    ) -> TyKind {
        database.lookup_intern_ty(self)
    }

    pub fn is_err(
        self,
        database: &dyn HirDatabase,
    ) -> bool {
        matches!(database.lookup_intern_ty(self), TyKind::Error)
    }

    /// `T` -> `T`, `vecN<T>` -> `T`
    #[must_use]
    pub fn this_or_vec_inner(
        self,
        database: &dyn HirDatabase,
    ) -> Self {
        match self.kind(database) {
            TyKind::Vector(vec) => vec.component_type,
            TyKind::Reference(r) => r.inner.this_or_vec_inner(database),
            _ => self,
        }
    }

    /// `ref<inner>` -> `inner`, `ptr<inner>` -> `ptr<inner>`
    #[must_use]
    pub fn unref(
        self,
        database: &dyn HirDatabase,
    ) -> Self {
        match self.kind(database) {
            TyKind::Reference(r) => r.inner,
            _ => self,
        }
    }

    pub fn contains_struct(
        self,
        database: &dyn HirDatabase,
        r#struct: StructId,
    ) -> bool {
        self.kind(database).contains_struct(database, r#struct)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TyKind {
    Error,
    Scalar(ScalarType),
    Atomic(AtomicType),
    /// <https://www.w3.org/TR/WGSL/#vector-types>
    Vector(VectorType),
    Matrix(MatrixType),
    Struct(StructId),
    Array(ArrayType),
    Texture(TextureType),
    Sampler(SamplerType),
    Reference(Reference),
    Pointer(Pointer),
    BoundVar(BoundVar),
    StorageTypeOfTexelFormat(BoundVar), // e.g. rgba8unorm -> vec4<f32>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoundVar {
    pub index: usize,
}

impl TyKind {
    #[must_use]
    pub const fn bool() -> Self {
        Self::Scalar(ScalarType::Bool)
    }

    #[must_use]
    pub const fn f32() -> Self {
        Self::Scalar(ScalarType::Bool)
    }

    #[must_use]
    pub const fn i32() -> Self {
        Self::Scalar(ScalarType::Bool)
    }

    #[must_use]
    pub const fn u32() -> Self {
        Self::Scalar(ScalarType::Bool)
    }

    #[must_use]
    pub const fn vec_of() -> Self {
        Self::Scalar(ScalarType::Bool)
    }
}

impl TyKind {
    pub fn unref(
        &self,
        database: &dyn HirDatabase,
    ) -> Cow<'_, Self> {
        match self {
            Self::Reference(r) => Cow::Owned(r.inner.kind(database)),
            _ => Cow::Borrowed(self),
        }
    }

    #[must_use]
    pub const fn is_numeric_scalar(&self) -> bool {
        match self {
            Self::Scalar(scalar) => scalar.is_numeric(),
            _ => false,
        }
    }

    pub fn intern(
        self,
        database: &dyn HirDatabase,
    ) -> Type {
        database.intern_ty(self)
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
                    ..
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

    pub fn is_io_shareable(
        &self,
        database: &dyn HirDatabase,
    ) -> bool {
        match self {
            Self::Scalar(_) => true,
            Self::Vector(vec) => vec.component_type.kind(database).is_numeric_scalar(),
            Self::Struct(r#struct) => database.field_types(*r#struct).iter().all(|(_, r#type)| {
                match r#type.kind(database) {
                    Self::Scalar(_) => true,
                    Self::Vector(vec) if vec.component_type.kind(database).is_numeric_scalar() => {
                        true
                    },
                    _ => false,
                }
            }),
            _ => false,
        }
    }

    pub fn is_host_shareable(
        &self,
        database: &dyn HirDatabase,
    ) -> bool {
        match self {
            Self::Scalar(scalar) => scalar.is_numeric(),
            Self::Vector(vec) => vec.component_type.kind(database).is_numeric_scalar(),
            Self::Matrix(_) | Self::Atomic(_) => true,
            Self::Array(array) => array.inner.kind(database).is_host_shareable(database),
            Self::Struct(r#struct) => database
                .field_types(*r#struct)
                .iter()
                .all(|(_, r#type)| r#type.kind(database).is_host_shareable(database)),
            _ => false,
        }
    }

    pub fn contains_runtime_sized_array(
        &self,
        database: &dyn HirDatabase,
    ) -> bool {
        match self {
            Self::Array(ArrayType {
                size: ArraySize::Dynamic,
                ..
            }) => true,
            Self::Struct(r#struct) => database
                .field_types(*r#struct)
                .iter()
                .any(|(_, r#type)| r#type.kind(database).contains_runtime_sized_array(database)),
            _ => false,
        }
    }

    pub fn contains_struct(
        &self,
        database: &dyn HirDatabase,
        r#struct: StructId,
    ) -> bool {
        match self {
            Self::Atomic(atomic) => atomic.inner.contains_struct(database, r#struct),
            Self::Struct(id) => {
                if *id == r#struct {
                    return true;
                }
                database
                    .field_types(*id)
                    .values()
                    .any(|r#type| r#type.contains_struct(database, r#struct))
            },
            Self::Array(array) => array.inner.contains_struct(database, r#struct),
            Self::Reference(reference) => reference.inner.contains_struct(database, r#struct),
            Self::Pointer(pointer) => pointer.inner.contains_struct(database, r#struct),
            _ => false,
        }
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
}

impl ScalarType {
    #[must_use]
    /// The numeric scalar types are [`AbstractInt`], [`AbstractFloat`], [`i32`], [`u32`], [`f32`], and [`f16`].
    ///
    /// <https://www.w3.org/TR/WGSL/#numeric-scalar>
    ///
    /// [`AbstractInt`]: <https://www.w3.org/TR/WGSL/#abstractint>
    /// [`AbstractFloat`]: <https://www.w3.org/TR/WGSL/#abstractfloat>
    /// [`i32`]: <https://www.w3.org/TR/WGSL/#i32>
    /// [`u32`]: <https://www.w3.org/TR/WGSL/#u32>
    /// [`f32`]: <https://www.w3.org/TR/WGSL/#f32>
    /// [`f16`]: <https://www.w3.org/TR/WGSL/#f16>
    pub const fn is_numeric(&self) -> bool {
        matches!(
            self,
            Self::AbstractInt | Self::AbstractFloat | Self::I32 | Self::U32 | Self::F32 | Self::F16
        )
    }

    #[must_use]
    /// The integer scalar types are [`AbstractInt`], [`i32`], and [`u32`].
    ///
    /// <https://www.w3.org/TR/WGSL/#integer-scalar>
    ///
    /// [`AbstractInt`]: <https://www.w3.org/TR/WGSL/#abstractint>
    /// [`i32`]: <https://www.w3.org/TR/WGSL/#i32>
    /// [`u32`]: <https://www.w3.org/TR/WGSL/#u32>
    pub const fn is_integer(&self) -> bool {
        matches!(self, Self::AbstractInt | Self::I32 | Self::U32)
    }
}

/// N must be in {2, 3, 4}
///
/// https://www.w3.org/TR/WGSL/#vector-types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VecSize {
    Two,
    Three,
    Four,
    // TODO: Is this spec?
    BoundVar(BoundVar),
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

impl From<VecDimensionality> for VecSize {
    fn from(dimensionality: VecDimensionality) -> Self {
        match dimensionality {
            VecDimensionality::Two => Self::Two,
            VecDimensionality::Three => Self::Three,
            VecDimensionality::Four => Self::Four,
        }
    }
}

impl fmt::Display for VecSize {
    fn fmt(
        &self,
        #[expect(clippy::min_ident_chars, reason = "trait impl")] f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Two => f.write_str("2"),
            Self::Three => f.write_str("3"),
            Self::Four => f.write_str("4"),
            Self::BoundVar(var) => {
                let mut names = "NMOPQRS".chars();
                write!(f, "{}", names.nth(var.index).unwrap())
            },
        }
    }
}

impl VecSize {
    /// Get the dimensionality of the vector (can be `2`, `3`, or `4`) as a [`u8`].
    ///
    /// # Panics
    ///
    /// Panics if self is the [`BoundVar`] variant.
    #[must_use]
    pub fn as_u8(&self) -> u8 {
        match self {
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::BoundVar(_) => panic!("VecSize::BoundVar cannot be made into an u8"),
        }
    }
}

/// [6.2.6. Vector Types](https://www.w3.org/TR/WGSL/#vector-types)
///
/// A vector is a grouped sequence of 2, 3, or 4 [scalar](https://www.w3.org/TR/WGSL/#scalar) components.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VectorType {
    /// N must be in {2, 3, 4}
    pub size: VecSize,
    /// T must be one of the [scalar types](https://www.w3.org/TR/WGSL/#scalar).
    pub component_type: Type,
}

impl VectorType {
    // fn is_numeric(&self) -> bool {
    //     self.component_type.is_numeric()
    // }
}

/// [6.2.7. Matrix Types](https://www.w3.org/TR/WGSL/#matrix-types)
///
/// A matrix is a grouped sequence of 2, 3, or 4 floating point vectors.
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ArraySize {
    Constant(u64),
    Dynamic,
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
        #[expect(clippy::min_ident_chars, reason = "trait impl")] f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::D1 => f.write_str("1d"),
            Self::D2 => f.write_str("2d"),
            Self::D3 => f.write_str("3d"),
            Self::Cube => f.write_str("cube"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SamplerType {
    pub comparison: bool,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum TexelFormat {
    Rgba8unorm,
    Rgba8snorm,
    Rgba8uint,
    Rgba8sint,
    Rgba16uint,
    Rgba16sint,
    Rgba16float,
    Rgba32uint,
    Rgba32sint,
    Rgba32float,

    R32uint,
    R32sint,
    R32float,
    Rg32uint,
    Rg32sint,
    Rg32float,

    BoundVar(BoundVar),
    // this is only used for builtins which do not care about the format
    Any,
}

impl fmt::Display for TexelFormat {
    fn fmt(
        &self,
        #[expect(clippy::min_ident_chars, reason = "trait impl")] f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let str = match self {
            Self::Rgba8unorm => "rgba8unorm",
            Self::Rgba8snorm => "rgba8snorm",
            Self::Rgba8uint => "rgba8uint",
            Self::Rgba8sint => "rgba8sint",
            Self::Rgba16uint => "rgba16uint",
            Self::Rgba16sint => "rgba16sint",
            Self::Rgba16float => "rgba16float",
            Self::Rgba32uint => "rgba32uint",
            Self::Rgba32sint => "rgba32sint",
            Self::Rgba32float => "rgba32float",
            Self::R32uint => "r32uint",
            Self::R32sint => "r32sint",
            Self::R32float => "r32float",
            Self::Rg32uint => "rg32uint",
            Self::Rg32sint => "rg32sint",
            Self::Rg32float => "rg32float",
            Self::BoundVar(var) => return f.write_char(('F'..).nth(var.index).unwrap()),
            Self::Any => "_",
        };
        f.write_str(str)
    }
}

impl FromStr for TexelFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "rgba8unorm" => Self::Rgba8unorm,
            "rgba8snorm" => Self::Rgba8snorm,
            "rgba8uint" => Self::Rgba8uint,
            "rgba8sint" => Self::Rgba8sint,
            "rgba16uint" => Self::Rgba16uint,
            "rgba16sint" => Self::Rgba16sint,
            "rgba16float" => Self::Rgba16float,
            "rgba32uint" => Self::Rgba32uint,
            "rgba32sint" => Self::Rgba32sint,
            "rgba32float" => Self::Rgba32float,
            "r32uint" => Self::R32uint,
            "r32sint" => Self::R32sint,
            "r32float" => Self::R32float,
            "rg32uint" => Self::Rg32uint,
            "rg32sint" => Self::Rg32sint,
            "rg32float" => Self::Rg32float,
            _ => return Err(()),
        })
    }
}
