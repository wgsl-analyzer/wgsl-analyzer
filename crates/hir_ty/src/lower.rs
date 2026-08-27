use std::{fmt, num::NonZeroU32};

use base_db::Intern as _;
use either::Either;
use hir_def::{
    body::BindingId,
    db::{GlobalConstantId, GlobalVariableId, OverrideId, StructId},
    expression::{
        ArithmeticOperation, BinaryOperation, ComparisonOperation, ExpressionId, LogicOperation,
        UnaryOperator,
    },
    expression_store::{ExpressionStore, path::Path},
    item_tree::Name,
    resolver::{ResolutionDiagnostic, ResolveKind, Resolver},
    signature::StructSignature,
    type_specifier::TypeSpecifierId,
};
use wgsl_types::{Instance, syntax::Enumerant};

use crate::{
    db::HirDatabase,
    function::ResolvedFunctionId,
    ty::{
        ArraySize, ArrayType, AtomicType, BuiltinStruct, MatrixType, Pointer, Reference,
        ScalarType, TextureDimensionality, TextureKind, TextureType, Type, TypeKind, VecSize,
        VectorType, pretty::pretty_type,
    },
};

pub use crate::lower::generics::{TemplateParameter, TemplateParameters};

mod builtin;
mod eval;
mod generics;

/// Lowers types and evaluates expressions, the two are deeply intertwined.
pub struct TypeLoweringContext<'db> {
    db: &'db dyn HirDatabase,
    /// Make sure to set the correct resolver when going into function scopes.
    resolver: &'db Resolver<'db>,
    store: &'db ExpressionStore,

    pub(crate) diagnostics: Vec<TypeLoweringError>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct TypeLoweringError {
    pub container: TypeContainer,
    pub kind: TypeLoweringErrorKind,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum UnexpectedTemplateArgumentValue {
    Type(Type),
    Instance(String),
    Enumerant(Enumerant),
}

impl From<TemplateParameter> for UnexpectedTemplateArgumentValue {
    fn from(value: TemplateParameter) -> Self {
        match value {
            TemplateParameter::Type(r#type) => r#type.into(),
            TemplateParameter::Instance(instance) => instance.into(),
            TemplateParameter::Enumerant(enumerant) => enumerant.into(),
        }
    }
}

impl From<Type> for UnexpectedTemplateArgumentValue {
    fn from(value: Type) -> Self {
        Self::Type(value)
    }
}

impl From<Option<Instance>> for UnexpectedTemplateArgumentValue {
    fn from(value: Option<Instance>) -> Self {
        Self::Instance(value.map_or_else(|| "[error]".to_owned(), |instance| instance.to_string()))
    }
}

impl From<Enumerant> for UnexpectedTemplateArgumentValue {
    fn from(value: Enumerant) -> Self {
        Self::Enumerant(value)
    }
}

impl UnexpectedTemplateArgumentValue {
    fn display(
        &self,
        db: &dyn HirDatabase,
    ) -> impl fmt::Display {
        match self {
            Self::Type(r#type) => pretty_type(db, *r#type),
            Self::Instance(instance) => instance.clone(),
            Self::Enumerant(enumerant) => enumerant.to_string(),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TypeLoweringErrorKind {
    Resolution(ResolutionDiagnostic),
    UnexpectedTemplateArgument(String, UnexpectedTemplateArgumentValue),
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

impl TypeLoweringErrorKind {
    pub fn display(
        &self,
        db: &dyn HirDatabase,
    ) -> impl fmt::Display {
        match self {
            Self::Resolution(ResolutionDiagnostic::UnresolvedName { name }) => {
                format!("`{}` not found in scope", name.as_str())
            },
            Self::Resolution(ResolutionDiagnostic::UnresolvedFile {
                package: _,
                path: _,
            }) => "could not find file".to_owned(),
            Self::Resolution(ResolutionDiagnostic::DetachedFile) => {
                "current file is detached".to_owned()
            },
            Self::Resolution(ResolutionDiagnostic::MissingName) => {
                "path is missing a name".to_owned()
            },
            Self::Resolution(ResolutionDiagnostic::PrivateItem {
                name,
                visibility: _,
            }) => {
                format!("`{}` is private", name.as_str())
            },
            Self::Resolution(ResolutionDiagnostic::TooManySupers) => {
                "too many `super::`s".to_owned()
            },
            Self::Resolution(ResolutionDiagnostic::UnresolvedItem { name, file_id: _ }) => {
                format!("`{}` not found in other file", name.as_str())
            },
            Self::Resolution(ResolutionDiagnostic::UnresolvedPackage { name }) => {
                format!("package `{}` not found", name.as_str())
            },
            Self::WgslError(error) => error.clone(),
            Self::UnexpectedTemplateArgument(expected, actual) => {
                format!(
                    "unexpected template argument, expected {expected}, actual: {}",
                    actual.display(db)
                )
            },
            Self::UnexpectedModule(path) => {
                format!(
                    "`{}` is a module, not a type or expression",
                    path.mod_path()
                )
            },
            Self::MissingTemplateArgument(expected) => {
                format!("missing template argument, expected {expected}")
            },
            Self::MissingTemplate => "missing template arguments".to_owned(),
            Self::WrongNumberOfTemplateArguments { expected, actual }
                if expected.start() == expected.end() =>
            {
                format!(
                    "expected {} template arguments, but got {actual}",
                    expected.start()
                )
            },
            Self::WrongNumberOfTemplateArguments { expected, actual } => {
                format!(
                    "expected {} to {} template arguments, but got {actual}",
                    expected.start(),
                    expected.end()
                )
            },
            Self::ExpectedType(path) => {
                format!("{} is not a type", path.mod_path())
            },
            Self::ExpectedFunctionToBeCalled(path) => {
                format!("{0:} was written, write {0:}() instead", path.mod_path())
            },
        }
    }
}

/// A lowered type, or the definition of an item.
/// Also covers built-ins.
pub enum Lowered {
    Type(Type),
    ConstructibleTypeGenerator(ConstructibleTypeGenerator),
    Function(ResolvedFunctionId),
    GlobalConstant(GlobalConstantId),
    GlobalVariable(GlobalVariableId),
    Override(OverrideId),
    Local(BindingId),
    Enumerant(Enumerant),
    BuiltinFunction(Name, Option<TemplateParameters>),
    // BuiltinConstructor(Name, Option<TemplateParameters>),
    BuiltinDeclaration(Name, Instance),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConstructibleTypeGenerator {
    Vector(VectorType),
    Matrix(MatrixType),
    Array(ArrayType),
}

impl Lowered {
    #[must_use]
    pub const fn kind(&self) -> LoweredKind {
        match self {
            Self::Type(_) | Self::ConstructibleTypeGenerator(_)
            // | Self::BuiltinConstructor(_, _)
            => {
                LoweredKind::Type
            },
            Self::Function(_) | Self::BuiltinFunction(_, _) => LoweredKind::Function,
            Self::GlobalConstant(_) | Self::BuiltinDeclaration(_, _) => LoweredKind::Constant,
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

impl<'db> TypeLoweringContext<'db> {
    pub fn new(
        db: &'db dyn HirDatabase,
        resolver: &'db Resolver<'db>,
        store: &'db ExpressionStore,
    ) -> Self {
        Self {
            db,
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
        match self.lower_expression(
            TypeContainer::Expression(expression),
            path,
            template_parameters,
        ) {
            Ok(lowered) => lowered,
            Err(error) => {
                self.diagnostics.push(error);
                Lowered::Type(TypeKind::Error.intern(self.db))
            },
        }
    }

    /// Will lower types, and resolve the definition of other items.
    pub fn lower_expression(
        &mut self,
        type_container: TypeContainer,
        path: &Path,
        template_parameters: &[ExpressionId],
    ) -> Result<Lowered, TypeLoweringError> {
        let resolved_type = self.resolver.resolve(self.db, path);
        let mut template_parameters = self.eval_template_args(type_container, template_parameters);
        match resolved_type {
            Ok(ResolveKind::TypeAlias(id)) => {
                self.expect_no_template(&template_parameters);
                Ok(Lowered::Type(self.db.type_alias_type(id).0))
            },
            Ok(ResolveKind::Struct(id)) => {
                self.expect_no_template(&template_parameters);
                Ok(Lowered::Type(TypeKind::Struct(id).intern(self.db)))
            },
            Ok(ResolveKind::Function(id)) => {
                self.expect_no_template(&template_parameters);
                Ok(Lowered::Function(self.db.function_type(id)))
            },
            Ok(ResolveKind::GlobalConstant(id)) => {
                self.expect_no_template(&template_parameters);
                Ok(Lowered::GlobalConstant(id))
            },
            Ok(ResolveKind::GlobalVariable(id)) => {
                self.expect_no_template(&template_parameters);
                Ok(Lowered::GlobalVariable(id))
            },
            Ok(ResolveKind::Override(id)) => {
                self.expect_no_template(&template_parameters);
                Ok(Lowered::Override(id))
            },
            Ok(ResolveKind::Local(local, _function_parent)) => {
                self.expect_no_template(&template_parameters);
                Ok(Lowered::Local(local))
            },
            Ok(ResolveKind::BuiltinFunction(name)) => {
                Ok(Lowered::BuiltinFunction(name, Some(template_parameters)))
            },
            Ok(ResolveKind::BuiltinType(name)) => {
                self.lower_builtin_type(name, type_container, &mut template_parameters)
            },
            Ok(ResolveKind::BuiltinTypeGenerator(name)) => {
                match self.lower_builtin_type_generator(
                    type_container,
                    &name,
                    &template_parameters,
                )? {
                    Either::Left(generator) => Ok(Lowered::ConstructibleTypeGenerator(generator)),
                    Either::Right(r#type) => Ok(Lowered::Type(r#type)),
                }
            },
            // Ok(ResolveKind::BuiltinTypeConstructor(name)) => self
            //     .lower_builtin_type(type_container, &name, &template_parameters)?
            //     .ok_or_else(|| TypeLoweringError {
            //         container: type_container,
            //         kind: TypeLoweringErrorKind::Resolution(ResolutionDiagnostic::UnresolvedName {
            //             name,
            //         }),
            //     }),
            Ok(ResolveKind::BuiltinEnumerant(name)) => {
                self.expect_no_template(&template_parameters);
                self.lower_builtin_enumerant(&name)
                    .map_err(|()| TypeLoweringError {
                        container: type_container,
                        kind: TypeLoweringErrorKind::Resolution(
                            ResolutionDiagnostic::UnresolvedName { name },
                        ),
                    })
            },
            Ok(ResolveKind::BuiltinDeclaration(name)) => {
                self.expect_no_template(&template_parameters);
                self.lower_builtin_declaration(type_container, name)
            },
            Err(diagnostic) => Err(TypeLoweringError {
                container: type_container,
                kind: TypeLoweringErrorKind::Resolution(diagnostic),
            }),
        }
    }

    fn expect_no_template(
        &mut self,
        template_parameters: &TemplateParameters,
    ) {
        if template_parameters.len() == 0 {
            return;
        }
        let mut iter = template_parameters.clone();
        while let Some((parameter, template_expression)) = iter.take_next() {
            self.diagnostics.push(TypeLoweringError {
                container: TypeContainer::Expression(template_expression),
                kind: TypeLoweringErrorKind::UnexpectedTemplateArgument(
                    "nothing".to_owned(),
                    parameter.into(),
                ),
            });
        }
    }

    fn expect_n_templates(
        &mut self,
        template_parameters: &TemplateParameters,
        expected: std::ops::RangeInclusive<usize>,
    ) {
        if !expected.contains(&template_parameters.len()) {
            self.diagnostics.push(TypeLoweringError {
                container: *template_parameters.container(),
                kind: TypeLoweringErrorKind::WrongNumberOfTemplateArguments {
                    expected,
                    actual: template_parameters.len(),
                },
            });
        }
    }

    pub fn lower_type(
        &mut self,
        type_specifier_id: TypeSpecifierId,
    ) -> Type {
        let type_specifier = &self.store[type_specifier_id];
        let lowered = self.lower_expression(
            TypeContainer::TypeSpecifier(type_specifier_id),
            &type_specifier.path,
            &type_specifier.template_parameters,
        );
        match lowered {
            Ok(Lowered::Type(r#type)) => r#type,
            Ok(Lowered::ConstructibleTypeGenerator(_)) => {
                self.diagnostics.push(TypeLoweringError {
                    container: TypeContainer::TypeSpecifier(type_specifier_id),
                    kind: TypeLoweringErrorKind::MissingTemplate,
                });
                TypeKind::Error.intern(self.db)
            },
            Ok(
                Lowered::Enumerant(_)
                | Lowered::Function(_)
                | Lowered::BuiltinFunction(_, _)
                // | Lowered::BuiltinConstructor(_, _)
                | Lowered::GlobalConstant(_)
                | Lowered::BuiltinDeclaration(_, _)
                | Lowered::GlobalVariable(_)
                | Lowered::Override(_)
                | Lowered::Local(_),
            ) => {
                self.diagnostics.push(TypeLoweringError {
                    container: TypeContainer::TypeSpecifier(type_specifier_id),
                    kind: TypeLoweringErrorKind::ExpectedType(type_specifier.path.clone()),
                });
                TypeKind::Error.intern(self.db)
            },
            Err(error) => {
                self.diagnostics.push(error);
                TypeKind::Error.intern(self.db)
            },
        }
    }
}

pub(crate) struct WgslTypeConverter<'db> {
    db: &'db dyn HirDatabase,
    interned_structs: Vec<StructId>,
}

impl<'db> WgslTypeConverter<'db> {
    pub fn new(db: &'db dyn HirDatabase) -> Self {
        Self {
            db,
            interned_structs: Vec::default(),
        }
    }

    #[expect(
        clippy::wrong_self_convention,
        reason = "naming things is hard and this is probably changing in the future"
    )]
    pub fn to_maybe_vec_template(
        &mut self,
        template_parameters: Option<TemplateParameters>,
    ) -> Result<Option<Vec<wgsl_types::tplt::TpltParam>>, ()> {
        match self.to_wgsl_template_parameters(template_parameters) {
            Ok(items) if items.is_empty() => Ok(None),
            Ok(items) => Ok(Some(items)),
            Err(()) => Err(()),
        }
    }

    #[expect(
        clippy::wrong_self_convention,
        reason = "naming things is hard and this is probably changing in the future"
    )]
    pub fn to_wt_vec(
        &mut self,
        argument_types: &[Type],
    ) -> Vec<wgsl_types::Type> {
        argument_types
            .iter()
            .copied()
            .map(|r#type| self.to_wgsl_types(r#type))
            .collect()
    }

    #[expect(
        clippy::wrong_self_convention,
        reason = "naming things is hard and this is probably changing in the future"
    )]
    pub fn to_wgsl_types(
        &mut self,
        r#type: Type,
    ) -> wgsl_types::Type {
        match r#type.kind(self.db) {
            TypeKind::Error => wgsl_types::Type::Unknown,
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
                wgsl_types::Type::Atomic(Box::new(self.to_wgsl_types(inner)))
            },
            TypeKind::Vector(VectorType {
                size,
                component_type,
            }) => wgsl_types::Type::Vec(size.as_u8(), Box::new(self.to_wgsl_types(component_type))),
            TypeKind::Matrix(MatrixType {
                columns,
                rows,
                inner,
            }) => wgsl_types::Type::Mat(
                columns.as_u8(),
                rows.as_u8(),
                Box::new(self.to_wgsl_types(inner)),
            ),
            TypeKind::Struct(struct_id) => {
                let struct_type = self.to_wgsl_struct(struct_id);
                wgsl_types::Type::Struct(Box::new(struct_type))
            },
            TypeKind::BuiltinStruct(builtin_struct) => {
                wgsl_types::Type::Struct(Box::new(wgsl_types::ty::StructType {
                    name: builtin_struct.name,
                    members: builtin_struct
                        .fields
                        .into_iter()
                        .map(|(name, r#type)| {
                            wgsl_types::ty::StructMemberType {
                                name,
                                ty: self.to_wgsl_types(r#type),
                                // Don't bother reconstructing the correct layout
                                size: None,
                                align: None,
                            }
                        })
                        .collect::<Vec<_>>(),
                }))
            },
            TypeKind::Array(ArrayType {
                inner,
                binding_array: false,
                size,
            }) => wgsl_types::Type::Array(
                Box::new(self.to_wgsl_types(inner)),
                match size {
                    #[expect(clippy::as_conversions, reason = "externally defined")]
                    ArraySize::Constant(size) => Some(size.get() as usize),
                    ArraySize::Dynamic => None,
                },
            ),
            TypeKind::Array(ArrayType {
                inner,
                binding_array: true,
                size,
            }) => wgsl_types::Type::BindingArray(
                Box::new(self.to_wgsl_types(inner)),
                match size {
                    #[expect(clippy::as_conversions, reason = "externally defined")]
                    ArraySize::Constant(size) => Some(size.get() as usize),
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
                Box::new(self.to_wgsl_types(inner)),
                access_mode,
            ),
            TypeKind::Pointer(Pointer {
                address_space,
                inner,
                access_mode,
            }) => wgsl_types::Type::Ptr(
                address_space,
                Box::new(self.to_wgsl_types(inner)),
                access_mode,
            ),
            TypeKind::AccelerationStructure(tags) => wgsl_types::Type::AccelerationStructure(tags),
        }
    }

    #[expect(
        clippy::wrong_self_convention,
        reason = "naming things is hard and this is probably changing in the future"
    )]
    pub fn to_wgsl_struct(
        &mut self,
        struct_id: StructId,
    ) -> wgsl_types::ty::StructType {
        let data = StructSignature::of(self.db, struct_id);
        let fields = &self.db.field_types(struct_id).0;
        let name = self.intern_struct(struct_id);
        wgsl_types::ty::StructType {
            name,
            members: data
                .fields
                .iter()
                .map(|(id, data)| {
                    wgsl_types::ty::StructMemberType {
                        name: data.name.as_str().to_owned(),
                        ty: self.to_wgsl_types(fields[id]),
                        // Don't bother reconstructing the correct layout
                        size: None,
                        align: None,
                    }
                })
                .collect::<Vec<_>>(),
        }
    }

    /// Returns `None` if it is an error type.
    pub fn template_parameter_to_wgsl_types(
        &mut self,
        param: TemplateParameter,
    ) -> Option<wgsl_types::tplt::TpltParam> {
        Some(match param {
            TemplateParameter::Type(r#type) => {
                wgsl_types::tplt::TpltParam::Type(self.to_wgsl_types(r#type))
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
            wgsl_types::Type::Bool => TypeKind::Scalar(ScalarType::Bool).intern(self.db),
            wgsl_types::Type::AbstractInt => {
                TypeKind::Scalar(ScalarType::AbstractInt).intern(self.db)
            },
            wgsl_types::Type::AbstractFloat => {
                TypeKind::Scalar(ScalarType::AbstractFloat).intern(self.db)
            },
            wgsl_types::Type::I32 => TypeKind::Scalar(ScalarType::I32).intern(self.db),
            wgsl_types::Type::U32 => TypeKind::Scalar(ScalarType::U32).intern(self.db),
            wgsl_types::Type::I64 => TypeKind::Scalar(ScalarType::I64).intern(self.db),
            wgsl_types::Type::U64 => TypeKind::Scalar(ScalarType::U64).intern(self.db),
            wgsl_types::Type::F16 => TypeKind::Scalar(ScalarType::F16).intern(self.db),
            wgsl_types::Type::F32 => TypeKind::Scalar(ScalarType::F32).intern(self.db),
            wgsl_types::Type::F64 => todo!("naga extension"),
            wgsl_types::Type::Struct(struct_type) => {
                if let Some(struct_id) = self.get_interned_struct(&struct_type.name) {
                    TypeKind::Struct(struct_id).intern(self.db)
                } else {
                    // fallback, assume that it is a builtin struct
                    let fields = struct_type
                        .members
                        .into_iter()
                        .map(|member| (member.name, self.from_wgsl_types(member.ty)))
                        .collect();
                    TypeKind::BuiltinStruct(BuiltinStruct {
                        name: struct_type.name,
                        fields,
                    })
                    .intern(self.db)
                }
            },
            // TODO: bufferArrayView
            wgsl_types::Type::Array(r#type, size) => TypeKind::Array(ArrayType {
                inner: self.from_wgsl_types(*r#type),
                binding_array: false,
                size: from_wgsl_array_size(size),
            })
            .intern(self.db),
            wgsl_types::Type::BindingArray(r#type, size) => TypeKind::Array(ArrayType {
                inner: self.from_wgsl_types(*r#type),
                binding_array: true,
                size: from_wgsl_array_size(size),
            })
            .intern(self.db),
            wgsl_types::Type::Vec(size, r#type) => TypeKind::Vector(VectorType {
                size: VecSize::try_from(size).unwrap(),
                component_type: self.from_wgsl_types(*r#type),
            })
            .intern(self.db),
            wgsl_types::Type::Mat(columns, rows, r#type) => TypeKind::Matrix(MatrixType {
                columns: VecSize::try_from(columns).unwrap(),
                rows: VecSize::try_from(rows).unwrap(),
                inner: self.from_wgsl_types(*r#type),
            })
            .intern(self.db),
            wgsl_types::Type::Atomic(r#type) => TypeKind::Atomic(AtomicType {
                inner: self.from_wgsl_types(*r#type),
            })
            .intern(self.db),
            wgsl_types::Type::Ptr(address_space, r#type, access_mode) => {
                TypeKind::Pointer(Pointer {
                    address_space,
                    inner: self.from_wgsl_types(*r#type),
                    access_mode,
                })
                .intern(self.db)
            },
            wgsl_types::Type::Ref(address_space, r#type, access_mode) => {
                TypeKind::Reference(Reference {
                    address_space,
                    inner: self.from_wgsl_types(*r#type),
                    access_mode,
                })
                .intern(self.db)
            },
            wgsl_types::Type::Texture(texture_type) => {
                TypeKind::Texture(self.from_wgsl_texture_type(&texture_type)).intern(self.db)
            },
            wgsl_types::Type::Sampler(sampler_type) => {
                TypeKind::Sampler(sampler_type).intern(self.db)
            },
            wgsl_types::Type::RayQuery(_) => todo!("naga extension"),
            wgsl_types::Type::AccelerationStructure(tags) => TypeKind::AccelerationStructure(tags).intern(self.db),
            wgsl_types::Type::Unknown => TypeKind::Error.intern(self.db),
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
                kind: TextureKind::from_sampled(sampled_type, self.db),
                dimension: TextureDimensionality::D1,
                arrayed: false,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Sampled1DArray(sampled_type) => TextureType {
                kind: TextureKind::from_sampled(sampled_type, self.db),
                dimension: TextureDimensionality::D1,
                arrayed: true,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Sampled2D(sampled_type) => TextureType {
                kind: TextureKind::from_sampled(sampled_type, self.db),
                dimension: TextureDimensionality::D2,
                arrayed: false,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Sampled2DArray(sampled_type) => TextureType {
                kind: TextureKind::from_sampled(sampled_type, self.db),
                dimension: TextureDimensionality::D2,
                arrayed: true,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Sampled3D(sampled_type) => TextureType {
                kind: TextureKind::from_sampled(sampled_type, self.db),
                dimension: TextureDimensionality::D3,
                arrayed: false,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::SampledCube(sampled_type) => TextureType {
                kind: TextureKind::from_sampled(sampled_type, self.db),
                dimension: TextureDimensionality::Cube,
                arrayed: false,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::SampledCubeArray(sampled_type) => TextureType {
                kind: TextureKind::from_sampled(sampled_type, self.db),
                dimension: TextureDimensionality::Cube,
                arrayed: true,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Multisampled2D(sampled_type) => TextureType {
                kind: TextureKind::from_sampled(sampled_type, self.db),
                dimension: TextureDimensionality::D2,
                arrayed: false,
                multisampled: true,
            },
            wgsl_types::ty::TextureType::Multisampled2DArray(sampled_type) => TextureType {
                kind: TextureKind::from_sampled(sampled_type, self.db),
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
                kind: TextureKind::Storage(texel_format, access_mode),
                dimension: TextureDimensionality::D1,
                arrayed: false,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Storage1DArray(texel_format, access_mode) => TextureType {
                kind: TextureKind::Storage(texel_format, access_mode),
                dimension: TextureDimensionality::D1,
                arrayed: true,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Storage2D(texel_format, access_mode) => TextureType {
                kind: TextureKind::Storage(texel_format, access_mode),
                dimension: TextureDimensionality::D2,
                arrayed: false,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Storage2DArray(texel_format, access_mode) => TextureType {
                kind: TextureKind::Storage(texel_format, access_mode),
                dimension: TextureDimensionality::D2,
                arrayed: true,
                multisampled: false,
            },
            wgsl_types::ty::TextureType::Storage3D(texel_format, access_mode) => TextureType {
                kind: TextureKind::Storage(texel_format, access_mode),
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
                wgsl_types::ty::TextureType::Storage1D(texel_format, access_mode)
            },
            (TextureKind::Storage(texel_format, access_mode), TextureDimensionality::D1, true) => {
                wgsl_types::ty::TextureType::Storage1DArray(texel_format, access_mode)
            },
            (TextureKind::Storage(texel_format, access_mode), TextureDimensionality::D2, false) => {
                wgsl_types::ty::TextureType::Storage2D(texel_format, access_mode)
            },
            (TextureKind::Storage(texel_format, access_mode), TextureDimensionality::D2, true) => {
                wgsl_types::ty::TextureType::Storage2DArray(texel_format, access_mode)
            },
            (TextureKind::Storage(texel_format, access_mode), TextureDimensionality::D3, false) => {
                wgsl_types::ty::TextureType::Storage3D(texel_format, access_mode)
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
        match sampled.kind(self.db) {
            TypeKind::Scalar(ScalarType::I32) => wgsl_types::syntax::SampledType::I32,
            TypeKind::Scalar(ScalarType::U32) => wgsl_types::syntax::SampledType::U32,
            TypeKind::Scalar(ScalarType::F32) => wgsl_types::syntax::SampledType::F32,
            TypeKind::Scalar(ScalarType::U64) => wgsl_types::syntax::SampledType::U64,
            kind @ (TypeKind::Error
            | TypeKind::Scalar(_)
            | TypeKind::Atomic(_)
            | TypeKind::Vector(_)
            | TypeKind::Matrix(_)
            | TypeKind::Struct(_)
            | TypeKind::BuiltinStruct(_)
            | TypeKind::Array(_)
            | TypeKind::Texture(_)
            | TypeKind::Sampler(_)
            | TypeKind::AccelerationStructure(_)
            | TypeKind::Reference(_)
            | TypeKind::Pointer(_)) => panic!("invalid sampled type {kind:?}"),
        }
    }

    #[expect(
        clippy::wrong_self_convention,
        reason = "naming things is hard and this is probably changing in the future"
    )]
    pub fn to_wgsl_template_parameters(
        &mut self,
        template_parameters: Option<TemplateParameters>,
    ) -> Result<Vec<wgsl_types::tplt::TpltParam>, ()> {
        let Some(mut template_parameters) = template_parameters else {
            return Ok(vec![]);
        };
        let mut template_args = vec![];
        while let Some((template_parameter, _)) = template_parameters.take_next() {
            let template_parameter = self
                .template_parameter_to_wgsl_types(template_parameter)
                .ok_or(())?;
            template_args.push(template_parameter);
        }
        Ok(template_args)
    }
}

fn from_wgsl_array_size(size: Option<usize>) -> ArraySize {
    match size.map(|size| u32::try_from(size).map(NonZeroU32::try_from)) {
        Some(Ok(Ok(size))) => ArraySize::Constant(size),
        None => ArraySize::Dynamic,
        Some(Ok(Err(error))) => {
            panic!("size cannot be 0, error: {error}, got: {size:?}");
        },
        Some(Err(error)) => {
            panic!("size must not be > u32::MAX, error: {error}, got: {size:?}");
        },
    }
}

#[must_use]
pub const fn to_wgsl_binary_operator(
    operation: BinaryOperation
) -> wgsl_types::syntax::BinaryOperator {
    use wgsl_types::syntax::BinaryOperator as Wtbo;
    match operation {
        BinaryOperation::Logical(logic_operation) => match logic_operation {
            LogicOperation::ShortCircuitAnd => Wtbo::ShortCircuitAnd,
            LogicOperation::ShortCircuitOr => Wtbo::ShortCircuitOr,
        },
        BinaryOperation::Arithmetic(arithmetic_operation) => match arithmetic_operation {
            ArithmeticOperation::Addition => Wtbo::Addition,
            ArithmeticOperation::Multiplication => Wtbo::Multiplication,
            ArithmeticOperation::Subtraction => Wtbo::Subtraction,
            ArithmeticOperation::Division => Wtbo::Division,
            ArithmeticOperation::ShiftLeft => Wtbo::ShiftLeft,
            ArithmeticOperation::ShiftRight => Wtbo::ShiftRight,
            ArithmeticOperation::BitwiseXor => Wtbo::BitwiseXor,
            ArithmeticOperation::BitwiseOr => Wtbo::BitwiseOr,
            ArithmeticOperation::BitwiseAnd => Wtbo::BitwiseAnd,
            ArithmeticOperation::Remainder => Wtbo::Remainder,
        },
        BinaryOperation::Comparison(comparison_operation) => match comparison_operation {
            ComparisonOperation::Equality => Wtbo::Equality,
            ComparisonOperation::Inequality => Wtbo::Inequality,
            ComparisonOperation::LessThan => Wtbo::LessThan,
            ComparisonOperation::LessThanEqual => Wtbo::LessThanEqual,
            ComparisonOperation::GreaterThan => Wtbo::GreaterThan,
            ComparisonOperation::GreaterThanEqual => Wtbo::GreaterThanEqual,
        },
    }
}

#[must_use]
pub const fn from_wgsl_binary_operator(
    operation: wgsl_types::syntax::BinaryOperator
) -> BinaryOperation {
    use syntax::ast::operators::BinaryOperation as Bo;
    use wgsl_types::syntax::BinaryOperator as Wtbo;
    match operation {
        Wtbo::ShortCircuitAnd => Bo::Logical(LogicOperation::ShortCircuitAnd),
        Wtbo::ShortCircuitOr => Bo::Logical(LogicOperation::ShortCircuitOr),
        Wtbo::Addition => Bo::Arithmetic(ArithmeticOperation::Addition),
        Wtbo::Multiplication => Bo::Arithmetic(ArithmeticOperation::Multiplication),
        Wtbo::Subtraction => Bo::Arithmetic(ArithmeticOperation::Subtraction),
        Wtbo::Division => Bo::Arithmetic(ArithmeticOperation::Division),
        Wtbo::ShiftLeft => Bo::Arithmetic(ArithmeticOperation::ShiftLeft),
        Wtbo::ShiftRight => Bo::Arithmetic(ArithmeticOperation::ShiftRight),
        Wtbo::BitwiseXor => Bo::Arithmetic(ArithmeticOperation::BitwiseXor),
        Wtbo::BitwiseOr => Bo::Arithmetic(ArithmeticOperation::BitwiseOr),
        Wtbo::BitwiseAnd => Bo::Arithmetic(ArithmeticOperation::BitwiseAnd),
        Wtbo::Remainder => Bo::Arithmetic(ArithmeticOperation::Remainder),
        Wtbo::Equality => Bo::Comparison(ComparisonOperation::Equality),
        Wtbo::Inequality => Bo::Comparison(ComparisonOperation::Inequality),
        Wtbo::LessThan => Bo::Comparison(ComparisonOperation::LessThan),
        Wtbo::LessThanEqual => Bo::Comparison(ComparisonOperation::LessThanEqual),
        Wtbo::GreaterThan => Bo::Comparison(ComparisonOperation::GreaterThan),
        Wtbo::GreaterThanEqual => Bo::Comparison(ComparisonOperation::GreaterThanEqual),
    }
}

#[must_use]
pub const fn to_wgsl_unary_operator(operation: UnaryOperator) -> wgsl_types::syntax::UnaryOperator {
    use wgsl_types::syntax::UnaryOperator as Wtuo;
    match operation {
        UnaryOperator::Negation => Wtuo::Negation,
        UnaryOperator::LogicalNegation => Wtuo::LogicalNegation,
        UnaryOperator::AddressOf => Wtuo::AddressOf,
        UnaryOperator::Indirection => Wtuo::Indirection,
        UnaryOperator::BitwiseComplement => Wtuo::BitwiseComplement,
    }
}

#[must_use]
pub const fn from_wgsl_unary_operator(
    operation: wgsl_types::syntax::UnaryOperator
) -> UnaryOperator {
    use wgsl_types::syntax::UnaryOperator as Wtuo;
    match operation {
        Wtuo::LogicalNegation => UnaryOperator::Negation,
        Wtuo::Negation => UnaryOperator::LogicalNegation,
        Wtuo::BitwiseComplement => UnaryOperator::AddressOf,
        Wtuo::AddressOf => UnaryOperator::Indirection,
        Wtuo::Indirection => UnaryOperator::BitwiseComplement,
    }
}
