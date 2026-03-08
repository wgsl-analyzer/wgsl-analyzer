import package::shared::lighting::compute_sh_fill_lighting;
import package::shared::fog::compute_radial_fog_density;
import package::shared::fog::compute_fog;
import package::shared::fog::compute_planar_fog_density;
import package::shared::normal::compute_tbn;
import package::shared::normal::unpack_dxn_normal;
import package::shared::shadows::vsm_shadow;
import package::shared::blackmap::apply_blackmap;
import package::shared::local_lights::evaluate_omni_specular;

// Heightfield/Decal patch shader - WGSL implementation of terrainHeightField.fx
//
// Renders instanced quad patches ("decals") projected onto terrain.
// Features from original HLSL:
//   - Bilinear quad interpolation with forward/right vectors
//   - Terrain-conforming via heightfield texture sampling
//   - Full lit pipeline: directional + SH ambient
//   - Normal mapping with TBN reconstruction
//   - Specular (Blinn-Phong)
//   - Fog (radial + planar)
//   - Opacity-based alpha blending

struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct HeightfieldParams {
    world_to_heightfield: mat4x4<f32>,
    y_scale_ofs: vec4<f32>,          // x=yScale, y=yOfs, z=yLowLimit, w=yHighLimit
    sample_params: vec4<f32>,        // x=1/numTrisPerRow, y=numTrisPerRow, z=1/w, w=1/h
    camera_pos: vec4<f32>,
    dir_light_vec: vec4<f32>,        // xyz = toward light
    dir_light_color: vec4<f32>,      // rgb = color, a = shadow_darkness
    sh_fill_ar: vec4<f32>,
    sh_fill_ag: vec4<f32>,
    sh_fill_ab: vec4<f32>,
    sh_fill_br: vec4<f32>,
    sh_fill_bg: vec4<f32>,
    sh_fill_bb: vec4<f32>,
    sh_fill_c: vec4<f32>,
    fog_color: vec4<f32>,
    fog_params: vec4<f32>,           // x=density2, y=start2
    planar_fog_color: vec4<f32>,
    planar_fog_params: vec4<f32>,    // x=enabled, y=start, z=density2
    spec_power: vec4<f32>,           // x=spec_power, y=conform_to_terrain (0/1)
    // Shadow params
    shadow_vp_col0: vec4<f32>,
    shadow_vp_col1: vec4<f32>,
    shadow_vp_col2: vec4<f32>,
    shadow_vp_col3: vec4<f32>,
    shadow_params: vec4<f32>,       // x = csm_scale, y = num_passes, z = enabled
    // Blackmap params
    blackmap_params0: vec4<f32>,    // rgb = bg_color, w = fog_scalar
    blackmap_params1: vec4<f32>,    // x = unexplored_scalar, yz = bounds_lo_xz, w = enabled
    blackmap_params2: vec4<f32>,    // x = pad, yz = bounds_hi_xz, w = bounds_falloff
    // Local light params
    local_light_params: vec4<f32>,  // x = num_lights, y = spec_power
};
@group(1) @binding(0)
var<uniform> params: HeightfieldParams;

@group(1) @binding(1)
var t_heightfield: texture_2d<f32>;
@group(1) @binding(2)
var s_heightfield: sampler;

// Shadow, blackmap, and local light resources
@group(1) @binding(3)
var t_shadow_map: texture_2d<f32>;
@group(1) @binding(4)
var t_blackmap: texture_2d<f32>;
@group(1) @binding(5)
var t_unexplored: texture_2d<f32>;
@group(1) @binding(6)
var<storage, read> local_lights: array<vec4<f32>>;

@group(2) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(2) @binding(1)
var s_diffuse: sampler;
@group(2) @binding(2)
var t_normal: texture_2d<f32>;
@group(2) @binding(3)
var t_opacity: texture_2d<f32>;
@group(2) @binding(4)
var t_specular: texture_2d<f32>;

struct PatchInstance {
    @location(0) position: vec3<f32>,
    @location(1) forward: vec3<f32>,
    @location(2) right: vec3<f32>,
    @location(3) y_offset_intensity: vec2<f32>,
    @location(4) color: vec4<f32>,
    @location(5) tex_uv: vec4<f32>,
};

struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) tex_uv: vec4<f32>,
    @location(2) color: vec4<f32>,
    @location(3) world_pos: vec3<f32>,
    @location(4) normal: vec3<f32>,
    @location(5) tangent: vec3<f32>,
    @location(6) binormal: vec3<f32>,
    @location(7) fog_densities: vec2<f32>,
};

// --- Local helpers ---

fn heightfield_depth_to_y(s: vec2<f32>) -> vec2<f32> {
    return s * params.y_scale_ofs.x + params.y_scale_ofs.y;
}

// --- Vertex shader for instanced patch rendering ---

@vertex
fn vs_main(in: VertexInput, instance: PatchInstance) -> VertexOutput {
    var out: VertexOutput;

    // Generate quad vertices from vertex_index (0-5 for two triangles)
    var local_uv: vec2<f32>;
    let vi = in.vertex_index % 6u;
    if vi == 0u { local_uv = vec2<f32>(0.0, 0.0); }
    else if vi == 1u { local_uv = vec2<f32>(1.0, 0.0); }
    else if vi == 2u { local_uv = vec2<f32>(0.0, 1.0); }
    else if vi == 3u { local_uv = vec2<f32>(0.0, 1.0); }
    else if vi == 4u { local_uv = vec2<f32>(1.0, 0.0); }
    else { local_uv = vec2<f32>(1.0, 1.0); }

    // Interpolate position on quad using forward/right vectors
    let p = instance.position
        + instance.right * (local_uv.x * 2.0 - 1.0)
        + instance.forward * (local_uv.y * 2.0 - 1.0);

    var world_pos = vec4<f32>(p, 1.0);

    // Terrain conforming via heightfield sampling
    if params.spec_power.y > 0.5 {
        let hf_pos = params.world_to_heightfield * world_pos;
        let hf_uv = hf_pos.xy * params.sample_params.zw;
        let hf_sample = textureSampleLevel(t_heightfield, s_heightfield, hf_uv, 0.0);
        let lo_hi_y = heightfield_depth_to_y(hf_sample.xy);

        let center_hf_pos = params.world_to_heightfield * vec4<f32>(instance.position, 1.0);
        let center_uv = center_hf_pos.xy * params.sample_params.zw;
        let center_sample = textureSampleLevel(t_heightfield, s_heightfield, center_uv, 0.0);
        let center_lo_hi_y = heightfield_depth_to_y(center_sample.xy);

        var center_y = center_lo_hi_y.x;
        if abs(instance.position.y - center_lo_hi_y.y) < abs(instance.position.y - center_y) {
            center_y = center_lo_hi_y.y;
        }

        var sample_y = lo_hi_y.x;
        if abs(center_y - lo_hi_y.y) < abs(center_y - lo_hi_y.x) {
            sample_y = lo_hi_y.y;
        }

        world_pos.y = sample_y;
    }

    world_pos.y += instance.y_offset_intensity.x;

    // TBN - heightfield patches use up-facing normal (0,1,0)
    let normal = vec3<f32>(0.0, 1.0, 0.0);
    let tbn = compute_tbn(normal);

    // Fog
    let z_fog = compute_radial_fog_density(world_pos.xyz, params.camera_pos.xyz, params.fog_params.x, params.fog_params.y);
    var p_fog = 1.0;
    if params.planar_fog_params.x > 0.5 {
        p_fog = compute_planar_fog_density(world_pos.y, params.planar_fog_params.y, params.planar_fog_params.z);
    }

    // sRGB to linear for instance color
    let linear_color = pow(instance.color.rgb, vec3<f32>(2.2));

    out.clip_position = camera.view_proj * world_pos;
    out.uv = local_uv;
    out.tex_uv = instance.tex_uv;
    out.color = vec4<f32>(linear_color * instance.y_offset_intensity.y, instance.color.a);
    out.world_pos = world_pos.xyz;
    out.normal = normal;
    out.tangent = tbn[0];
    out.binormal = tbn[1];
    out.fog_densities = vec2<f32>(z_fog, p_fog);

    return out;
}

// --- Fragment shader with full lighting ---

const SPEC_TO_DIFFUSE_RATIO: f32 = 3.14159265;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Compute texture UV from instance UV rect + interpolated local UV
    let uv = vec2<f32>(
        in.tex_uv.x + in.uv.x * in.tex_uv.z,
        in.tex_uv.y + in.uv.y * in.tex_uv.w
    );

    // Sample textures
    let albedo = textureSample(t_diffuse, s_diffuse, uv).rgb;
    let spec_map = textureSample(t_specular, s_diffuse, uv).rgb;
    let opacity = textureSample(t_opacity, s_diffuse, uv).r;
    let tex_normal = unpack_dxn_normal(textureSample(t_normal, s_diffuse, uv));

    // Transform tangent-space normal to world space
    let world_normal = normalize(
        tex_normal.x * in.tangent +
        tex_normal.y * in.binormal +
        tex_normal.z * in.normal
    );

    // Directional lighting
    let n_dot_l = max(dot(world_normal, params.dir_light_vec.xyz), 0.0);
    var diffuse_sum = params.dir_light_color.rgb * n_dot_l;

    // Specular (Blinn-Phong)
    let view_dir = normalize(params.camera_pos.xyz - in.world_pos);
    let half_vec = normalize(params.dir_light_vec.xyz + view_dir);
    let n_dot_h = max(dot(world_normal, half_vec), 0.0);
    var spec_sum = params.dir_light_color.rgb * pow(n_dot_h, params.spec_power.x);

    // === Shadows (VSM) ===
    var shadow_factor = 1.0;
    if params.shadow_params.z > 0.5 {
        let shadow_vp = mat4x4<f32>(
            params.shadow_vp_col0,
            params.shadow_vp_col1,
            params.shadow_vp_col2,
            params.shadow_vp_col3,
        );
        let shadow_pos = shadow_vp * vec4<f32>(in.world_pos, 1.0);
        let shadow_ndc = shadow_pos.xyz / shadow_pos.w;
        let shadow_uv = vec2<f32>(shadow_ndc.x * 0.5 + 0.5, shadow_ndc.y * -0.5 + 0.5);
        let shadow_z = clamp(shadow_ndc.z, 0.0, 1.0);
        shadow_factor = vsm_shadow(shadow_uv, shadow_z, params.dir_light_color.a, t_shadow_map, s_diffuse);
    }
    diffuse_sum *= shadow_factor;
    spec_sum *= shadow_factor;

    // SH ambient
    let ambient_sum = compute_sh_fill_lighting(world_normal, params.sh_fill_ar, params.sh_fill_ag, params.sh_fill_ab, params.sh_fill_br, params.sh_fill_bg, params.sh_fill_bb, params.sh_fill_c);

    // === Local lights ===
    var local_diffuse = vec3<f32>(0.0);
    var local_spec = 0.0;
    let num_lights = u32(params.local_light_params.x);
    let local_spec_power = params.local_light_params.y;
    let world_reflect = reflect(-view_dir, world_normal);
    for (var i = 0u; i < num_lights; i++) {
        let base = i * 8u;
        let light_pos = local_lights[base].xyz;
        let omni_mul = local_lights[base].w;
        let light_color = local_lights[base + 1u].xyz;
        let omni_add = local_lights[base + 1u].w;
        let decay_dist = local_lights[base + 2u].x;
        let spec_int = local_lights[base + 3u].w;
        let result = evaluate_omni_specular(
            light_pos, light_color, omni_mul, omni_add, decay_dist, spec_int,
            in.world_pos, world_normal, world_reflect, local_spec_power,
        );
        local_diffuse += result.xyz;
        local_spec += result.w;
    }

    // Combine lighting
    diffuse_sum += ambient_sum + local_diffuse;
    var lit_color = diffuse_sum * albedo;
    lit_color += (spec_sum * spec_map + vec3<f32>(local_spec)) * SPEC_TO_DIFFUSE_RATIO;

    // === Blackmap (fog-of-war) ===
    if params.blackmap_params1.w > 0.5 {
        let bm_uv = in.uv;
        let vis_factor = textureSample(t_blackmap, s_diffuse, bm_uv).a;
        let unexplored_sample = textureSample(t_unexplored, s_diffuse, bm_uv);
        lit_color = apply_blackmap(
            lit_color, in.world_pos, vis_factor, unexplored_sample,
            params.blackmap_params0.xyz,
            params.blackmap_params0.w,
            params.blackmap_params1.x,
            params.blackmap_params1.yz,
            params.blackmap_params2.yz,
            params.blackmap_params2.w,
        );
    }

    // Apply fog
    lit_color = compute_fog(lit_color, in.fog_densities.x, in.fog_densities.y, params.fog_color.xyz, params.planar_fog_color.xyz);

    return vec4<f32>(lit_color, in.color.a * opacity);
}
