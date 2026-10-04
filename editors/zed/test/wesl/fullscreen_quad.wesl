struct QuadVertex {
    @builtin(position) position: vec4f,
    @location(0) uv: vec2f
}

fn fullscreen_quad(index: u32) -> QuadVertex {
    const pos = array(
        vec2(1.0, 1.0),
        vec2(1.0, -1.0),
        vec2(-1.0, -1.0),
        vec2(1.0, 1.0),
        vec2(-1.0, -1.0),
        vec2(-1.0, 1.0),
    );
    const uv = array(
        vec2(1.0, 0.0),
        vec2(1.0, 1.0),
        vec2(0.0, 1.0),
        vec2(1.0, 0.0),
        vec2(0.0, 1.0),
        vec2(0.0, 0.0),
    );
    var output: QuadVertex;
    output.position = vec4f(pos[index], 0.0, 1.0);
    output.uv = uv[index];
    return output;
}
