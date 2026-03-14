use super::Completions;
use crate::{
    context::{CompletionContext, ImmediateLocation},
    item::{CompletionItem, CompletionItemKind},
};

/// Built-in scalar types.
const SCALAR_TYPES: &[&str] = &["bool", "f16", "f32", "i32", "u32"];

/// Built-in vector types (generic and aliases).
const VECTOR_TYPES: &[&str] = &[
    "vec2", "vec3", "vec4", "vec2i", "vec2u", "vec2f", "vec2h", "vec3i", "vec3u", "vec3f", "vec3h",
    "vec4i", "vec4u", "vec4f", "vec4h",
];

/// Built-in matrix types (generic and aliases).
const MATRIX_TYPES: &[&str] = &[
    "mat2x2", "mat2x3", "mat2x4", "mat3x2", "mat3x3", "mat3x4", "mat4x2", "mat4x3", "mat4x4",
    "mat2x2f", "mat2x2h", "mat2x3f", "mat2x3h", "mat2x4f", "mat2x4h", "mat3x2f", "mat3x2h",
    "mat3x3f", "mat3x3h", "mat3x4f", "mat3x4h", "mat4x2f", "mat4x2h", "mat4x3f", "mat4x3h",
    "mat4x4f", "mat4x4h",
];

/// Other built-in types.
const OTHER_TYPES: &[&str] = &[
    "array",
    "atomic",
    "ptr",
    "sampler",
    "sampler_comparison",
    "texture_1d",
    "texture_2d",
    "texture_2d_array",
    "texture_3d",
    "texture_cube",
    "texture_cube_array",
    "texture_multisampled_2d",
    "texture_storage_1d",
    "texture_storage_2d",
    "texture_storage_2d_array",
    "texture_storage_3d",
    "texture_depth_2d",
    "texture_depth_2d_array",
    "texture_depth_cube",
    "texture_depth_cube_array",
    "texture_depth_multisampled_2d",
    "texture_external",
];

pub(crate) fn complete_types(
    accumulator: &mut Completions,
    context: &CompletionContext<'_>,
) -> Option<()> {
    match context.completion_location {
        Some(ImmediateLocation::InsideStatement | ImmediateLocation::ItemList) => {},
        _ => return None,
    }

    let all_types = SCALAR_TYPES
        .iter()
        .chain(VECTOR_TYPES)
        .chain(MATRIX_TYPES)
        .chain(OTHER_TYPES);

    for type_name in all_types {
        CompletionItem::new(
            CompletionItemKind::TypeAlias,
            context.source_range(),
            *type_name,
        )
        .add_to(accumulator, context.database);
    }

    Some(())
}
