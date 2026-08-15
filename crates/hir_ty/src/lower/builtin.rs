use std::{num::NonZeroU32, str::FromStr as _};

use base_db::{CapabilitiesInput, Intern as _};
use hir_def::item_tree::Name;
use wgsl_types::{
    Instance,
    inst::LiteralInstance,
    syntax::{AccessMode, AddressSpace, Enumerant, SampledType, TexelFormat},
};

use crate::{
    lower::{
        Lowered, TypeContainer, TypeLoweringContext, TypeLoweringError, TypeLoweringErrorKind,
        generics::TemplateParameters,
    },
    ty::{
        ArraySize, ArrayType, AtomicType, MatrixType, Pointer, ScalarType, TextureDimensionality,
        TextureKind, TextureType, Type, TypeKind, VecSize, VectorType,
    },
};

impl TypeLoweringContext<'_> {
    pub fn lower_if_predeclared(
        &mut self,
        name: &Name,
        template_parameters: TemplateParameters,
    ) -> Option<Lowered> {
        // If lowering the predeclared type failed, we should return a error type
        // As opposed to ignoring it when it's not a predeclared type
        match self.lower_predeclared_type(name, &template_parameters) {
            Ok(Some(lowered)) => Some(lowered),
            Ok(None) => {
                if wgsl_types::idents::BUILTIN_FUNCTION_NAMES.contains(&name.as_str()) {
                    Some(Lowered::BuiltinFunction(
                        name.clone(),
                        Some(template_parameters),
                    ))
                // } else if wgsl_types::idents::BUILTIN_CONSTRUCTOR_NAMES.contains(&name.as_str()) {
                //     Some(Lowered::BuiltinConstructor(
                //         name.clone(),
                //         Some(template_parameters),
                //     ))
                } else if let Ok(enum_value) = Enumerant::from_str(name.as_str()) {
                    self.expect_no_template(&template_parameters);
                    Some(Lowered::Enumerant(enum_value))
                } else {
                    None
                }
            },
            Err(error) => {
                self.diagnostics.push(error);
                Some(Lowered::Type(TypeKind::Error.intern(self.db)))
            },
        }
    }

    #[expect(
        clippy::too_many_lines,
        reason = "it is just a big match and each arm is not complex at all"
    )]
    fn lower_predeclared_type(
        &mut self,
        name: &Name,
        template_parameters: &TemplateParameters,
    ) -> Result<Option<Lowered>, TypeLoweringError> {
        let type_kind = match name.as_str() {
            "bool" => {
                self.expect_no_template(template_parameters);
                TypeKind::Scalar(ScalarType::Bool)
            },
            "i32" => {
                self.expect_no_template(template_parameters);
                TypeKind::Scalar(ScalarType::I32)
            },
            "u32" => {
                self.expect_no_template(template_parameters);
                TypeKind::Scalar(ScalarType::U32)
            },
            "i64" if CapabilitiesInput::get_capabilities(self.db).shader_int64 => {
                self.expect_no_template(template_parameters);
                TypeKind::Scalar(ScalarType::I64)
            },
            "u64" if CapabilitiesInput::get_capabilities(self.db).shader_int64 => {
                self.expect_no_template(template_parameters);
                TypeKind::Scalar(ScalarType::U64)
            },
            "f32" => {
                self.expect_no_template(template_parameters);
                TypeKind::Scalar(ScalarType::F32)
            },
            "f16" => {
                self.expect_no_template(template_parameters);
                TypeKind::Scalar(ScalarType::F16)
            },
            "array" => {
                if !template_parameters.has_next() {
                    return Ok(Some(Lowered::TypeWithoutTemplate(
                        TypeKind::Array(ArrayType {
                            inner: TypeKind::Error.intern(self.db),
                            binding_array: false,
                            size: ArraySize::Dynamic,
                        })
                        .intern(self.db),
                    )));
                }
                let array_template = self.array_template(template_parameters)?;
                TypeKind::Array(ArrayType {
                    inner: array_template.r#type,
                    binding_array: false,
                    size: array_template.size,
                })
            },
            "binding_array" => {
                if !template_parameters.has_next() {
                    return Ok(Some(Lowered::TypeWithoutTemplate(
                        TypeKind::Array(ArrayType {
                            inner: TypeKind::Error.intern(self.db),
                            binding_array: true,
                            size: ArraySize::Dynamic,
                        })
                        .intern(self.db),
                    )));
                }
                let array_template = self.array_template(template_parameters)?;
                TypeKind::Array(ArrayType {
                    inner: array_template.r#type,
                    binding_array: true,
                    size: array_template.size,
                })
            },
            "vec2" => {
                if !template_parameters.has_next() {
                    return Ok(Some(Lowered::TypeWithoutTemplate(
                        TypeKind::Vector(VectorType {
                            size: VecSize::Two,
                            component_type: TypeKind::Error.intern(self.db),
                        })
                        .intern(self.db),
                    )));
                }
                let component_type = self.vector_template(template_parameters);
                TypeKind::Vector(VectorType {
                    size: VecSize::Two,
                    component_type,
                })
            },
            "vec3" => {
                if !template_parameters.has_next() {
                    return Ok(Some(Lowered::TypeWithoutTemplate(
                        TypeKind::Vector(VectorType {
                            size: VecSize::Three,
                            component_type: TypeKind::Error.intern(self.db),
                        })
                        .intern(self.db),
                    )));
                }
                let component_type = self.vector_template(template_parameters);
                TypeKind::Vector(VectorType {
                    size: VecSize::Three,
                    component_type,
                })
            },
            "vec4" => {
                if !template_parameters.has_next() {
                    return Ok(Some(Lowered::TypeWithoutTemplate(
                        TypeKind::Vector(VectorType {
                            size: VecSize::Four,
                            component_type: TypeKind::Error.intern(self.db),
                        })
                        .intern(self.db),
                    )));
                }
                let component_type = self.vector_template(template_parameters);
                TypeKind::Vector(VectorType {
                    size: VecSize::Four,
                    component_type,
                })
            },
            // TODO: Move those aliases to a separate file
            // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/559
            "vec2i" => {
                self.expect_no_template(template_parameters);
                TypeKind::Vector(VectorType {
                    size: VecSize::Two,
                    component_type: TypeKind::Scalar(ScalarType::I32).intern(self.db),
                })
            },
            "vec3i" => {
                self.expect_no_template(template_parameters);
                TypeKind::Vector(VectorType {
                    size: VecSize::Three,
                    component_type: TypeKind::Scalar(ScalarType::I32).intern(self.db),
                })
            },
            "vec4i" => {
                self.expect_no_template(template_parameters);
                TypeKind::Vector(VectorType {
                    size: VecSize::Four,
                    component_type: TypeKind::Scalar(ScalarType::I32).intern(self.db),
                })
            },
            "vec2u" => {
                self.expect_no_template(template_parameters);
                TypeKind::Vector(VectorType {
                    size: VecSize::Two,
                    component_type: TypeKind::Scalar(ScalarType::U32).intern(self.db),
                })
            },
            "vec3u" => {
                self.expect_no_template(template_parameters);
                TypeKind::Vector(VectorType {
                    size: VecSize::Three,
                    component_type: TypeKind::Scalar(ScalarType::U32).intern(self.db),
                })
            },
            "vec4u" => {
                self.expect_no_template(template_parameters);
                TypeKind::Vector(VectorType {
                    size: VecSize::Four,
                    component_type: TypeKind::Scalar(ScalarType::U32).intern(self.db),
                })
            },
            "vec2f" => {
                self.expect_no_template(template_parameters);
                TypeKind::Vector(VectorType {
                    size: VecSize::Two,
                    component_type: TypeKind::Scalar(ScalarType::F32).intern(self.db),
                })
            },
            "vec3f" => {
                self.expect_no_template(template_parameters);
                TypeKind::Vector(VectorType {
                    size: VecSize::Three,
                    component_type: TypeKind::Scalar(ScalarType::F32).intern(self.db),
                })
            },
            "vec4f" => {
                self.expect_no_template(template_parameters);
                TypeKind::Vector(VectorType {
                    size: VecSize::Four,
                    component_type: TypeKind::Scalar(ScalarType::F32).intern(self.db),
                })
            },

            "vec2h" => {
                self.expect_no_template(template_parameters);
                TypeKind::Vector(VectorType {
                    size: VecSize::Two,
                    component_type: TypeKind::Scalar(ScalarType::F16).intern(self.db),
                })
            },
            "vec3h" => {
                self.expect_no_template(template_parameters);
                TypeKind::Vector(VectorType {
                    size: VecSize::Three,
                    component_type: TypeKind::Scalar(ScalarType::F16).intern(self.db),
                })
            },
            "vec4h" => {
                self.expect_no_template(template_parameters);
                TypeKind::Vector(VectorType {
                    size: VecSize::Four,
                    component_type: TypeKind::Scalar(ScalarType::F16).intern(self.db),
                })
            },
            name @ ("mat2x2" | "mat2x3" | "mat2x4" | "mat3x2" | "mat3x3" | "mat3x4" | "mat4x2"
            | "mat4x3" | "mat4x4") => {
                let (columns, rows) = match name {
                    "mat2x2" => (VecSize::Two, VecSize::Two),
                    "mat2x3" => (VecSize::Two, VecSize::Three),
                    "mat2x4" => (VecSize::Two, VecSize::Four),

                    "mat3x2" => (VecSize::Three, VecSize::Two),
                    "mat3x3" => (VecSize::Three, VecSize::Three),
                    "mat3x4" => (VecSize::Three, VecSize::Four),

                    "mat4x2" => (VecSize::Four, VecSize::Two),
                    "mat4x3" => (VecSize::Four, VecSize::Three),
                    "mat4x4" => (VecSize::Four, VecSize::Four),
                    #[expect(clippy::unreachable, reason = "no type patterns 😔")]
                    _ => unreachable!(),
                };

                if !template_parameters.has_next() {
                    return Ok(Some(Lowered::TypeWithoutTemplate(
                        TypeKind::Matrix(MatrixType {
                            columns,
                            rows,
                            inner: TypeKind::Error.intern(self.db),
                        })
                        .intern(self.db),
                    )));
                }
                let inner = self.matrix_template(template_parameters);
                TypeKind::Matrix(MatrixType {
                    columns,
                    rows,
                    inner,
                })
            },
            "mat2x2f" => {
                self.expect_no_template(template_parameters);
                TypeKind::Matrix(MatrixType {
                    columns: VecSize::Two,
                    rows: VecSize::Two,
                    inner: TypeKind::Scalar(ScalarType::F32).intern(self.db),
                })
            },
            "mat2x3f" => {
                self.expect_no_template(template_parameters);
                TypeKind::Matrix(MatrixType {
                    columns: VecSize::Two,
                    rows: VecSize::Three,
                    inner: TypeKind::Scalar(ScalarType::F32).intern(self.db),
                })
            },
            "mat2x4f" => {
                self.expect_no_template(template_parameters);
                TypeKind::Matrix(MatrixType {
                    columns: VecSize::Two,
                    rows: VecSize::Four,
                    inner: TypeKind::Scalar(ScalarType::F32).intern(self.db),
                })
            },
            "mat3x2f" => {
                self.expect_no_template(template_parameters);
                TypeKind::Matrix(MatrixType {
                    columns: VecSize::Three,
                    rows: VecSize::Two,
                    inner: TypeKind::Scalar(ScalarType::F32).intern(self.db),
                })
            },
            "mat3x3f" => {
                self.expect_no_template(template_parameters);
                TypeKind::Matrix(MatrixType {
                    columns: VecSize::Three,
                    rows: VecSize::Three,
                    inner: TypeKind::Scalar(ScalarType::F32).intern(self.db),
                })
            },
            "mat3x4f" => {
                self.expect_no_template(template_parameters);
                TypeKind::Matrix(MatrixType {
                    columns: VecSize::Three,
                    rows: VecSize::Four,
                    inner: TypeKind::Scalar(ScalarType::F32).intern(self.db),
                })
            },
            "mat4x2f" => {
                self.expect_no_template(template_parameters);
                TypeKind::Matrix(MatrixType {
                    columns: VecSize::Four,
                    rows: VecSize::Two,
                    inner: TypeKind::Scalar(ScalarType::F32).intern(self.db),
                })
            },
            "mat4x3f" => {
                self.expect_no_template(template_parameters);
                TypeKind::Matrix(MatrixType {
                    columns: VecSize::Four,
                    rows: VecSize::Three,
                    inner: TypeKind::Scalar(ScalarType::F32).intern(self.db),
                })
            },
            "mat4x4f" => {
                self.expect_no_template(template_parameters);
                TypeKind::Matrix(MatrixType {
                    columns: VecSize::Four,
                    rows: VecSize::Four,
                    inner: TypeKind::Scalar(ScalarType::F32).intern(self.db),
                })
            },
            "mat2x2h" => {
                self.expect_no_template(template_parameters);
                TypeKind::Matrix(MatrixType {
                    columns: VecSize::Two,
                    rows: VecSize::Two,
                    inner: TypeKind::Scalar(ScalarType::F16).intern(self.db),
                })
            },
            "mat2x3h" => {
                self.expect_no_template(template_parameters);
                TypeKind::Matrix(MatrixType {
                    columns: VecSize::Two,
                    rows: VecSize::Three,
                    inner: TypeKind::Scalar(ScalarType::F16).intern(self.db),
                })
            },
            "mat2x4h" => {
                self.expect_no_template(template_parameters);
                TypeKind::Matrix(MatrixType {
                    columns: VecSize::Two,
                    rows: VecSize::Four,
                    inner: TypeKind::Scalar(ScalarType::F16).intern(self.db),
                })
            },
            "mat3x2h" => {
                self.expect_no_template(template_parameters);
                TypeKind::Matrix(MatrixType {
                    columns: VecSize::Three,
                    rows: VecSize::Two,
                    inner: TypeKind::Scalar(ScalarType::F16).intern(self.db),
                })
            },
            "mat3x3h" => {
                self.expect_no_template(template_parameters);
                TypeKind::Matrix(MatrixType {
                    columns: VecSize::Three,
                    rows: VecSize::Three,
                    inner: TypeKind::Scalar(ScalarType::F16).intern(self.db),
                })
            },
            "mat3x4h" => {
                self.expect_no_template(template_parameters);
                TypeKind::Matrix(MatrixType {
                    columns: VecSize::Three,
                    rows: VecSize::Four,
                    inner: TypeKind::Scalar(ScalarType::F16).intern(self.db),
                })
            },
            "mat4x2h" => {
                self.expect_no_template(template_parameters);
                TypeKind::Matrix(MatrixType {
                    columns: VecSize::Four,
                    rows: VecSize::Two,
                    inner: TypeKind::Scalar(ScalarType::F16).intern(self.db),
                })
            },
            "mat4x3h" => {
                self.expect_no_template(template_parameters);
                TypeKind::Matrix(MatrixType {
                    columns: VecSize::Four,
                    rows: VecSize::Three,
                    inner: TypeKind::Scalar(ScalarType::F16).intern(self.db),
                })
            },
            "mat4x4h" => {
                self.expect_no_template(template_parameters);
                TypeKind::Matrix(MatrixType {
                    columns: VecSize::Four,
                    rows: VecSize::Four,
                    inner: TypeKind::Scalar(ScalarType::F16).intern(self.db),
                })
            },
            "ptr" => {
                let pointer_template = self.pointer_template(template_parameters)?;
                TypeKind::Pointer(Pointer {
                    address_space: pointer_template.address_space,
                    inner: pointer_template.inner,
                    access_mode: pointer_template.access_mode,
                })
            },
            "atomic" => {
                let inner = self.atomic_template(template_parameters);
                TypeKind::Atomic(AtomicType { inner })
            },
            "texture_1d" => {
                let sampled = self.texture_sampled_template(template_parameters)?;
                TypeKind::Texture(TextureType {
                    kind: TextureKind::from_sampled(sampled, self.db),
                    dimension: TextureDimensionality::D1,
                    arrayed: false,
                    multisampled: false,
                })
            },
            "texture_2d" => {
                let sampled = self.texture_sampled_template(template_parameters)?;
                TypeKind::Texture(TextureType {
                    kind: TextureKind::from_sampled(sampled, self.db),
                    dimension: TextureDimensionality::D2,
                    arrayed: false,
                    multisampled: false,
                })
            },
            "texture_2d_array" => {
                let sampled = self.texture_sampled_template(template_parameters)?;
                TypeKind::Texture(TextureType {
                    kind: TextureKind::from_sampled(sampled, self.db),
                    dimension: TextureDimensionality::D2,
                    arrayed: true,
                    multisampled: false,
                })
            },
            "texture_3d" => {
                let sampled = self.texture_sampled_template(template_parameters)?;
                TypeKind::Texture(TextureType {
                    kind: TextureKind::from_sampled(sampled, self.db),
                    dimension: TextureDimensionality::D3,
                    arrayed: false,
                    multisampled: false,
                })
            },
            "texture_cube" => {
                let sampled = self.texture_sampled_template(template_parameters)?;
                TypeKind::Texture(TextureType {
                    kind: TextureKind::from_sampled(sampled, self.db),
                    dimension: TextureDimensionality::Cube,
                    arrayed: false,
                    multisampled: false,
                })
            },
            "texture_cube_array" => {
                let sampled = self.texture_sampled_template(template_parameters)?;
                TypeKind::Texture(TextureType {
                    kind: TextureKind::from_sampled(sampled, self.db),
                    dimension: TextureDimensionality::Cube,
                    arrayed: true,
                    multisampled: false,
                })
            },
            "texture_multisampled_2d" => {
                let sampled = self.texture_sampled_template(template_parameters)?;
                TypeKind::Texture(TextureType {
                    kind: TextureKind::from_sampled(sampled, self.db),
                    dimension: TextureDimensionality::D2,
                    arrayed: false,
                    multisampled: true,
                })
            },
            "texture_storage_1d" => {
                let storage_template = self.storage_texture_template(template_parameters)?;
                TypeKind::Texture(TextureType {
                    kind: TextureKind::Storage(
                        storage_template.texel_format,
                        storage_template.access_mode,
                    ),
                    dimension: TextureDimensionality::D1,
                    arrayed: false,
                    multisampled: false,
                })
            },
            "texture_storage_2d" => {
                let storage_template = self.storage_texture_template(template_parameters)?;
                TypeKind::Texture(TextureType {
                    kind: TextureKind::Storage(
                        storage_template.texel_format,
                        storage_template.access_mode,
                    ),
                    dimension: TextureDimensionality::D2,
                    arrayed: false,
                    multisampled: false,
                })
            },
            "texture_storage_2d_array" => {
                let storage_template = self.storage_texture_template(template_parameters)?;
                TypeKind::Texture(TextureType {
                    kind: TextureKind::Storage(
                        storage_template.texel_format,
                        storage_template.access_mode,
                    ),
                    dimension: TextureDimensionality::D2,
                    arrayed: true,
                    multisampled: false,
                })
            },
            "texture_storage_3d" => {
                let storage_template = self.storage_texture_template(template_parameters)?;
                TypeKind::Texture(TextureType {
                    kind: TextureKind::Storage(
                        storage_template.texel_format,
                        storage_template.access_mode,
                    ),
                    dimension: TextureDimensionality::D3,
                    arrayed: false,
                    multisampled: false,
                })
            },
            "texture_depth_multisampled_2d" => {
                self.expect_no_template(template_parameters);
                TypeKind::Texture(TextureType {
                    kind: TextureKind::Depth,
                    dimension: TextureDimensionality::D2,
                    arrayed: false,
                    multisampled: true,
                })
            },
            "texture_external" => {
                self.expect_no_template(template_parameters);
                TypeKind::Texture(TextureType {
                    kind: TextureKind::External,
                    dimension: TextureDimensionality::D2,
                    arrayed: false,
                    multisampled: false,
                })
            },
            "texture_depth_2d" => {
                self.expect_no_template(template_parameters);
                TypeKind::Texture(TextureType {
                    kind: TextureKind::Depth,
                    dimension: TextureDimensionality::D2,
                    arrayed: false,
                    multisampled: false,
                })
            },
            "texture_depth_2d_array" => {
                self.expect_no_template(template_parameters);
                TypeKind::Texture(TextureType {
                    kind: TextureKind::Depth,
                    dimension: TextureDimensionality::D2,
                    arrayed: true,
                    multisampled: false,
                })
            },
            "texture_depth_cube" => {
                self.expect_no_template(template_parameters);
                TypeKind::Texture(TextureType {
                    kind: TextureKind::Depth,
                    dimension: TextureDimensionality::Cube,
                    arrayed: false,
                    multisampled: false,
                })
            },
            "texture_depth_cube_array" => {
                self.expect_no_template(template_parameters);
                TypeKind::Texture(TextureType {
                    kind: TextureKind::Depth,
                    dimension: TextureDimensionality::Cube,
                    arrayed: true,
                    multisampled: false,
                })
            },
            "sampler" => {
                self.expect_no_template(template_parameters);
                TypeKind::Sampler(wgsl_types::ty::SamplerType::Sampler)
            },
            "sampler_comparison" => {
                self.expect_no_template(template_parameters);
                TypeKind::Sampler(wgsl_types::ty::SamplerType::SamplerComparison)
            },
            _ => {
                return Ok(None);
            },
        };
        Ok(Some(Lowered::Type(type_kind.intern(self.db))))
    }

    fn array_template(
        &mut self,
        template_parameters: &TemplateParameters,
    ) -> Result<ArrayTemplate, TypeLoweringError> {
        self.expect_n_templates(template_parameters, 1..=2);
        let mut template_parameters = template_parameters.clone();
        let r#type = match template_parameters.next_as_type() {
            Ok((r#type, _)) => r#type,
            Err(error) => {
                self.diagnostics.push(error);
                TypeKind::Error.intern(self.db)
            },
        };

        let size = if template_parameters.has_next() {
            match template_parameters.next_as_instance() {
                Ok((Some(Instance::Literal(LiteralInstance::I32(number))), _))
                    if let Ok(validated) = u32::try_from(number).and_then(NonZeroU32::try_from) =>
                {
                    ArraySize::Constant(validated)
                },
                Ok((Some(Instance::Literal(LiteralInstance::U32(number))), _))
                    if let Ok(validated) = NonZeroU32::try_from(number) =>
                {
                    ArraySize::Constant(validated)
                },
                Ok((
                    Some(Instance::Literal(
                        LiteralInstance::AbstractInt(number) | LiteralInstance::I64(number),
                    )),
                    _,
                )) if let Ok(validated) = u32::try_from(number).and_then(NonZeroU32::try_from) => {
                    // skips handling array<E, 1li>() or array<E, 99999999999999999999999999>()
                    ArraySize::Constant(validated)
                },
                Ok((Some(Instance::Literal(LiteralInstance::U64(number))), _))
                    if let Ok(validated) = u32::try_from(number).and_then(NonZeroU32::try_from) =>
                {
                    // skips handling array<E, 1uL>() or array<E, 99999999999999999999999999uL>()
                    ArraySize::Constant(validated)
                },
                Ok((instance, expression)) => {
                    let error = TypeLoweringError {
                        container: TypeContainer::Expression(expression),
                        kind: TypeLoweringErrorKind::UnexpectedTemplateArgument(
                            "a `u32` or a `i32` greater than `0`".to_owned(),
                            instance.into(),
                        ),
                    };
                    return Err(error);
                },
                Err(error) => {
                    return Err(error);
                },
            }
        } else {
            ArraySize::Dynamic
        };

        Ok(ArrayTemplate { r#type, size })
    }

    fn vector_template(
        &mut self,
        template_parameters: &TemplateParameters,
    ) -> Type {
        self.expect_n_templates(template_parameters, 1..=1);
        let mut template_parameters = template_parameters.clone();

        match template_parameters.next_as_type() {
            Ok((r#type, expression)) => {
                let type_kind = r#type.kind(self.db);
                if matches!(type_kind, TypeKind::Scalar(_)) && !type_kind.is_abstract(self.db) {
                    r#type
                } else {
                    self.diagnostics.push(TypeLoweringError {
                        container: TypeContainer::Expression(expression),
                        kind: TypeLoweringErrorKind::UnexpectedTemplateArgument(
                            "a scalar".to_owned(),
                            r#type.into(),
                        ),
                    });
                    TypeKind::Error.intern(self.db)
                }
            },
            Err(error) => {
                self.diagnostics.push(error);
                TypeKind::Error.intern(self.db)
            },
        }
    }

    fn matrix_template(
        &mut self,
        template_parameters: &TemplateParameters,
    ) -> Type {
        self.expect_n_templates(template_parameters, 1..=1);
        let mut template_parameters = template_parameters.clone();

        match template_parameters.next_as_type() {
            Ok((r#type, expression)) => {
                let type_kind = r#type.kind(self.db);
                if matches!(
                    type_kind,
                    TypeKind::Scalar(ScalarType::F16 | ScalarType::F32)
                ) {
                    r#type
                } else {
                    self.diagnostics.push(TypeLoweringError {
                        container: TypeContainer::Expression(expression),
                        kind: TypeLoweringErrorKind::UnexpectedTemplateArgument(
                            "one of: f32 or f16".to_owned(),
                            r#type.into(),
                        ),
                    });
                    TypeKind::Error.intern(self.db)
                }
            },
            Err(error) => {
                self.diagnostics.push(error);
                TypeKind::Error.intern(self.db)
            },
        }
    }

    fn pointer_template(
        &mut self,
        template_parameters: &TemplateParameters,
    ) -> Result<PointerTemplate, TypeLoweringError> {
        self.expect_n_templates(template_parameters, 2..=3);
        let mut template_parameters = template_parameters.clone();
        let address_space = match template_parameters.next_as_enumerant() {
            Ok((Enumerant::AddressSpace(address_space), _)) => address_space,
            Ok((enumerant, expression)) => {
                let error = TypeLoweringError {
                    container: TypeContainer::Expression(expression),
                    kind: TypeLoweringErrorKind::UnexpectedTemplateArgument(
                        "an address space".to_owned(),
                        enumerant.into(),
                    ),
                };
                return Err(error);
            },
            Err(error) => {
                return Err(error);
            },
        };
        let inner = match template_parameters.next_as_type() {
            Ok((inner, _)) if inner.kind(self.db).is_storable() => inner,
            Ok((non_storable, expression)) => {
                self.diagnostics.push(TypeLoweringError {
                    container: TypeContainer::Expression(expression),
                    kind: TypeLoweringErrorKind::UnexpectedTemplateArgument(
                        "a storable type".to_owned(),
                        non_storable.into(),
                    ),
                });
                TypeKind::Error.intern(self.db)
            },
            Err(error) => {
                self.diagnostics.push(error);
                TypeKind::Error.intern(self.db)
            },
        };

        let access_mode = if template_parameters.has_next() {
            match template_parameters.next_as_enumerant() {
                // uniform address space requires the read access mode
                Ok((
                    enumerant
                    @ Enumerant::AccessMode(AccessMode::ReadWrite | AccessMode::ReadWrite),
                    expression,
                )) if address_space == AddressSpace::Uniform => {
                    self.diagnostics.push(TypeLoweringError {
                        container: TypeContainer::Expression(expression),
                        kind: TypeLoweringErrorKind::UnexpectedTemplateArgument(
                            "`read` access mode for uniforms".to_owned(),
                            enumerant.into(),
                        ),
                    });
                    AccessMode::Read
                },
                // everything else has no such constraints
                Ok((Enumerant::AccessMode(access_mode), _)) => access_mode,
                Ok((enumerant, expression)) => {
                    let error = TypeLoweringError {
                        container: TypeContainer::Expression(expression),
                        kind: TypeLoweringErrorKind::UnexpectedTemplateArgument(
                            "one of: (read, read_write, write)".to_owned(),
                            enumerant.into(),
                        ),
                    };
                    return Err(error);
                },
                Err(error) => {
                    return Err(error);
                },
            }
        } else {
            address_space.default_access_mode()
        };

        Ok(PointerTemplate {
            address_space,
            inner,
            access_mode,
        })
    }

    fn atomic_template(
        &mut self,
        template_parameters: &TemplateParameters,
    ) -> Type {
        self.expect_n_templates(template_parameters, 1..=1);
        let mut template_parameters = template_parameters.clone();

        match template_parameters.next_as_type() {
            Ok((r#type, expression)) => {
                let type_kind = r#type.kind(self.db);
                if matches!(
                    type_kind,
                    TypeKind::Scalar(
                        ScalarType::I32 | ScalarType::U32 | ScalarType::I64 | ScalarType::U64
                    )
                ) {
                    r#type
                } else {
                    // TODO: improve the error message and support naga atomics
                    // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/677
                    // Naga supports more types (f32, i64, u64) here
                    let possible_types =
                        if CapabilitiesInput::get_capabilities(self.db).shader_int64 {
                            "i32, u32, i64, or u64".to_owned()
                        } else {
                            "i32 or u32".to_owned()
                        };
                    self.diagnostics.push(TypeLoweringError {
                        container: TypeContainer::Expression(expression),
                        kind: TypeLoweringErrorKind::UnexpectedTemplateArgument(
                            possible_types,
                            r#type.into(),
                        ),
                    });
                    TypeKind::Error.intern(self.db)
                }
            },
            Err(error) => {
                self.diagnostics.push(error);
                TypeKind::Error.intern(self.db)
            },
        }
    }

    fn texture_sampled_template(
        &mut self,
        template_parameters: &TemplateParameters,
    ) -> Result<SampledType, TypeLoweringError> {
        self.expect_n_templates(template_parameters, 1..=1);
        let mut template_parameters = template_parameters.clone();

        match template_parameters.next_as_type() {
            Ok((r#type, expression)) => {
                let type_kind = r#type.kind(self.db);
                match type_kind {
                    TypeKind::Scalar(ScalarType::I32) => Ok(SampledType::I32),
                    TypeKind::Scalar(ScalarType::U32) => Ok(SampledType::U32),
                    TypeKind::Scalar(ScalarType::F32) => Ok(SampledType::F32),
                    TypeKind::Error
                    | TypeKind::Scalar(_)
                    | TypeKind::Atomic(_)
                    | TypeKind::Vector(_)
                    | TypeKind::Matrix(_)
                    | TypeKind::Struct(_)
                    | TypeKind::BuiltinStruct(_)
                    | TypeKind::Array(_)
                    | TypeKind::Texture(_)
                    | TypeKind::Sampler(_)
                    | TypeKind::Reference(_)
                    | TypeKind::Pointer(_) => {
                        // texture_2d<invalid>()
                        let error = TypeLoweringError {
                            container: TypeContainer::Expression(expression),
                            kind: TypeLoweringErrorKind::UnexpectedTemplateArgument(
                                "i32 or u32 or f32".to_owned(),
                                r#type.into(),
                            ),
                        };
                        Err(error)
                    },
                }
            },
            Err(error) => Err(error),
        }
    }

    fn storage_texture_template(
        &mut self,
        template_parameters: &TemplateParameters,
    ) -> Result<StorageTextureTemplate, TypeLoweringError> {
        self.expect_n_templates(template_parameters, 1..=2);
        let mut template_parameters = template_parameters.clone();
        let texel_format = match template_parameters.next_as_enumerant() {
            Ok((Enumerant::TexelFormat(texel_format), _)) => texel_format,
            Ok((enumerant, expression)) => {
                let error = TypeLoweringError {
                    container: TypeContainer::Expression(expression),
                    kind: TypeLoweringErrorKind::UnexpectedTemplateArgument(
                        "a texel format (`rgba8unorm`, `rgba8snorm`, ...)".to_owned(),
                        enumerant.into(),
                    ),
                };
                return Err(error);
            },
            Err(error) => {
                return Err(error);
            },
        };
        let access_mode = match template_parameters.next_as_enumerant() {
            Ok((Enumerant::AccessMode(access_mode), _)) => access_mode,
            Ok((enumerant, expression)) => {
                let error = TypeLoweringError {
                    container: TypeContainer::Expression(expression),
                    kind: TypeLoweringErrorKind::UnexpectedTemplateArgument(
                        "one of: read, write, read_write".to_owned(),
                        enumerant.into(),
                    ),
                };
                return Err(error);
            },
            Err(error) => {
                return Err(error);
            },
        };
        Ok(StorageTextureTemplate {
            texel_format,
            access_mode,
        })
    }
}

struct ArrayTemplate {
    r#type: Type,
    size: ArraySize,
}

struct PointerTemplate {
    address_space: AddressSpace,
    inner: Type,
    access_mode: AccessMode,
}

struct StorageTextureTemplate {
    texel_format: TexelFormat,
    access_mode: AccessMode,
}
