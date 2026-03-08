enable f16;
requires unrestricted_pointer_parameters;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) tangent: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) @interpolate(flat) instance_index: u32,
}

struct PBRMaterial {
    base_color: vec4<f32>,
    metallic: f32,
    roughness: f32,
    ao: f32,
    emissive: vec3<f32>,
    emissive_strength: f32,
}

struct LightData {
    position: vec3<f32>,
    color: vec3<f32>,
    intensity: f32,
    radius: f32,
}

struct SceneUniforms {
    view_proj: mat4x4<f32>,
    camera_pos: vec3<f32>,
    time: f32,
    num_lights: u32,
}

@group(0) @binding(0) var<uniform> scene: SceneUniforms;
@group(0) @binding(1) var<storage,read> lights: array<LightData>;
@group(1) @binding(0) var<uniform> material: PBRMaterial;
@group(1) @binding(1) var t_albedo: texture_2d<f32>;
@group(1) @binding(2) var t_normal: texture_2d<f32>;
@group(1) @binding(3) var t_metallic_roughness: texture_2d<f32>;
@group(1) @binding(4) var s_linear: sampler;

const PI: f32 = 3.14159265359;
const EPSILON: f32 = 0.0001;
alias Vec3F = vec3<f32>;
alias Vec4F = vec4<f32>;
override MAX_LIGHTS: u32 = 128u;

fn distribution_ggx(N: vec3<f32>, H: vec3<f32>, roughness: f32) -> f32 {
    let a = roughness * roughness;
    let a2 = a * a;
    let NdotH = max(dot(N, H), 0.0);
    let NdotH2 = NdotH * NdotH;
    let num = a2;
    var denom = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;
    return num / denom;
}

fn geometry_schlick_ggx(NdotV: f32, roughness: f32) -> f32 {
    let r = (roughness + 1.0);
    let k = (r * r) / 8.0;
    let num = NdotV;
    let denom = NdotV * (1.0 - k) + k;
    return num / denom;
}

fn geometry_smith(N: vec3<f32>, V: vec3<f32>, L: vec3<f32>, roughness: f32) -> f32 {
    let NdotV = max(dot(N, V), 0.0);
    let NdotL = max(dot(N, L), 0.0);
    let ggx2 = geometry_schlick_ggx(NdotV, roughness);
    let ggx1 = geometry_schlick_ggx(NdotL, roughness);
    return ggx1 * ggx2;
}

fn fresnel_schlick(cosTheta: f32, F0: vec3<f32>) -> vec3<f32> {
    return F0 + (1.0 - F0) * pow(clamp(1.0 - cosTheta, 0.0, 1.0), 5.0);
}

@vertex fn vs_main(in: VertexInput, @builtin(instance_index) instance: u32) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = scene.view_proj * vec4<f32>(in.position, 1.0);
    out.world_position = in.position;
    out.world_normal = in.normal;
    out.uv = in.uv;
    out.instance_index = instance;
    return out;
}

@fragment fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let albedo = textureSample(t_albedo, s_linear, in.uv).rgb * material.base_color.rgb;
    let mr = textureSample(t_metallic_roughness, s_linear, in.uv);
    let metallic = mr.b * material.metallic;
    let roughness = mr.g * material.roughness;

    let N = normalize(in.world_normal);
    let V = normalize(scene.camera_pos - in.world_position);
    var F0 = vec3<f32>(0.04);
    F0 = mix(F0, albedo, metallic);

    var Lo = vec3<f32>(0.0);
    for (var i: u32 = 0u; i < scene.num_lights; i += 1u) {
        if i >= MAX_LIGHTS { break; }
        let light = lights[i];
        let L = normalize(light.position - in.world_position);
        let H = normalize(V + L);
        let distance = length(light.position - in.world_position);
        let attenuation = 1.0 / (distance * distance);
        let radiance = light.color * light.intensity * attenuation;

        let NDF = distribution_ggx(N, H, roughness);
        let G = geometry_smith(N, V, L, roughness);
        let F = fresnel_schlick(max(dot(H, V), 0.0), F0);

        let kS = F;
        let kD = (vec3<f32>(1.0) - kS) * (1.0 - metallic);
        let NdotL = max(dot(N, L), 0.0);
        let numerator = NDF * G * F;
        let denominator = 4.0 * max(dot(N, V), 0.0) * NdotL + EPSILON;
        let specular = numerator / denominator;

        Lo += (kD * albedo / PI + specular) * radiance * NdotL;
    }

    let ambient = vec3<f32>(0.03) * albedo * material.ao;
    let color = ambient + Lo + material.emissive * material.emissive_strength;
    let mapped = color / (color + vec3<f32>(1.0));
    let gamma_corrected = pow(mapped, vec3<f32>(1.0 / 2.2));
    return vec4<f32>(gamma_corrected, material.base_color.a);
}
