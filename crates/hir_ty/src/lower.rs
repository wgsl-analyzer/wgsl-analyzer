use std::fmt;

use hir_def::{
    body::BindingId,
    database::{GlobalConstantId, GlobalVariableId, OverrideId, StructId},
    expression::ExpressionId,
    expression_store::{ExpressionStore, path::Path},
    item_tree::Name,
    mod_path::PathKind,
    resolver::{ResolveKind, Resolver},
    type_specifier::TypeSpecifierId,
};
use wgsl_types::syntax::Enumerant;

use crate::{
    database::HirDatabase,
    function::ResolvedFunctionId,
    ty::{
        ArraySize, ArrayType, AtomicType, MatrixType, Pointer, Reference, ScalarType,
        TextureDimensionality, TextureKind, TextureType, Type, TypeKind, VecSize, VectorType,
    },
};

pub use crate::lower::generics::{TemplateParameter, TemplateParameters};

mod builtin;
mod eval;
mod generics;

/// Lowers types and evaluates expressions, the two are deeply intertwined.
pub struct TypeLoweringContext<'database> {
    database: &'database dyn HirDatabase,
    /// Make sure to set the correct resolver when going into function scopes.
    resolver: &'database Resolver,
    store: &'database ExpressionStore,

    pub(crate) diagnostics: Vec<TypeLoweringError>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TypeLoweringError {
    pub container: TypeContainer,
    pub kind: TypeLoweringErrorKind,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TypeLoweringErrorKind {
    UnresolvedName(Name),
    UnresolvedPath {
        path: Path,
        failed_segment: usize,
    },
    UnexpectedTemplateArgument(String),
    UnexpectedModule(Path),
    MissingTemplateArgument(String),
    MissingTemplate,
    WrongNumberOfTemplateArguments {
        expected: std::ops::RangeInclusive<usize>,
        actual: usize,
    },
    /// A value was provided where a type was expected.
    ExpectedType(Path),
    /// A function was provided but not called.
    ExpectedFunctionToBeCalled(Path),
    // TODO: Change this to a strongly typed wgsl_types::Error
    // The challenge here is that wgsl_types::Error doesn't implement Eq,
    // However the inference result keeps track of all the diagnostics and is cached
    // wgsl_types::Error cannot trivially implement Eq, because the `Instance` would
    // need to implement Eq. And it would have to be eq where "floating point NaNs" are
    // prooobably equal, if their bits are equal?
    WgslError(String),
}

impl fmt::Display for TypeLoweringErrorKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::UnresolvedName(name) => {
                write!(formatter, "`{}` not found in scope", name.as_str())
            },
            Self::UnresolvedPath {
                path,
                failed_segment,
            } => {
                if *failed_segment == 0 {
                    let name = path.mod_path().display_iter().next().unwrap_or_default();
                    write!(formatter, "`{name}` not found in scope")
                } else {
                    let mut segments = path.mod_path().display_iter().skip(*failed_segment - 1);
                    let previous_name = segments.next().unwrap_or_default();
                    let name = segments.next().unwrap_or_default();
                    write!(formatter, "`{name}` not found in `{previous_name}`")
                }
            },
            Self::WgslError(error) => {
                write!(formatter, "{error}")
            },
            Self::UnexpectedTemplateArgument(expected) => {
                write!(
                    formatter,
                    "unexpected template argument, expected {expected}"
                )
            },
            Self::UnexpectedModule(path) => {
                write!(
                    formatter,
                    "`{}` is a module, not a type or expression",
                    path.mod_path()
                )
            },
            Self::MissingTemplateArgument(expected) => {
                write!(formatter, "missing template argument, expected {expected}")
            },
            Self::MissingTemplate => {
                write!(formatter, "missing template arguments")
            },
            Self::WrongNumberOfTemplateArguments { expected, actual }
                if expected.start() == expected.end() =>
            {
                write!(
                    formatter,
                    "expected {} template arguments, but got {actual}",
                    expected.start()
                )
            },
            Self::WrongNumberOfTemplateArguments { expected, actual } => {
                write!(
                    formatter,
                    "expected {} to {} template arguments, but got {actual}",
                    expected.start(),
                    expected.end()
                )
            },
            Self::ExpectedType(path) => {
                write!(formatter, "{} is not a type", path.mod_path())
            },
            Self::ExpectedFunctionToBeCalled(path) => {
                write!(
                    formatter,
                    "{0:} was written, write {0:}() instead",
                    path.mod_path()
                )
            },
        }
    }
}

/// A lowered type, or the definition of an item.
/// Also covers built-ins.
pub enum Lowered {
    Type(Type),
    TypeWithoutTemplate(Type),
    Function(ResolvedFunctionId),
    GlobalConstant(GlobalConstantId),
    GlobalVariable(GlobalVariableId),
    Override(OverrideId),
    Local(BindingId),
    Enumerant(Enumerant),
    BuiltinFunction,
}

impl Lowered {
    #[must_use]
    pub const fn kind(&self) -> LoweredKind {
        match self {
            Self::Type(_) | Self::TypeWithoutTemplate(_) => LoweredKind::Type,
            Self::Function(_) | Self::BuiltinFunction => LoweredKind::Function,
            Self::GlobalConstant(_) => LoweredKind::Constant,
            Self::GlobalVariable(_) => LoweredKind::Variable,
            Self::Override(_) => LoweredKind::Override,
            Self::Local(_) => LoweredKind::Local,
            Self::Enumerant(_) => LoweredKind::Enumerant,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LoweredKind {
    Type,
    Function,
    Constant,
    Variable,
    Override,
    Local,
    Enumerant,
}

impl std::fmt::Display for LoweredKind {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Type => write!(f, "type"),
            Self::Function => write!(f, "function"),
            Self::Constant => write!(f, "constant"),
            Self::Variable => write!(f, "variable"),
            Self::Override => write!(f, "override"),
            Self::Local => write!(f, "local variable"),
            Self::Enumerant => write!(f, "enumerant"),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TypeContainer {
    Expression(ExpressionId),
    TypeSpecifier(TypeSpecifierId),
}

impl From<ExpressionId> for TypeContainer {
    fn from(id: ExpressionId) -> Self {
        Self::Expression(id)
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum ResolvedCall {
    Function(ResolvedFunctionId),
    OtherTypeInitializer(Type),
}

impl<'database> TypeLoweringContext<'database> {
    pub fn new(
        database: &'database dyn HirDatabase,
        resolver: &'database Resolver,
        store: &'database ExpressionStore,
    ) -> Self {
        Self {
            database,
            resolver,
            store,
            diagnostics: Vec::new(),
        }
    }

    pub fn lower(
        &mut self,
        expression: ExpressionId,
        path: &Path,
        template_parameters: &[ExpressionId],
    ) -> Lowered {
        match self.try_lower(
            TypeContainer::Expression(expression),
            path,
            template_parameters,
        ) {
            Ok(lowered) => lowered,
            Err(error) => {
                self.diagnostics.push(error);
                Lowered::Type(self.database.intern_type(TypeKind::Error))
            },
        }
    }

    /// Will lower types, and resolve the definition of other items.
    pub fn try_lower(
        &mut self,
        type_container: TypeContainer,
        path: &Path,
        template_parameters: &[ExpressionId],
    ) -> Result<Lowered, TypeLoweringError> {
        let resolved_type = self.resolver.resolve(self.database, path);

        if resolved_type.is_ok() {
            self.expect_no_template(template_parameters);
        }

        match resolved_type {
            Ok(ResolveKind::Module(module_id)) => Err(TypeLoweringError {
                container: type_container,
                kind: TypeLoweringErrorKind::UnexpectedModule(path.clone()),
            }),
            Ok(ResolveKind::TypeAlias(id)) => {
                Ok(Lowered::Type(self.database.type_alias_type(id).0))
            },
            Ok(ResolveKind::Struct(id)) => Ok(Lowered::Type(
                self.database.intern_type(TypeKind::Struct(id)),
            )),
            Ok(ResolveKind::Function(id)) => Ok(Lowered::Function(self.database.function_type(id))),
            Ok(ResolveKind::GlobalConstant(id)) => Ok(Lowered::GlobalConstant(id)),
            Ok(ResolveKind::GlobalVariable(id)) => Ok(Lowered::GlobalVariable(id)),
            Ok(ResolveKind::Override(id)) => Ok(Lowered::Override(id)),
            Ok(ResolveKind::Local(local, _)) => Ok(Lowered::Local(local)),
            Err(diagnostic) if path.mod_path().kind() == PathKind::Plain => path
                .mod_path()
                .segments()
                .first()
                .and_then(|predeclared_name| {
                    self.lower_if_predeclared(type_container, predeclared_name, template_parameters)
                })
                .ok_or_else(|| TypeLoweringError {
                    container: type_container,
                    kind: TypeLoweringErrorKind::UnresolvedPath {
                        path: path.clone(),
                        failed_segment: diagnostic.failed_segment,
                    },
                }),
            Err(diagnostic) => Err(TypeLoweringError {
                container: type_container,
                kind: TypeLoweringErrorKind::UnresolvedPath {
                    path: path.clone(),
                    failed_segment: diagnostic.failed_segment,
                },
            }),
        }
    }

    fn expect_no_template(
        &mut self,
        template_parameters: &[ExpressionId],
    ) {
        if template_parameters.is_empty() {
            return;
        }
        for template_expression in template_parameters {
            self.diagnostics.push(TypeLoweringError {
                container: TypeContainer::Expression(*template_expression),
                kind: TypeLoweringErrorKind::UnexpectedTemplateArgument("nothing".to_owned()),
            });
        }
    }

    fn expect_n_templates(
        &mut self,
        template_parameters: &TemplateParameters,
        expected: std::ops::RangeInclusive<usize>,
    ) -> bool {
        if expected.contains(&template_parameters.len()) {
            true
        } else {
            self.diagnostics.push(TypeLoweringError {
                container: *template_parameters.container(),
                kind: TypeLoweringErrorKind::WrongNumberOfTemplateArguments {
                    expected,
                    actual: template_parameters.len(),
                },
            });

            false
        }
    }

    pub fn lower_type(
        &mut self,
        type_specifier_id: TypeSpecifierId,
    ) -> Type {
        let type_specifier = &self.store[type_specifier_id];
        let lowered = self.try_lower(
            TypeContainer::TypeSpecifier(type_specifier_id),
            &type_specifier.path,
            &type_specifier.template_parameters,
        );
        match lowered {
            Ok(Lowered::Type(r#type)) => r#type,
            Ok(Lowered::TypeWithoutTemplate(_)) => {
                self.diagnostics.push(TypeLoweringError {
                    container: TypeContainer::TypeSpecifier(type_specifier_id),
                    kind: TypeLoweringErrorKind::MissingTemplate,
                });
                self.database.intern_type(TypeKind::Error)
            },
            Ok(
                Lowered::Enumerant(_)
                | Lowered::Function(_)
                | Lowered::BuiltinFunction
                | Lowered::GlobalConstant(_)
                | Lowered::GlobalVariable(_)
                | Lowered::Override(_)
                | Lowered::Local(_),
            ) => {
                self.diagnostics.push(TypeLoweringError {
                    container: TypeContainer::TypeSpecifier(type_specifier_id),
                    kind: TypeLoweringErrorKind::ExpectedType(type_specifier.path.clone()),
                });
                self.database.intern_type(TypeKind::Error)
            },
            Err(error) => {
                self.diagnostics.push(error);
                self.database.intern_type(TypeKind::Error)
            },
        }
    }
}

pub(crate) struct WgslTypeConverter<'database> {
    database: &'database dyn HirDatabase,
    interned_structs: Vec<StructId>,
}

impl<'database> WgslTypeConverter<'database> {
    pub fn new(database: &'database dyn HirDatabase) -> Self {
        Self {
            database,
            interned_structs: Vec::default(),
        }
    }

    #[expect(
        clippy::wrong_self_convention,
        reason = "naming things is hard and this is probably changing in the future"
    )]
    pub fn to_wgsl_types(
        &mut self,
        r#type: Type,
    ) -> Option<wgsl_types::Type> {
        Some(match r#type.kind(self.database) {
            // TODO: This should not be necessary because the types should align 1:1
            // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/672
            TypeKind::Error
            | TypeKind::BoundVariable(_)
            | TypeKind::StorageTypeOfTexelFormat(_) => {
                return None;
            },
            TypeKind::Scalar(ScalarType::AbstractFloat) => wgsl_types::Type::AbstractFloat,
            TypeKind::Scalar(ScalarType::AbstractInt) => wgsl_types::Type::AbstractInt,
            TypeKind::Scalar(ScalarType::Bool) => wgsl_types::Type::Bool,
            TypeKind::Scalar(ScalarType::F16) => wgsl_types::Type::F16,
            TypeKind::Scalar(ScalarType::F32) => wgsl_types::Type::F32,
            TypeKind::Scalar(ScalarType::I32) => wgsl_types::Type::I32,
            TypeKind::Scalar(ScalarType::U32) => wgsl_types::Type::U32,
            TypeKind::Scalar(ScalarType::I64) => wgsl_types::Type::I64,
            TypeKind::Scalar(ScalarType::U64) => wgsl_types::Type::U64,
            TypeKind::Atomic(AtomicType { inner }) => {
                wgsl_types::Type::Atomic(Box::new(self.to_wgsl_types(inner)?))
            },
            TypeKind::Vector(VectorType {
                size,
                component_type,
            }) => {
                wgsl_types::Type::Vec(size.as_u8(), Box::new(self.to_wgsl_types(component_type)?))
            },
            TypeKind::Matrix(MatrixType {
                columns,
                rows,
                inner,
            }) => wgsl_types::Type::Mat(
                columns.as_u8(),
                rows.as_u8(),
                Box::new(self.to_wgsl_types(inner)?),
            ),
            TypeKind::Struct(struct_id) => {
                let data = self.database.struct_data(struct_id).0;
                let fields = &self.database.field_types(struct_id).0;
                let name = self.intern_struct(struct_id);
                wgsl_types::Type::Struct(Box::new(wgsl_types::ty::StructType {
                    name,
                    members: data
                        .fields
                        .iter()
                        .map(|(id, data)| {
                            Some(wgsl_types::ty::StructMemberType {
                                name: data.name.as_str().to_owned(),
                                // Skip broken struct fields
                                ty: self.to_wgsl_types(fields[id])?,
                                // Don't bother reconstructing the correct layout
                                size: None,
                                align: None,
                            })
                        })
                        .collect::<Option<Vec<_>>>()?,
                }))
            },
            TypeKind::Array(ArrayType {
                inner,
                binding_array: false,
                size,
            }) => wgsl_types::Type::Array(
                Box::new(self.to_wgsl_types(inner)?),
                match size {
                    #[expect(clippy::as_conversions, reason = "externally defined")]
                    ArraySize::Constant(size) => Some(size as usize),
                    ArraySize::Dynamic => None,
                },
            ),
            TypeKind::Array(ArrayType {
                inner,
                binding_array: true,
                size,
            }) => wgsl_types::Type::BindingArray(
                Box::new(self.to_wgsl_types(inner)?),
                match size {
                    #[expect(clippy::as_conversions, reason = "externally defined")]
                    ArraySize::Constant(size) => Some(size as usize),
                    ArraySize::Dynamic => None,
                },
            ),
            TypeKind::Texture(texture_type) => {
                wgsl_types::Type::Texture(self.to_wgsl_texture_type(texture_type))
            },
            TypeKind::Sampler(sampler_type) => wgsl_types::Type::Sampler(sampler_type),
            TypeKind::Reference(Reference {
                address_space,
                inner,
                access_mode,
            }) => wgsl_types::Type::Ref(
                address_space,
                Box::new(self.to_wgsl_types(inner)?),
                access_mode,
            ),
            TypeKind::Pointer(Pointer {
                address_space,
                inner,
                access_mode,
            }) => wgsl_types::Type::Ptr(
                address_space,
                Box::new(self.to_wgsl_types(inner)?),
                access_mode,
            ),
        })
    }

    /// Returns `None` if it is an error type.
    pub fn template_parameter_to_wgsl_types(
        &mut self,
        param: TemplateParameter,
    ) -> Option<wgsl_types::tplt::TpltParam> {
        Some(match param {
            TemplateParameter::Type(r#type) => {
                wgsl_types::tplt::TpltParam::Type(self.to_wgsl_types(r#type)?)
            },
            TemplateParameter::Instance(instance) => {
                wgsl_types::tplt::TpltParam::Instance(instance?)
            },
            TemplateParameter::Enumerant(enumerant) => {
                wgsl_types::tplt::TpltParam::Enumerant(enumerant)
            },
        })
    }

    #[expect(
        clippy::wrong_self_convention,
        reason = "naming things is hard and this is probably changing in the future"
    )]
    pub fn from_wgsl_types(
        &self,
        r#type: wgsl_types::Type,
    ) -> Type {
        #[expect(
            clippy::todo,
            reason = "See https://github.com/wgsl-analyzer/wgsl-analyzer/issues/442"
        )]
        match r#type {
            wgsl_types::Type::Bool => TypeKind::Scalar(ScalarType::Bool).intern(self.database),
            wgsl_types::Type::AbstractInt => {
                TypeKind::Scalar(ScalarType::AbstractInt).intern(self.database)
            },
            wgsl_types::Type::AbstractFloat => {
                TypeKind::Scalar(ScalarType::AbstractFloat).intern(self.database)
            },
            wgsl_types::Type::I32 => TypeKind::Scalar(ScalarType::I32).intern(self.database),
            wgsl_types::Type::U32 => TypeKind::Scalar(ScalarType::U32).intern(self.database),
            wgsl_types::Type::I64 => TypeKind::Scalar(ScalarType::I64).intern(self.database),
            wgsl_types::Type::U64 => TypeKind::Scalar(ScalarType::U64).intern(self.database),
            wgsl_types::Type::F16 => TypeKind::Scalar(ScalarType::F16).intern(self.database),
            wgsl_types::Type::F32 => TypeKind::Scalar(ScalarType::F32).intern(self.database),
            wgsl_types::Type::F64 => todo!("naga extension"),
            wgsl_types::Type::Struct(struct_type) => {
                let struct_id = self
                    .get_interned_struct(&struct_type.name)
                    // I think this doesn't hold true when calling `atomicCompareExchangeWeak`
                    .expect("Only struct types that have been passed in should be returned");
                TypeKind::Struct(struct_id).intern(self.database)
            },
            wgsl_types::Type::Array(r#type, size) => TypeKind::Array(ArrayType {
                inner: self.from_wgsl_types(*r#type),
                binding_array: false,
                size: match size {
                    Some(size) => {
                        debug_assert!(u32::try_from(size).is_ok());
                        #[expect(
                            clippy::cast_possible_truncation,
                            clippy::as_conversions,
                            reason = "externally defined"
                        )]
                        ArraySize::Constant(size as u32)
                    },
                    None => ArraySize::Dynamic,
                },
            })
            .intern(self.database),
            wgsl_types::Type::BindingArray(r#type, size) => TypeKind::Array(ArrayType {
                inner: self.from_wgsl_types(*r#type),
                binding_array: true,
                size: match size {
                    Some(size) => {
                        debug_assert!(u32::try_from(size).is_ok());
                        #[expect(
                            clippy::cast_possible_truncation,
                            clippy::as_conversions,
                            reason = "externally defined"
                        )]
                        ArraySize::Constant(size as u32)
                    },
                    None => ArraySize::Dynamic,
                },
            })
            .intern(self.database),
            wgsl_types::Type::Vec(size, r#type) => TypeKind::Vector(VectorType {
                size: VecSize::try_from(size).unwrap(),
                component_type: self.from_wgsl_types(*r#type),
            })
            .intern(self.database),
            wgsl_types::Type::Mat(columns, rows, r#type) => TypeKind::Matrix(MatrixType {
                columns: VecSize::try_from(columns).unwrap(),
                rows: VecSize::try_from(rows).unwrap(),
                inner: self.from_wgsl_types(*r#type),
            })
            .intern(self.database),
            wgsl_types::Type::Atomic(r#type) => TypeKind::Atomic(AtomicType {
                inner: self.from_wgsl_types(*r#type),
            })
            .intern(self.database),
            wgsl_types::Type::Ptr(address_space, r#type, access_mode) => {
                TypeKind::Pointer(Pointer {
                    address_space,
                    inner: self.from_wgsl_types(*r#type),
                    access_mode,
                })
                .intern(self.database)
            },
            wgsl_types::Type::Ref(address_space, r#type, access_mode) => {
                TypeKind::Reference(Reference {
                    address_space,
                    inner: self.from_wgsl_types(*r#type),
                    access_mode,
                })
                .intern(self.database)
            },
            wgsl_types::Type::Texture(texture_type) => {
                TypeKind::Texture(self.from_wgsl_texture_type(&texture_type)).intern(self.database)
            },
            wgsl_types::Type::Sampler(sampler_type) => {
                TypeKind::Sampler(sampler_type).intern(self.database)
            },
            wgsl_types::Type::RayQuery(_) => todo!("naga extension"),
            wgsl_types::Type::AccelerationStructure(_) => todo!("naga extension"),
        }
    }

    #[expect(clippy::too_many_lines, reason = "long but simple match")]
    #[expect(
        clippy::wrong_self_convention,
        reason = "naming things is hard and this is probably changing in the future"
    )]
    fn from_wgsl_texture_type(
        &self,
        value: &wgsl_types::ty::TextureType,
    ) -> TextureType {
        match *value {
            wgsl_types::ty::TextureType::Sampled1D(sampled_type) => TextureType {
                kind: TextureKind::from_sampled(sampled_type, self.database),
                dimension: TextureDimensionality::D1,
                arrayed: false,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Sampled1DArray(sampled_type) => TextureType {
                kind: TextureKind::from_sampled(sampled_type, self.database),
                dimension: TextureDimensionality::D1,
                arrayed: true,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Sampled2D(sampled_type) => TextureType {
                kind: TextureKind::from_sampled(sampled_type, self.database),
                dimension: TextureDimensionality::D2,
                arrayed: false,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Sampled2DArray(sampled_type) => TextureType {
                kind: TextureKind::from_sampled(sampled_type, self.database),
                dimension: TextureDimensionality::D2,
                arrayed: true,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Sampled3D(sampled_type) => TextureType {
                kind: TextureKind::from_sampled(sampled_type, self.database),
                dimension: TextureDimensionality::D3,
                arrayed: false,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::SampledCube(sampled_type) => TextureType {
                kind: TextureKind::from_sampled(sampled_type, self.database),
                dimension: TextureDimensionality::Cube,
                arrayed: false,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::SampledCubeArray(sampled_type) => TextureType {
                kind: TextureKind::from_sampled(sampled_type, self.database),
                dimension: TextureDimensionality::Cube,
                arrayed: true,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Multisampled2D(sampled_type) => TextureType {
                kind: TextureKind::from_sampled(sampled_type, self.database),
                dimension: TextureDimensionality::D2,
                arrayed: false,
                multisampled: true,
            },
            wgsl_types::ty::TextureType::Multisampled2DArray(sampled_type) => TextureType {
                kind: TextureKind::from_sampled(sampled_type, self.database),
                dimension: TextureDimensionality::D2,
                arrayed: true,
                multisampled: true,
            },
            wgsl_types::ty::TextureType::DepthMultisampled2D => TextureType {
                kind: TextureKind::Depth,
                dimension: TextureDimensionality::D2,
                arrayed: false,
                multisampled: true,
            },
            wgsl_types::ty::TextureType::External => TextureType {
                kind: TextureKind::External,
                dimension: TextureDimensionality::D2,
                arrayed: false,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Storage1D(texel_format, access_mode) => TextureType {
                kind: TextureKind::Storage(from_wgsl_texel_format(texel_format), access_mode),
                dimension: TextureDimensionality::D1,
                arrayed: false,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Storage1DArray(texel_format, access_mode) => TextureType {
                kind: TextureKind::Storage(from_wgsl_texel_format(texel_format), access_mode),
                dimension: TextureDimensionality::D1,
                arrayed: true,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Storage2D(texel_format, access_mode) => TextureType {
                kind: TextureKind::Storage(from_wgsl_texel_format(texel_format), access_mode),
                dimension: TextureDimensionality::D2,
                arrayed: false,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Storage2DArray(texel_format, access_mode) => TextureType {
                kind: TextureKind::Storage(from_wgsl_texel_format(texel_format), access_mode),
                dimension: TextureDimensionality::D2,
                arrayed: true,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Storage3D(texel_format, access_mode) => TextureType {
                kind: TextureKind::Storage(from_wgsl_texel_format(texel_format), access_mode),
                dimension: TextureDimensionality::D3,
                arrayed: false,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Depth2D => TextureType {
                kind: TextureKind::Depth,
                dimension: TextureDimensionality::D2,
                arrayed: false,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Depth2DArray => TextureType {
                kind: TextureKind::Depth,
                dimension: TextureDimensionality::D2,
                arrayed: true,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::DepthCube => TextureType {
                kind: TextureKind::Depth,
                dimension: TextureDimensionality::Cube,
                arrayed: false,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::DepthCubeArray => TextureType {
                kind: TextureKind::Depth,
                dimension: TextureDimensionality::Cube,
                arrayed: true,
                multisampled: false,
            },
        }
    }

    fn to_wgsl_texture_type(
        &self,
        value: TextureType,
    ) -> wgsl_types::ty::TextureType {
        match (value.kind, value.dimension, value.arrayed) {
            (TextureKind::Sampled(sampled), TextureDimensionality::D1, false) => {
                wgsl_types::ty::TextureType::Sampled1D(self.to_wgsl_sampled(sampled))
            },
            (TextureKind::Sampled(sampled), TextureDimensionality::D1, true) => {
                wgsl_types::ty::TextureType::Sampled1DArray(self.to_wgsl_sampled(sampled))
            },
            (TextureKind::Sampled(sampled), TextureDimensionality::D2, false) => {
                wgsl_types::ty::TextureType::Sampled2D(self.to_wgsl_sampled(sampled))
            },
            (TextureKind::Sampled(sampled), TextureDimensionality::D2, true) => {
                wgsl_types::ty::TextureType::Sampled2DArray(self.to_wgsl_sampled(sampled))
            },
            (TextureKind::Sampled(sampled), TextureDimensionality::D3, false) => {
                wgsl_types::ty::TextureType::Sampled3D(self.to_wgsl_sampled(sampled))
            },
            (TextureKind::Sampled(sampled), TextureDimensionality::Cube, false) => {
                wgsl_types::ty::TextureType::SampledCube(self.to_wgsl_sampled(sampled))
            },
            (TextureKind::Sampled(sampled), TextureDimensionality::Cube, true) => {
                wgsl_types::ty::TextureType::SampledCubeArray(self.to_wgsl_sampled(sampled))
            },
            (TextureKind::Storage(texel_format, access_mode), TextureDimensionality::D1, false) => {
                wgsl_types::ty::TextureType::Storage1D(
                    to_wgsl_texel_format(texel_format),
                    access_mode,
                )
            },
            (TextureKind::Storage(texel_format, access_mode), TextureDimensionality::D1, true) => {
                wgsl_types::ty::TextureType::Storage1DArray(
                    to_wgsl_texel_format(texel_format),
                    access_mode,
                )
            },
            (TextureKind::Storage(texel_format, access_mode), TextureDimensionality::D2, false) => {
                wgsl_types::ty::TextureType::Storage2D(
                    to_wgsl_texel_format(texel_format),
                    access_mode,
                )
            },
            (TextureKind::Storage(texel_format, access_mode), TextureDimensionality::D2, true) => {
                wgsl_types::ty::TextureType::Storage2DArray(
                    to_wgsl_texel_format(texel_format),
                    access_mode,
                )
            },
            (TextureKind::Storage(texel_format, access_mode), TextureDimensionality::D3, false) => {
                wgsl_types::ty::TextureType::Storage3D(
                    to_wgsl_texel_format(texel_format),
                    access_mode,
                )
            },
            (TextureKind::Depth, TextureDimensionality::D2, false) => {
                wgsl_types::ty::TextureType::Depth2D
            },
            (TextureKind::Depth, TextureDimensionality::D2, true) => {
                wgsl_types::ty::TextureType::Depth2DArray
            },
            (TextureKind::Depth, TextureDimensionality::Cube, false) => {
                wgsl_types::ty::TextureType::DepthCube
            },
            (TextureKind::Depth, TextureDimensionality::Cube, true) => {
                wgsl_types::ty::TextureType::DepthCubeArray
            },
            (TextureKind::External, _, _) => wgsl_types::ty::TextureType::External,
            (_, _, _) => panic!("invalid texture"),
        }
    }

    fn intern_struct(
        &mut self,
        struct_id: StructId,
    ) -> String {
        let index = self.interned_structs.len();
        self.interned_structs.push(struct_id);
        format!("struct{index}")
    }

    fn get_interned_struct(
        &self,
        name: &str,
    ) -> Option<StructId> {
        let index = name.strip_prefix("struct")?.parse::<usize>().ok()?;
        self.interned_structs.get(index).copied()
    }

    fn to_wgsl_sampled(
        &self,
        sampled: Type,
    ) -> wgsl_types::syntax::SampledType {
        match sampled.kind(self.database) {
            TypeKind::Scalar(ScalarType::I32) => wgsl_types::syntax::SampledType::I32,
            TypeKind::Scalar(ScalarType::U32) => wgsl_types::syntax::SampledType::U32,
            TypeKind::Scalar(ScalarType::F32) => wgsl_types::syntax::SampledType::F32,
            kind @ (TypeKind::Error
            | TypeKind::Scalar(_)
            | TypeKind::Atomic(_)
            | TypeKind::Vector(_)
            | TypeKind::Matrix(_)
            | TypeKind::Struct(_)
            | TypeKind::Array(_)
            | TypeKind::Texture(_)
            | TypeKind::Sampler(_)
            | TypeKind::Reference(_)
            | TypeKind::Pointer(_)
            | TypeKind::BoundVariable(_)
            | TypeKind::StorageTypeOfTexelFormat(_)) => panic!("invalid sampled type {kind:?}"),
        }
    }
}

#[must_use]
pub fn from_wgsl_texel_format(
    texel_format: wgsl_types::syntax::TexelFormat
) -> crate::ty::TexelFormat {
    match texel_format {
        wgsl_types::syntax::TexelFormat::Rgba8Unorm => crate::ty::TexelFormat::Rgba8unorm,
        wgsl_types::syntax::TexelFormat::Rgba8Snorm => crate::ty::TexelFormat::Rgba8snorm,
        wgsl_types::syntax::TexelFormat::Rgba8Uint => crate::ty::TexelFormat::Rgba8uint,
        wgsl_types::syntax::TexelFormat::Rgba8Sint => crate::ty::TexelFormat::Rgba8sint,
        wgsl_types::syntax::TexelFormat::Rgba16Uint => crate::ty::TexelFormat::Rgba16uint,
        wgsl_types::syntax::TexelFormat::Rgba16Sint => crate::ty::TexelFormat::Rgba16sint,
        wgsl_types::syntax::TexelFormat::Rgba16Float => crate::ty::TexelFormat::Rgba16float,
        wgsl_types::syntax::TexelFormat::R32Uint => crate::ty::TexelFormat::R32uint,
        wgsl_types::syntax::TexelFormat::R32Sint => crate::ty::TexelFormat::R32sint,
        wgsl_types::syntax::TexelFormat::R32Float => crate::ty::TexelFormat::R32float,
        wgsl_types::syntax::TexelFormat::Rg32Uint => crate::ty::TexelFormat::Rg32uint,
        wgsl_types::syntax::TexelFormat::Rg32Sint => crate::ty::TexelFormat::Rg32sint,
        wgsl_types::syntax::TexelFormat::Rg32Float => crate::ty::TexelFormat::Rg32float,
        wgsl_types::syntax::TexelFormat::Rgba32Uint => crate::ty::TexelFormat::Rgba32uint,
        wgsl_types::syntax::TexelFormat::Rgba32Sint => crate::ty::TexelFormat::Rgba32sint,
        wgsl_types::syntax::TexelFormat::Rgba32Float => crate::ty::TexelFormat::Rgba32float,
        wgsl_types::syntax::TexelFormat::Bgra8Unorm => crate::ty::TexelFormat::Bgra8unorm,
        wgsl_types::syntax::TexelFormat::R8Unorm
        | wgsl_types::syntax::TexelFormat::R8Snorm
        | wgsl_types::syntax::TexelFormat::R8Uint
        | wgsl_types::syntax::TexelFormat::R8Sint
        | wgsl_types::syntax::TexelFormat::R16Unorm
        | wgsl_types::syntax::TexelFormat::R16Snorm
        | wgsl_types::syntax::TexelFormat::R16Uint
        | wgsl_types::syntax::TexelFormat::R16Sint
        | wgsl_types::syntax::TexelFormat::R16Float
        | wgsl_types::syntax::TexelFormat::Rg8Unorm
        | wgsl_types::syntax::TexelFormat::Rg8Snorm
        | wgsl_types::syntax::TexelFormat::Rg8Uint
        | wgsl_types::syntax::TexelFormat::Rg8Sint
        | wgsl_types::syntax::TexelFormat::Rg16Unorm
        | wgsl_types::syntax::TexelFormat::Rg16Snorm
        | wgsl_types::syntax::TexelFormat::Rg16Uint
        | wgsl_types::syntax::TexelFormat::Rg16Sint
        | wgsl_types::syntax::TexelFormat::Rg16Float
        | wgsl_types::syntax::TexelFormat::Rgb10a2Uint
        | wgsl_types::syntax::TexelFormat::Rgb10a2Unorm
        | wgsl_types::syntax::TexelFormat::Rg11b10Float
        | wgsl_types::syntax::TexelFormat::R64Uint
        | wgsl_types::syntax::TexelFormat::Rgba16Unorm
        | wgsl_types::syntax::TexelFormat::Rgba16Snorm => {
            #[expect(
                clippy::unimplemented,
                reason = "TODO: support naga texture formats, see: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/675"
            )]
            {
                unimplemented!("not yet supported naga extension")
            }
        },
    }
}

/// Convert a [`crate::ty::TexelFormat`] into a [`wgsl_types::syntax::TexelFormat`].
///
/// # Panics
///
/// Panics if `texel_format` is `BoundVariable` or `Any`.
#[expect(
    deprecated,
    reason = "TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/559"
)]
#[must_use]
pub fn to_wgsl_texel_format(
    texel_format: crate::ty::TexelFormat
) -> wgsl_types::syntax::TexelFormat {
    match texel_format {
        crate::ty::TexelFormat::Rgba8unorm => wgsl_types::syntax::TexelFormat::Rgba8Unorm,
        crate::ty::TexelFormat::Rgba8snorm => wgsl_types::syntax::TexelFormat::Rgba8Snorm,
        crate::ty::TexelFormat::Rgba8uint => wgsl_types::syntax::TexelFormat::Rgba8Uint,
        crate::ty::TexelFormat::Rgba8sint => wgsl_types::syntax::TexelFormat::Rgba8Sint,
        crate::ty::TexelFormat::Rgba16uint => wgsl_types::syntax::TexelFormat::Rgba16Uint,
        crate::ty::TexelFormat::Rgba16sint => wgsl_types::syntax::TexelFormat::Rgba16Sint,
        crate::ty::TexelFormat::Rgba16float => wgsl_types::syntax::TexelFormat::Rgba16Float,
        crate::ty::TexelFormat::R32uint => wgsl_types::syntax::TexelFormat::R32Uint,
        crate::ty::TexelFormat::R32sint => wgsl_types::syntax::TexelFormat::R32Sint,
        crate::ty::TexelFormat::R32float => wgsl_types::syntax::TexelFormat::R32Float,
        crate::ty::TexelFormat::Rg32uint => wgsl_types::syntax::TexelFormat::Rg32Uint,
        crate::ty::TexelFormat::Rg32sint => wgsl_types::syntax::TexelFormat::Rg32Sint,
        crate::ty::TexelFormat::Rg32float => wgsl_types::syntax::TexelFormat::Rg32Float,
        crate::ty::TexelFormat::Rgba32uint => wgsl_types::syntax::TexelFormat::Rgba32Uint,
        crate::ty::TexelFormat::Rgba32sint => wgsl_types::syntax::TexelFormat::Rgba32Sint,
        crate::ty::TexelFormat::Rgba32float => wgsl_types::syntax::TexelFormat::Rgba32Float,
        crate::ty::TexelFormat::Bgra8unorm => wgsl_types::syntax::TexelFormat::Bgra8Unorm,
        crate::ty::TexelFormat::BoundVariable(_) => {
            panic!("bound var is not a valid texel format to convert")
        },
        crate::ty::TexelFormat::Any => panic!("any is not a valid texel format to convert"),
    }
}
