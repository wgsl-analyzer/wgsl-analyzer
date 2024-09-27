use syntax::{ast, HasGenerics};

use crate::{expr::parse_literal, module_data::Name};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TypeRef {
    Error,
    Scalar(ScalarType),
    Vec(VecType),
    Matrix(MatrixType),
    Texture(TextureType),
    Sampler(SamplerType),
    Atomic(AtomicType),
    Array(ArrayType),
    Path(Name),
    Ptr(PtrType),
}
impl std::fmt::Display for TypeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeRef::Error => write!(f, "[error]"),
            TypeRef::Scalar(val) => write!(f, "{}", val),
            TypeRef::Vec(val) => write!(f, "{}", val),
            TypeRef::Matrix(val) => write!(f, "{}", val),
            TypeRef::Texture(val) => write!(f, "{}", val),
            TypeRef::Sampler(val) => write!(f, "{}", val),
            TypeRef::Atomic(val) => write!(f, "{}", val),
            TypeRef::Array(val) => write!(f, "{}", val),
            TypeRef::Path(val) => write!(f, "{}", val.as_str()),
            TypeRef::Ptr(val) => write!(f, "{}", val),
        }
    }
}

impl TryFrom<ast::Type> for TypeRef {
    type Error = ();

    fn try_from(ty: ast::Type) -> Result<Self, ()> {
        let type_ref = match ty {
            ast::Type::PathType(path) => TypeRef::Path(path.name().ok_or(())?.text().into()),
            ast::Type::ScalarType(scalar) => TypeRef::Scalar(scalar.into()),
            ast::Type::VecType(vec) => TypeRef::Vec(vec.try_into()?),
            ast::Type::MatrixType(matrix) => TypeRef::Matrix(matrix.try_into()?),
            ast::Type::TextureType(tex) => TypeRef::Texture(tex.try_into()?),
            ast::Type::SamplerType(sampler) => TypeRef::Sampler(sampler.into()),
            ast::Type::AtomicType(atomic) => TypeRef::Atomic(atomic.try_into()?),
            ast::Type::ArrayType(array) => TypeRef::Array(array.try_into()?),
            ast::Type::BindingArrayType(array) => TypeRef::Array(array.try_into()?),
            ast::Type::PtrType(ptr) => TypeRef::Ptr(ptr.try_into()?),
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
    fn from(ty: ast::ScalarType) -> Self {
        match ty {
            ast::ScalarType::Bool(_) => ScalarType::Bool,
            ast::ScalarType::Float32(_) => ScalarType::Float32,
            ast::ScalarType::Int32(_) => ScalarType::Int32,
            ast::ScalarType::Uint32(_) => ScalarType::Uint32,
        }
    }
}
impl std::fmt::Display for ScalarType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScalarType::Bool => f.write_str("bool"),
            ScalarType::Float32 => f.write_str("f32"),
            ScalarType::Int32 => f.write_str("i32"),
            ScalarType::Uint32 => f.write_str("u32"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct VecType {
    pub size: VecDimensionality,
    pub inner: Box<TypeRef>,
}

impl std::fmt::Display for VecType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vec{}<{}>", self.size, &*self.inner)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum VecDimensionality {
    Two,
    Three,
    Four,
}
impl std::fmt::Display for VecDimensionality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VecDimensionality::Two => f.write_str("2"),
            VecDimensionality::Three => f.write_str("3"),
            VecDimensionality::Four => f.write_str("4"),
        }
    }
}
impl TryFrom<ast::VecType> for VecType {
    type Error = ();
    fn try_from(ty: ast::VecType) -> Result<Self, ()> {
        let size = vector_dimensions(&ty);
        let inner = first_type_generic(&ty)?;

        Ok(VecType {
            size,
            inner: Box::new(inner.try_into()?),
        })
    }
}

pub(crate) fn vector_dimensions(ty: &ast::VecType) -> VecDimensionality {
    
    match *ty {
        ast::VecType::Vec2(_) => VecDimensionality::Two,
        ast::VecType::Vec3(_) => VecDimensionality::Three,
        ast::VecType::Vec4(_) => VecDimensionality::Four,
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MatrixType {
    pub columns: VecDimensionality,
    pub rows: VecDimensionality,
    pub inner: Box<TypeRef>,
}
impl TryFrom<ast::MatrixType> for MatrixType {
    type Error = ();
    fn try_from(ty: ast::MatrixType) -> Result<Self, ()> {
        let (columns, rows) = matrix_dimensions(&ty);
        let inner = first_type_generic(&ty)?;

        Ok(MatrixType {
            columns,
            rows,
            inner: Box::new(inner.try_into()?),
        })
    }
}

pub(crate) fn matrix_dimensions(ty: &ast::MatrixType) -> (VecDimensionality, VecDimensionality) {
    let (columns, rows) = match *ty {
        ast::MatrixType::Mat2x2(_) => (VecDimensionality::Two, VecDimensionality::Two),
        ast::MatrixType::Mat2x3(_) => (VecDimensionality::Two, VecDimensionality::Three),
        ast::MatrixType::Mat2x4(_) => (VecDimensionality::Two, VecDimensionality::Two),
        ast::MatrixType::Mat3x2(_) => (VecDimensionality::Three, VecDimensionality::Two),
        ast::MatrixType::Mat3x3(_) => (VecDimensionality::Three, VecDimensionality::Three),
        ast::MatrixType::Mat3x4(_) => (VecDimensionality::Three, VecDimensionality::Four),
        ast::MatrixType::Mat4x2(_) => (VecDimensionality::Four, VecDimensionality::Two),
        ast::MatrixType::Mat4x3(_) => (VecDimensionality::Four, VecDimensionality::Three),
        ast::MatrixType::Mat4x4(_) => (VecDimensionality::Four, VecDimensionality::Four),
    };
    (columns, rows)
}

impl std::fmt::Display for MatrixType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mat{}x{}<{}>", self.columns, self.rows, &*self.inner)
    }
}

fn first_type_generic<T: HasGenerics>(ty: &T) -> Result<ast::Type, ()> {
    let mut generics = ty.generic_arg_list().ok_or(())?.generics();
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
impl std::fmt::Display for TextureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            TextureKind::Sampled(ty) => write!(
                f,
                "texture_{}{}{}<{}>",
                if self.multisampled {
                    "multisampled_"
                } else {
                    ""
                },
                self.dimension,
                if self.arrayed { "_array" } else { "" },
                ty
            ),
            TextureKind::Storage(format, mode) => write!(
                f,
                "texture_storage_{}{}{}<{}, {}>",
                self.dimension,
                if self.multisampled {
                    "_multisampled"
                } else {
                    ""
                },
                if self.arrayed { "_array" } else { "" },
                format,
                mode,
            ),
            TextureKind::Depth => write!(
                f,
                "texture_depth_{}{}{}",
                if self.multisampled {
                    "multisampled_"
                } else {
                    ""
                },
                self.dimension,
                if self.arrayed { "_array" } else { "" },
            ),
            TextureKind::External => write!(f, "texture_external"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TextureKind {
    Sampled(Box<TypeRef>),
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

impl std::fmt::Display for TextureDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextureDimension::D1 => f.write_str("1d"),
            TextureDimension::D2 => f.write_str("2d"),
            TextureDimension::D3 => f.write_str("3d"),
            TextureDimension::Cube => f.write_str("cube"),
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
            }
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
            }
            TextureKindVariant::Depth => TextureKind::Depth,
            TextureKindVariant::External => TextureKind::External,
        };

        Ok(TextureType {
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

    // this is only used for builtins which don't care about the access mode (e.g. textureDimensions)
    Any,
}
impl std::fmt::Display for AccessMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccessMode::ReadWrite => f.write_str("read_write"),
            AccessMode::Read => f.write_str("read"),
            AccessMode::Write => f.write_str("write"),
            AccessMode::Any => f.write_str("_"),
        }
    }
}
impl AccessMode {
    pub fn read_write() -> AccessMode {
        AccessMode::ReadWrite
    }
}

impl From<ast::AccessMode> for AccessMode {
    fn from(value: ast::AccessMode) -> Self {
        match value {
            ast::AccessMode::Read(_) => AccessMode::Read,
            ast::AccessMode::Write(_) => AccessMode::Write,
            ast::AccessMode::ReadWrite(_) => AccessMode::ReadWrite,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum StorageClass {
    Function,
    Private,
    Workgroup,
    Uniform,
    Storage,
    Handle,
    PushConstant,
}
impl std::fmt::Display for StorageClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            StorageClass::Function => "function",
            StorageClass::Private => "private",
            StorageClass::Workgroup => "workgroup",
            StorageClass::Uniform => "uniform",
            StorageClass::Storage => "storage",
            StorageClass::Handle => "handle",
            StorageClass::PushConstant => "push_constant",
        })
    }
}
impl StorageClass {
    pub fn default_access_mode(self) -> AccessMode {
        match self {
            StorageClass::Storage => AccessMode::Read,
            StorageClass::Function => AccessMode::ReadWrite,
            StorageClass::Private => AccessMode::ReadWrite,
            StorageClass::Workgroup => AccessMode::ReadWrite,
            StorageClass::Uniform => AccessMode::Read,
            StorageClass::Handle => AccessMode::Read,
            StorageClass::PushConstant => AccessMode::Read,
        }
    }
}
impl From<ast::StorageClass> for StorageClass {
    fn from(class: ast::StorageClass) -> Self {
        match class {
            ast::StorageClass::FunctionClass(_) => StorageClass::Function,
            ast::StorageClass::Private(_) => StorageClass::Private,
            ast::StorageClass::Workgroup(_) => StorageClass::Workgroup,
            ast::StorageClass::Uniform(_) => StorageClass::Uniform,
            ast::StorageClass::Storage(_) => StorageClass::Storage,
            ast::StorageClass::PushConstant(_) => StorageClass::PushConstant,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SamplerType {
    pub comparison: bool,
}

impl std::fmt::Display for SamplerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.comparison {
            true => f.write_str("sampler_comparison"),
            false => f.write_str("sampler"),
        }
    }
}

impl From<ast::SamplerType> for SamplerType {
    fn from(ty: ast::SamplerType) -> Self {
        match ty {
            ast::SamplerType::Sampler(_) => SamplerType { comparison: false },
            ast::SamplerType::SamplerComparison(_) => SamplerType { comparison: true },
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct AtomicType {
    pub inner: Box<TypeRef>,
}
impl std::fmt::Display for AtomicType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "atomic<{}>", self.inner)
    }
}

impl TryFrom<ast::AtomicType> for AtomicType {
    type Error = ();

    fn try_from(atomic: ast::AtomicType) -> Result<Self, Self::Error> {
        let inner = first_type_generic(&atomic)?;
        Ok(AtomicType {
            inner: Box::new(inner.try_into()?),
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ArrayType {
    pub inner: Box<TypeRef>,
    pub binding_array: bool,
    pub size: ArraySize,
}

impl std::fmt::Display for ArrayType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = if self.binding_array { "binding_" } else { "" };
        match self.size {
            ArraySize::Int(size) => write!(f, "{}array<{}, {}>", prefix, self.inner, size),
            ArraySize::Uint(size) => write!(f, "{}array<{}, {}>", prefix, self.inner, size),
            ArraySize::Path(ref size) => {
                write!(f, "{}array<{}, {}>", prefix, self.inner, size.as_str())
            }
            ArraySize::Dynamic => write!(f, "{}array<{}>", prefix, self.inner),
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
            Some(ast::GenericArg::Type(ty)) => ArraySize::Path(Name::from(ty.as_name().ok_or(())?)),
            Some(ast::GenericArg::Literal(lit)) => match parse_literal(lit.kind()) {
                crate::expr::Literal::Int(val, _) => ArraySize::Int(val),
                crate::expr::Literal::Uint(val, _) => ArraySize::Uint(val),
                _ => return Err(()),
            },
            None => ArraySize::Dynamic,
            _ => return Err(()),
        };
        Ok(ArrayType {
            inner: Box::new(inner.try_into()?),
            size,
            binding_array: false,
        })
    }
}
impl TryFrom<ast::BindingArrayType> for ArrayType {
    type Error = ();

    fn try_from(array: ast::BindingArrayType) -> Result<Self, Self::Error> {
        let mut generics = array.generic_arg_list().ok_or(())?.generics();
        let inner = generics.next().ok_or(())?.as_type().ok_or(())?;
        let size = match generics.next() {
            Some(ast::GenericArg::Type(ty)) => ArraySize::Path(Name::from(ty.as_name().ok_or(())?)),
            Some(ast::GenericArg::Literal(lit)) => match parse_literal(lit.kind()) {
                crate::expr::Literal::Int(val, _) => ArraySize::Int(val),
                crate::expr::Literal::Uint(val, _) => ArraySize::Uint(val),
                _ => return Err(()),
            },
            None => ArraySize::Dynamic,
            _ => return Err(()),
        };
        Ok(ArrayType {
            inner: Box::new(inner.try_into()?),
            size,
            binding_array: true,
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct PtrType {
    pub storage_class: StorageClass,
    pub access_mode: AccessMode,
    pub inner: Box<TypeRef>,
}
impl std::fmt::Display for PtrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ptr<{}, {}>", self.storage_class, self.inner)
    }
}

impl TryFrom<ast::PtrType> for PtrType {
    type Error = ();

    fn try_from(ptr: ast::PtrType) -> Result<Self, Self::Error> {
        let mut generics = ptr.generic_arg_list().ok_or(())?.generics();
        let storage_class: StorageClass = match generics.next() {
            Some(ast::GenericArg::StorageClass(class)) => class.into(),
            _ => return Err(()),
        };
        let inner = generics.next().ok_or(())?.as_type().ok_or(())?;

        let access_mode = match generics.next() {
            Some(ast::GenericArg::AccessMode(mode)) => mode.into(),
            None => storage_class.default_access_mode(),
            _ => return Err(()),
        };

        Ok(PtrType {
            inner: Box::new(inner.try_into()?),
            access_mode,
            storage_class,
        })
    }
}
