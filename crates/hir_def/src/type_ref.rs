use std::fmt;

use syntax::{HasGenerics, ast};

use crate::{expression::parse_literal, module_data::Name};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TypeReference {
    Error,
    Scalar(ScalarType),
    Vec(VecType),
    Matrix(MatrixType),
    Texture(TextureType),
    Sampler(SamplerType),
    Atomic(AtomicType),
    Array(ArrayType),
    Path(Name),
    Pointer(PointerType),
}

impl fmt::Display for TypeReference {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Error => write!(formatter, "[error]"),
            Self::Scalar(value) => write!(formatter, "{value}"),
            Self::Vec(value) => write!(formatter, "{value}"),
            Self::Matrix(value) => write!(formatter, "{value}"),
            Self::Texture(value) => write!(formatter, "{value}"),
            Self::Sampler(value) => write!(formatter, "{value}"),
            Self::Atomic(value) => write!(formatter, "{value}"),
            Self::Array(value) => write!(formatter, "{value}"),
            Self::Path(value) => write!(formatter, "{}", value.as_str()),
            Self::Pointer(value) => write!(formatter, "{value}"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ScalarType {
    Bool,
    Float32,
    Int32,
    Uint32,
}

impl fmt::Display for ScalarType {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Bool => formatter.write_str("bool"),
            Self::Float32 => formatter.write_str("f32"),
            Self::Int32 => formatter.write_str("i32"),
            Self::Uint32 => formatter.write_str("u32"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct VecType {
    pub size: VecDimensionality,
    pub inner: Box<TypeReference>,
}

impl fmt::Display for VecType {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(formatter, "vec{}<{}>", self.size, &*self.inner)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum VecDimensionality {
    Two,
    Three,
    Four,
}

impl fmt::Display for VecDimensionality {
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MatrixType {
    pub columns: VecDimensionality,
    pub rows: VecDimensionality,
    pub inner: Box<TypeReference>,
}

impl fmt::Display for MatrixType {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "mat{}x{}<{}>",
            self.columns, self.rows, &*self.inner
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TextureType {
    pub dimension: TextureDimension,
    pub arrayed: bool,
    pub multisampled: bool,
    pub kind: TextureKind,
}

impl fmt::Display for TextureType {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match &self.kind {
            TextureKind::Sampled(r#type) => write!(
                formatter,
                "texture_{}{}{}<{type}>",
                if self.multisampled {
                    "multisampled_"
                } else {
                    ""
                },
                self.dimension,
                if self.arrayed { "_array" } else { "" }
            ),
            TextureKind::Storage(format, mode) => write!(
                formatter,
                "texture_storage_{}{}{}<{format}, {mode}>",
                self.dimension,
                if self.multisampled {
                    "_multisampled"
                } else {
                    ""
                },
                if self.arrayed { "_array" } else { "" },
            ),
            TextureKind::Depth => write!(
                formatter,
                "texture_depth_{}{}{}",
                if self.multisampled {
                    "multisampled_"
                } else {
                    ""
                },
                self.dimension,
                if self.arrayed { "_array" } else { "" },
            ),
            TextureKind::External => write!(formatter, "texture_external"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TextureKind {
    Sampled(Box<TypeReference>),
    Storage(String, AccessMode),
    Depth,
    External,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TextureDimension {
    D1,
    D2,
    D3,
    Cube,
}

impl fmt::Display for TextureDimension {
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AccessMode {
    ReadWrite,
    Read,
    Write,

    // this is only used for builtins which do not care about the access mode (e.g. textureDimensions)
    Any,
}

impl fmt::Display for AccessMode {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::ReadWrite => formatter.write_str("read_write"),
            Self::Read => formatter.write_str("read"),
            Self::Write => formatter.write_str("write"),
            Self::Any => formatter.write_str("_"),
        }
    }
}

impl AccessMode {
    #[must_use]
    pub const fn read_write() -> Self {
        Self::ReadWrite
    }
}

/// <https://www.w3.org/TR/WGSL/#address-space>
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AddressSpace {
    /// <https://www.w3.org/TR/WGSL/#address-spaces-function>
    Function,
    /// <https://www.w3.org/TR/WGSL/#address-spaces-private>
    Private,
    /// <https://www.w3.org/TR/WGSL/#address-spaces-workgroup>
    Workgroup,
    /// <https://www.w3.org/TR/WGSL/#address-spaces-uniform>
    Uniform,
    /// <https://www.w3.org/TR/WGSL/#address-spaces-storage>
    Storage,
    /// <https://www.w3.org/TR/WGSL/#address-spaces-handle>
    Handle,
    /// WGPU extension
    /// See: <https://docs.rs/wgpu/latest/wgpu/struct.Features.html#associatedconstant.PUSH_CONSTANTS>
    PushConstant,
}

impl fmt::Display for AddressSpace {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(match self {
            Self::Function => "function",
            Self::Private => "private",
            Self::Workgroup => "workgroup",
            Self::Uniform => "uniform",
            Self::Storage => "storage",
            Self::Handle => "handle",
            Self::PushConstant => "push_constant",
        })
    }
}

impl AddressSpace {
    #[must_use]
    /// Sourced from table at <https://www.w3.org/TR/WGSL/#address-space>
    pub const fn default_access_mode(self) -> AccessMode {
        match self {
            Self::Workgroup | Self::Private | Self::Function => AccessMode::ReadWrite,
            Self::Uniform | Self::Storage | Self::Handle | Self::PushConstant => AccessMode::Read,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SamplerType {
    pub comparison: bool,
}

impl fmt::Display for SamplerType {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        if self.comparison {
            formatter.write_str("sampler_comparison")
        } else {
            formatter.write_str("sampler")
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct AtomicType {
    pub inner: Box<TypeReference>,
}

impl fmt::Display for AtomicType {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(formatter, "atomic<{}>", self.inner)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ArrayType {
    pub inner: Box<TypeReference>,
    pub binding_array: bool,
    pub size: ArraySize,
}

impl fmt::Display for ArrayType {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let prefix = if self.binding_array { "binding_" } else { "" };
        match &self.size {
            ArraySize::Int(size) => write!(formatter, "{prefix}array<{}, {size}>", self.inner),
            ArraySize::Uint(size) => write!(formatter, "{prefix}array<{}, {size}>", self.inner),
            ArraySize::Path(size) => {
                write!(
                    formatter,
                    "{prefix}array<{}, {}>",
                    self.inner,
                    size.as_str()
                )
            },
            ArraySize::Dynamic => write!(formatter, "{prefix}array<{}>", self.inner),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ArraySize {
    Int(i64),
    Uint(u64),
    Path(Name),
    Dynamic,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct PointerType {
    pub address_space: AddressSpace,
    pub access_mode: AccessMode,
    pub inner: Box<TypeReference>,
}

impl fmt::Display for PointerType {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(formatter, "ptr<{}, {}>", self.address_space, self.inner)
    }
}
