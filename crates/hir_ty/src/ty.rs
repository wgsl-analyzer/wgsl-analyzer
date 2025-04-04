pub mod pretty;

use std::{borrow::Cow, fmt::Write, str::FromStr};

pub use hir_def::type_ref::{AccessMode, StorageClass};
use hir_def::{db::StructId, type_ref};
use salsa::InternKey;

use crate::db::HirDatabase;

// TODO:
// [ ] nesting depth
// [ ] constructable
// [ ] io-shareable
// [ ] host-shareable
// [ ] storable

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Ty {
    r#type: salsa::InternId,
}

impl InternKey for Ty {
    fn from_intern_id(r#type: salsa::InternId) -> Self {
        Ty { r#type }
    }

    fn as_intern_id(&self) -> salsa::InternId {
        self.r#type
    }
}

impl Ty {
    pub fn kind(
        self,
        db: &dyn HirDatabase,
    ) -> TyKind {
        db.lookup_intern_ty(self)
    }

    pub fn is_err(
        self,
        db: &dyn HirDatabase,
    ) -> bool {
        matches!(db.lookup_intern_ty(self), TyKind::Error)
    }

    /// `T` -> `T`, `vecN<T>` -> `T`
    #[must_use]
    pub fn this_or_vec_inner(
        self,
        db: &dyn HirDatabase,
    ) -> Ty {
        match self.kind(db) {
            TyKind::Vector(vec) => vec.inner,
            TyKind::Reference(r) => r.inner.this_or_vec_inner(db),
            _ => self,
        }
    }

    /// `ref<inner>` -> `inner`, `ptr<inner>` -> `ptr<inner>`
    #[must_use]
    pub fn unref(
        self,
        db: &dyn HirDatabase,
    ) -> Ty {
        match self.kind(db) {
            TyKind::Reference(r) => r.inner,
            _ => self,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TyKind {
    Error,
    Scalar(ScalarType),
    Atomic(AtomicType),
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
    pub fn bool() -> TyKind {
        TyKind::Scalar(ScalarType::Bool)
    }

    pub fn f32() -> TyKind {
        TyKind::Scalar(ScalarType::Bool)
    }

    pub fn i32() -> TyKind {
        TyKind::Scalar(ScalarType::Bool)
    }

    pub fn u32() -> TyKind {
        TyKind::Scalar(ScalarType::Bool)
    }

    pub fn vec_of() -> TyKind {
        TyKind::Scalar(ScalarType::Bool)
    }
}

impl TyKind {
    pub fn unref(
        &self,
        db: &dyn HirDatabase,
    ) -> Cow<'_, TyKind> {
        match self {
            TyKind::Reference(r) => Cow::Owned(r.inner.kind(db)),
            _ => Cow::Borrowed(self),
        }
    }

    pub fn is_numeric_scalar(&self) -> bool {
        match self {
            TyKind::Scalar(scalar) => scalar.is_numeric(),
            _ => false,
        }
    }

    pub fn intern(
        self,
        db: &dyn HirDatabase,
    ) -> Ty {
        db.intern_ty(self)
    }

    pub fn is_error(&self) -> bool {
        matches!(self, TyKind::Error)
    }

    pub fn is_plain(&self) -> bool {
        matches!(
            self,
            TyKind::Scalar(_)
                | TyKind::Vector(_)
                | TyKind::Matrix(_)
                | TyKind::Atomic(_)
                | TyKind::Array(_)
                | TyKind::Struct(_)
        )
    }

    pub fn is_constructable(&self) -> bool {
        matches!(
            self,
            TyKind::Scalar(_)
                | TyKind::Vector(_)
                | TyKind::Matrix(_)
                | TyKind::Array(ArrayType {
                    size: ArraySize::Constant(_),
                    ..
                })
                | TyKind::Struct(_)
        )
    }

    pub fn is_storable(&self) -> bool {
        matches!(
            self,
            TyKind::Scalar(_)
                | TyKind::Vector(_)
                | TyKind::Matrix(_)
                | TyKind::Atomic(_)
                | TyKind::Array(_)
                | TyKind::Struct(_)
                | TyKind::Texture(_)
                | TyKind::Sampler(_)
        )
    }

    pub fn is_io_shareable(
        &self,
        db: &dyn HirDatabase,
    ) -> bool {
        match self {
            TyKind::Scalar(_) => true,
            TyKind::Vector(vec) => vec.inner.kind(db).is_numeric_scalar(),
            TyKind::Struct(r#struct) => {
                db.field_types(*r#struct)
                    .iter()
                    .all(|(_, r#type)| match r#type.kind(db) {
                        TyKind::Scalar(_) => true,
                        TyKind::Vector(vec) if vec.inner.kind(db).is_numeric_scalar() => true,
                        _ => false,
                    })
            },
            _ => false,
        }
    }

    pub fn is_host_shareable(
        &self,
        db: &dyn HirDatabase,
    ) -> bool {
        match self {
            TyKind::Scalar(scalar) => scalar.is_numeric(),
            TyKind::Vector(vec) => vec.inner.kind(db).is_numeric_scalar(),
            TyKind::Matrix(_) | TyKind::Atomic(_) => true,
            TyKind::Array(array) => array.inner.kind(db).is_host_shareable(db),
            TyKind::Struct(r#struct) => db
                .field_types(*r#struct)
                .iter()
                .all(|(_, r#type)| r#type.kind(db).is_host_shareable(db)),
            _ => false,
        }
    }

    pub fn contains_runtime_sized_array(
        &self,
        db: &dyn HirDatabase,
    ) -> bool {
        match self {
            TyKind::Array(ArrayType {
                size: ArraySize::Dynamic,
                ..
            }) => true,
            TyKind::Struct(r#struct) => db
                .field_types(*r#struct)
                .iter()
                .any(|(_, r#type)| r#type.kind(db).contains_runtime_sized_array(db)),
            _ => false,
        }
    }

    pub fn contains_struct(
        &self,
        db: &dyn HirDatabase,
        r#struct: StructId,
    ) -> bool {
        match self {
            TyKind::Atomic(atomic) => atomic.inner.contains_struct(db, r#struct),
            TyKind::Struct(id) => {
                if *id == r#struct {
                    return true;
                }
                db.field_types(*id)
                    .values()
                    .any(|r#type| r#type.contains_struct(db, r#struct))
            },
            TyKind::Array(array) => array.inner.contains_struct(db, r#struct),
            TyKind::Reference(r) => r.inner.contains_struct(db, r#struct),
            TyKind::Pointer(pointer) => pointer.inner.contains_struct(db, r#struct),
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScalarType {
    Bool,
    I32,
    U32,
    F32,
}

impl ScalarType {
    pub fn is_numeric(&self) -> bool {
        matches!(self, ScalarType::F32 | ScalarType::U32 | ScalarType::I32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VecSize {
    Two,
    Three,
    Four,
    BoundVar(BoundVar),
}

impl TryFrom<u8> for VecSize {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            2 => VecSize::Two,
            3 => VecSize::Three,
            4 => VecSize::Four,
            _ => return Err(()),
        })
    }
}

impl From<type_ref::VecDimensionality> for VecSize {
    fn from(dim: type_ref::VecDimensionality) -> Self {
        match dim {
            type_ref::VecDimensionality::Two => VecSize::Two,
            type_ref::VecDimensionality::Three => VecSize::Three,
            type_ref::VecDimensionality::Four => VecSize::Four,
        }
    }
}

impl std::fmt::Display for VecSize {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            VecSize::Two => f.write_str("2"),
            VecSize::Three => f.write_str("3"),
            VecSize::Four => f.write_str("4"),
            VecSize::BoundVar(var) => {
                let mut names = "NMOPQRS".chars();
                write!(f, "{}", names.nth(var.index).unwrap())
            },
        }
    }
}

impl VecSize {
    pub fn as_u8(&self) -> u8 {
        match self {
            VecSize::Two => 2,
            VecSize::Three => 3,
            VecSize::Four => 4,
            VecSize::BoundVar(_) => panic!("VecSize::BoundVar cannot be made into an u8"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VectorType {
    pub size: VecSize,
    pub inner: Ty,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatrixType {
    pub columns: VecSize,
    pub rows: VecSize,
    pub inner: Ty,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AtomicType {
    pub inner: Ty,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayType {
    pub inner: Ty,
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
    pub storage_class: StorageClass,
    pub inner: Ty,
    pub access_mode: AccessMode,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Reference {
    pub storage_class: StorageClass,
    pub inner: Ty,
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
    Sampled(Ty),
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

impl std::fmt::Display for TextureDimensionality {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            TextureDimensionality::D1 => f.write_str("1d"),
            TextureDimensionality::D2 => f.write_str("2d"),
            TextureDimensionality::D3 => f.write_str("3d"),
            TextureDimensionality::Cube => f.write_str("cube"),
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

impl std::fmt::Display for TexelFormat {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let str = match self {
            TexelFormat::Rgba8unorm => "rgba8unorm",
            TexelFormat::Rgba8snorm => "rgba8snorm",
            TexelFormat::Rgba8uint => "rgba8uint",
            TexelFormat::Rgba8sint => "rgba8sint",
            TexelFormat::Rgba16uint => "rgba16uint",
            TexelFormat::Rgba16sint => "rgba16sint",
            TexelFormat::Rgba16float => "rgba16float",
            TexelFormat::Rgba32uint => "rgba32uint",
            TexelFormat::Rgba32sint => "rgba32sint",
            TexelFormat::Rgba32float => "rgba32float",
            TexelFormat::R32uint => "r32uint",
            TexelFormat::R32sint => "r32sint",
            TexelFormat::R32float => "r32float",
            TexelFormat::Rg32uint => "rg32uint",
            TexelFormat::Rg32sint => "rg32sint",
            TexelFormat::Rg32float => "rg32float",
            TexelFormat::BoundVar(var) => return f.write_char(('F'..).nth(var.index).unwrap()),
            TexelFormat::Any => "_",
        };
        f.write_str(str)
    }
}

impl FromStr for TexelFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "rgba8unorm" => TexelFormat::Rgba8unorm,
            "rgba8snorm" => TexelFormat::Rgba8snorm,
            "rgba8uint" => TexelFormat::Rgba8uint,
            "rgba8sint" => TexelFormat::Rgba8sint,
            "rgba16uint" => TexelFormat::Rgba16uint,
            "rgba16sint" => TexelFormat::Rgba16sint,
            "rgba16float" => TexelFormat::Rgba16float,
            "rgba32uint" => TexelFormat::Rgba32uint,
            "rgba32sint" => TexelFormat::Rgba32sint,
            "rgba32float" => TexelFormat::Rgba32float,
            "r32uint" => TexelFormat::R32uint,
            "r32sint" => TexelFormat::R32sint,
            "r32float" => TexelFormat::R32float,
            "rg32uint" => TexelFormat::Rg32uint,
            "rg32sint" => TexelFormat::Rg32sint,
            "rg32float" => TexelFormat::Rg32float,
            _ => return Err(()),
        })
    }
}
