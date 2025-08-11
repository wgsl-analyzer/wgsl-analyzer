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

impl TryFrom<ast::TypeSpecifier> for TypeReference {
    type Error = ();

    fn try_from(r#type: ast::TypeSpecifier) -> Result<Self, ()> {
        let type_ref = match r#type {
            ast::Type::PathType(path) => Self::Path(path.name().ok_or(())?.text().into()),
            ast::Type::ScalarType(scalar) => Self::Scalar(scalar.into()),
            ast::Type::VecType(vec) => Self::Vec(vec.try_into()?),
            ast::Type::MatrixType(matrix) => Self::Matrix(matrix.try_into()?),
            ast::Type::TextureType(tex) => Self::Texture(tex.try_into()?),
            ast::Type::SamplerType(sampler) => Self::Sampler(sampler.into()),
            ast::Type::AtomicType(atomic) => Self::Atomic(atomic.try_into()?),
            ast::Type::ArrayType(array) => Self::Array(array.try_into()?),
            ast::Type::BindingArrayType(array) => Self::Array(array.try_into()?),
            ast::Type::PointerType(pointer) => Self::Pointer(pointer.try_into()?),
        };
        Ok(type_ref)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ScalarType {
    Bool,
    Float32,
    Int32,
    Uint32,
}

impl From<ast::ScalarType> for ScalarType {
    fn from(r#type: ast::ScalarType) -> Self {
        match r#type {
            ast::ScalarType::Bool(_) => Self::Bool,
            ast::ScalarType::Float32(_) => Self::Float32,
            ast::ScalarType::Int32(_) => Self::Int32,
            ast::ScalarType::Uint32(_) => Self::Uint32,
        }
    }
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

impl TryFrom<ast::VecType> for VecType {
    type Error = ();

    fn try_from(r#type: ast::VecType) -> Result<Self, ()> {
        let size = vector_dimensions(&r#type);
        let inner = first_type_generic(&r#type)?;

        Ok(Self {
            size,
            inner: Box::new(inner.try_into()?),
        })
    }
}

pub(crate) const fn vector_dimensions(r#type: &ast::VecType) -> VecDimensionality {
    match *r#type {
        ast::VecType::Vec2(_) => VecDimensionality::Two,
        ast::VecType::Vec3(_) => VecDimensionality::Three,
        ast::VecType::Vec4(_) => VecDimensionality::Four,
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MatrixType {
    pub columns: VecDimensionality,
    pub rows: VecDimensionality,
    pub inner: Box<TypeReference>,
}

impl TryFrom<ast::MatrixType> for MatrixType {
    type Error = ();

    fn try_from(r#type: ast::MatrixType) -> Result<Self, ()> {
        let (columns, rows) = matrix_dimensions(&r#type);
        let inner = first_type_generic(&r#type)?;

        Ok(Self {
            columns,
            rows,
            inner: Box::new(inner.try_into()?),
        })
    }
}

pub(crate) const fn matrix_dimensions(
    r#type: &ast::MatrixType
) -> (VecDimensionality, VecDimensionality) {
    let (columns, rows) = match *r#type {
        ast::MatrixType::Mat2x2(_) => (VecDimensionality::Two, VecDimensionality::Two),
        ast::MatrixType::Mat2x3(_) => (VecDimensionality::Two, VecDimensionality::Three),
        ast::MatrixType::Mat2x4(_) => (VecDimensionality::Two, VecDimensionality::Four),
        ast::MatrixType::Mat3x2(_) => (VecDimensionality::Three, VecDimensionality::Two),
        ast::MatrixType::Mat3x3(_) => (VecDimensionality::Three, VecDimensionality::Three),
        ast::MatrixType::Mat3x4(_) => (VecDimensionality::Three, VecDimensionality::Four),
        ast::MatrixType::Mat4x2(_) => (VecDimensionality::Four, VecDimensionality::Two),
        ast::MatrixType::Mat4x3(_) => (VecDimensionality::Four, VecDimensionality::Three),
        ast::MatrixType::Mat4x4(_) => (VecDimensionality::Four, VecDimensionality::Four),
    };
    (columns, rows)
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

fn first_type_generic<T: HasGenerics>(r#type: &T) -> Result<ast::Type, ()> {
    let mut generics = r#type.generic_arg_list().ok_or(())?.generics();
    let first_generic = generics.next().ok_or(())?;
    let generic = first_generic.as_type().ok_or(())?;
    Ok(generic)
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

impl TryFrom<ast::TextureType> for TextureType {
    type Error = ();

    fn try_from(texture: ast::TextureType) -> Result<Self, Self::Error> {
        enum TextureKindVariant {
            Sampled,
            Storage,
            Depth,
            External,
        }
        #[rustfmt::skip]
        let (kind, dimension, arrayed, multisampled) = match texture {
            ast::TextureType::Texture1d(_) => (TextureKindVariant::Sampled, TextureDimension::D1, false, false),
            ast::TextureType::Texture2d(_) => (TextureKindVariant::Sampled, TextureDimension::D2, false, false),
            ast::TextureType::Texture2dArray(_) => (TextureKindVariant::Sampled, TextureDimension::D2, true, false),
            ast::TextureType::Texture3d(_) => (TextureKindVariant::Sampled, TextureDimension::D3, false, false),
            ast::TextureType::TextureCube(_) => (TextureKindVariant::Sampled, TextureDimension::Cube, false, false),
            ast::TextureType::TextureCubeArray(_) => (TextureKindVariant::Sampled, TextureDimension::Cube, true, false),

            ast::TextureType::TextureMultisampled2d(_) => (TextureKindVariant::Sampled, TextureDimension::D2, false, true),
            ast::TextureType::TextureExternal(_) => (TextureKindVariant::External, TextureDimension::D1, false, false),

            ast::TextureType::TextureStorage1d(_) => (TextureKindVariant::Storage, TextureDimension::D1, false, false),
            ast::TextureType::TextureStorage2d(_) => (TextureKindVariant::Storage, TextureDimension::D2, false, false),
            ast::TextureType::TextureStorage2dArray(_) => (TextureKindVariant::Storage, TextureDimension::D2, true, false),
            ast::TextureType::TextureStorage3d(_) => (TextureKindVariant::Storage, TextureDimension::D3, false, false),

            ast::TextureType::TextureDepth2d(_) => (TextureKindVariant::Depth, TextureDimension::D2, false, false),
            ast::TextureType::TextureDepth2dArray(_) => (TextureKindVariant::Depth, TextureDimension::D2, true, false),
            ast::TextureType::TextureDepthCube(_) => (TextureKindVariant::Depth, TextureDimension::Cube, false, false),
            ast::TextureType::TextureDepthCubeArray(_) => (TextureKindVariant::Depth, TextureDimension::Cube, true, false),
            ast::TextureType::TextureDepthMultisampled2d(_) => (TextureKindVariant::Depth, TextureDimension::D2, false, true),
        };

        let kind = match kind {
            TextureKindVariant::Sampled => {
                let inner = first_type_generic(&texture)?;
                TextureKind::Sampled(Box::new(inner.try_into()?))
            },
            TextureKindVariant::Storage => {
                let mut generics = texture.generic_arg_list().ok_or(())?.generics();

                let texel_format = generics.next().ok_or(())?;
                let name = texel_format.as_type().ok_or(())?.as_name().ok_or(())?;
                let texel_format = name.text().to_string();

                let access_mode = generics
                    .next()
                    .ok_or(())?
                    .as_access_mode()
                    .ok_or(())?
                    .into();

                TextureKind::Storage(texel_format, access_mode)
            },
            TextureKindVariant::Depth => TextureKind::Depth,
            TextureKindVariant::External => TextureKind::External,
        };

        Ok(Self {
            dimension,
            arrayed,
            multisampled,
            kind,
        })
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

impl From<ast::AccessMode> for AccessMode {
    fn from(value: ast::AccessMode) -> Self {
        match value {
            ast::AccessMode::Read(_) => Self::Read,
            ast::AccessMode::Write(_) => Self::Write,
            ast::AccessMode::ReadWrite(_) => Self::ReadWrite,
        }
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

impl From<ast::AddressSpace> for AddressSpace {
    fn from(class: ast::AddressSpace) -> Self {
        match class {
            ast::AddressSpace::FunctionClass(_) => Self::Function,
            ast::AddressSpace::Private(_) => Self::Private,
            ast::AddressSpace::Workgroup(_) => Self::Workgroup,
            ast::AddressSpace::Uniform(_) => Self::Uniform,
            ast::AddressSpace::Storage(_) => Self::Storage,
            ast::AddressSpace::PushConstant(_) => Self::PushConstant,
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

impl From<ast::SamplerType> for SamplerType {
    fn from(r#type: ast::SamplerType) -> Self {
        match r#type {
            ast::SamplerType::Sampler(_) => Self { comparison: false },
            ast::SamplerType::SamplerComparison(_) => Self { comparison: true },
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

impl TryFrom<ast::AtomicType> for AtomicType {
    type Error = ();

    fn try_from(atomic: ast::AtomicType) -> Result<Self, Self::Error> {
        let inner = first_type_generic(&atomic)?;
        Ok(Self {
            inner: Box::new(inner.try_into()?),
        })
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

impl TryFrom<ast::ArrayType> for ArrayType {
    type Error = ();

    fn try_from(array: ast::ArrayType) -> Result<Self, Self::Error> {
        let mut generics = array.generic_arg_list().ok_or(())?.generics();
        let inner = generics.next().ok_or(())?.as_type().ok_or(())?;
        let size = match generics.next() {
            Some(ast::GenericArg::Type(r#type)) => {
                ArraySize::Path(Name::from(r#type.as_name().ok_or(())?))
            },
            Some(ast::GenericArg::Literal(literal)) => match parse_literal(literal.kind()) {
                crate::expression::Literal::Int(value, _) => ArraySize::Int(value),
                crate::expression::Literal::Uint(value, _) => ArraySize::Uint(value),
                crate::expression::Literal::Float(..) | crate::expression::Literal::Bool(_) => {
                    return Err(());
                },
            },
            None => ArraySize::Dynamic,
            _ => return Err(()),
        };
        Ok(Self {
            inner: Box::new(inner.try_into()?),
            binding_array: false,
            size,
        })
    }
}

impl TryFrom<ast::BindingArrayType> for ArrayType {
    type Error = ();

    fn try_from(array: ast::BindingArrayType) -> Result<Self, Self::Error> {
        let mut generics = array.generic_arg_list().ok_or(())?.generics();
        let inner = generics.next().ok_or(())?.as_type().ok_or(())?;
        let size = match generics.next() {
            Some(ast::GenericArg::Type(r#type)) => {
                ArraySize::Path(Name::from(r#type.as_name().ok_or(())?))
            },
            Some(ast::GenericArg::Literal(literal)) => match parse_literal(literal.kind()) {
                crate::expression::Literal::Int(value, _) => ArraySize::Int(value),
                crate::expression::Literal::Uint(value, _) => ArraySize::Uint(value),
                crate::expression::Literal::Float(..) | crate::expression::Literal::Bool(_) => {
                    return Err(());
                },
            },
            None => ArraySize::Dynamic,
            _ => return Err(()),
        };
        Ok(Self {
            inner: Box::new(inner.try_into()?),
            binding_array: true,
            size,
        })
    }
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

impl TryFrom<ast::PointerType> for PointerType {
    type Error = ();

    fn try_from(pointer: ast::PointerType) -> Result<Self, Self::Error> {
        let mut generics = pointer.generic_arg_list().ok_or(())?.generics();
        let address_space: AddressSpace = match generics.next() {
            Some(ast::GenericArg::AddressSpace(class)) => class.into(),
            _ => return Err(()),
        };
        let inner = generics.next().ok_or(())?.as_type().ok_or(())?;

        let access_mode = match generics.next() {
            Some(ast::GenericArg::AccessMode(mode)) => mode.into(),
            None => address_space.default_access_mode(),
            _ => return Err(()),
        };

        Ok(Self {
            address_space,
            access_mode,
            inner: Box::new(inner.try_into()?),
        })
    }
}
