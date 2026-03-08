// A comprehensive badly-formatted WGSL file for testing the formatter
// Covers every major language feature

/* Block comment
   spanning multiple lines */

// ── Enable / diagnostic directives ──
enable f16;
diagnostic(off,derivative_uniformity);

// ── Override declarations ──
override block_size: u32 = 64;
  override brightness_factor: f32;

// ── Constants ──
const PI: f32 = 3.14159265;
const TAU: f32 = PI * 2.0;
const MAX_LIGHTS: u32 = 16u;
const HEX_VAL: u32 = 0xFF00u;

// ── const_assert ──
const_assert MAX_LIGHTS <= 128u;

// ── Type aliases ──
alias Vec3F = vec3<f32>;
alias Mat4 = mat4x4<f32>;

// ── Structs (with and without trailing semicolons) ──
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) uv: vec2<f32>,
}

struct Light {
    position: vec3<f32>,
    color: vec3<f32>,
    intensity: f32,
    range: f32,
}

struct CameraUniform {
    view_proj: mat4x4<f32>,
    eye_position: vec3<f32>,
};

struct LightingResult {
    diffuse: vec3<f32>,
    specular: vec3<f32>,
}

// ── Global variables: uniform, storage, texture, sampler ──
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(0) @binding(1)
var<storage,read> lights: array<Light>;

@group(0) @binding(2)
var<storage, read_write> output_data: array<f32>;

@group(1) @binding(0) var diffuse_texture: texture_2d<f32>;
@group(1) @binding(1) var diffuse_sampler: sampler;
@group(1) @binding(2) var depth_texture: texture_depth_2d;

// ── Private and workgroup variables ──
var<private> seed: u32 = 0u;
var<workgroup> shared_data: array<f32,256>;

// ── Vertex shader ──
@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    out.color = model.color;
    out.uv = model.uv;
    return out;
}

// ── Fragment shader with return-type attribute ──
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let tex_color = textureSample(diffuse_texture, diffuse_sampler, in.uv);
    let final_color = tex_color.rgb * in.color;
    return vec4<f32>(final_color, tex_color.a);
}

// ── Compute shader ──
@compute @workgroup_size(64)
fn cs_main(@builtin(global_invocation_id) global_id: vec3<u32>, @builtin(local_invocation_id) local_id: vec3<u32>) {
    let index = global_id.x;
    if index >= arrayLength(&output_data) { return; }
    output_data[index] = output_data[index] * 2.0 + brightness_factor;
}

// ── If / else if / else chains ──
fn classify_value(x: f32) -> i32 {
    if x < 0.0 { return -1; } else if x > 0.0 { return 1; } else { return 0; }
}

// ── For loops ──
fn sum_array(count: u32) -> f32 {
    var total: f32 = 0.0;
    for (var i: u32 = 0u; i < count; i = i + 1u) {
        total = total + output_data[i];
    }
    return total;
}

// ── While loops ──
fn find_first_nonzero(count: u32) -> u32 {
    var i: u32 = 0u;
    while i < count {
        if output_data[i] != 0.0 { return i; }
        i = i + 1u;
    }
    return count;
}

// ── Loop with break and continuing ──
fn loop_example() -> u32 {
    var i: u32 = 0u;
    loop {
        if i >= 10u { break; }
        i = i + 1u;
        continuing {
            // continuing block
        }
    }
    return i;
}

// ── Switch statement ──
fn switch_example(val: u32) -> f32 {
    var result: f32 = 0.0;
    switch val {
        case 0u: { result = 1.0; }
        case 1u, 2u: { result = 2.0; }
        default: { result = -1.0; }
    }
    return result;
}

// ── Nested generics and complex types ──
fn nested_generics() -> vec4<f32> {
    let a: array<vec3<f32>,4> = array<vec3<f32>,4>(vec3<f32>(1.0, 0.0, 0.0), vec3<f32>(0.0, 1.0, 0.0), vec3<f32>(0.0, 0.0, 1.0), vec3<f32>(1.0, 1.0, 1.0));
    return vec4<f32>(a[0], 1.0);
}

// ── Pointer parameters ──
fn increment(p: ptr<function,f32>) {
    *p = *p + 1.0;
}

fn use_pointer() -> f32 {
    var x: f32 = 5.0;
    increment(&x);
    return x;
}

// ── Matrix operations ──
fn transform_normal(n: vec3<f32>, model_matrix: mat4x4<f32>) -> vec3<f32> {
    let world_normal = (model_matrix * vec4<f32>(n, 0.0)).xyz;
    return normalize(world_normal);
}

// ── Bitwise operations ──
fn pack_color(r: u32, g: u32, b: u32, a: u32) -> u32 {
    return (a << 24u) | (r << 16u) | (g << 8u) | b;
}

fn unpack_red(packed: u32) -> u32 {
    return (packed >> 16u) & 0xFFu;
}

// ── Built-in math functions ──
fn lighting_calc(normal: vec3<f32>, light_dir: vec3<f32>, view_dir: vec3<f32>) -> LightingResult {
    let n_dot_l = max(dot(normal, light_dir), 0.0);
    let reflect_dir = reflect(-light_dir, normal);
    let spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32.0);
    var result: LightingResult;
    result.diffuse = vec3<f32>(n_dot_l);
    result.specular = vec3<f32>(spec);
    return result;
}

// ── Discard in fragment shader ──
@fragment
fn alpha_test_fs(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let color = textureSample(diffuse_texture, diffuse_sampler, uv);
    if color.a < 0.5 { discard; }
    return color;
}

// ── Multiple attributes on compute ──
@compute
@workgroup_size(8,8,1)
fn cs_2d(@builtin(global_invocation_id) id: vec3<u32>) {
    let x = id.x;
    let y = id.y;
    let idx = y * 256u + x;
    shared_data[idx % 256u] = f32(idx);
}
