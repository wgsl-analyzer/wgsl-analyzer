use std::{collections::HashSet, ops::Range};

use regex::Regex;
use std::sync::OnceLock;

pub fn get_shader_processor() -> &'static ShaderProcessor {
    static SHADER_PROCESSOR: OnceLock<ShaderProcessor> = OnceLock::new();
    SHADER_PROCESSOR.get_or_init(ShaderProcessor::default)
}

pub struct ShaderProcessor {
    ifdef_regex: Regex,
    ifndef_regex: Regex,
    else_regex: Regex,
    endif_regex: Regex,
    define_import_path_regex: Regex,
}

impl Default for ShaderProcessor {
    fn default() -> Self {
        Self {
            ifdef_regex: Regex::new(r"^\s*#\s*ifdef\s*([\w|\d|_]+)").unwrap(),
            ifndef_regex: Regex::new(r"^\s*#\s*ifndef\s*([\w|\d|_]+)").unwrap(),
            else_regex: Regex::new(r"^\s*#\s*else").unwrap(),
            endif_regex: Regex::new(r"^\s*#\s*endif").unwrap(),
            define_import_path_regex: Regex::new(r"^\s*#\s*define_import_path").unwrap(),
        }
    }
}

impl ShaderProcessor {
    pub fn process(
        &self,
        shader_str: &str,
        shader_defs: &HashSet<String>,
        mut emit_unconfigured: impl FnMut(Range<usize>, &str),
    ) -> String {
        self.process_inner(shader_str, shader_defs, &mut emit_unconfigured)
    }

    fn process_inner(
        &self,
        shader_str: &str,
        shader_defs: &HashSet<String>,
        emit_unconfigured: &mut dyn FnMut(Range<usize>, &str),
    ) -> String {
        let mut scopes = vec![(true, 0, "root scope")];
        let mut final_string = String::with_capacity(shader_str.len());

        for (line, offset) in lines_with_offsets(shader_str) {
            let use_line = if let Some(cap) = self.ifdef_regex.captures(line) {
                let def = cap.get(1).unwrap().as_str();
                scopes.push((
                    scopes.last().unwrap().0 && shader_defs.contains(def),
                    offset,
                    def,
                ));
                false
            } else if let Some(cap) = self.ifndef_regex.captures(line) {
                let def = cap.get(1).unwrap().as_str();
                scopes.push((
                    scopes.last().unwrap().0 && !shader_defs.contains(def),
                    offset,
                    def,
                ));
                false
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
                false
            } else if self.endif_regex.is_match(line) {
                // HACK: Ignore endifs without a corresponding
                // This does need proper error reporting somewhere, which is not yet implemented
                // Presumably this would be through a side channel
                if scopes.len() == 1 {
                    // return Err(ProcessShaderError::TooManyEndIfs);
                } else if let Some((used, start_offset, def)) = scopes.pop() {
                    if !used {
                        let range = start_offset..offset + line.len();
                        emit_unconfigured(range, def);
                    }
                }
                false
            } else if self.define_import_path_regex.is_match(line) {
                false
            } else {
                scopes.last().map(|&(used, _, _)| used).unwrap_or(true)
            };

            if use_line {
                final_string.push_str(line);
            } else {
                final_string.extend(std::iter::repeat(' ').take(line.len()));
            }

            final_string.push('\n');
        }

        if scopes.len() != 1 {
            // return Err(ProcessShaderError::NotEnoughEndIfs);
        }

        final_string
    }
}

fn lines_with_offsets(input: &str) -> impl Iterator<Item = (&str, usize)> {
    input.lines().scan(0, |offset, line| {
        let the_offset = *offset;
        *offset = the_offset + line.len() + 1;

        Some((line, the_offset))
    })
}

#[cfg(test)]
mod tests {
    use super::ShaderProcessor;
    use std::collections::HashSet;

    fn test_shader(input: &str, defs: &[&str], output: &str) {
        let processor = ShaderProcessor::default();
        let defs = HashSet::from_iter(defs.iter().map(|s| s.to_string()));
        let result = processor.process(input, &defs, |_, _| {});

        pretty_assertions::assert_eq!(result, output);
    }

    #[test]
    fn test_empty() {
        test_shader(
            r#"
"#,
            &[],
            r#"
"#,
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
        )
    }
}
