//! The edition of the shader language.

use std::{error, fmt, str};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
#[non_exhaustive]
pub enum Edition {
    // The syntax context stuff needs the discriminants to start from 0 and be consecutive.
    #[default]
    Wgsl = 0,
    Wesl2025Unstable,
}

impl Edition {
    pub const CURRENT: Self = Self::Wgsl;
    /// The current latest stable edition, note this is usually not the right choice in code.
    pub const CURRENT_FIXME: Self = Self::Wgsl;
    pub const DEFAULT: Self = Self::Wgsl;
    pub const LATEST: Self = Self::Wesl2025Unstable;

    /// # Panics
    ///
    /// Panics if the value does not correspond to a variant of [`Edition`].
    #[must_use]
    pub fn from_u32(u32: u32) -> Self {
        match u32 {
            0 => Self::Wgsl,
            1 => Self::Wesl2025Unstable,
            _ => panic!("invalid edition"),
        }
    }

    #[must_use]
    pub fn at_least_wesl_0_0_1(self) -> bool {
        self >= Self::Wesl2025Unstable
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Wgsl, Self::Wesl2025Unstable].iter().copied()
    }
}

#[derive(Debug)]
pub struct ParseEditionError {
    invalid_input: String,
}

impl error::Error for ParseEditionError {}

impl fmt::Display for ParseEditionError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(formatter, "invalid edition: {}", self.invalid_input)
    }
}

impl str::FromStr for Edition {
    type Err = ParseEditionError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        // https://github.com/wgsl-tooling-wg/wesl-rs/tree/main/crates/wesl/src/wesl_toml.rs#L78
        match string {
            "2026_pre" => Ok(Self::Wesl2025Unstable),
            // "WGSL" is not an edition that can be selected.
            // Therefore it is not included here.
            _ => Err(ParseEditionError {
                invalid_input: string.to_owned(),
            }),
        }
    }
}

impl fmt::Display for Edition {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(match self {
            Self::Wgsl => "WGSL",
            Self::Wesl2025Unstable => "WESL 2025 (Unstable)",
        })
    }
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExtensionsConfig {
    // == Enable extensions
    // base WGSL
    /// Enables `f16`/`half` primitive support in all shader languages.
    ///
    /// In the WGSL standard, this corresponds to [`enable f16;`].
    ///
    /// [`enable f16;`]: https://www.w3.org/TR/WGSL/#extension-f16
    pub f16: bool,

    // TODO: actually implement https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1400
    /// Enables the `clip_distances` variable in WGSL.
    ///
    /// In the WGSL standard, this corresponds to [`enable clip_distances;`].
    ///
    /// [`enable clip_distances;`]: https://www.w3.org/TR/WGSL/#extension-clip_distances
    pub clip_distances: bool,

    // TODO: actually implement https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1401
    /// Enables the `blend_src` attribute in WGSL.
    ///
    /// In the WGSL standard, this corresponds to [`enable dual_source_blending;`].
    ///
    /// [`enable dual_source_blending;`]: https://www.w3.org/TR/WGSL/#extension-dual_source_blending
    pub dual_source_blending: bool,

    // TODO: actually implement https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1402
    /// Enables subgroup built-ins in all languages.
    ///
    /// In the WGSL standard, this corresponds to [`enable subgroups;`].
    ///
    /// [`enable subgroups;`]: https://www.w3.org/TR/WGSL/#extension-subgroups
    pub subgroups: bool,

    // TODO: actually implement https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1403
    /// Enables the `@builtin(primitive_index)` attribute in WGSL.
    ///
    /// In the WGSL standard, this corresponds to [`enable primitive_index;`].
    ///
    /// [`enable primitive-index;`]: https://www.w3.org/TR/WGSL/#extension-primitive_index
    pub primitive_index: bool,

    // TODO: actually implement https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1404
    /// The attribute `subgroup_size` is valid to use in the WGSL module. Otherwise, using `subgroup_size` will result in a shader-creation error. The subgroups will be automatically enabled when `subgroup_size_control` is enabled.
    pub subgroup_size_control: bool,

    // naga enable extensions
    // TODO: actually implement https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1360
    /// Enables the `wgpu_mesh_shader` extension, native only.
    pub wgpu_mesh_shader: bool,

    // TODO: actually implement https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1399
    /// Enables the `wgpu_ray_query` extension, native only.
    pub wgpu_ray_query: bool,

    /// Enables the `wgpu_ray_query_vertex_return` extension, native only.
    pub wgpu_ray_query_vertex_return: bool,

    // TODO: actually implement https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1388
    /// Enables the `wgpu_ray_tracing_pipeline` extension, native only.
    pub wgpu_ray_tracing_pipeline: bool,

    // TODO: actually implement https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1406
    /// Enables the `wgpu_cooperative_matrix` extension, native only.
    pub wgpu_cooperative_matrix: bool,

    // TODO: actually implement https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1283
    /// Enables the `wgpu_binding_array` extension, native only.
    pub wgpu_binding_array: bool,

    // TODO: actually implement https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1407
    /// Enables the `wgpu_per_vertex` extension, allows using `@interpolate(per_vertex)` attribute in WGSL, native only.
    pub per_vertex: bool,

    // TODO: actually implement https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1408
    /// Enables `i16`/`u16` 16-bit integer support in WGSL, native only.
    pub wgpu_int16: bool,

    // TODO: actually implement https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1409
    /// Enables the `draw_index` builtin. Not currently part of the WGSL spec but probably will be at some point.
    pub draw_index: bool,

    // == Language extensions
    // TODO: actually implement https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1410
    /// Allows the use of `read` and `read_write` access modes with storage textures. Additionally, adds the textureBarrier built-in function.
    pub readonly_and_readwrite_storage_textures: bool,

    // TODO: actually implement https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1411
    /// Supports using 32-bit integer scalars packing 4-component vectors of 8-bit integers as inputs to the dot product instructions with dot4U8Packed and dot4I8Packed built-in functions. Additionally, adds packing and unpacking instructions with packed 4-component vectors of 8-bit integers with pack4xI8, pack4xU8, pack4xI8Clamp, pack4xU8Clamp, unpack4xI8, and unpack4xU8 built-in functions.
    pub packed_4x8_integer_dot_product: bool,

    // TODO: actually implement https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1412
    /// Removes the following restrictions from user-defined functions:
    // For user-defined functions, a parameter of pointer type must be in one of the following address spaces:
    /// - function
    /// - private
    ///
    /// Each argument of pointer type to a user-defined function must have the same memory view as its root identifier.
    pub unrestricted_pointer_parameters: bool,

    // TODO: actually implement https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1413
    /// Supports composite-value decomposition expressions where the root expression is a pointer, yielding a reference.
    pub pointer_composite_access: bool,

    // TODO: actually implement https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1358
    /// Allow buffers in the uniform address space to use the same memory layout constraints as other address spaces.
    pub uniform_buffer_standard_layout: bool,

    // TODO: actually implement https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1414
    /// Allows the use of the `subgroup_id` and `num_subgroups` built-in values when the subgroups extension is enabled.
    pub subgroup_id: bool,

    // TODO: actually implement https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1415
    /// Adds an additional scope, `subgroup`, for uniform control flow subgroup and quad built-in functions to be all invocations in the same subgroup.
    pub subgroup_uniformity: bool,

    // TODO: actually implement https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1416
    /// Allows the effective-value-type of a let-declaration to be a texture or sampler type.
    pub texture_and_sampler_let: bool,

    // TODO: actually implement https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1417
    /// Supports additional texel formats: `rgba16unorm`, `rgba16snorm`, `rg8unorm`, `rg8snorm`, `rg8uint`, `rg8sint`, `rg16unorm`, `rg16snorm`, `rg16uint`, `rg16sint`, `rg16float`, `r8unorm`, `r8snorm`, `r8uint`, `r8sint`, `r16unorm`, `r16snorm`, `r16uint`, `r16sint`, `r16float`, `rgb10a2unorm`, `rgb10a2uint`, `rg11b10ufloat`.
    pub texture_formats_tier1: bool,

    // TODO: actually implement https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1418
    /// Supports the `global_invocation_index` and `workgroup_index` built-in values.
    pub linear_indexing: bool,

    // TODO: actually implement https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1419
    /// Enables the immediate address space, allowing variables to be declared with var<immediate> and bound to small amounts of frequently updated data passed directly from the command encoder via the WebGPU API.
    pub immediate_address_space: bool,

    // TODO: actually implement https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1420
    /// Enables the use of `buffer` types and the `buffer_view` built-in functions.
    ///
    /// Allow the declaration of variables with an opaque store type that can be reinterpreted as other host-shareable types.
    pub buffer_view: bool,

    /// Supports swizzle view types.
    ///
    /// This enables swizzle assignments:
    /// a single assignment statement can update multiple components of a vector without having to update the entire vector.
    ///
    /// For example, if variable `v` is a 4-element vector, then `v.xz = vec2(1,2);` is
    /// a shorthand way of writing `v.x = 1; v.z = 2;`, while performing one read and one write.
    ///
    /// If `pointer_composite_access` is also supported, then this also works on pointers.
    /// If `p` is a pointer to a vector of at least 3 elements, then `p.xz = vec2(1,2);` is shorthand for `(*p).x = 1; (*p).z = 2;`.
    pub swizzle_assignment: bool,
}

// TODO: implement this in the frontend and add more https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1421
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Capabilities {
    // naga capabilities
    pub shader_int64: bool,
    pub early_depth_test: bool,
}
