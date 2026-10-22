struct Camera {
    view_proj: mat4x4<f32>,
    position: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: Camera;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) padding1: f32,
    @location(2) velocity: vec3<f32>,
    @location(3) padding2: f32,
    @location(4) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) velocity: vec3<f32>,
};

@vertex
fn vs_main(
    vertex: VertexInput,
    @builtin(vertex_index) vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(vertex.position, 1.0);

    // Color based on color mode (handled in compute shader)
    out.color = vertex.color;
    out.velocity = vertex.velocity;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Simple circle point sprite
    let speed = length(in.velocity);
    let brightness = min(speed * 2.0, 1.0);

    return vec4<f32>(in.color.rgb * brightness, in.color.a);
}
