use std::{num::NonZeroU32, str::FromStr};

use base_db::{CapabilitiesInput, Intern as _};
use either::Either;
use hir_def::{item_tree::Name, resolver::ResolutionDiagnostic};
use wgsl_types::{
    Instance,
    inst::LiteralInstance,
    syntax::{AccessMode, AddressSpace, Enumerant, SampledType, TexelFormat},
};

use crate::{
    lower::{
        ConstructibleTypeGenerator, Lowered, TypeContainer, TypeLoweringContext, TypeLoweringError,
        TypeLoweringErrorKind, UnexpectedTemplateArgumentValue, generics::TemplateParameters,
    },
    ty::{
        ArraySize, ArrayType, AtomicType, MatrixType, Pointer, ScalarType, TextureDimensionality,
        TextureKind, TextureType, Type, TypeKind, VecSize, VectorType,
    },
};

impl TypeLoweringContext<'_> {
    #[expect(
        clippy::too_many_lines,
        reason = "it is just a big match and each arm is not complex at all"
    )]
    pub fn lower_builtin_type(
        &mut self,
        name: Name,
        type_container: TypeContainer,
        template_parameters: &TemplateParameters,
    ) -> Result<Lowered, TypeLoweringError> {
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
            "acceleration_structure" => {
                if template_parameters.has_next() {
                    let mut template_parameters = template_parameters.clone();
                    match template_parameters.next_as_enumerant() {
                        Ok((Enumerant::AccelerationStructureFlags(tags), _)) => {
                            TypeKind::AccelerationStructure(Some(tags))
                        },
                        Ok((other, expression)) => {
                            return Err(TypeLoweringError {
                                container: TypeContainer::Expression(expression),
                                kind: TypeLoweringErrorKind::UnexpectedTemplateArgument(
                                    "an acceleration structure tag".to_owned(),
                                    UnexpectedTemplateArgumentValue::from(other),
                                ),
                            });
                        },
                        Err(error) => return Err(error),
                    }
                } else {
                    TypeKind::AccelerationStructure(None)
                }
            },
            _ => {
                return Err(TypeLoweringError {
                    container: type_container,
                    kind: TypeLoweringErrorKind::Resolution(ResolutionDiagnostic::UnresolvedName {
                        name,
                    }),
                });
            },
        };
        Ok(Lowered::Type(type_kind.intern(self.db)))
    }

    #[expect(
        clippy::too_many_lines,
        reason = "it is just a big match and each arm is not complex at all"
    )]
    pub fn lower_builtin_type_generator(
        &mut self,
        type_container: TypeContainer,
        name: &Name,
        template_parameters: &TemplateParameters,
    ) -> Result<Either<ConstructibleTypeGenerator, Type>, TypeLoweringError> {
        match name.as_str() {
            "array" => {
                if !template_parameters.has_next() {
                    return Ok(Either::Left(ConstructibleTypeGenerator::Array(ArrayType {
                        inner: TypeKind::Error.intern(self.db),
                        binding_array: false,
                        size: ArraySize::Dynamic,
                    })));
                }
                let array_template = self.array_template(template_parameters)?;
                Ok(Either::Right(
                    TypeKind::Array(ArrayType {
                        inner: array_template.r#type,
                        binding_array: false,
                        size: array_template.size,
                    })
                    .intern(self.db),
                ))
            },
            "binding_array" => {
                if !template_parameters.has_next() {
                    return Ok(Either::Left(ConstructibleTypeGenerator::Array(ArrayType {
                        inner: TypeKind::Error.intern(self.db),
                        binding_array: true,
                        size: ArraySize::Dynamic,
                    })));
                }
                let array_template = self.array_template(template_parameters)?;
                Ok(Either::Right(
                    TypeKind::Array(ArrayType {
                        inner: array_template.r#type,
                        binding_array: true,
                        size: array_template.size,
                    })
                    .intern(self.db),
                ))
            },
            "vec2" => {
                if !template_parameters.has_next() {
                    return Ok(Either::Left(ConstructibleTypeGenerator::Vector(
                        VectorType {
                            size: VecSize::Two,
                            component_type: TypeKind::Error.intern(self.db),
                        },
                    )));
                }
                let component_type = self.vector_template(template_parameters);
                Ok(Either::Right(
                    TypeKind::Vector(VectorType {
                        size: VecSize::Two,
                        component_type,
                    })
                    .intern(self.db),
                ))
            },
            "vec3" => {
                if !template_parameters.has_next() {
                    return Ok(Either::Left(ConstructibleTypeGenerator::Vector(
                        VectorType {
                            size: VecSize::Three,
                            component_type: TypeKind::Error.intern(self.db),
                        },
                    )));
                }
                let component_type = self.vector_template(template_parameters);
                Ok(Either::Right(
                    TypeKind::Vector(VectorType {
                        size: VecSize::Three,
                        component_type,
                    })
                    .intern(self.db),
                ))
            },
            "vec4" => {
                if !template_parameters.has_next() {
                    return Ok(Either::Left(ConstructibleTypeGenerator::Vector(
                        VectorType {
                            size: VecSize::Four,
                            component_type: TypeKind::Error.intern(self.db),
                        },
                    )));
                }
                let component_type = self.vector_template(template_parameters);
                Ok(Either::Right(
                    TypeKind::Vector(VectorType {
                        size: VecSize::Four,
                        component_type,
                    })
                    .intern(self.db),
                ))
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
                    return Ok(Either::Left(ConstructibleTypeGenerator::Matrix(
                        MatrixType {
                            columns,
                            rows,
                            inner: TypeKind::Error.intern(self.db),
                        },
                    )));
                }
                let inner = self.matrix_template(template_parameters);
                Ok(Either::Right(
                    TypeKind::Matrix(MatrixType {
                        columns,
                        rows,
                        inner,
                    })
                    .intern(self.db),
                ))
            },

            // not constructible
            "ptr" => {
                let pointer_template = self.pointer_template(template_parameters)?;
                Ok(Either::Right(
                    TypeKind::Pointer(Pointer {
                        address_space: pointer_template.address_space,
                        inner: pointer_template.inner,
                        access_mode: pointer_template.access_mode,
                    })
                    .intern(self.db),
                ))
            },
            "atomic" => {
                let inner = self.atomic_template(template_parameters);
                Ok(Either::Right(
                    TypeKind::Atomic(AtomicType { inner }).intern(self.db),
                ))
            },
            "texture_1d" => {
                let sampled = self.texture_sampled_template(template_parameters)?;
                Ok(Either::Right(
                    TypeKind::Texture(TextureType {
                        kind: TextureKind::from_sampled(sampled, self.db),
                        dimension: TextureDimensionality::D1,
                        arrayed: false,
                        multisampled: false,
                    })
                    .intern(self.db),
                ))
            },
            "texture_2d" => {
                let sampled = self.texture_sampled_template(template_parameters)?;
                Ok(Either::Right(
                    TypeKind::Texture(TextureType {
                        kind: TextureKind::from_sampled(sampled, self.db),
                        dimension: TextureDimensionality::D2,
                        arrayed: false,
                        multisampled: false,
                    })
                    .intern(self.db),
                ))
            },
            "texture_2d_array" => {
                let sampled = self.texture_sampled_template(template_parameters)?;
                Ok(Either::Right(
                    TypeKind::Texture(TextureType {
                        kind: TextureKind::from_sampled(sampled, self.db),
                        dimension: TextureDimensionality::D2,
                        arrayed: true,
                        multisampled: false,
                    })
                    .intern(self.db),
                ))
            },
            "texture_3d" => {
                let sampled = self.texture_sampled_template(template_parameters)?;
                Ok(Either::Right(
                    TypeKind::Texture(TextureType {
                        kind: TextureKind::from_sampled(sampled, self.db),
                        dimension: TextureDimensionality::D3,
                        arrayed: false,
                        multisampled: false,
                    })
                    .intern(self.db),
                ))
            },
            "texture_cube" => {
                let sampled = self.texture_sampled_template(template_parameters)?;
                Ok(Either::Right(
                    TypeKind::Texture(TextureType {
                        kind: TextureKind::from_sampled(sampled, self.db),
                        dimension: TextureDimensionality::Cube,
                        arrayed: false,
                        multisampled: false,
                    })
                    .intern(self.db),
                ))
            },
            "texture_cube_array" => {
                let sampled = self.texture_sampled_template(template_parameters)?;
                Ok(Either::Right(
                    TypeKind::Texture(TextureType {
                        kind: TextureKind::from_sampled(sampled, self.db),
                        dimension: TextureDimensionality::Cube,
                        arrayed: true,
                        multisampled: false,
                    })
                    .intern(self.db),
                ))
            },
            "texture_multisampled_2d" => {
                let sampled = self.texture_sampled_template(template_parameters)?;
                Ok(Either::Right(
                    TypeKind::Texture(TextureType {
                        kind: TextureKind::from_sampled(sampled, self.db),
                        dimension: TextureDimensionality::D2,
                        arrayed: false,
                        multisampled: true,
                    })
                    .intern(self.db),
                ))
            },
            "texture_storage_1d" => {
                let storage_template = self.storage_texture_template(template_parameters)?;
                Ok(Either::Right(
                    TypeKind::Texture(TextureType {
                        kind: TextureKind::Storage(
                            storage_template.texel_format,
                            storage_template.access_mode,
                        ),
                        dimension: TextureDimensionality::D1,
                        arrayed: false,
                        multisampled: false,
                    })
                    .intern(self.db),
                ))
            },
            "texture_storage_2d" => {
                let storage_template = self.storage_texture_template(template_parameters)?;
                Ok(Either::Right(
                    TypeKind::Texture(TextureType {
                        kind: TextureKind::Storage(
                            storage_template.texel_format,
                            storage_template.access_mode,
                        ),
                        dimension: TextureDimensionality::D2,
                        arrayed: false,
                        multisampled: false,
                    })
                    .intern(self.db),
                ))
            },
            "texture_storage_2d_array" => {
                let storage_template = self.storage_texture_template(template_parameters)?;
                Ok(Either::Right(
                    TypeKind::Texture(TextureType {
                        kind: TextureKind::Storage(
                            storage_template.texel_format,
                            storage_template.access_mode,
                        ),
                        dimension: TextureDimensionality::D2,
                        arrayed: true,
                        multisampled: false,
                    })
                    .intern(self.db),
                ))
            },
            "texture_storage_3d" => {
                let storage_template = self.storage_texture_template(template_parameters)?;
                Ok(Either::Right(
                    TypeKind::Texture(TextureType {
                        kind: TextureKind::Storage(
                            storage_template.texel_format,
                            storage_template.access_mode,
                        ),
                        dimension: TextureDimensionality::D3,
                        arrayed: false,
                        multisampled: false,
                    })
                    .intern(self.db),
                ))
            },
            // "texture_1d_array" => {
            //     unimplemented!()
            // },
            // "texture_storage_1d_array" => {
            //     unimplemented!()
            // },
            // "texture_multisampled_2d_array" => {
            //     unimplemented!()
            // },
            _ => {
                debug_assert!(
                    false,
                    "reaching this means that this function needs to be updated to handle a new builtin defined in wgsl-types"
                );
                Err(TypeLoweringError {
                    container: type_container,
                    kind: TypeLoweringErrorKind::Resolution(ResolutionDiagnostic::UnresolvedName {
                        name: name.clone(),
                    }),
                })
            },
        }
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
                    | TypeKind::AccelerationStructure(_)
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

    #[expect(clippy::unused_self, reason = "intended API")]
    pub fn lower_builtin_enumerant(
        &self,
        name: &Name,
    ) -> Result<Lowered, <Enumerant as FromStr>::Err> {
        match Enumerant::from_str(name.as_str()) {
            Ok(enumerant) => Ok(Lowered::Enumerant(enumerant)),
            Err(()) => Err(()),
        }
    }

    #[expect(
        clippy::match_same_arms,
        reason = "better to write it this way because they are unrelated"
    )]
    #[expect(clippy::unused_self, reason = "intended API")]
    pub fn lower_builtin_declaration(
        &self,
        type_container: TypeContainer,
        name: Name,
    ) -> Result<Lowered, TypeLoweringError> {
        let literal_instance = match name.as_str() {
            "RAY_FLAG_NONE" => LiteralInstance::U32(0x0),
            "RAY_FLAG_FORCE_OPAQUE" => LiteralInstance::U32(0x1),
            "RAY_FLAG_FORCE_NO_OPAQUE" => LiteralInstance::U32(0x2),
            "RAY_FLAG_TERMINATE_ON_FIRST_HIT" => LiteralInstance::U32(0x4),
            "RAY_FLAG_SKIP_CLOSEST_HIT_SHADER" => LiteralInstance::U32(0x8),
            "RAY_FLAG_CULL_BACK_FACING" => LiteralInstance::U32(0x10),
            "RAY_FLAG_CULL_FRONT_FACING" => LiteralInstance::U32(0x20),
            "RAY_FLAG_CULL_OPAQUE" => LiteralInstance::U32(0x40),
            "RAY_FLAG_CULL_NO_OPAQUE" => LiteralInstance::U32(0x80),
            "RAY_FLAG_SKIP_TRIANGLES" => LiteralInstance::U32(0x100),
            "RAY_FLAG_SKIP_AABBS" => LiteralInstance::U32(0x200),
            "RAY_QUERY_INTERSECTION_NONE" => LiteralInstance::U32(0),
            "RAY_QUERY_INTERSECTION_TRIANGLE" => LiteralInstance::U32(1),
            "RAY_QUERY_INTERSECTION_GENERATED" => LiteralInstance::U32(2),
            "RAY_QUERY_INTERSECTION_AABB" => LiteralInstance::U32(3),
            _ => {
                return Err(TypeLoweringError {
                    container: type_container,
                    kind: TypeLoweringErrorKind::Resolution(ResolutionDiagnostic::UnresolvedName {
                        name,
                    }),
                });
            },
        };
        Ok(Lowered::BuiltinDeclaration(
            name,
            Instance::Literal(literal_instance),
        ))
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
