use std::str::FromStr as _;

use hir_def::{expression::ExpressionId, module_data::Name};
use wgsl_types::{
    Instance,
    inst::LiteralInstance,
    syntax::{AccessMode, AddressSpace, Enumerant, SampledType, TexelFormat},
};

use crate::{
    infer::{
        Lowered, TyLoweringContext, TypeContainer, TypeLoweringError, TypeLoweringErrorKind,
        WgslTypeConverter,
        eval::{TemplateParameter, TemplateParameters},
        from_wgsl_texel_format,
    },
    ty::{
        ArraySize, ArrayType, AtomicType, MatrixType, Pointer, ScalarType, TextureDimensionality,
        TextureKind, TextureType, TyKind, Type, VecSize, VectorType,
    },
};

impl TyLoweringContext<'_> {
    fn is_predeclared_ty(name: &Name) -> bool {
        matches!(
            name.as_str(),
            "bool"
                | "i32"
                | "u32"
                | "f32"
                | "f16"
                | "array"
                | "binding_array"
                | "vec2"
                | "vec3"
                | "vec4"
                | "vec2i"
                | "vec3i"
                | "vec4i"
                | "vec2u"
                | "vec3u"
                | "vec4u"
                | "vec2f"
                | "vec3f"
                | "vec4f"
                | "vec2h"
                | "vec3h"
                | "vec4h"
                | "mat2x2"
                | "mat2x3"
                | "mat2x4"
                | "mat3x2"
                | "mat3x3"
                | "mat3x4"
                | "mat4x2"
                | "mat4x3"
                | "mat4x4"
                | "mat2x2f"
                | "mat2x3f"
                | "mat2x4f"
                | "mat3x2f"
                | "mat3x3f"
                | "mat3x4f"
                | "mat4x2f"
                | "mat4x3f"
                | "mat4x4f"
                | "mat2x2h"
                | "mat2x3h"
                | "mat2x4h"
                | "mat3x2h"
                | "mat3x3h"
                | "mat3x4h"
                | "mat4x2h"
                | "mat4x3h"
                | "mat4x4h"
                | "ptr"
                | "atomic"
                | "texture_1d"
                | "texture_2d"
                | "texture_2d_array"
                | "texture_3d"
                | "texture_cube"
                | "texture_cube_array"
                | "texture_multisampled_2d"
                | "texture_storage_1d"
                | "texture_storage_2d"
                | "texture_storage_2d_array"
                | "texture_storage_3d"
                | "texture_depth_multisampled_2d"
                | "texture_external"
                | "texture_depth_2d"
                | "texture_depth_2d_array"
                | "texture_depth_cube"
                | "texture_depth_cube_array"
                | "sampler"
                | "sampler_comparison"
        )
    }

    pub fn lower_predeclared(
        &mut self,
        type_container: TypeContainer,
        path: &Name,
        template_parameters: &[ExpressionId],
    ) -> Result<Lowered, TypeLoweringError> {
        // Lower predeclared types
        if Self::is_predeclared_ty(path) {
            self.lower_predeclared_ty(type_container, path, template_parameters)
        } else if crate::builtins::Builtin::ALL_BUILTINS.contains(&path.as_str()) {
            Ok(Lowered::BuiltinFunction)
        } else if let Ok(enum_value) = Enumerant::from_str(path.as_str()) {
            self.expect_no_template(template_parameters);
            Ok(Lowered::Enumerant(enum_value))
        } else {
            self.diagnostics.push(TypeLoweringError {
                container: type_container,
                kind: TypeLoweringErrorKind::UnresolvedName(path.clone()),
            });
            Ok(Lowered::Type(TyKind::Error.intern(self.database)))
        }
    }

    #[expect(
        clippy::too_many_lines,
        reason = "it is just a big match and each arm is not complex at all"
    )]
    fn lower_predeclared_ty(
        &mut self,
        type_container: TypeContainer,
        path: &Name,
        template_parameters: &[ExpressionId],
    ) -> Result<Lowered, TypeLoweringError> {
        let evaluated_parameters = self.eval_template_args(type_container, template_parameters);

        let ty_kind = match path.as_str() {
            "bool" => {
                self.expect_no_template(template_parameters);
                TyKind::Scalar(ScalarType::Bool)
            },
            "i32" => {
                self.expect_no_template(template_parameters);
                TyKind::Scalar(ScalarType::I32)
            },
            "u32" => {
                self.expect_no_template(template_parameters);
                TyKind::Scalar(ScalarType::U32)
            },
            "f32" => {
                self.expect_no_template(template_parameters);
                TyKind::Scalar(ScalarType::F32)
            },
            "f16" => {
                self.expect_no_template(template_parameters);
                TyKind::Scalar(ScalarType::F16)
            },
            "array" => {
                if template_parameters.is_empty() {
                    return Ok(Lowered::TypeWithoutTemplate(
                        TyKind::Array(ArrayType {
                            inner: TyKind::Error.intern(self.database),
                            binding_array: false,
                            size: ArraySize::Dynamic,
                        })
                        .intern(self.database),
                    ));
                }
                let array_template = self.array_template(evaluated_parameters)?;
                TyKind::Array(ArrayType {
                    inner: array_template.r#type,
                    binding_array: false,
                    size: array_template.size,
                })
            },
            "binding_array" => {
                if template_parameters.is_empty() {
                    return Ok(Lowered::TypeWithoutTemplate(
                        TyKind::Array(ArrayType {
                            inner: TyKind::Error.intern(self.database),
                            binding_array: true,
                            size: ArraySize::Dynamic,
                        })
                        .intern(self.database),
                    ));
                }
                let array_template = self.array_template(evaluated_parameters)?;
                TyKind::Array(ArrayType {
                    inner: array_template.r#type,
                    binding_array: true,
                    size: array_template.size,
                })
            },
            "vec2" => {
                if template_parameters.is_empty() {
                    return Ok(Lowered::TypeWithoutTemplate(
                        TyKind::Vector(VectorType {
                            size: VecSize::Two,
                            component_type: TyKind::Error.intern(self.database),
                        })
                        .intern(self.database),
                    ));
                }
                let component_type = self.vector_template(evaluated_parameters);
                TyKind::Vector(VectorType {
                    size: VecSize::Two,
                    component_type,
                })
            },
            "vec3" => {
                if template_parameters.is_empty() {
                    return Ok(Lowered::TypeWithoutTemplate(
                        TyKind::Vector(VectorType {
                            size: VecSize::Three,
                            component_type: TyKind::Error.intern(self.database),
                        })
                        .intern(self.database),
                    ));
                }
                let component_type = self.vector_template(evaluated_parameters);
                TyKind::Vector(VectorType {
                    size: VecSize::Three,
                    component_type,
                })
            },
            "vec4" => {
                if template_parameters.is_empty() {
                    return Ok(Lowered::TypeWithoutTemplate(
                        TyKind::Vector(VectorType {
                            size: VecSize::Four,
                            component_type: TyKind::Error.intern(self.database),
                        })
                        .intern(self.database),
                    ));
                }
                let component_type = self.vector_template(evaluated_parameters);
                TyKind::Vector(VectorType {
                    size: VecSize::Four,
                    component_type,
                })
            },
            // TODO: Move those aliases to a separate file
            // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/559
            "vec2i" => {
                self.expect_no_template(template_parameters);
                TyKind::Vector(VectorType {
                    size: VecSize::Two,
                    component_type: TyKind::Scalar(ScalarType::I32).intern(self.database),
                })
            },
            "vec3i" => {
                self.expect_no_template(template_parameters);
                TyKind::Vector(VectorType {
                    size: VecSize::Three,
                    component_type: TyKind::Scalar(ScalarType::I32).intern(self.database),
                })
            },
            "vec4i" => {
                self.expect_no_template(template_parameters);
                TyKind::Vector(VectorType {
                    size: VecSize::Four,
                    component_type: TyKind::Scalar(ScalarType::I32).intern(self.database),
                })
            },
            "vec2u" => {
                self.expect_no_template(template_parameters);
                TyKind::Vector(VectorType {
                    size: VecSize::Two,
                    component_type: TyKind::Scalar(ScalarType::U32).intern(self.database),
                })
            },
            "vec3u" => {
                self.expect_no_template(template_parameters);
                TyKind::Vector(VectorType {
                    size: VecSize::Three,
                    component_type: TyKind::Scalar(ScalarType::U32).intern(self.database),
                })
            },
            "vec4u" => {
                self.expect_no_template(template_parameters);
                TyKind::Vector(VectorType {
                    size: VecSize::Four,
                    component_type: TyKind::Scalar(ScalarType::U32).intern(self.database),
                })
            },
            "vec2f" => {
                self.expect_no_template(template_parameters);
                TyKind::Vector(VectorType {
                    size: VecSize::Two,
                    component_type: TyKind::Scalar(ScalarType::F32).intern(self.database),
                })
            },
            "vec3f" => {
                self.expect_no_template(template_parameters);
                TyKind::Vector(VectorType {
                    size: VecSize::Three,
                    component_type: TyKind::Scalar(ScalarType::F32).intern(self.database),
                })
            },
            "vec4f" => {
                self.expect_no_template(template_parameters);
                TyKind::Vector(VectorType {
                    size: VecSize::Four,
                    component_type: TyKind::Scalar(ScalarType::F32).intern(self.database),
                })
            },

            "vec2h" => {
                self.expect_no_template(template_parameters);
                TyKind::Vector(VectorType {
                    size: VecSize::Two,
                    component_type: TyKind::Scalar(ScalarType::F16).intern(self.database),
                })
            },
            "vec3h" => {
                self.expect_no_template(template_parameters);
                TyKind::Vector(VectorType {
                    size: VecSize::Three,
                    component_type: TyKind::Scalar(ScalarType::F16).intern(self.database),
                })
            },
            "vec4h" => {
                self.expect_no_template(template_parameters);
                TyKind::Vector(VectorType {
                    size: VecSize::Four,
                    component_type: TyKind::Scalar(ScalarType::F16).intern(self.database),
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

                if template_parameters.is_empty() {
                    return Ok(Lowered::TypeWithoutTemplate(
                        TyKind::Matrix(MatrixType {
                            columns,
                            rows,
                            inner: TyKind::Error.intern(self.database),
                        })
                        .intern(self.database),
                    ));
                }
                let inner = self.matrix_template(evaluated_parameters);
                TyKind::Matrix(MatrixType {
                    columns,
                    rows,
                    inner,
                })
            },
            "mat2x2f" => {
                self.expect_no_template(template_parameters);
                TyKind::Matrix(MatrixType {
                    columns: VecSize::Two,
                    rows: VecSize::Two,
                    inner: TyKind::Scalar(ScalarType::F32).intern(self.database),
                })
            },
            "mat2x3f" => {
                self.expect_no_template(template_parameters);
                TyKind::Matrix(MatrixType {
                    columns: VecSize::Two,
                    rows: VecSize::Three,
                    inner: TyKind::Scalar(ScalarType::F32).intern(self.database),
                })
            },
            "mat2x4f" => {
                self.expect_no_template(template_parameters);
                TyKind::Matrix(MatrixType {
                    columns: VecSize::Two,
                    rows: VecSize::Four,
                    inner: TyKind::Scalar(ScalarType::F32).intern(self.database),
                })
            },
            "mat3x2f" => {
                self.expect_no_template(template_parameters);
                TyKind::Matrix(MatrixType {
                    columns: VecSize::Three,
                    rows: VecSize::Two,
                    inner: TyKind::Scalar(ScalarType::F32).intern(self.database),
                })
            },
            "mat3x3f" => {
                self.expect_no_template(template_parameters);
                TyKind::Matrix(MatrixType {
                    columns: VecSize::Three,
                    rows: VecSize::Three,
                    inner: TyKind::Scalar(ScalarType::F32).intern(self.database),
                })
            },
            "mat3x4f" => {
                self.expect_no_template(template_parameters);
                TyKind::Matrix(MatrixType {
                    columns: VecSize::Three,
                    rows: VecSize::Four,
                    inner: TyKind::Scalar(ScalarType::F32).intern(self.database),
                })
            },
            "mat4x2f" => {
                self.expect_no_template(template_parameters);
                TyKind::Matrix(MatrixType {
                    columns: VecSize::Four,
                    rows: VecSize::Two,
                    inner: TyKind::Scalar(ScalarType::F32).intern(self.database),
                })
            },
            "mat4x3f" => {
                self.expect_no_template(template_parameters);
                TyKind::Matrix(MatrixType {
                    columns: VecSize::Four,
                    rows: VecSize::Three,
                    inner: TyKind::Scalar(ScalarType::F32).intern(self.database),
                })
            },
            "mat4x4f" => {
                self.expect_no_template(template_parameters);
                TyKind::Matrix(MatrixType {
                    columns: VecSize::Four,
                    rows: VecSize::Four,
                    inner: TyKind::Scalar(ScalarType::F32).intern(self.database),
                })
            },
            "mat2x2h" => {
                self.expect_no_template(template_parameters);
                TyKind::Matrix(MatrixType {
                    columns: VecSize::Two,
                    rows: VecSize::Two,
                    inner: TyKind::Scalar(ScalarType::F16).intern(self.database),
                })
            },
            "mat2x3h" => {
                self.expect_no_template(template_parameters);
                TyKind::Matrix(MatrixType {
                    columns: VecSize::Two,
                    rows: VecSize::Three,
                    inner: TyKind::Scalar(ScalarType::F16).intern(self.database),
                })
            },
            "mat2x4h" => {
                self.expect_no_template(template_parameters);
                TyKind::Matrix(MatrixType {
                    columns: VecSize::Two,
                    rows: VecSize::Four,
                    inner: TyKind::Scalar(ScalarType::F16).intern(self.database),
                })
            },
            "mat3x2h" => {
                self.expect_no_template(template_parameters);
                TyKind::Matrix(MatrixType {
                    columns: VecSize::Three,
                    rows: VecSize::Two,
                    inner: TyKind::Scalar(ScalarType::F16).intern(self.database),
                })
            },
            "mat3x3h" => {
                self.expect_no_template(template_parameters);
                TyKind::Matrix(MatrixType {
                    columns: VecSize::Three,
                    rows: VecSize::Three,
                    inner: TyKind::Scalar(ScalarType::F16).intern(self.database),
                })
            },
            "mat3x4h" => {
                self.expect_no_template(template_parameters);
                TyKind::Matrix(MatrixType {
                    columns: VecSize::Three,
                    rows: VecSize::Four,
                    inner: TyKind::Scalar(ScalarType::F16).intern(self.database),
                })
            },
            "mat4x2h" => {
                self.expect_no_template(template_parameters);
                TyKind::Matrix(MatrixType {
                    columns: VecSize::Four,
                    rows: VecSize::Two,
                    inner: TyKind::Scalar(ScalarType::F16).intern(self.database),
                })
            },
            "mat4x3h" => {
                self.expect_no_template(template_parameters);
                TyKind::Matrix(MatrixType {
                    columns: VecSize::Four,
                    rows: VecSize::Three,
                    inner: TyKind::Scalar(ScalarType::F16).intern(self.database),
                })
            },
            "mat4x4h" => {
                self.expect_no_template(template_parameters);
                TyKind::Matrix(MatrixType {
                    columns: VecSize::Four,
                    rows: VecSize::Four,
                    inner: TyKind::Scalar(ScalarType::F16).intern(self.database),
                })
            },
            "ptr" => {
                let pointer_template = self.pointer_template(evaluated_parameters)?;
                TyKind::Pointer(Pointer {
                    address_space: pointer_template.address_space,
                    inner: pointer_template.inner,
                    access_mode: pointer_template.access_mode,
                })
            },
            "atomic" => {
                let inner = self.atomic_template(evaluated_parameters);
                TyKind::Atomic(AtomicType { inner })
            },
            "texture_1d" => {
                let sampled = self.texture_sampled_template(evaluated_parameters)?;
                TyKind::Texture(TextureType {
                    kind: TextureKind::from_sampled(sampled, self.database),
                    dimension: TextureDimensionality::D1,
                    arrayed: false,
                    multisampled: false,
                })
            },
            "texture_2d" => {
                let sampled = self.texture_sampled_template(evaluated_parameters)?;
                TyKind::Texture(TextureType {
                    kind: TextureKind::from_sampled(sampled, self.database),
                    dimension: TextureDimensionality::D2,
                    arrayed: false,
                    multisampled: false,
                })
            },
            "texture_2d_array" => {
                let sampled = self.texture_sampled_template(evaluated_parameters)?;
                TyKind::Texture(TextureType {
                    kind: TextureKind::from_sampled(sampled, self.database),
                    dimension: TextureDimensionality::D2,
                    arrayed: true,
                    multisampled: false,
                })
            },
            "texture_3d" => {
                let sampled = self.texture_sampled_template(evaluated_parameters)?;
                TyKind::Texture(TextureType {
                    kind: TextureKind::from_sampled(sampled, self.database),
                    dimension: TextureDimensionality::D3,
                    arrayed: false,
                    multisampled: false,
                })
            },
            "texture_cube" => {
                let sampled = self.texture_sampled_template(evaluated_parameters)?;
                TyKind::Texture(TextureType {
                    kind: TextureKind::from_sampled(sampled, self.database),
                    dimension: TextureDimensionality::Cube,
                    arrayed: false,
                    multisampled: false,
                })
            },
            "texture_cube_array" => {
                let sampled = self.texture_sampled_template(evaluated_parameters)?;
                TyKind::Texture(TextureType {
                    kind: TextureKind::from_sampled(sampled, self.database),
                    dimension: TextureDimensionality::Cube,
                    arrayed: true,
                    multisampled: false,
                })
            },
            "texture_multisampled_2d" => {
                let sampled = self.texture_sampled_template(evaluated_parameters)?;
                TyKind::Texture(TextureType {
                    kind: TextureKind::from_sampled(sampled, self.database),
                    dimension: TextureDimensionality::D2,
                    arrayed: false,
                    multisampled: true,
                })
            },
            "texture_storage_1d" => {
                let storage_template = self.storage_texture_template(evaluated_parameters)?;
                TyKind::Texture(TextureType {
                    kind: TextureKind::Storage(
                        from_wgsl_texel_format(storage_template.texel_format),
                        storage_template.access_mode,
                    ),
                    dimension: TextureDimensionality::D1,
                    arrayed: false,
                    multisampled: false,
                })
            },
            "texture_storage_2d" => {
                let storage_template = self.storage_texture_template(evaluated_parameters)?;
                TyKind::Texture(TextureType {
                    kind: TextureKind::Storage(
                        from_wgsl_texel_format(storage_template.texel_format),
                        storage_template.access_mode,
                    ),
                    dimension: TextureDimensionality::D2,
                    arrayed: false,
                    multisampled: false,
                })
            },
            "texture_storage_2d_array" => {
                let storage_template = self.storage_texture_template(evaluated_parameters)?;
                TyKind::Texture(TextureType {
                    kind: TextureKind::Storage(
                        from_wgsl_texel_format(storage_template.texel_format),
                        storage_template.access_mode,
                    ),
                    dimension: TextureDimensionality::D2,
                    arrayed: true,
                    multisampled: false,
                })
            },
            "texture_storage_3d" => {
                let storage_template = self.storage_texture_template(evaluated_parameters)?;
                TyKind::Texture(TextureType {
                    kind: TextureKind::Storage(
                        from_wgsl_texel_format(storage_template.texel_format),
                        storage_template.access_mode,
                    ),
                    dimension: TextureDimensionality::D3,
                    arrayed: false,
                    multisampled: false,
                })
            },
            "texture_depth_multisampled_2d" => {
                self.expect_no_template(template_parameters);
                TyKind::Texture(TextureType {
                    kind: TextureKind::Depth,
                    dimension: TextureDimensionality::D2,
                    arrayed: false,
                    multisampled: true,
                })
            },
            "texture_external" => {
                self.expect_no_template(template_parameters);
                TyKind::Texture(TextureType {
                    kind: TextureKind::External,
                    dimension: TextureDimensionality::D2,
                    arrayed: false,
                    multisampled: false,
                })
            },
            "texture_depth_2d" => {
                self.expect_no_template(template_parameters);
                TyKind::Texture(TextureType {
                    kind: TextureKind::Depth,
                    dimension: TextureDimensionality::D2,
                    arrayed: false,
                    multisampled: false,
                })
            },
            "texture_depth_2d_array" => {
                self.expect_no_template(template_parameters);
                TyKind::Texture(TextureType {
                    kind: TextureKind::Depth,
                    dimension: TextureDimensionality::D2,
                    arrayed: true,
                    multisampled: false,
                })
            },
            "texture_depth_cube" => {
                self.expect_no_template(template_parameters);
                TyKind::Texture(TextureType {
                    kind: TextureKind::Depth,
                    dimension: TextureDimensionality::Cube,
                    arrayed: false,
                    multisampled: false,
                })
            },
            "texture_depth_cube_array" => {
                self.expect_no_template(template_parameters);
                TyKind::Texture(TextureType {
                    kind: TextureKind::Depth,
                    dimension: TextureDimensionality::Cube,
                    arrayed: true,
                    multisampled: false,
                })
            },
            "sampler" => {
                self.expect_no_template(template_parameters);
                TyKind::Sampler(wgsl_types::ty::SamplerType::Sampler)
            },
            "sampler_comparison" => {
                self.expect_no_template(template_parameters);
                TyKind::Sampler(wgsl_types::ty::SamplerType::SamplerComparison)
            },
            _ => {
                return Err(TypeLoweringError {
                    container: type_container,
                    kind: TypeLoweringErrorKind::UnresolvedName(path.clone()),
                });
            },
        };
        Ok(Lowered::Type(ty_kind.intern(self.database)))
    }

    fn array_template(
        &mut self,
        mut template_parameters: TemplateParameters,
    ) -> Result<ArrayTemplate, TypeLoweringError> {
        self.expect_n_templates(&template_parameters, 1..=2);
        let r#type = match template_parameters.next_as_type() {
            Ok((r#type, _)) => r#type,
            Err(error) => {
                self.diagnostics.push(error);
                TyKind::Error.intern(self.database)
            },
        };

        let size = if template_parameters.has_next() {
            match template_parameters.next_as_instance() {
                Ok((Some(Instance::Literal(LiteralInstance::I32(number))), _)) if number > 0 =>
                {
                    #[expect(
                        clippy::cast_sign_loss,
                        clippy::as_conversions,
                        reason = "this is checked, could refactor into `if let Ok(validated) = u32::try_from(number)` once that is stable"
                    )]
                    ArraySize::Constant(number as u32)
                },
                Ok((Some(Instance::Literal(LiteralInstance::U32(number))), _)) if number > 0 => {
                    ArraySize::Constant(number)
                },
                Ok((
                    Some(Instance::Literal(
                        LiteralInstance::AbstractInt(number) | LiteralInstance::I64(number),
                    )),
                    _,
                )) if number > 0 && number <= ArraySize::MAX.into() => {
                    // skips handling array<E, 1li64>() or array<E, 99999999999999999999999999>()
                    #[expect(
                        clippy::cast_possible_truncation,
                        clippy::cast_sign_loss,
                        clippy::as_conversions,
                        reason = "this is checked, could refactor into `if let Ok(validated) = u32::try_from(number)` once that is stable"
                    )]
                    ArraySize::Constant(number as u32)
                },
                Ok((Some(Instance::Literal(LiteralInstance::U64(number))), _))
                    if number > 0 && number <= ArraySize::MAX.into() =>
                {
                    // skips handling array<E, 1lu64>() or array<E, 99999999999999999999999999lu64>()
                    #[expect(
                        clippy::cast_possible_truncation,
                        clippy::as_conversions,
                        reason = "this is checked, could refactor into `if let Ok(validated) = u32::try_from(number)` once that is stable"
                    )]
                    ArraySize::Constant(number as u32)
                },
                Ok((_, expression)) => {
                    let error = TypeLoweringError {
                        container: TypeContainer::Expression(expression),
                        kind: TypeLoweringErrorKind::UnexpectedTemplateArgument(
                            "a `u32` or a `i32` greater than `0`".to_owned(),
                        ),
                    };
                    self.diagnostics.push(error.clone());
                    return Err(error);
                },
                Err(error) => {
                    self.diagnostics.push(error.clone());
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
        mut template_parameters: TemplateParameters,
    ) -> Type {
        self.expect_n_templates(&template_parameters, 1..=1);

        match template_parameters.next_as_type() {
            Ok((r#type, expression)) => {
                let ty_kind = r#type.kind(self.database);
                if matches!(ty_kind, TyKind::Scalar(_)) && !ty_kind.is_abstract(self.database) {
                    r#type
                } else {
                    self.diagnostics.push(TypeLoweringError {
                        container: TypeContainer::Expression(expression),
                        kind: TypeLoweringErrorKind::UnexpectedTemplateArgument(
                            "a scalar".to_owned(),
                        ),
                    });
                    TyKind::Error.intern(self.database)
                }
            },
            Err(error) => {
                self.diagnostics.push(error);
                TyKind::Error.intern(self.database)
            },
        }
    }

    fn matrix_template(
        &mut self,
        mut template_parameters: TemplateParameters,
    ) -> Type {
        self.expect_n_templates(&template_parameters, 1..=1);

        match template_parameters.next_as_type() {
            Ok((r#type, expression)) => {
                let ty_kind = r#type.kind(self.database);
                if matches!(ty_kind, TyKind::Scalar(ScalarType::F16 | ScalarType::F32)) {
                    r#type
                } else {
                    self.diagnostics.push(TypeLoweringError {
                        container: TypeContainer::Expression(expression),
                        kind: TypeLoweringErrorKind::UnexpectedTemplateArgument(
                            "one of: f32 or f16".to_owned(),
                        ),
                    });
                    TyKind::Error.intern(self.database)
                }
            },
            Err(error) => {
                self.diagnostics.push(error);
                TyKind::Error.intern(self.database)
            },
        }
    }

    fn pointer_template(
        &mut self,
        mut template_parameters: TemplateParameters,
    ) -> Result<PointerTemplate, TypeLoweringError> {
        self.expect_n_templates(&template_parameters, 2..=3);
        let address_space = match template_parameters.next_as_enumerant() {
            Ok((Enumerant::AddressSpace(address_space), _)) => address_space,
            Ok((_, expression)) => {
                let error = TypeLoweringError {
                    container: TypeContainer::Expression(expression),
                    kind: TypeLoweringErrorKind::UnexpectedTemplateArgument(
                        "an address space".to_owned(),
                    ),
                };
                self.diagnostics.push(error.clone());
                return Err(error);
            },
            Err(error) => {
                self.diagnostics.push(error.clone());
                return Err(error);
            },
        };
        let inner = match template_parameters.next_as_type() {
            Ok((inner, expression)) => {
                let ty_kind = inner.kind(self.database);
                if ty_kind.is_storable() {
                    inner
                } else {
                    self.diagnostics.push(TypeLoweringError {
                        container: TypeContainer::Expression(expression),
                        kind: TypeLoweringErrorKind::UnexpectedTemplateArgument(
                            "a storable type".to_owned(),
                        ),
                    });
                    TyKind::Error.intern(self.database)
                }
            },
            Err(error) => {
                self.diagnostics.push(error);
                TyKind::Error.intern(self.database)
            },
        };

        let access_mode = if template_parameters.has_next() {
            match template_parameters.next_as_enumerant() {
                // uniform address space requires the read access mode
                Ok((
                    Enumerant::AccessMode(AccessMode::ReadWrite | AccessMode::ReadWrite),
                    expression,
                )) if address_space == AddressSpace::Uniform => {
                    self.diagnostics.push(TypeLoweringError {
                        container: TypeContainer::Expression(expression),
                        kind: TypeLoweringErrorKind::UnexpectedTemplateArgument(
                            "`read` access mode for uniforms".to_owned(),
                        ),
                    });
                    AccessMode::Read
                },
                // everything else has no such constraints
                Ok((Enumerant::AccessMode(access_mode), _)) => access_mode,
                Ok((_, expression)) => {
                    let error = TypeLoweringError {
                        container: TypeContainer::Expression(expression),
                        kind: TypeLoweringErrorKind::UnexpectedTemplateArgument(
                            "on of: (read, read_write, write)".to_owned(),
                        ),
                    };
                    self.diagnostics.push(error.clone());
                    return Err(error);
                },
                Err(error) => {
                    self.diagnostics.push(error.clone());
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
        mut template_parameters: TemplateParameters,
    ) -> Type {
        self.expect_n_templates(&template_parameters, 1..=1);

        match template_parameters.next_as_type() {
            Ok((r#type, expression)) => {
                let ty_kind = r#type.kind(self.database);
                if matches!(ty_kind, TyKind::Scalar(ScalarType::I32 | ScalarType::U32)) {
                    r#type
                } else {
                    // TODO: improve the error message and support naga atomics
                    // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/677
                    self.diagnostics.push(TypeLoweringError {
                        container: TypeContainer::Expression(expression),
                        kind: TypeLoweringErrorKind::UnexpectedTemplateArgument(
                            "i32 or u32".to_owned(), // Naga supports more types (f32, i64, u64) here
                        ),
                    });
                    TyKind::Error.intern(self.database)
                }
            },
            Err(error) => {
                self.diagnostics.push(error);
                TyKind::Error.intern(self.database)
            },
        }
    }

    fn texture_sampled_template(
        &mut self,
        mut template_parameters: TemplateParameters,
    ) -> Result<SampledType, TypeLoweringError> {
        self.expect_n_templates(&template_parameters, 1..=1);

        match template_parameters.next_as_type() {
            Ok((r#type, expression)) => {
                let ty_kind = r#type.kind(self.database);
                match ty_kind {
                    TyKind::Scalar(ScalarType::I32) => Ok(SampledType::I32),
                    TyKind::Scalar(ScalarType::U32) => Ok(SampledType::U32),
                    TyKind::Scalar(ScalarType::F32) => Ok(SampledType::F32),
                    TyKind::Error
                    | TyKind::Scalar(_)
                    | TyKind::Atomic(_)
                    | TyKind::Vector(_)
                    | TyKind::Matrix(_)
                    | TyKind::Struct(_)
                    | TyKind::Array(_)
                    | TyKind::Texture(_)
                    | TyKind::Sampler(_)
                    | TyKind::Reference(_)
                    | TyKind::Pointer(_)
                    | TyKind::BoundVar(_)
                    | TyKind::StorageTypeOfTexelFormat(_) => {
                        // texture_2d<invalid>()
                        let error = TypeLoweringError {
                            container: TypeContainer::Expression(expression),
                            kind: TypeLoweringErrorKind::UnexpectedTemplateArgument(
                                "i32 or u32 or f32".to_owned(),
                            ),
                        };
                        self.diagnostics.push(error.clone());
                        Err(error)
                    },
                }
            },
            Err(error) => {
                self.diagnostics.push(error.clone());
                Err(error)
            },
        }
    }

    fn storage_texture_template(
        &mut self,
        mut template_parameters: TemplateParameters,
    ) -> Result<StorageTextureTemplate, TypeLoweringError> {
        self.expect_n_templates(&template_parameters, 1..=1);
        let texel_format = match template_parameters.next_as_enumerant() {
            Ok((Enumerant::TexelFormat(texel_format), _)) => texel_format,
            Ok((_, expression)) => {
                let error = TypeLoweringError {
                    container: TypeContainer::Expression(expression),
                    kind: TypeLoweringErrorKind::UnexpectedTemplateArgument(
                        "a texel format (`rgba8unorm`, `rgba8snorm`, ...)".to_owned(),
                    ),
                };
                self.diagnostics.push(error.clone());
                return Err(error);
            },
            Err(error) => {
                self.diagnostics.push(error.clone());
                return Err(error);
            },
        };
        let access_mode = match template_parameters.next_as_enumerant() {
            Ok((Enumerant::AccessMode(access_mode), _)) => access_mode,
            Ok((_, expression)) => {
                let error = TypeLoweringError {
                    container: TypeContainer::Expression(expression),
                    kind: TypeLoweringErrorKind::UnexpectedTemplateArgument(
                        "one of: read, write, read_write".to_owned(),
                    ),
                };
                self.diagnostics.push(error.clone());
                return Err(error);
            },
            Err(error) => {
                self.diagnostics.push(error.clone());
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
