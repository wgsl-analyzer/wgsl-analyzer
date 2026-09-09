// Predeclared vector aliases from https://www.w3.org/TR/WGSL/#vector-types

alias vec2i = vec2<i32>;
alias vec3i = vec3<i32>;
alias vec4i = vec4<i32>;
alias vec2u = vec2<u32>;
alias vec3u = vec3<u32>;
alias vec4u = vec4<u32>;
alias vec2f = vec2<f32>;
alias vec3f = vec3<f32>;
alias vec4f = vec4<f32>;
// vec2h and co are unconditionally included
// See https://github.com/wgsl-analyzer/wgsl-analyzer/issues/907
alias vec2h = vec2<f16>;
alias vec3h = vec3<f16>;
alias vec4h = vec4<f16>;

// Predeclared matrix aliases from https://www.w3.org/TR/WGSL/#matrix-types
alias mat2x2f = mat2x2<f32>;
alias mat2x3f = mat2x3<f32>;
alias mat2x4f = mat2x4<f32>;
alias mat3x2f = mat3x2<f32>;
alias mat3x3f = mat3x3<f32>;
alias mat3x4f = mat3x4<f32>;
alias mat4x2f = mat4x2<f32>;
alias mat4x3f = mat4x3<f32>;
alias mat4x4f = mat4x4<f32>;
// mat2x2h and co are unconditionally included
// See https://github.com/wgsl-analyzer/wgsl-analyzer/issues/907
alias mat2x2h = mat2x2<f16>;
alias mat2x3h = mat2x3<f16>;
alias mat2x4h = mat2x4<f16>;
alias mat3x2h = mat3x2<f16>;
alias mat3x3h = mat3x3<f16>;
alias mat3x4h = mat3x4<f16>;
alias mat4x2h = mat4x2<f16>;
alias mat4x3h = mat4x3<f16>;
alias mat4x4h = mat4x4<f16>;