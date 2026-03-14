/// Metadata for WGSL predeclared (builtin) types.
///
/// Used by hover and completion to show descriptions for types like `f32`, `vec3`, `mat4x4`, etc.

/// A WGSL builtin type with its description and spec reference.
pub struct WgslBuiltinType {
    /// The type name as written in WGSL (e.g., `f32`, `vec3`, `mat4x4`).
    pub name: &'static str,
    /// A brief description of the type.
    pub description: &'static str,
    /// The WGSL spec anchor (appended to `https://www.w3.org/TR/WGSL/#`).
    pub spec_anchor: &'static str,
}

impl WgslBuiltinType {
    /// Returns the full URL to the WGSL spec section for this type.
    #[must_use]
    pub fn spec_url(&self) -> String {
        format!("https://www.w3.org/TR/WGSL/#{}", self.spec_anchor)
    }
}

/// All WGSL predeclared types.
pub static WGSL_BUILTIN_TYPES: &[WgslBuiltinType] = &[
    // Scalar types
    WgslBuiltinType {
        name: "bool",
        description: "Boolean type. Either `true` or `false`.",
        spec_anchor: "bool-type",
    },
    WgslBuiltinType {
        name: "i32",
        description: "32-bit signed integer.",
        spec_anchor: "i32-type",
    },
    WgslBuiltinType {
        name: "u32",
        description: "32-bit unsigned integer.",
        spec_anchor: "u32-type",
    },
    WgslBuiltinType {
        name: "f32",
        description: "32-bit IEEE-754 floating point number.",
        spec_anchor: "f32-type",
    },
    WgslBuiltinType {
        name: "f16",
        description: "16-bit IEEE-754 floating point number. Requires the `f16` extension.",
        spec_anchor: "f16-type",
    },
    // Vector types
    WgslBuiltinType {
        name: "vec2",
        description: "2-component vector.",
        spec_anchor: "vector-types",
    },
    WgslBuiltinType {
        name: "vec3",
        description: "3-component vector.",
        spec_anchor: "vector-types",
    },
    WgslBuiltinType {
        name: "vec4",
        description: "4-component vector.",
        spec_anchor: "vector-types",
    },
    WgslBuiltinType {
        name: "vec2i",
        description: "2-component vector of `i32`.",
        spec_anchor: "vector-types",
    },
    WgslBuiltinType {
        name: "vec3i",
        description: "3-component vector of `i32`.",
        spec_anchor: "vector-types",
    },
    WgslBuiltinType {
        name: "vec4i",
        description: "4-component vector of `i32`.",
        spec_anchor: "vector-types",
    },
    WgslBuiltinType {
        name: "vec2u",
        description: "2-component vector of `u32`.",
        spec_anchor: "vector-types",
    },
    WgslBuiltinType {
        name: "vec3u",
        description: "3-component vector of `u32`.",
        spec_anchor: "vector-types",
    },
    WgslBuiltinType {
        name: "vec4u",
        description: "4-component vector of `u32`.",
        spec_anchor: "vector-types",
    },
    WgslBuiltinType {
        name: "vec2f",
        description: "2-component vector of `f32`.",
        spec_anchor: "vector-types",
    },
    WgslBuiltinType {
        name: "vec3f",
        description: "3-component vector of `f32`.",
        spec_anchor: "vector-types",
    },
    WgslBuiltinType {
        name: "vec4f",
        description: "4-component vector of `f32`.",
        spec_anchor: "vector-types",
    },
    WgslBuiltinType {
        name: "vec2h",
        description: "2-component vector of `f16`.",
        spec_anchor: "vector-types",
    },
    WgslBuiltinType {
        name: "vec3h",
        description: "3-component vector of `f16`.",
        spec_anchor: "vector-types",
    },
    WgslBuiltinType {
        name: "vec4h",
        description: "4-component vector of `f16`.",
        spec_anchor: "vector-types",
    },
    // Matrix types
    WgslBuiltinType {
        name: "mat2x2",
        description: "2×2 matrix.",
        spec_anchor: "matrix-types",
    },
    WgslBuiltinType {
        name: "mat2x3",
        description: "2×3 matrix (2 columns, 3 rows).",
        spec_anchor: "matrix-types",
    },
    WgslBuiltinType {
        name: "mat2x4",
        description: "2×4 matrix (2 columns, 4 rows).",
        spec_anchor: "matrix-types",
    },
    WgslBuiltinType {
        name: "mat3x2",
        description: "3×2 matrix (3 columns, 2 rows).",
        spec_anchor: "matrix-types",
    },
    WgslBuiltinType {
        name: "mat3x3",
        description: "3×3 matrix.",
        spec_anchor: "matrix-types",
    },
    WgslBuiltinType {
        name: "mat3x4",
        description: "3×4 matrix (3 columns, 4 rows).",
        spec_anchor: "matrix-types",
    },
    WgslBuiltinType {
        name: "mat4x2",
        description: "4×2 matrix (4 columns, 2 rows).",
        spec_anchor: "matrix-types",
    },
    WgslBuiltinType {
        name: "mat4x3",
        description: "4×3 matrix (4 columns, 3 rows).",
        spec_anchor: "matrix-types",
    },
    WgslBuiltinType {
        name: "mat4x4",
        description: "4×4 matrix.",
        spec_anchor: "matrix-types",
    },
    // Shorthand matrix types (f32)
    WgslBuiltinType {
        name: "mat2x2f",
        description: "2×2 matrix of `f32`.",
        spec_anchor: "matrix-types",
    },
    WgslBuiltinType {
        name: "mat2x3f",
        description: "2×3 matrix of `f32`.",
        spec_anchor: "matrix-types",
    },
    WgslBuiltinType {
        name: "mat2x4f",
        description: "2×4 matrix of `f32`.",
        spec_anchor: "matrix-types",
    },
    WgslBuiltinType {
        name: "mat3x2f",
        description: "3×2 matrix of `f32`.",
        spec_anchor: "matrix-types",
    },
    WgslBuiltinType {
        name: "mat3x3f",
        description: "3×3 matrix of `f32`.",
        spec_anchor: "matrix-types",
    },
    WgslBuiltinType {
        name: "mat3x4f",
        description: "3×4 matrix of `f32`.",
        spec_anchor: "matrix-types",
    },
    WgslBuiltinType {
        name: "mat4x2f",
        description: "4×2 matrix of `f32`.",
        spec_anchor: "matrix-types",
    },
    WgslBuiltinType {
        name: "mat4x3f",
        description: "4×3 matrix of `f32`.",
        spec_anchor: "matrix-types",
    },
    WgslBuiltinType {
        name: "mat4x4f",
        description: "4×4 matrix of `f32`.",
        spec_anchor: "matrix-types",
    },
    // Shorthand matrix types (f16)
    WgslBuiltinType {
        name: "mat2x2h",
        description: "2×2 matrix of `f16`.",
        spec_anchor: "matrix-types",
    },
    WgslBuiltinType {
        name: "mat2x3h",
        description: "2×3 matrix of `f16`.",
        spec_anchor: "matrix-types",
    },
    WgslBuiltinType {
        name: "mat2x4h",
        description: "2×4 matrix of `f16`.",
        spec_anchor: "matrix-types",
    },
    WgslBuiltinType {
        name: "mat3x2h",
        description: "3×2 matrix of `f16`.",
        spec_anchor: "matrix-types",
    },
    WgslBuiltinType {
        name: "mat3x3h",
        description: "3×3 matrix of `f16`.",
        spec_anchor: "matrix-types",
    },
    WgslBuiltinType {
        name: "mat3x4h",
        description: "3×4 matrix of `f16`.",
        spec_anchor: "matrix-types",
    },
    WgslBuiltinType {
        name: "mat4x2h",
        description: "4×2 matrix of `f16`.",
        spec_anchor: "matrix-types",
    },
    WgslBuiltinType {
        name: "mat4x3h",
        description: "4×3 matrix of `f16`.",
        spec_anchor: "matrix-types",
    },
    WgslBuiltinType {
        name: "mat4x4h",
        description: "4×4 matrix of `f16`.",
        spec_anchor: "matrix-types",
    },
    // Other types
    WgslBuiltinType {
        name: "array",
        description: "Fixed-size or runtime-sized array type.",
        spec_anchor: "array-types",
    },
    WgslBuiltinType {
        name: "atomic",
        description: "Atomic type for shared mutable access. Supports `i32` and `u32`.",
        spec_anchor: "atomic-types",
    },
    WgslBuiltinType {
        name: "ptr",
        description: "Pointer type. References a value in a particular address space.",
        spec_anchor: "ref-ptr-types",
    },
    // Sampler types
    WgslBuiltinType {
        name: "sampler",
        description: "Sampler for filtered texture reads.",
        spec_anchor: "sampler-type",
    },
    WgslBuiltinType {
        name: "sampler_comparison",
        description: "Comparison sampler for depth texture reads.",
        spec_anchor: "sampler-type",
    },
    // Texture types
    WgslBuiltinType {
        name: "texture_1d",
        description: "1-dimensional sampled texture.",
        spec_anchor: "texture-types",
    },
    WgslBuiltinType {
        name: "texture_2d",
        description: "2-dimensional sampled texture.",
        spec_anchor: "texture-types",
    },
    WgslBuiltinType {
        name: "texture_2d_array",
        description: "Array of 2-dimensional sampled textures.",
        spec_anchor: "texture-types",
    },
    WgslBuiltinType {
        name: "texture_3d",
        description: "3-dimensional sampled texture.",
        spec_anchor: "texture-types",
    },
    WgslBuiltinType {
        name: "texture_cube",
        description: "Cube-mapped sampled texture.",
        spec_anchor: "texture-types",
    },
    WgslBuiltinType {
        name: "texture_cube_array",
        description: "Array of cube-mapped sampled textures.",
        spec_anchor: "texture-types",
    },
    WgslBuiltinType {
        name: "texture_multisampled_2d",
        description: "2-dimensional multisampled texture.",
        spec_anchor: "texture-types",
    },
    WgslBuiltinType {
        name: "texture_storage_1d",
        description: "1-dimensional storage texture.",
        spec_anchor: "texture-storage",
    },
    WgslBuiltinType {
        name: "texture_storage_2d",
        description: "2-dimensional storage texture.",
        spec_anchor: "texture-storage",
    },
    WgslBuiltinType {
        name: "texture_storage_2d_array",
        description: "Array of 2-dimensional storage textures.",
        spec_anchor: "texture-storage",
    },
    WgslBuiltinType {
        name: "texture_storage_3d",
        description: "3-dimensional storage texture.",
        spec_anchor: "texture-storage",
    },
    WgslBuiltinType {
        name: "texture_depth_2d",
        description: "2-dimensional depth texture.",
        spec_anchor: "texture-depth",
    },
    WgslBuiltinType {
        name: "texture_depth_2d_array",
        description: "Array of 2-dimensional depth textures.",
        spec_anchor: "texture-depth",
    },
    WgslBuiltinType {
        name: "texture_depth_cube",
        description: "Cube-mapped depth texture.",
        spec_anchor: "texture-depth",
    },
    WgslBuiltinType {
        name: "texture_depth_cube_array",
        description: "Array of cube-mapped depth textures.",
        spec_anchor: "texture-depth",
    },
    WgslBuiltinType {
        name: "texture_depth_multisampled_2d",
        description: "2-dimensional multisampled depth texture.",
        spec_anchor: "texture-depth",
    },
    WgslBuiltinType {
        name: "texture_external",
        description: "External texture for sampling video frames.",
        spec_anchor: "texture-external",
    },
];

/// Look up a WGSL builtin type by name.
#[must_use]
pub fn find_builtin_type(name: &str) -> Option<&'static WgslBuiltinType> {
    WGSL_BUILTIN_TYPES.iter().find(|ty| ty.name == name)
}
