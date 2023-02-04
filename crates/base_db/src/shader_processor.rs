use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

use once_cell::sync::Lazy;
use regex::Regex;

use crate::translator::{PreProcessTranslator, TranslationStrategy};

pub static SHADER_PROCESSOR: Lazy<ShaderProcessor> = Lazy::new(ShaderProcessor::default);

pub struct ShaderProcessor {
    ifdef_regex: Regex,
    ifndef_regex: Regex,
    else_regex: Regex,
    endif_regex: Regex,

    import_asset_path_regex: Regex,
    import_custom_path_regex: Regex,
    define_import_path_regex: Regex,
}

impl Default for ShaderProcessor {
    fn default() -> Self {
        Self {
            ifdef_regex: Regex::new(r"^\s*#\s*ifdef\s*([\w|\d|_]+)").unwrap(),
            ifndef_regex: Regex::new(r"^\s*#\s*ifndef\s*([\w|\d|_]+)").unwrap(),
            else_regex: Regex::new(r"^\s*#\s*else").unwrap(),
            endif_regex: Regex::new(r"^\s*#\s*endif").unwrap(),

            import_asset_path_regex: Regex::new(r#"^\s*#\s*import\s+"(.+)""#).unwrap(),
            import_custom_path_regex: Regex::new(r"^\s*#\s*import\s+(.+)").unwrap(),
            define_import_path_regex: Regex::new(r"^\s*#\s*define_import_path\s+(.+)").unwrap(),
        }
    }
}

impl ShaderProcessor {
    pub fn process(
        &self,
        shader_str: &str,
        shader_defs: &HashSet<String>,
        custom_imports: &HashMap<String, String>,
        mut emit_unconfigured: impl FnMut(Range<usize>, &str),
    ) -> (String, PreProcessTranslator) {
        self.process_inner(
            shader_str,
            shader_defs,
            custom_imports,
            &mut emit_unconfigured,
        )
    }

    fn process_inner(
        &self,
        shader_str: &str,
        shader_defs: &HashSet<String>,
        custom_imports: &HashMap<String, String>,
        emit_unconfigured: &mut dyn FnMut(Range<usize>, &str),
    ) -> (String, PreProcessTranslator) {
        let mut offset_table = PreProcessTranslator::new();
        let mut scopes = vec![(true, 0, "root scope")];
        let mut final_string = String::with_capacity(shader_str.len());

        let mut offset = 0_usize;

        for line in shader_str.lines() {
            if let Some(cap) = self.ifdef_regex.captures(line) {
                let def = cap.get(1).unwrap().as_str();
                scopes.push((
                    scopes.last().unwrap().0 && shader_defs.contains(def),
                    offset,
                    def,
                ));
            } else if let Some(cap) = self.ifndef_regex.captures(line) {
                let def = cap.get(1).unwrap().as_str();
                scopes.push((
                    scopes.last().unwrap().0 && !shader_defs.contains(def),
                    offset,
                    def,
                ));
            } else if self.else_regex.is_match(line) {
                let mut is_parent_scope_truthy = true;
                if scopes.len() > 1 {
                    is_parent_scope_truthy = scopes[scopes.len() - 2].0;
                }

                if let Some((last, start_offset, def)) = scopes.last_mut() {
                    if !*last {
                        let range = *start_offset..offset + line.len();
                        emit_unconfigured(range, def);
                    }

                    *start_offset = offset;
                    *last = is_parent_scope_truthy && !*last;
                }
            } else if self.endif_regex.is_match(line) {
                // HACK: Ignore endifs without a corresponding
                // This does need proper error reporting somewhere, which is not yet implemented
                // Presumably this would be through a side channel
                if scopes.len() == 1 {
                    // return Err(ProcessShaderError::TooManyEndIfs);
                } else {
                    if let Some((used, start_offset, def)) = scopes.pop() {
                        if !used {
                            let range = start_offset..offset + line.len();
                            emit_unconfigured(range, def);
                        }
                    }
                }
            } else if scopes.last().unwrap().0 {
                if let Some(_cap) = self.import_asset_path_regex.captures(line) {
                    // path import is not supported right now
                } else if let Some(cap) = self.import_custom_path_regex.captures(line) {
                    let default_import_str = "".to_string();
                    let import_str = custom_imports
                        .get(&cap.get(1).unwrap().as_str().to_string())
                        .unwrap_or(&default_import_str);
                    let import_final_str = self
                        .process_inner(import_str, shader_defs, &custom_imports, &mut |_, _| {})
                        .0;

                    final_string.push_str(&import_final_str);

                    offset_table.set_last_block_strategy(TranslationStrategy::Import);
                } else if self.define_import_path_regex.is_match(line) {
                    // ignore import path lines
                } else {
                    final_string.push_str(line);
                }
            }

            final_string.push('\n');
            let final_str_offset = final_string.len();
            offset = offset + line.len() + 1; // +1 for '\n'
            offset_table.insert(offset as u32, final_str_offset as u32);
        }

        if scopes.len() != 1 {
            // return Err(ProcessShaderError::NotEnoughEndIfs);
        }

        (final_string, offset_table)
    }
}

#[cfg(test)]
mod tests {
    use rowan::{TextRange, TextSize};

    use super::ShaderProcessor;
    use std::collections::{BTreeMap, HashMap, HashSet};

    fn test_shader_with_imports(
        input: &str,
        defs: &[&str],
        imports: &HashMap<String, String>,
        output: &str,
        offsets: &BTreeMap<u32, u32>,
    ) {
        let processor = ShaderProcessor::default();
        let defs = HashSet::from_iter(defs.iter().map(|s| s.to_string()));
        let result = processor.process_inner(input, &defs, imports, &mut |_, _| {});

        pretty_assertions::assert_eq!(result.0, output);

        for (expected_pre, expected_post) in offsets {
            let actual_pre = result
                .1
                .translate_size(TextSize::from(*expected_post as u32));
            pretty_assertions::assert_eq!(expected_pre, &actual_pre.into());
        }
    }

    fn test_shader(input: &str, defs: &[&str], output: &str, offsets: &BTreeMap<u32, u32>) {
        test_shader_with_imports(input, defs, &HashMap::new(), output, offsets)
    }

    #[test]
    fn test_empty() {
        test_shader(
            r#"
"#,
            &[],
            r#"
"#,
            &BTreeMap::from([(0, 0)]),
        );
    }

    #[test]
    fn test_false_replace_str() {
        test_shader(
            r#"
.
#ifdef FALSE
IGNORE
#endif
.
"#,
            &[],
            r#"
.



.
"#,
            &BTreeMap::from([
                (0, 0),
                (1, 1),
                (2, 2),
                (3, 3),
                (16, 4),
                (23, 5),
                (30, 6),
                (31, 7),
                (32, 8),
            ]),
        );
    }

    #[test]
    fn test_resolve_import_offset() {
        test_shader_with_imports(
            r#"
.
#ifdef FALSE
this should not be imported
#import test::one
#endif
here is a actual import
#import test::two
.
"#,
            &[],
            &HashMap::from([
                (
                    "test::one".to_string(),
                    r#"
#define_import_path test::one
This is test one.
"#
                    .to_string(),
                ),
                (
                    "test::two".to_string(),
                    r#"
#define_import_path test::two
This is test two.
Here is another line.
The last line.
"#
                    .to_string(),
                ),
            ]),
            r#"
.




here is a actual import


This is test two.
Here is another line.
The last line.

.
"#,
            &BTreeMap::from([(0, 0), (1, 1), (2, 2), (93, 31), (93, 32), (113, 91)]),
        );
    }

    #[test]
    fn pbr_wgsl() {
        test_shader(
            r#"
#define_import_path bevy_pbr::mesh_view_bind_group

struct View {
    view_proj: mat4x4<f32>;
    view: mat4x4<f32>;
    inverse_view: mat4x4<f32>;
    projection: mat4x4<f32>;
    world_position: vec3<f32>;
    near: f32;
    far: f32;
    width: f32;
    height: f32;
};

struct PointLight {
    // NOTE: [2][2] [2][3] [3][2] [3][3]
    projection_lr: vec4<f32>;
    color_inverse_square_range: vec4<f32>;
    position_radius: vec4<f32>;
    // 'flags' is a bit field indicating various options. u32 is 32 bits so we have up to 32 options.
    flags: u32;
    shadow_depth_bias: f32;
    shadow_normal_bias: f32;
};

let POINT_LIGHT_FLAGS_SHADOWS_ENABLED_BIT: u32 = 1u;

struct DirectionalLight {
    view_projection: mat4x4<f32>;
    color: vec4<f32>;
    direction_to_light: vec3<f32>;
    // 'flags' is a bit field indicating various options. u32 is 32 bits so we have up to 32 options.
    flags: u32;
    shadow_depth_bias: f32;
    shadow_normal_bias: f32;
};

let DIRECTIONAL_LIGHT_FLAGS_SHADOWS_ENABLED_BIT: u32 = 1u;

struct Lights {
    // NOTE: this array size must be kept in sync with the constants defined bevy_pbr2/src/render/light.rs
    directional_lights: array<DirectionalLight, 1u>;
    ambient_color: vec4<f32>;
    // x/y/z dimensions and n_clusters in w
    cluster_dimensions: vec4<u32>;
    // xy are vec2<f32>(cluster_dimensions.xy) / vec2<f32>(view.width, view.height)
    //
    // For perspective projections:
    // z is cluster_dimensions.z / log(far / near)
    // w is cluster_dimensions.z * log(near) / log(far / near)
    //
    // For orthographic projections:
    // NOTE: near and far are +ve but -z is infront of the camera
    // z is -near
    // w is cluster_dimensions.z / (-far - -near)
    cluster_factors: vec4<f32>;
    n_directional_lights: u32;
};

#ifdef NO_STORAGE_BUFFERS_SUPPORT
struct PointLights {
    data: array<PointLight, 256u>;
};
struct ClusterLightIndexLists {
    // each u32 contains 4 u8 indices into the PointLights array
    data: array<vec4<u32>, 1024u>;
};
struct ClusterOffsetsAndCounts {
    // each u32 contains a 24-bit index into ClusterLightIndexLists in the high 24 bits
    // and an 8-bit count of the number of lights in the low 8 bits
    data: array<vec4<u32>, 1024u>;
};
#else
struct PointLights {
    data: array<PointLight>;
};
struct ClusterLightIndexLists {
    data: array<u32>;
};
struct ClusterOffsetsAndCounts {
    data: array<vec2<u32>>;
};
#endif

[[group(0), binding(0)]]
var<uniform> view: View;
[[group(0), binding(1)]]
var<uniform> lights: Lights;
#ifdef NO_ARRAY_TEXTURES_SUPPORT
[[group(0), binding(2)]]
var point_shadow_textures: texture_depth_cube;
#else
[[group(0), binding(2)]]
var point_shadow_textures: texture_depth_cube_array;
#endif
[[group(0), binding(3)]]
var point_shadow_textures_sampler: sampler_comparison;
#ifdef NO_ARRAY_TEXTURES_SUPPORT
[[group(0), binding(4)]]
var directional_shadow_textures: texture_depth_2d;
#else
[[group(0), binding(4)]]
var directional_shadow_textures: texture_depth_2d_array;
#endif
[[group(0), binding(5)]]
var directional_shadow_textures_sampler: sampler_comparison;

#ifdef NO_STORAGE_BUFFERS_SUPPORT
[[group(0), binding(6)]]
var<uniform> point_lights: PointLights;
[[group(0), binding(7)]]
var<uniform> cluster_light_index_lists: ClusterLightIndexLists;
[[group(0), binding(8)]]
var<uniform> cluster_offsets_and_counts: ClusterOffsetsAndCounts;
#else
[[group(0), binding(6)]]
var<storage> point_lights: PointLights;
[[group(0), binding(7)]]
var<storage> cluster_light_index_lists: ClusterLightIndexLists;
[[group(0), binding(8)]]
var<storage> cluster_offsets_and_counts: ClusterOffsetsAndCounts;
#endif
"#,
            &[],
            r#"


struct View {
    view_proj: mat4x4<f32>;
    view: mat4x4<f32>;
    inverse_view: mat4x4<f32>;
    projection: mat4x4<f32>;
    world_position: vec3<f32>;
    near: f32;
    far: f32;
    width: f32;
    height: f32;
};

struct PointLight {
    // NOTE: [2][2] [2][3] [3][2] [3][3]
    projection_lr: vec4<f32>;
    color_inverse_square_range: vec4<f32>;
    position_radius: vec4<f32>;
    // 'flags' is a bit field indicating various options. u32 is 32 bits so we have up to 32 options.
    flags: u32;
    shadow_depth_bias: f32;
    shadow_normal_bias: f32;
};

let POINT_LIGHT_FLAGS_SHADOWS_ENABLED_BIT: u32 = 1u;

struct DirectionalLight {
    view_projection: mat4x4<f32>;
    color: vec4<f32>;
    direction_to_light: vec3<f32>;
    // 'flags' is a bit field indicating various options. u32 is 32 bits so we have up to 32 options.
    flags: u32;
    shadow_depth_bias: f32;
    shadow_normal_bias: f32;
};

let DIRECTIONAL_LIGHT_FLAGS_SHADOWS_ENABLED_BIT: u32 = 1u;

struct Lights {
    // NOTE: this array size must be kept in sync with the constants defined bevy_pbr2/src/render/light.rs
    directional_lights: array<DirectionalLight, 1u>;
    ambient_color: vec4<f32>;
    // x/y/z dimensions and n_clusters in w
    cluster_dimensions: vec4<u32>;
    // xy are vec2<f32>(cluster_dimensions.xy) / vec2<f32>(view.width, view.height)
    //
    // For perspective projections:
    // z is cluster_dimensions.z / log(far / near)
    // w is cluster_dimensions.z * log(near) / log(far / near)
    //
    // For orthographic projections:
    // NOTE: near and far are +ve but -z is infront of the camera
    // z is -near
    // w is cluster_dimensions.z / (-far - -near)
    cluster_factors: vec4<f32>;
    n_directional_lights: u32;
};















struct PointLights {
    data: array<PointLight>;
};
struct ClusterLightIndexLists {
    data: array<u32>;
};
struct ClusterOffsetsAndCounts {
    data: array<vec2<u32>>;
};


[[group(0), binding(0)]]
var<uniform> view: View;
[[group(0), binding(1)]]
var<uniform> lights: Lights;




[[group(0), binding(2)]]
var point_shadow_textures: texture_depth_cube_array;

[[group(0), binding(3)]]
var point_shadow_textures_sampler: sampler_comparison;




[[group(0), binding(4)]]
var directional_shadow_textures: texture_depth_2d_array;

[[group(0), binding(5)]]
var directional_shadow_textures_sampler: sampler_comparison;









[[group(0), binding(6)]]
var<storage> point_lights: PointLights;
[[group(0), binding(7)]]
var<storage> cluster_light_index_lists: ClusterLightIndexLists;
[[group(0), binding(8)]]
var<storage> cluster_offsets_and_counts: ClusterOffsetsAndCounts;

"#,
            &BTreeMap::from([(0, 0), (3652, 2636)]),
        )
    }

    #[test]
    fn offset_table_translation() {
        let imports = HashMap::from([
            (
                "test::one".to_string(),
                r#"#define_import_path test::one
one"#
                    .to_string(),
            ),
            (
                "test::two".to_string(),
                r#"#define_import_path test::two
This is test two.
Here is another line.
The last line."#
                    .to_string(),
            ),
        ]);

        let input = r#"
here is the shorter import
#import test::one
here is a longer import
#import test::two
"#
        .to_string();

        let processor = ShaderProcessor::default();
        let (output, table) =
            processor.process_inner(&input, &HashSet::new(), &imports, &mut |_, _| {});

        {
            pretty_assertions::assert_eq!(&output[29..32], "one");
            pretty_assertions::assert_eq!(&input[28..45], "#import test::one");

            let processed_range = TextRange::new(TextSize::from(29), TextSize::from(32));
            let original_range = TextRange::new(TextSize::from(28), TextSize::from(45));

            let translated_range = table.translate_range(processed_range);

            pretty_assertions::assert_eq!(translated_range, original_range);
        }

        {
            pretty_assertions::assert_eq!(&output[85..97], "another line");
            pretty_assertions::assert_eq!(&input[70..87], "#import test::two");

            let processed_range = TextRange::new(TextSize::from(85), TextSize::from(97));
            let original_range = TextRange::new(TextSize::from(70), TextSize::from(87));

            let translated_range = table.translate_range(processed_range);

            pretty_assertions::assert_eq!(translated_range, original_range);
        }
    }
}
