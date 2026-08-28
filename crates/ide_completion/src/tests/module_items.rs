//! Test for each completion "kind" works to provide basic coverage.

#![expect(clippy::too_many_lines, reason = "snapshot test data")]

use crate::tests::{check, completion_list};
use expect_test::expect;

#[test]
fn complete_struct_field() {
    check(
        "
        struct Foo { bar: u32 }
        fn test() {
            let test = Foo(0);
            let x = test.$0;
        }
        ",
        expect![[r#"
            field bar
        "#]],
    );
}

#[test]
fn complete_builtin_field() {
    check(
        "
        fn test() {
            let test = modf(0.0);
            let x = test.$0;
        }
        ",
        expect![[r#"
            field fract
            field whole
        "#]],
    );
}

#[test]
fn complete_vec_field() {
    check(
        "
        fn test() {
            let test = vec2(0);
            let x = test.$0;
        }
        ",
        expect![[r#"
            field g
            field r
            field x
            field y
        "#]],
    );
}

#[test]
fn complete_function() {
    (
        completion_list(
            "
            fn foo() { }
            fn bar() { $0 }
            ",
        ),
        expect![["function abs
function acos
function acosh
function all
function any
function arrayLength
function asin
function atan
function atanh
function atan2
function atomicAdd
function atomicAnd
function atomicExchange
function atomicLoad
function atomicMax
function atomicMin
function atomicOr
function atomicStore
function atomicSub
function atomicXor
function bar                 fn bar()
function bitcast
function ceil
function clamp
function cos
function cosh
function countLeadingZeros
function countOneBits
function countTrailingZeros
function cross
function degrees
function determinant
function distance
function dot
function dpdx
function dpdxCoarse
function dpdxFine
function dpdy
function dpdyCoarse
function dpdyFine
function exp
function exp2
function extractBits
function faceForward
function firstLeadingBit
function firstTrailingBit
function floor
function fma
function foo                 fn foo()
function fract
function fwidth
function fwidthCoarse
function fwidthFine
function insertBits
function inverseSqrt
function isFinite
function isInf
function isNan
function isNormal
function length
function log
function log2
function max
function min
function mix
function normalize
function pack2x16float
function pack2x16snorm
function pack2x16unorm
function pack4x8snorm
function pack4x8unorm
function pow
function quantizeToF16
function radians
function reflect
function refract
function reverseBits
function round
function saturate
function select
function sign
function sin
function sinh
function smoothstep
function sqrt
function step
function storageBarrier
function tan
function tanh
function textureDimensions
function textureGather
function textureGatherCompare
function textureLoad
function textureNumLayers
function textureNumLevels
function textureNumSamples
function textureSample
function textureSampleBaseClampToEdge
function textureSampleBias
function textureSampleCompare
function textureSampleCompareLevel
function textureSampleGrad
function textureSampleLevel
function textureStore
function transpose
function trunc
function unpack2x16float
function unpack2x16snorm
function unpack2x16unorm
function unpack4x8snorm
function unpack4x8unorm
function workgroupBarrier
function workgroupUniformLoad
"]],
    );
}

#[test]
fn complete_variable() {
    check(
        "
            fn test() {
                let test = 0;
                let x = $0;
            }
            ",
        expect![[r#"
            builtin declaration RAY_FLAG_CULL_BACK_FACING
            builtin declaration RAY_FLAG_CULL_FRONT_FACING
            builtin declaration RAY_FLAG_CULL_NO_OPAQUE
            builtin declaration RAY_FLAG_CULL_OPAQUE
            builtin declaration RAY_FLAG_FORCE_NO_OPAQUE
            builtin declaration RAY_FLAG_FORCE_OPAQUE
            builtin declaration RAY_FLAG_NONE
            builtin declaration RAY_FLAG_SKIP_AABBS
            builtin declaration RAY_FLAG_SKIP_CLOSEST_HIT_SHADER
            builtin declaration RAY_FLAG_SKIP_TRIANGLES
            builtin declaration RAY_FLAG_TERMINATE_ON_FIRST_HIT
            builtin declaration RAY_QUERY_INTERSECTION_AABB
            builtin declaration RAY_QUERY_INTERSECTION_GENERATED
            builtin declaration RAY_QUERY_INTERSECTION_NONE
            builtin declaration RAY_QUERY_INTERSECTION_TRIANGLE
            builtin constructor RayDesc
            builtin constructor RayIntersection
            builtin function abs
            builtin type acceleration_structure
            builtin function acos
            builtin function acosh
            builtin function all
            builtin function any
            builtin constructor array
            builtin type generator array
            builtin function arrayLength
            builtin function asin
            builtin function asinh
            builtin function atan
            builtin function atan2
            builtin function atanh
            builtin type generator atomic
            builtin function atomicAdd
            builtin function atomicAnd
            builtin function atomicCompareExchangeWeak
            builtin function atomicExchange
            builtin function atomicLoad
            builtin function atomicMax
            builtin function atomicMin
            builtin function atomicOr
            builtin function atomicStore
            builtin function atomicSub
            builtin function atomicXor
            builtin enumerant bgra8unorm
            builtin type generator binding_array
            builtin function bitcast
            builtin constructor bool
            builtin type bool
            builtin function ceil
            builtin function clamp
            builtin function cos
            builtin function cosh
            builtin function countLeadingZeros
            builtin function countOneBits
            builtin function countTrailingZeros
            builtin function cross
            builtin function degrees
            builtin function determinant
            builtin function distance
            builtin function dot
            builtin function dot4I8Packed
            builtin function dot4U8Packed
            builtin function dpdx
            builtin function dpdxCoarse
            builtin function dpdxFine
            builtin function dpdy
            builtin function dpdyCoarse
            builtin function dpdyFine
            builtin function exp
            builtin function exp2
            builtin function extractBits
            builtin constructor f16
            builtin type f16
            builtin constructor f32
            builtin type f32
            builtin constructor f64
            builtin type f64
            builtin function faceForward
            builtin function firstLeadingBit
            builtin function firstTrailingBit
            builtin function floor
            builtin function fma
            builtin function fract
            builtin function frexp
            builtin enumerant function
            builtin function fwidth
            builtin function fwidthCoarse
            builtin function fwidthFine
            builtin function getCandidateHitVertexPositions
            builtin function getCommittedHitVertexPositions
            builtin constructor i32
            builtin type i32
            builtin constructor i64
            builtin type i64
            builtin enumerant immediate
            builtin enumerant incoming_ray_payload
            builtin function insertBits
            builtin function inverseSqrt
            builtin function ldexp
            builtin function length
            builtin function log
            builtin function log2
            builtin constructor mat2x2
            builtin type generator mat2x2
            builtin alias mat2x2f
            builtin constructor mat2x2f
            builtin alias mat2x2h
            builtin constructor mat2x2h
            builtin constructor mat2x3
            builtin type generator mat2x3
            builtin alias mat2x3f
            builtin constructor mat2x3f
            builtin alias mat2x3h
            builtin constructor mat2x3h
            builtin constructor mat2x4
            builtin type generator mat2x4
            builtin alias mat2x4f
            builtin constructor mat2x4f
            builtin alias mat2x4h
            builtin constructor mat2x4h
            builtin constructor mat3x2
            builtin type generator mat3x2
            builtin alias mat3x2f
            builtin constructor mat3x2f
            builtin alias mat3x2h
            builtin constructor mat3x2h
            builtin constructor mat3x3
            builtin type generator mat3x3
            builtin alias mat3x3f
            builtin constructor mat3x3f
            builtin alias mat3x3h
            builtin constructor mat3x3h
            builtin constructor mat3x4
            builtin type generator mat3x4
            builtin alias mat3x4f
            builtin constructor mat3x4f
            builtin alias mat3x4h
            builtin constructor mat3x4h
            builtin constructor mat4x2
            builtin type generator mat4x2
            builtin alias mat4x2f
            builtin constructor mat4x2f
            builtin alias mat4x2h
            builtin constructor mat4x2h
            builtin constructor mat4x3
            builtin type generator mat4x3
            builtin alias mat4x3f
            builtin constructor mat4x3f
            builtin alias mat4x3h
            builtin constructor mat4x3h
            builtin constructor mat4x4
            builtin type generator mat4x4
            builtin alias mat4x4f
            builtin constructor mat4x4f
            builtin alias mat4x4h
            builtin constructor mat4x4h
            builtin function max
            builtin function min
            builtin function mix
            builtin function modf
            builtin function normalize
            builtin function pack2x16float
            builtin function pack2x16snorm
            builtin function pack2x16unorm
            builtin function pack4x8snorm
            builtin function pack4x8unorm
            builtin function pack4xI8
            builtin function pack4xI8Clamp
            builtin function pack4xU8
            builtin function pack4xU8Clamp
            builtin function pow
            builtin enumerant private
            builtin type generator ptr
            builtin function quadBroadcast
            builtin function quadSwapDiagonal
            builtin function quadSwapX
            builtin function quadSwapY
            builtin function quantizeToF16
            builtin enumerant r16float
            builtin enumerant r16sint
            builtin enumerant r16snorm
            builtin enumerant r16uint
            builtin enumerant r16unorm
            builtin enumerant r32float
            builtin enumerant r32sint
            builtin enumerant r32uint
            builtin enumerant r64uint
            builtin enumerant r8sint
            builtin enumerant r8snorm
            builtin enumerant r8uint
            builtin enumerant r8unorm
            builtin function radians
            builtin function rayQueryConfirmIntersection
            builtin function rayQueryGenerateIntersection
            builtin function rayQueryGetCandidateIntersection
            builtin function rayQueryGetCommittedIntersection
            builtin function rayQueryInitialize
            builtin function rayQueryProceed
            builtin function rayQueryTerminate
            builtin enumerant ray_payload
            builtin type ray_query
            builtin enumerant read
            builtin enumerant read_write
            builtin function reflect
            builtin function refract
            builtin function reverseBits
            builtin enumerant rg11b10float
            builtin enumerant rg16float
            builtin enumerant rg16sint
            builtin enumerant rg16snorm
            builtin enumerant rg16uint
            builtin enumerant rg16unorm
            builtin enumerant rg32float
            builtin enumerant rg32sint
            builtin enumerant rg32uint
            builtin enumerant rg8sint
            builtin enumerant rg8snorm
            builtin enumerant rg8uint
            builtin enumerant rg8unorm
            builtin enumerant rgb10a2uint
            builtin enumerant rgb10a2unorm
            builtin enumerant rgba16float
            builtin enumerant rgba16sint
            builtin enumerant rgba16snorm
            builtin enumerant rgba16uint
            builtin enumerant rgba16unorm
            builtin enumerant rgba32float
            builtin enumerant rgba32sint
            builtin enumerant rgba32uint
            builtin enumerant rgba8sint
            builtin enumerant rgba8snorm
            builtin enumerant rgba8uint
            builtin enumerant rgba8unorm
            builtin function round
            builtin type sampler
            builtin type sampler_comparison
            builtin function saturate
            builtin function select
            builtin function sign
            builtin function sin
            builtin function sinh
            builtin function smoothstep
            builtin function sqrt
            builtin function step
            builtin enumerant storage
            builtin function storageBarrier
            builtin function subgroupAdd
            builtin function subgroupAll
            builtin function subgroupAnd
            builtin function subgroupAny
            builtin function subgroupBallot
            builtin function subgroupBroadcast
            builtin function subgroupBroadcastFirst
            builtin function subgroupElect
            builtin function subgroupExclusiveAdd
            builtin function subgroupExclusiveMul
            builtin function subgroupInclusiveAdd
            builtin function subgroupInclusiveMul
            builtin function subgroupMax
            builtin function subgroupMin
            builtin function subgroupMul
            builtin function subgroupOr
            builtin function subgroupShuffle
            builtin function subgroupShuffleDown
            builtin function subgroupShuffleUp
            builtin function subgroupShuffleXor
            builtin function subgroupXor
            builtin function tan
            builtin function tanh
            builtin enumerant task_payload
            function test                   fn test()
            variable test                         i32
            builtin function textureBarrier
            builtin function textureDimensions
            builtin function textureGather
            builtin function textureGatherCompare
            builtin function textureLoad
            builtin function textureNumLayers
            builtin function textureNumLevels
            builtin function textureNumSamples
            builtin function textureSample
            builtin function textureSampleBaseClampToEdge
            builtin function textureSampleBias
            builtin function textureSampleCompare
            builtin function textureSampleCompareLevel
            builtin function textureSampleGrad
            builtin function textureSampleLevel
            builtin function textureStore
            builtin type generator texture_1d
            builtin type generator texture_1d_array
            builtin type generator texture_2d
            builtin type generator texture_2d_array
            builtin type generator texture_3d
            builtin type generator texture_cube
            builtin type generator texture_cube_array
            builtin type texture_depth_2d
            builtin type texture_depth_2d_array
            builtin type texture_depth_cube
            builtin type texture_depth_cube_array
            builtin type texture_depth_multisampled_2d
            builtin type texture_external
            builtin type generator texture_multisampled_2d
            builtin type generator texture_multisampled_2d_array
            builtin type generator texture_storage_1d
            builtin type generator texture_storage_1d_array
            builtin type generator texture_storage_2d
            builtin type generator texture_storage_2d_array
            builtin type generator texture_storage_3d
            builtin function traceRay
            builtin function transpose
            builtin function trunc
            builtin constructor u32
            builtin type u32
            builtin constructor u64
            builtin type u64
            builtin enumerant uniform
            builtin function unpack2x16float
            builtin function unpack2x16snorm
            builtin function unpack2x16unorm
            builtin function unpack4x8snorm
            builtin function unpack4x8unorm
            builtin function unpack4xI8
            builtin function unpack4xU8
            builtin constructor vec2
            builtin type generator vec2
            builtin alias vec2f
            builtin constructor vec2f
            builtin alias vec2h
            builtin constructor vec2h
            builtin alias vec2i
            builtin constructor vec2i
            builtin alias vec2u
            builtin constructor vec2u
            builtin constructor vec3
            builtin type generator vec3
            builtin alias vec3f
            builtin constructor vec3f
            builtin alias vec3h
            builtin constructor vec3h
            builtin alias vec3i
            builtin constructor vec3i
            builtin alias vec3u
            builtin constructor vec3u
            builtin constructor vec4
            builtin type generator vec4
            builtin alias vec4f
            builtin constructor vec4f
            builtin alias vec4h
            builtin constructor vec4h
            builtin alias vec4i
            builtin constructor vec4i
            builtin alias vec4u
            builtin constructor vec4u
            builtin enumerant vertex_return
            builtin enumerant workgroup
            builtin function workgroupBarrier
            builtin function workgroupUniformLoad
            builtin enumerant write
        "#]],
    );
}

#[test]
// TODO https://github.com/wgsl-analyzer/wgsl-analyzer/issues/314
fn complete_keyword() {
    check(
        "
            fn test() {
                $0
            }
            ",
        expect![[r#"
            builtin declaration RAY_FLAG_CULL_BACK_FACING
            builtin declaration RAY_FLAG_CULL_FRONT_FACING
            builtin declaration RAY_FLAG_CULL_NO_OPAQUE
            builtin declaration RAY_FLAG_CULL_OPAQUE
            builtin declaration RAY_FLAG_FORCE_NO_OPAQUE
            builtin declaration RAY_FLAG_FORCE_OPAQUE
            builtin declaration RAY_FLAG_NONE
            builtin declaration RAY_FLAG_SKIP_AABBS
            builtin declaration RAY_FLAG_SKIP_CLOSEST_HIT_SHADER
            builtin declaration RAY_FLAG_SKIP_TRIANGLES
            builtin declaration RAY_FLAG_TERMINATE_ON_FIRST_HIT
            builtin declaration RAY_QUERY_INTERSECTION_AABB
            builtin declaration RAY_QUERY_INTERSECTION_GENERATED
            builtin declaration RAY_QUERY_INTERSECTION_NONE
            builtin declaration RAY_QUERY_INTERSECTION_TRIANGLE
            builtin constructor RayDesc
            builtin constructor RayIntersection
            builtin function abs
            builtin type acceleration_structure
            builtin function acos
            builtin function acosh
            builtin function all
            builtin function any
            builtin constructor array
            builtin type generator array
            builtin function arrayLength
            builtin function asin
            builtin function asinh
            builtin function atan
            builtin function atan2
            builtin function atanh
            builtin type generator atomic
            builtin function atomicAdd
            builtin function atomicAnd
            builtin function atomicCompareExchangeWeak
            builtin function atomicExchange
            builtin function atomicLoad
            builtin function atomicMax
            builtin function atomicMin
            builtin function atomicOr
            builtin function atomicStore
            builtin function atomicSub
            builtin function atomicXor
            builtin enumerant bgra8unorm
            builtin type generator binding_array
            builtin function bitcast
            builtin constructor bool
            builtin type bool
            builtin function ceil
            builtin function clamp
            builtin function cos
            builtin function cosh
            builtin function countLeadingZeros
            builtin function countOneBits
            builtin function countTrailingZeros
            builtin function cross
            builtin function degrees
            builtin function determinant
            builtin function distance
            builtin function dot
            builtin function dot4I8Packed
            builtin function dot4U8Packed
            builtin function dpdx
            builtin function dpdxCoarse
            builtin function dpdxFine
            builtin function dpdy
            builtin function dpdyCoarse
            builtin function dpdyFine
            builtin function exp
            builtin function exp2
            builtin function extractBits
            builtin constructor f16
            builtin type f16
            builtin constructor f32
            builtin type f32
            builtin constructor f64
            builtin type f64
            builtin function faceForward
            builtin function firstLeadingBit
            builtin function firstTrailingBit
            builtin function floor
            builtin function fma
            builtin function fract
            builtin function frexp
            builtin enumerant function
            builtin function fwidth
            builtin function fwidthCoarse
            builtin function fwidthFine
            builtin function getCandidateHitVertexPositions
            builtin function getCommittedHitVertexPositions
            builtin constructor i32
            builtin type i32
            builtin constructor i64
            builtin type i64
            builtin enumerant immediate
            builtin enumerant incoming_ray_payload
            builtin function insertBits
            builtin function inverseSqrt
            builtin function ldexp
            builtin function length
            builtin function log
            builtin function log2
            builtin constructor mat2x2
            builtin type generator mat2x2
            builtin alias mat2x2f
            builtin constructor mat2x2f
            builtin alias mat2x2h
            builtin constructor mat2x2h
            builtin constructor mat2x3
            builtin type generator mat2x3
            builtin alias mat2x3f
            builtin constructor mat2x3f
            builtin alias mat2x3h
            builtin constructor mat2x3h
            builtin constructor mat2x4
            builtin type generator mat2x4
            builtin alias mat2x4f
            builtin constructor mat2x4f
            builtin alias mat2x4h
            builtin constructor mat2x4h
            builtin constructor mat3x2
            builtin type generator mat3x2
            builtin alias mat3x2f
            builtin constructor mat3x2f
            builtin alias mat3x2h
            builtin constructor mat3x2h
            builtin constructor mat3x3
            builtin type generator mat3x3
            builtin alias mat3x3f
            builtin constructor mat3x3f
            builtin alias mat3x3h
            builtin constructor mat3x3h
            builtin constructor mat3x4
            builtin type generator mat3x4
            builtin alias mat3x4f
            builtin constructor mat3x4f
            builtin alias mat3x4h
            builtin constructor mat3x4h
            builtin constructor mat4x2
            builtin type generator mat4x2
            builtin alias mat4x2f
            builtin constructor mat4x2f
            builtin alias mat4x2h
            builtin constructor mat4x2h
            builtin constructor mat4x3
            builtin type generator mat4x3
            builtin alias mat4x3f
            builtin constructor mat4x3f
            builtin alias mat4x3h
            builtin constructor mat4x3h
            builtin constructor mat4x4
            builtin type generator mat4x4
            builtin alias mat4x4f
            builtin constructor mat4x4f
            builtin alias mat4x4h
            builtin constructor mat4x4h
            builtin function max
            builtin function min
            builtin function mix
            builtin function modf
            builtin function normalize
            builtin function pack2x16float
            builtin function pack2x16snorm
            builtin function pack2x16unorm
            builtin function pack4x8snorm
            builtin function pack4x8unorm
            builtin function pack4xI8
            builtin function pack4xI8Clamp
            builtin function pack4xU8
            builtin function pack4xU8Clamp
            builtin function pow
            builtin enumerant private
            builtin type generator ptr
            builtin function quadBroadcast
            builtin function quadSwapDiagonal
            builtin function quadSwapX
            builtin function quadSwapY
            builtin function quantizeToF16
            builtin enumerant r16float
            builtin enumerant r16sint
            builtin enumerant r16snorm
            builtin enumerant r16uint
            builtin enumerant r16unorm
            builtin enumerant r32float
            builtin enumerant r32sint
            builtin enumerant r32uint
            builtin enumerant r64uint
            builtin enumerant r8sint
            builtin enumerant r8snorm
            builtin enumerant r8uint
            builtin enumerant r8unorm
            builtin function radians
            builtin function rayQueryConfirmIntersection
            builtin function rayQueryGenerateIntersection
            builtin function rayQueryGetCandidateIntersection
            builtin function rayQueryGetCommittedIntersection
            builtin function rayQueryInitialize
            builtin function rayQueryProceed
            builtin function rayQueryTerminate
            builtin enumerant ray_payload
            builtin type ray_query
            builtin enumerant read
            builtin enumerant read_write
            builtin function reflect
            builtin function refract
            builtin function reverseBits
            builtin enumerant rg11b10float
            builtin enumerant rg16float
            builtin enumerant rg16sint
            builtin enumerant rg16snorm
            builtin enumerant rg16uint
            builtin enumerant rg16unorm
            builtin enumerant rg32float
            builtin enumerant rg32sint
            builtin enumerant rg32uint
            builtin enumerant rg8sint
            builtin enumerant rg8snorm
            builtin enumerant rg8uint
            builtin enumerant rg8unorm
            builtin enumerant rgb10a2uint
            builtin enumerant rgb10a2unorm
            builtin enumerant rgba16float
            builtin enumerant rgba16sint
            builtin enumerant rgba16snorm
            builtin enumerant rgba16uint
            builtin enumerant rgba16unorm
            builtin enumerant rgba32float
            builtin enumerant rgba32sint
            builtin enumerant rgba32uint
            builtin enumerant rgba8sint
            builtin enumerant rgba8snorm
            builtin enumerant rgba8uint
            builtin enumerant rgba8unorm
            builtin function round
            builtin type sampler
            builtin type sampler_comparison
            builtin function saturate
            builtin function select
            builtin function sign
            builtin function sin
            builtin function sinh
            builtin function smoothstep
            builtin function sqrt
            builtin function step
            builtin enumerant storage
            builtin function storageBarrier
            builtin function subgroupAdd
            builtin function subgroupAll
            builtin function subgroupAnd
            builtin function subgroupAny
            builtin function subgroupBallot
            builtin function subgroupBroadcast
            builtin function subgroupBroadcastFirst
            builtin function subgroupElect
            builtin function subgroupExclusiveAdd
            builtin function subgroupExclusiveMul
            builtin function subgroupInclusiveAdd
            builtin function subgroupInclusiveMul
            builtin function subgroupMax
            builtin function subgroupMin
            builtin function subgroupMul
            builtin function subgroupOr
            builtin function subgroupShuffle
            builtin function subgroupShuffleDown
            builtin function subgroupShuffleUp
            builtin function subgroupShuffleXor
            builtin function subgroupXor
            builtin function tan
            builtin function tanh
            builtin enumerant task_payload
            function test                   fn test()
            builtin function textureBarrier
            builtin function textureDimensions
            builtin function textureGather
            builtin function textureGatherCompare
            builtin function textureLoad
            builtin function textureNumLayers
            builtin function textureNumLevels
            builtin function textureNumSamples
            builtin function textureSample
            builtin function textureSampleBaseClampToEdge
            builtin function textureSampleBias
            builtin function textureSampleCompare
            builtin function textureSampleCompareLevel
            builtin function textureSampleGrad
            builtin function textureSampleLevel
            builtin function textureStore
            builtin type generator texture_1d
            builtin type generator texture_1d_array
            builtin type generator texture_2d
            builtin type generator texture_2d_array
            builtin type generator texture_3d
            builtin type generator texture_cube
            builtin type generator texture_cube_array
            builtin type texture_depth_2d
            builtin type texture_depth_2d_array
            builtin type texture_depth_cube
            builtin type texture_depth_cube_array
            builtin type texture_depth_multisampled_2d
            builtin type texture_external
            builtin type generator texture_multisampled_2d
            builtin type generator texture_multisampled_2d_array
            builtin type generator texture_storage_1d
            builtin type generator texture_storage_1d_array
            builtin type generator texture_storage_2d
            builtin type generator texture_storage_2d_array
            builtin type generator texture_storage_3d
            builtin function traceRay
            builtin function transpose
            builtin function trunc
            builtin constructor u32
            builtin type u32
            builtin constructor u64
            builtin type u64
            builtin enumerant uniform
            builtin function unpack2x16float
            builtin function unpack2x16snorm
            builtin function unpack2x16unorm
            builtin function unpack4x8snorm
            builtin function unpack4x8unorm
            builtin function unpack4xI8
            builtin function unpack4xU8
            builtin constructor vec2
            builtin type generator vec2
            builtin alias vec2f
            builtin constructor vec2f
            builtin alias vec2h
            builtin constructor vec2h
            builtin alias vec2i
            builtin constructor vec2i
            builtin alias vec2u
            builtin constructor vec2u
            builtin constructor vec3
            builtin type generator vec3
            builtin alias vec3f
            builtin constructor vec3f
            builtin alias vec3h
            builtin constructor vec3h
            builtin alias vec3i
            builtin constructor vec3i
            builtin alias vec3u
            builtin constructor vec3u
            builtin constructor vec4
            builtin type generator vec4
            builtin alias vec4f
            builtin constructor vec4f
            builtin alias vec4h
            builtin constructor vec4h
            builtin alias vec4i
            builtin constructor vec4i
            builtin alias vec4u
            builtin constructor vec4u
            builtin enumerant vertex_return
            builtin enumerant workgroup
            builtin function workgroupBarrier
            builtin function workgroupUniformLoad
            builtin enumerant write
        "#]],
    );
}

#[test]
// TODO https://github.com/wgsl-analyzer/wgsl-analyzer/issues/921
fn complete_snippet() {
    check(
        "
            fn test() {
                $0
            }
            ",
        expect![[r#"
            builtin declaration RAY_FLAG_CULL_BACK_FACING
            builtin declaration RAY_FLAG_CULL_FRONT_FACING
            builtin declaration RAY_FLAG_CULL_NO_OPAQUE
            builtin declaration RAY_FLAG_CULL_OPAQUE
            builtin declaration RAY_FLAG_FORCE_NO_OPAQUE
            builtin declaration RAY_FLAG_FORCE_OPAQUE
            builtin declaration RAY_FLAG_NONE
            builtin declaration RAY_FLAG_SKIP_AABBS
            builtin declaration RAY_FLAG_SKIP_CLOSEST_HIT_SHADER
            builtin declaration RAY_FLAG_SKIP_TRIANGLES
            builtin declaration RAY_FLAG_TERMINATE_ON_FIRST_HIT
            builtin declaration RAY_QUERY_INTERSECTION_AABB
            builtin declaration RAY_QUERY_INTERSECTION_GENERATED
            builtin declaration RAY_QUERY_INTERSECTION_NONE
            builtin declaration RAY_QUERY_INTERSECTION_TRIANGLE
            builtin constructor RayDesc
            builtin constructor RayIntersection
            builtin function abs
            builtin type acceleration_structure
            builtin function acos
            builtin function acosh
            builtin function all
            builtin function any
            builtin constructor array
            builtin type generator array
            builtin function arrayLength
            builtin function asin
            builtin function asinh
            builtin function atan
            builtin function atan2
            builtin function atanh
            builtin type generator atomic
            builtin function atomicAdd
            builtin function atomicAnd
            builtin function atomicCompareExchangeWeak
            builtin function atomicExchange
            builtin function atomicLoad
            builtin function atomicMax
            builtin function atomicMin
            builtin function atomicOr
            builtin function atomicStore
            builtin function atomicSub
            builtin function atomicXor
            builtin enumerant bgra8unorm
            builtin type generator binding_array
            builtin function bitcast
            builtin constructor bool
            builtin type bool
            builtin function ceil
            builtin function clamp
            builtin function cos
            builtin function cosh
            builtin function countLeadingZeros
            builtin function countOneBits
            builtin function countTrailingZeros
            builtin function cross
            builtin function degrees
            builtin function determinant
            builtin function distance
            builtin function dot
            builtin function dot4I8Packed
            builtin function dot4U8Packed
            builtin function dpdx
            builtin function dpdxCoarse
            builtin function dpdxFine
            builtin function dpdy
            builtin function dpdyCoarse
            builtin function dpdyFine
            builtin function exp
            builtin function exp2
            builtin function extractBits
            builtin constructor f16
            builtin type f16
            builtin constructor f32
            builtin type f32
            builtin constructor f64
            builtin type f64
            builtin function faceForward
            builtin function firstLeadingBit
            builtin function firstTrailingBit
            builtin function floor
            builtin function fma
            builtin function fract
            builtin function frexp
            builtin enumerant function
            builtin function fwidth
            builtin function fwidthCoarse
            builtin function fwidthFine
            builtin function getCandidateHitVertexPositions
            builtin function getCommittedHitVertexPositions
            builtin constructor i32
            builtin type i32
            builtin constructor i64
            builtin type i64
            builtin enumerant immediate
            builtin enumerant incoming_ray_payload
            builtin function insertBits
            builtin function inverseSqrt
            builtin function ldexp
            builtin function length
            builtin function log
            builtin function log2
            builtin constructor mat2x2
            builtin type generator mat2x2
            builtin alias mat2x2f
            builtin constructor mat2x2f
            builtin alias mat2x2h
            builtin constructor mat2x2h
            builtin constructor mat2x3
            builtin type generator mat2x3
            builtin alias mat2x3f
            builtin constructor mat2x3f
            builtin alias mat2x3h
            builtin constructor mat2x3h
            builtin constructor mat2x4
            builtin type generator mat2x4
            builtin alias mat2x4f
            builtin constructor mat2x4f
            builtin alias mat2x4h
            builtin constructor mat2x4h
            builtin constructor mat3x2
            builtin type generator mat3x2
            builtin alias mat3x2f
            builtin constructor mat3x2f
            builtin alias mat3x2h
            builtin constructor mat3x2h
            builtin constructor mat3x3
            builtin type generator mat3x3
            builtin alias mat3x3f
            builtin constructor mat3x3f
            builtin alias mat3x3h
            builtin constructor mat3x3h
            builtin constructor mat3x4
            builtin type generator mat3x4
            builtin alias mat3x4f
            builtin constructor mat3x4f
            builtin alias mat3x4h
            builtin constructor mat3x4h
            builtin constructor mat4x2
            builtin type generator mat4x2
            builtin alias mat4x2f
            builtin constructor mat4x2f
            builtin alias mat4x2h
            builtin constructor mat4x2h
            builtin constructor mat4x3
            builtin type generator mat4x3
            builtin alias mat4x3f
            builtin constructor mat4x3f
            builtin alias mat4x3h
            builtin constructor mat4x3h
            builtin constructor mat4x4
            builtin type generator mat4x4
            builtin alias mat4x4f
            builtin constructor mat4x4f
            builtin alias mat4x4h
            builtin constructor mat4x4h
            builtin function max
            builtin function min
            builtin function mix
            builtin function modf
            builtin function normalize
            builtin function pack2x16float
            builtin function pack2x16snorm
            builtin function pack2x16unorm
            builtin function pack4x8snorm
            builtin function pack4x8unorm
            builtin function pack4xI8
            builtin function pack4xI8Clamp
            builtin function pack4xU8
            builtin function pack4xU8Clamp
            builtin function pow
            builtin enumerant private
            builtin type generator ptr
            builtin function quadBroadcast
            builtin function quadSwapDiagonal
            builtin function quadSwapX
            builtin function quadSwapY
            builtin function quantizeToF16
            builtin enumerant r16float
            builtin enumerant r16sint
            builtin enumerant r16snorm
            builtin enumerant r16uint
            builtin enumerant r16unorm
            builtin enumerant r32float
            builtin enumerant r32sint
            builtin enumerant r32uint
            builtin enumerant r64uint
            builtin enumerant r8sint
            builtin enumerant r8snorm
            builtin enumerant r8uint
            builtin enumerant r8unorm
            builtin function radians
            builtin function rayQueryConfirmIntersection
            builtin function rayQueryGenerateIntersection
            builtin function rayQueryGetCandidateIntersection
            builtin function rayQueryGetCommittedIntersection
            builtin function rayQueryInitialize
            builtin function rayQueryProceed
            builtin function rayQueryTerminate
            builtin enumerant ray_payload
            builtin type ray_query
            builtin enumerant read
            builtin enumerant read_write
            builtin function reflect
            builtin function refract
            builtin function reverseBits
            builtin enumerant rg11b10float
            builtin enumerant rg16float
            builtin enumerant rg16sint
            builtin enumerant rg16snorm
            builtin enumerant rg16uint
            builtin enumerant rg16unorm
            builtin enumerant rg32float
            builtin enumerant rg32sint
            builtin enumerant rg32uint
            builtin enumerant rg8sint
            builtin enumerant rg8snorm
            builtin enumerant rg8uint
            builtin enumerant rg8unorm
            builtin enumerant rgb10a2uint
            builtin enumerant rgb10a2unorm
            builtin enumerant rgba16float
            builtin enumerant rgba16sint
            builtin enumerant rgba16snorm
            builtin enumerant rgba16uint
            builtin enumerant rgba16unorm
            builtin enumerant rgba32float
            builtin enumerant rgba32sint
            builtin enumerant rgba32uint
            builtin enumerant rgba8sint
            builtin enumerant rgba8snorm
            builtin enumerant rgba8uint
            builtin enumerant rgba8unorm
            builtin function round
            builtin type sampler
            builtin type sampler_comparison
            builtin function saturate
            builtin function select
            builtin function sign
            builtin function sin
            builtin function sinh
            builtin function smoothstep
            builtin function sqrt
            builtin function step
            builtin enumerant storage
            builtin function storageBarrier
            builtin function subgroupAdd
            builtin function subgroupAll
            builtin function subgroupAnd
            builtin function subgroupAny
            builtin function subgroupBallot
            builtin function subgroupBroadcast
            builtin function subgroupBroadcastFirst
            builtin function subgroupElect
            builtin function subgroupExclusiveAdd
            builtin function subgroupExclusiveMul
            builtin function subgroupInclusiveAdd
            builtin function subgroupInclusiveMul
            builtin function subgroupMax
            builtin function subgroupMin
            builtin function subgroupMul
            builtin function subgroupOr
            builtin function subgroupShuffle
            builtin function subgroupShuffleDown
            builtin function subgroupShuffleUp
            builtin function subgroupShuffleXor
            builtin function subgroupXor
            builtin function tan
            builtin function tanh
            builtin enumerant task_payload
            function test                   fn test()
            builtin function textureBarrier
            builtin function textureDimensions
            builtin function textureGather
            builtin function textureGatherCompare
            builtin function textureLoad
            builtin function textureNumLayers
            builtin function textureNumLevels
            builtin function textureNumSamples
            builtin function textureSample
            builtin function textureSampleBaseClampToEdge
            builtin function textureSampleBias
            builtin function textureSampleCompare
            builtin function textureSampleCompareLevel
            builtin function textureSampleGrad
            builtin function textureSampleLevel
            builtin function textureStore
            builtin type generator texture_1d
            builtin type generator texture_1d_array
            builtin type generator texture_2d
            builtin type generator texture_2d_array
            builtin type generator texture_3d
            builtin type generator texture_cube
            builtin type generator texture_cube_array
            builtin type texture_depth_2d
            builtin type texture_depth_2d_array
            builtin type texture_depth_cube
            builtin type texture_depth_cube_array
            builtin type texture_depth_multisampled_2d
            builtin type texture_external
            builtin type generator texture_multisampled_2d
            builtin type generator texture_multisampled_2d_array
            builtin type generator texture_storage_1d
            builtin type generator texture_storage_1d_array
            builtin type generator texture_storage_2d
            builtin type generator texture_storage_2d_array
            builtin type generator texture_storage_3d
            builtin function traceRay
            builtin function transpose
            builtin function trunc
            builtin constructor u32
            builtin type u32
            builtin constructor u64
            builtin type u64
            builtin enumerant uniform
            builtin function unpack2x16float
            builtin function unpack2x16snorm
            builtin function unpack2x16unorm
            builtin function unpack4x8snorm
            builtin function unpack4x8unorm
            builtin function unpack4xI8
            builtin function unpack4xU8
            builtin constructor vec2
            builtin type generator vec2
            builtin alias vec2f
            builtin constructor vec2f
            builtin alias vec2h
            builtin constructor vec2h
            builtin alias vec2i
            builtin constructor vec2i
            builtin alias vec2u
            builtin constructor vec2u
            builtin constructor vec3
            builtin type generator vec3
            builtin alias vec3f
            builtin constructor vec3f
            builtin alias vec3h
            builtin constructor vec3h
            builtin alias vec3i
            builtin constructor vec3i
            builtin alias vec3u
            builtin constructor vec3u
            builtin constructor vec4
            builtin type generator vec4
            builtin alias vec4f
            builtin constructor vec4f
            builtin alias vec4h
            builtin constructor vec4h
            builtin alias vec4i
            builtin constructor vec4i
            builtin alias vec4u
            builtin constructor vec4u
            builtin enumerant vertex_return
            builtin enumerant workgroup
            builtin function workgroupBarrier
            builtin function workgroupUniformLoad
            builtin enumerant write
        "#]],
    );
}

#[test]
fn complete_constant() {
    check(
        "
            const Foo: u32 = 0;
            fn test() {
                let x = $0;
            }
            ",
        expect![[r#"
            constant Foo               const Foo: u32
            builtin declaration RAY_FLAG_CULL_BACK_FACING
            builtin declaration RAY_FLAG_CULL_FRONT_FACING
            builtin declaration RAY_FLAG_CULL_NO_OPAQUE
            builtin declaration RAY_FLAG_CULL_OPAQUE
            builtin declaration RAY_FLAG_FORCE_NO_OPAQUE
            builtin declaration RAY_FLAG_FORCE_OPAQUE
            builtin declaration RAY_FLAG_NONE
            builtin declaration RAY_FLAG_SKIP_AABBS
            builtin declaration RAY_FLAG_SKIP_CLOSEST_HIT_SHADER
            builtin declaration RAY_FLAG_SKIP_TRIANGLES
            builtin declaration RAY_FLAG_TERMINATE_ON_FIRST_HIT
            builtin declaration RAY_QUERY_INTERSECTION_AABB
            builtin declaration RAY_QUERY_INTERSECTION_GENERATED
            builtin declaration RAY_QUERY_INTERSECTION_NONE
            builtin declaration RAY_QUERY_INTERSECTION_TRIANGLE
            builtin constructor RayDesc
            builtin constructor RayIntersection
            builtin function abs
            builtin type acceleration_structure
            builtin function acos
            builtin function acosh
            builtin function all
            builtin function any
            builtin constructor array
            builtin type generator array
            builtin function arrayLength
            builtin function asin
            builtin function asinh
            builtin function atan
            builtin function atan2
            builtin function atanh
            builtin type generator atomic
            builtin function atomicAdd
            builtin function atomicAnd
            builtin function atomicCompareExchangeWeak
            builtin function atomicExchange
            builtin function atomicLoad
            builtin function atomicMax
            builtin function atomicMin
            builtin function atomicOr
            builtin function atomicStore
            builtin function atomicSub
            builtin function atomicXor
            builtin enumerant bgra8unorm
            builtin type generator binding_array
            builtin function bitcast
            builtin constructor bool
            builtin type bool
            builtin function ceil
            builtin function clamp
            builtin function cos
            builtin function cosh
            builtin function countLeadingZeros
            builtin function countOneBits
            builtin function countTrailingZeros
            builtin function cross
            builtin function degrees
            builtin function determinant
            builtin function distance
            builtin function dot
            builtin function dot4I8Packed
            builtin function dot4U8Packed
            builtin function dpdx
            builtin function dpdxCoarse
            builtin function dpdxFine
            builtin function dpdy
            builtin function dpdyCoarse
            builtin function dpdyFine
            builtin function exp
            builtin function exp2
            builtin function extractBits
            builtin constructor f16
            builtin type f16
            builtin constructor f32
            builtin type f32
            builtin constructor f64
            builtin type f64
            builtin function faceForward
            builtin function firstLeadingBit
            builtin function firstTrailingBit
            builtin function floor
            builtin function fma
            builtin function fract
            builtin function frexp
            builtin enumerant function
            builtin function fwidth
            builtin function fwidthCoarse
            builtin function fwidthFine
            builtin function getCandidateHitVertexPositions
            builtin function getCommittedHitVertexPositions
            builtin constructor i32
            builtin type i32
            builtin constructor i64
            builtin type i64
            builtin enumerant immediate
            builtin enumerant incoming_ray_payload
            builtin function insertBits
            builtin function inverseSqrt
            builtin function ldexp
            builtin function length
            builtin function log
            builtin function log2
            builtin constructor mat2x2
            builtin type generator mat2x2
            builtin alias mat2x2f
            builtin constructor mat2x2f
            builtin alias mat2x2h
            builtin constructor mat2x2h
            builtin constructor mat2x3
            builtin type generator mat2x3
            builtin alias mat2x3f
            builtin constructor mat2x3f
            builtin alias mat2x3h
            builtin constructor mat2x3h
            builtin constructor mat2x4
            builtin type generator mat2x4
            builtin alias mat2x4f
            builtin constructor mat2x4f
            builtin alias mat2x4h
            builtin constructor mat2x4h
            builtin constructor mat3x2
            builtin type generator mat3x2
            builtin alias mat3x2f
            builtin constructor mat3x2f
            builtin alias mat3x2h
            builtin constructor mat3x2h
            builtin constructor mat3x3
            builtin type generator mat3x3
            builtin alias mat3x3f
            builtin constructor mat3x3f
            builtin alias mat3x3h
            builtin constructor mat3x3h
            builtin constructor mat3x4
            builtin type generator mat3x4
            builtin alias mat3x4f
            builtin constructor mat3x4f
            builtin alias mat3x4h
            builtin constructor mat3x4h
            builtin constructor mat4x2
            builtin type generator mat4x2
            builtin alias mat4x2f
            builtin constructor mat4x2f
            builtin alias mat4x2h
            builtin constructor mat4x2h
            builtin constructor mat4x3
            builtin type generator mat4x3
            builtin alias mat4x3f
            builtin constructor mat4x3f
            builtin alias mat4x3h
            builtin constructor mat4x3h
            builtin constructor mat4x4
            builtin type generator mat4x4
            builtin alias mat4x4f
            builtin constructor mat4x4f
            builtin alias mat4x4h
            builtin constructor mat4x4h
            builtin function max
            builtin function min
            builtin function mix
            builtin function modf
            builtin function normalize
            builtin function pack2x16float
            builtin function pack2x16snorm
            builtin function pack2x16unorm
            builtin function pack4x8snorm
            builtin function pack4x8unorm
            builtin function pack4xI8
            builtin function pack4xI8Clamp
            builtin function pack4xU8
            builtin function pack4xU8Clamp
            builtin function pow
            builtin enumerant private
            builtin type generator ptr
            builtin function quadBroadcast
            builtin function quadSwapDiagonal
            builtin function quadSwapX
            builtin function quadSwapY
            builtin function quantizeToF16
            builtin enumerant r16float
            builtin enumerant r16sint
            builtin enumerant r16snorm
            builtin enumerant r16uint
            builtin enumerant r16unorm
            builtin enumerant r32float
            builtin enumerant r32sint
            builtin enumerant r32uint
            builtin enumerant r64uint
            builtin enumerant r8sint
            builtin enumerant r8snorm
            builtin enumerant r8uint
            builtin enumerant r8unorm
            builtin function radians
            builtin function rayQueryConfirmIntersection
            builtin function rayQueryGenerateIntersection
            builtin function rayQueryGetCandidateIntersection
            builtin function rayQueryGetCommittedIntersection
            builtin function rayQueryInitialize
            builtin function rayQueryProceed
            builtin function rayQueryTerminate
            builtin enumerant ray_payload
            builtin type ray_query
            builtin enumerant read
            builtin enumerant read_write
            builtin function reflect
            builtin function refract
            builtin function reverseBits
            builtin enumerant rg11b10float
            builtin enumerant rg16float
            builtin enumerant rg16sint
            builtin enumerant rg16snorm
            builtin enumerant rg16uint
            builtin enumerant rg16unorm
            builtin enumerant rg32float
            builtin enumerant rg32sint
            builtin enumerant rg32uint
            builtin enumerant rg8sint
            builtin enumerant rg8snorm
            builtin enumerant rg8uint
            builtin enumerant rg8unorm
            builtin enumerant rgb10a2uint
            builtin enumerant rgb10a2unorm
            builtin enumerant rgba16float
            builtin enumerant rgba16sint
            builtin enumerant rgba16snorm
            builtin enumerant rgba16uint
            builtin enumerant rgba16unorm
            builtin enumerant rgba32float
            builtin enumerant rgba32sint
            builtin enumerant rgba32uint
            builtin enumerant rgba8sint
            builtin enumerant rgba8snorm
            builtin enumerant rgba8uint
            builtin enumerant rgba8unorm
            builtin function round
            builtin type sampler
            builtin type sampler_comparison
            builtin function saturate
            builtin function select
            builtin function sign
            builtin function sin
            builtin function sinh
            builtin function smoothstep
            builtin function sqrt
            builtin function step
            builtin enumerant storage
            builtin function storageBarrier
            builtin function subgroupAdd
            builtin function subgroupAll
            builtin function subgroupAnd
            builtin function subgroupAny
            builtin function subgroupBallot
            builtin function subgroupBroadcast
            builtin function subgroupBroadcastFirst
            builtin function subgroupElect
            builtin function subgroupExclusiveAdd
            builtin function subgroupExclusiveMul
            builtin function subgroupInclusiveAdd
            builtin function subgroupInclusiveMul
            builtin function subgroupMax
            builtin function subgroupMin
            builtin function subgroupMul
            builtin function subgroupOr
            builtin function subgroupShuffle
            builtin function subgroupShuffleDown
            builtin function subgroupShuffleUp
            builtin function subgroupShuffleXor
            builtin function subgroupXor
            builtin function tan
            builtin function tanh
            builtin enumerant task_payload
            function test                   fn test()
            builtin function textureBarrier
            builtin function textureDimensions
            builtin function textureGather
            builtin function textureGatherCompare
            builtin function textureLoad
            builtin function textureNumLayers
            builtin function textureNumLevels
            builtin function textureNumSamples
            builtin function textureSample
            builtin function textureSampleBaseClampToEdge
            builtin function textureSampleBias
            builtin function textureSampleCompare
            builtin function textureSampleCompareLevel
            builtin function textureSampleGrad
            builtin function textureSampleLevel
            builtin function textureStore
            builtin type generator texture_1d
            builtin type generator texture_1d_array
            builtin type generator texture_2d
            builtin type generator texture_2d_array
            builtin type generator texture_3d
            builtin type generator texture_cube
            builtin type generator texture_cube_array
            builtin type texture_depth_2d
            builtin type texture_depth_2d_array
            builtin type texture_depth_cube
            builtin type texture_depth_cube_array
            builtin type texture_depth_multisampled_2d
            builtin type texture_external
            builtin type generator texture_multisampled_2d
            builtin type generator texture_multisampled_2d_array
            builtin type generator texture_storage_1d
            builtin type generator texture_storage_1d_array
            builtin type generator texture_storage_2d
            builtin type generator texture_storage_2d_array
            builtin type generator texture_storage_3d
            builtin function traceRay
            builtin function transpose
            builtin function trunc
            builtin constructor u32
            builtin type u32
            builtin constructor u64
            builtin type u64
            builtin enumerant uniform
            builtin function unpack2x16float
            builtin function unpack2x16snorm
            builtin function unpack2x16unorm
            builtin function unpack4x8snorm
            builtin function unpack4x8unorm
            builtin function unpack4xI8
            builtin function unpack4xU8
            builtin constructor vec2
            builtin type generator vec2
            builtin alias vec2f
            builtin constructor vec2f
            builtin alias vec2h
            builtin constructor vec2h
            builtin alias vec2i
            builtin constructor vec2i
            builtin alias vec2u
            builtin constructor vec2u
            builtin constructor vec3
            builtin type generator vec3
            builtin alias vec3f
            builtin constructor vec3f
            builtin alias vec3h
            builtin constructor vec3h
            builtin alias vec3i
            builtin constructor vec3i
            builtin alias vec3u
            builtin constructor vec3u
            builtin constructor vec4
            builtin type generator vec4
            builtin alias vec4f
            builtin constructor vec4f
            builtin alias vec4h
            builtin constructor vec4h
            builtin alias vec4i
            builtin constructor vec4i
            builtin alias vec4u
            builtin constructor vec4u
            builtin enumerant vertex_return
            builtin enumerant workgroup
            builtin function workgroupBarrier
            builtin function workgroupUniformLoad
            builtin enumerant write
        "#]],
    );
}

#[test]
fn complete_struct() {
    check(
        "
            struct Foo { bar: u32 }
            fn test() {
                let x = $0;
            }
            ",
        expect![[r#"
            struct Foo                   struct Foo
            builtin declaration RAY_FLAG_CULL_BACK_FACING
            builtin declaration RAY_FLAG_CULL_FRONT_FACING
            builtin declaration RAY_FLAG_CULL_NO_OPAQUE
            builtin declaration RAY_FLAG_CULL_OPAQUE
            builtin declaration RAY_FLAG_FORCE_NO_OPAQUE
            builtin declaration RAY_FLAG_FORCE_OPAQUE
            builtin declaration RAY_FLAG_NONE
            builtin declaration RAY_FLAG_SKIP_AABBS
            builtin declaration RAY_FLAG_SKIP_CLOSEST_HIT_SHADER
            builtin declaration RAY_FLAG_SKIP_TRIANGLES
            builtin declaration RAY_FLAG_TERMINATE_ON_FIRST_HIT
            builtin declaration RAY_QUERY_INTERSECTION_AABB
            builtin declaration RAY_QUERY_INTERSECTION_GENERATED
            builtin declaration RAY_QUERY_INTERSECTION_NONE
            builtin declaration RAY_QUERY_INTERSECTION_TRIANGLE
            builtin constructor RayDesc
            builtin constructor RayIntersection
            builtin function abs
            builtin type acceleration_structure
            builtin function acos
            builtin function acosh
            builtin function all
            builtin function any
            builtin constructor array
            builtin type generator array
            builtin function arrayLength
            builtin function asin
            builtin function asinh
            builtin function atan
            builtin function atan2
            builtin function atanh
            builtin type generator atomic
            builtin function atomicAdd
            builtin function atomicAnd
            builtin function atomicCompareExchangeWeak
            builtin function atomicExchange
            builtin function atomicLoad
            builtin function atomicMax
            builtin function atomicMin
            builtin function atomicOr
            builtin function atomicStore
            builtin function atomicSub
            builtin function atomicXor
            builtin enumerant bgra8unorm
            builtin type generator binding_array
            builtin function bitcast
            builtin constructor bool
            builtin type bool
            builtin function ceil
            builtin function clamp
            builtin function cos
            builtin function cosh
            builtin function countLeadingZeros
            builtin function countOneBits
            builtin function countTrailingZeros
            builtin function cross
            builtin function degrees
            builtin function determinant
            builtin function distance
            builtin function dot
            builtin function dot4I8Packed
            builtin function dot4U8Packed
            builtin function dpdx
            builtin function dpdxCoarse
            builtin function dpdxFine
            builtin function dpdy
            builtin function dpdyCoarse
            builtin function dpdyFine
            builtin function exp
            builtin function exp2
            builtin function extractBits
            builtin constructor f16
            builtin type f16
            builtin constructor f32
            builtin type f32
            builtin constructor f64
            builtin type f64
            builtin function faceForward
            builtin function firstLeadingBit
            builtin function firstTrailingBit
            builtin function floor
            builtin function fma
            builtin function fract
            builtin function frexp
            builtin enumerant function
            builtin function fwidth
            builtin function fwidthCoarse
            builtin function fwidthFine
            builtin function getCandidateHitVertexPositions
            builtin function getCommittedHitVertexPositions
            builtin constructor i32
            builtin type i32
            builtin constructor i64
            builtin type i64
            builtin enumerant immediate
            builtin enumerant incoming_ray_payload
            builtin function insertBits
            builtin function inverseSqrt
            builtin function ldexp
            builtin function length
            builtin function log
            builtin function log2
            builtin constructor mat2x2
            builtin type generator mat2x2
            builtin alias mat2x2f
            builtin constructor mat2x2f
            builtin alias mat2x2h
            builtin constructor mat2x2h
            builtin constructor mat2x3
            builtin type generator mat2x3
            builtin alias mat2x3f
            builtin constructor mat2x3f
            builtin alias mat2x3h
            builtin constructor mat2x3h
            builtin constructor mat2x4
            builtin type generator mat2x4
            builtin alias mat2x4f
            builtin constructor mat2x4f
            builtin alias mat2x4h
            builtin constructor mat2x4h
            builtin constructor mat3x2
            builtin type generator mat3x2
            builtin alias mat3x2f
            builtin constructor mat3x2f
            builtin alias mat3x2h
            builtin constructor mat3x2h
            builtin constructor mat3x3
            builtin type generator mat3x3
            builtin alias mat3x3f
            builtin constructor mat3x3f
            builtin alias mat3x3h
            builtin constructor mat3x3h
            builtin constructor mat3x4
            builtin type generator mat3x4
            builtin alias mat3x4f
            builtin constructor mat3x4f
            builtin alias mat3x4h
            builtin constructor mat3x4h
            builtin constructor mat4x2
            builtin type generator mat4x2
            builtin alias mat4x2f
            builtin constructor mat4x2f
            builtin alias mat4x2h
            builtin constructor mat4x2h
            builtin constructor mat4x3
            builtin type generator mat4x3
            builtin alias mat4x3f
            builtin constructor mat4x3f
            builtin alias mat4x3h
            builtin constructor mat4x3h
            builtin constructor mat4x4
            builtin type generator mat4x4
            builtin alias mat4x4f
            builtin constructor mat4x4f
            builtin alias mat4x4h
            builtin constructor mat4x4h
            builtin function max
            builtin function min
            builtin function mix
            builtin function modf
            builtin function normalize
            builtin function pack2x16float
            builtin function pack2x16snorm
            builtin function pack2x16unorm
            builtin function pack4x8snorm
            builtin function pack4x8unorm
            builtin function pack4xI8
            builtin function pack4xI8Clamp
            builtin function pack4xU8
            builtin function pack4xU8Clamp
            builtin function pow
            builtin enumerant private
            builtin type generator ptr
            builtin function quadBroadcast
            builtin function quadSwapDiagonal
            builtin function quadSwapX
            builtin function quadSwapY
            builtin function quantizeToF16
            builtin enumerant r16float
            builtin enumerant r16sint
            builtin enumerant r16snorm
            builtin enumerant r16uint
            builtin enumerant r16unorm
            builtin enumerant r32float
            builtin enumerant r32sint
            builtin enumerant r32uint
            builtin enumerant r64uint
            builtin enumerant r8sint
            builtin enumerant r8snorm
            builtin enumerant r8uint
            builtin enumerant r8unorm
            builtin function radians
            builtin function rayQueryConfirmIntersection
            builtin function rayQueryGenerateIntersection
            builtin function rayQueryGetCandidateIntersection
            builtin function rayQueryGetCommittedIntersection
            builtin function rayQueryInitialize
            builtin function rayQueryProceed
            builtin function rayQueryTerminate
            builtin enumerant ray_payload
            builtin type ray_query
            builtin enumerant read
            builtin enumerant read_write
            builtin function reflect
            builtin function refract
            builtin function reverseBits
            builtin enumerant rg11b10float
            builtin enumerant rg16float
            builtin enumerant rg16sint
            builtin enumerant rg16snorm
            builtin enumerant rg16uint
            builtin enumerant rg16unorm
            builtin enumerant rg32float
            builtin enumerant rg32sint
            builtin enumerant rg32uint
            builtin enumerant rg8sint
            builtin enumerant rg8snorm
            builtin enumerant rg8uint
            builtin enumerant rg8unorm
            builtin enumerant rgb10a2uint
            builtin enumerant rgb10a2unorm
            builtin enumerant rgba16float
            builtin enumerant rgba16sint
            builtin enumerant rgba16snorm
            builtin enumerant rgba16uint
            builtin enumerant rgba16unorm
            builtin enumerant rgba32float
            builtin enumerant rgba32sint
            builtin enumerant rgba32uint
            builtin enumerant rgba8sint
            builtin enumerant rgba8snorm
            builtin enumerant rgba8uint
            builtin enumerant rgba8unorm
            builtin function round
            builtin type sampler
            builtin type sampler_comparison
            builtin function saturate
            builtin function select
            builtin function sign
            builtin function sin
            builtin function sinh
            builtin function smoothstep
            builtin function sqrt
            builtin function step
            builtin enumerant storage
            builtin function storageBarrier
            builtin function subgroupAdd
            builtin function subgroupAll
            builtin function subgroupAnd
            builtin function subgroupAny
            builtin function subgroupBallot
            builtin function subgroupBroadcast
            builtin function subgroupBroadcastFirst
            builtin function subgroupElect
            builtin function subgroupExclusiveAdd
            builtin function subgroupExclusiveMul
            builtin function subgroupInclusiveAdd
            builtin function subgroupInclusiveMul
            builtin function subgroupMax
            builtin function subgroupMin
            builtin function subgroupMul
            builtin function subgroupOr
            builtin function subgroupShuffle
            builtin function subgroupShuffleDown
            builtin function subgroupShuffleUp
            builtin function subgroupShuffleXor
            builtin function subgroupXor
            builtin function tan
            builtin function tanh
            builtin enumerant task_payload
            function test                   fn test()
            builtin function textureBarrier
            builtin function textureDimensions
            builtin function textureGather
            builtin function textureGatherCompare
            builtin function textureLoad
            builtin function textureNumLayers
            builtin function textureNumLevels
            builtin function textureNumSamples
            builtin function textureSample
            builtin function textureSampleBaseClampToEdge
            builtin function textureSampleBias
            builtin function textureSampleCompare
            builtin function textureSampleCompareLevel
            builtin function textureSampleGrad
            builtin function textureSampleLevel
            builtin function textureStore
            builtin type generator texture_1d
            builtin type generator texture_1d_array
            builtin type generator texture_2d
            builtin type generator texture_2d_array
            builtin type generator texture_3d
            builtin type generator texture_cube
            builtin type generator texture_cube_array
            builtin type texture_depth_2d
            builtin type texture_depth_2d_array
            builtin type texture_depth_cube
            builtin type texture_depth_cube_array
            builtin type texture_depth_multisampled_2d
            builtin type texture_external
            builtin type generator texture_multisampled_2d
            builtin type generator texture_multisampled_2d_array
            builtin type generator texture_storage_1d
            builtin type generator texture_storage_1d_array
            builtin type generator texture_storage_2d
            builtin type generator texture_storage_2d_array
            builtin type generator texture_storage_3d
            builtin function traceRay
            builtin function transpose
            builtin function trunc
            builtin constructor u32
            builtin type u32
            builtin constructor u64
            builtin type u64
            builtin enumerant uniform
            builtin function unpack2x16float
            builtin function unpack2x16snorm
            builtin function unpack2x16unorm
            builtin function unpack4x8snorm
            builtin function unpack4x8unorm
            builtin function unpack4xI8
            builtin function unpack4xU8
            builtin constructor vec2
            builtin type generator vec2
            builtin alias vec2f
            builtin constructor vec2f
            builtin alias vec2h
            builtin constructor vec2h
            builtin alias vec2i
            builtin constructor vec2i
            builtin alias vec2u
            builtin constructor vec2u
            builtin constructor vec3
            builtin type generator vec3
            builtin alias vec3f
            builtin constructor vec3f
            builtin alias vec3h
            builtin constructor vec3h
            builtin alias vec3i
            builtin constructor vec3i
            builtin alias vec3u
            builtin constructor vec3u
            builtin constructor vec4
            builtin type generator vec4
            builtin alias vec4f
            builtin constructor vec4f
            builtin alias vec4h
            builtin constructor vec4h
            builtin alias vec4i
            builtin constructor vec4i
            builtin alias vec4u
            builtin constructor vec4u
            builtin enumerant vertex_return
            builtin enumerant workgroup
            builtin function workgroupBarrier
            builtin function workgroupUniformLoad
            builtin enumerant write
        "#]],
    );
}

// completion test for modules in ./wesl.rs

#[test]
fn complete_type_alias() {
    check(
        "
            alias Foo = u32;
            fn test() {
                let x: $0 = 1;
            }
            ",
        expect![[r#"
            type alias Foo                    alias Foo
            builtin declaration RAY_FLAG_CULL_BACK_FACING
            builtin declaration RAY_FLAG_CULL_FRONT_FACING
            builtin declaration RAY_FLAG_CULL_NO_OPAQUE
            builtin declaration RAY_FLAG_CULL_OPAQUE
            builtin declaration RAY_FLAG_FORCE_NO_OPAQUE
            builtin declaration RAY_FLAG_FORCE_OPAQUE
            builtin declaration RAY_FLAG_NONE
            builtin declaration RAY_FLAG_SKIP_AABBS
            builtin declaration RAY_FLAG_SKIP_CLOSEST_HIT_SHADER
            builtin declaration RAY_FLAG_SKIP_TRIANGLES
            builtin declaration RAY_FLAG_TERMINATE_ON_FIRST_HIT
            builtin declaration RAY_QUERY_INTERSECTION_AABB
            builtin declaration RAY_QUERY_INTERSECTION_GENERATED
            builtin declaration RAY_QUERY_INTERSECTION_NONE
            builtin declaration RAY_QUERY_INTERSECTION_TRIANGLE
            builtin constructor RayDesc
            builtin constructor RayIntersection
            builtin function abs
            builtin type acceleration_structure
            builtin function acos
            builtin function acosh
            builtin function all
            builtin function any
            builtin constructor array
            builtin type generator array
            builtin function arrayLength
            builtin function asin
            builtin function asinh
            builtin function atan
            builtin function atan2
            builtin function atanh
            builtin type generator atomic
            builtin function atomicAdd
            builtin function atomicAnd
            builtin function atomicCompareExchangeWeak
            builtin function atomicExchange
            builtin function atomicLoad
            builtin function atomicMax
            builtin function atomicMin
            builtin function atomicOr
            builtin function atomicStore
            builtin function atomicSub
            builtin function atomicXor
            builtin enumerant bgra8unorm
            builtin type generator binding_array
            builtin function bitcast
            builtin constructor bool
            builtin type bool
            builtin function ceil
            builtin function clamp
            builtin function cos
            builtin function cosh
            builtin function countLeadingZeros
            builtin function countOneBits
            builtin function countTrailingZeros
            builtin function cross
            builtin function degrees
            builtin function determinant
            builtin function distance
            builtin function dot
            builtin function dot4I8Packed
            builtin function dot4U8Packed
            builtin function dpdx
            builtin function dpdxCoarse
            builtin function dpdxFine
            builtin function dpdy
            builtin function dpdyCoarse
            builtin function dpdyFine
            builtin function exp
            builtin function exp2
            builtin function extractBits
            builtin constructor f16
            builtin type f16
            builtin constructor f32
            builtin type f32
            builtin constructor f64
            builtin type f64
            builtin function faceForward
            builtin function firstLeadingBit
            builtin function firstTrailingBit
            builtin function floor
            builtin function fma
            builtin function fract
            builtin function frexp
            builtin enumerant function
            builtin function fwidth
            builtin function fwidthCoarse
            builtin function fwidthFine
            builtin function getCandidateHitVertexPositions
            builtin function getCommittedHitVertexPositions
            builtin constructor i32
            builtin type i32
            builtin constructor i64
            builtin type i64
            builtin enumerant immediate
            builtin enumerant incoming_ray_payload
            builtin function insertBits
            builtin function inverseSqrt
            builtin function ldexp
            builtin function length
            builtin function log
            builtin function log2
            builtin constructor mat2x2
            builtin type generator mat2x2
            builtin alias mat2x2f
            builtin constructor mat2x2f
            builtin alias mat2x2h
            builtin constructor mat2x2h
            builtin constructor mat2x3
            builtin type generator mat2x3
            builtin alias mat2x3f
            builtin constructor mat2x3f
            builtin alias mat2x3h
            builtin constructor mat2x3h
            builtin constructor mat2x4
            builtin type generator mat2x4
            builtin alias mat2x4f
            builtin constructor mat2x4f
            builtin alias mat2x4h
            builtin constructor mat2x4h
            builtin constructor mat3x2
            builtin type generator mat3x2
            builtin alias mat3x2f
            builtin constructor mat3x2f
            builtin alias mat3x2h
            builtin constructor mat3x2h
            builtin constructor mat3x3
            builtin type generator mat3x3
            builtin alias mat3x3f
            builtin constructor mat3x3f
            builtin alias mat3x3h
            builtin constructor mat3x3h
            builtin constructor mat3x4
            builtin type generator mat3x4
            builtin alias mat3x4f
            builtin constructor mat3x4f
            builtin alias mat3x4h
            builtin constructor mat3x4h
            builtin constructor mat4x2
            builtin type generator mat4x2
            builtin alias mat4x2f
            builtin constructor mat4x2f
            builtin alias mat4x2h
            builtin constructor mat4x2h
            builtin constructor mat4x3
            builtin type generator mat4x3
            builtin alias mat4x3f
            builtin constructor mat4x3f
            builtin alias mat4x3h
            builtin constructor mat4x3h
            builtin constructor mat4x4
            builtin type generator mat4x4
            builtin alias mat4x4f
            builtin constructor mat4x4f
            builtin alias mat4x4h
            builtin constructor mat4x4h
            builtin function max
            builtin function min
            builtin function mix
            builtin function modf
            builtin function normalize
            builtin function pack2x16float
            builtin function pack2x16snorm
            builtin function pack2x16unorm
            builtin function pack4x8snorm
            builtin function pack4x8unorm
            builtin function pack4xI8
            builtin function pack4xI8Clamp
            builtin function pack4xU8
            builtin function pack4xU8Clamp
            builtin function pow
            builtin enumerant private
            builtin type generator ptr
            builtin function quadBroadcast
            builtin function quadSwapDiagonal
            builtin function quadSwapX
            builtin function quadSwapY
            builtin function quantizeToF16
            builtin enumerant r16float
            builtin enumerant r16sint
            builtin enumerant r16snorm
            builtin enumerant r16uint
            builtin enumerant r16unorm
            builtin enumerant r32float
            builtin enumerant r32sint
            builtin enumerant r32uint
            builtin enumerant r64uint
            builtin enumerant r8sint
            builtin enumerant r8snorm
            builtin enumerant r8uint
            builtin enumerant r8unorm
            builtin function radians
            builtin function rayQueryConfirmIntersection
            builtin function rayQueryGenerateIntersection
            builtin function rayQueryGetCandidateIntersection
            builtin function rayQueryGetCommittedIntersection
            builtin function rayQueryInitialize
            builtin function rayQueryProceed
            builtin function rayQueryTerminate
            builtin enumerant ray_payload
            builtin type ray_query
            builtin enumerant read
            builtin enumerant read_write
            builtin function reflect
            builtin function refract
            builtin function reverseBits
            builtin enumerant rg11b10float
            builtin enumerant rg16float
            builtin enumerant rg16sint
            builtin enumerant rg16snorm
            builtin enumerant rg16uint
            builtin enumerant rg16unorm
            builtin enumerant rg32float
            builtin enumerant rg32sint
            builtin enumerant rg32uint
            builtin enumerant rg8sint
            builtin enumerant rg8snorm
            builtin enumerant rg8uint
            builtin enumerant rg8unorm
            builtin enumerant rgb10a2uint
            builtin enumerant rgb10a2unorm
            builtin enumerant rgba16float
            builtin enumerant rgba16sint
            builtin enumerant rgba16snorm
            builtin enumerant rgba16uint
            builtin enumerant rgba16unorm
            builtin enumerant rgba32float
            builtin enumerant rgba32sint
            builtin enumerant rgba32uint
            builtin enumerant rgba8sint
            builtin enumerant rgba8snorm
            builtin enumerant rgba8uint
            builtin enumerant rgba8unorm
            builtin function round
            builtin type sampler
            builtin type sampler_comparison
            builtin function saturate
            builtin function select
            builtin function sign
            builtin function sin
            builtin function sinh
            builtin function smoothstep
            builtin function sqrt
            builtin function step
            builtin enumerant storage
            builtin function storageBarrier
            builtin function subgroupAdd
            builtin function subgroupAll
            builtin function subgroupAnd
            builtin function subgroupAny
            builtin function subgroupBallot
            builtin function subgroupBroadcast
            builtin function subgroupBroadcastFirst
            builtin function subgroupElect
            builtin function subgroupExclusiveAdd
            builtin function subgroupExclusiveMul
            builtin function subgroupInclusiveAdd
            builtin function subgroupInclusiveMul
            builtin function subgroupMax
            builtin function subgroupMin
            builtin function subgroupMul
            builtin function subgroupOr
            builtin function subgroupShuffle
            builtin function subgroupShuffleDown
            builtin function subgroupShuffleUp
            builtin function subgroupShuffleXor
            builtin function subgroupXor
            builtin function tan
            builtin function tanh
            builtin enumerant task_payload
            function test                   fn test()
            builtin function textureBarrier
            builtin function textureDimensions
            builtin function textureGather
            builtin function textureGatherCompare
            builtin function textureLoad
            builtin function textureNumLayers
            builtin function textureNumLevels
            builtin function textureNumSamples
            builtin function textureSample
            builtin function textureSampleBaseClampToEdge
            builtin function textureSampleBias
            builtin function textureSampleCompare
            builtin function textureSampleCompareLevel
            builtin function textureSampleGrad
            builtin function textureSampleLevel
            builtin function textureStore
            builtin type generator texture_1d
            builtin type generator texture_1d_array
            builtin type generator texture_2d
            builtin type generator texture_2d_array
            builtin type generator texture_3d
            builtin type generator texture_cube
            builtin type generator texture_cube_array
            builtin type texture_depth_2d
            builtin type texture_depth_2d_array
            builtin type texture_depth_cube
            builtin type texture_depth_cube_array
            builtin type texture_depth_multisampled_2d
            builtin type texture_external
            builtin type generator texture_multisampled_2d
            builtin type generator texture_multisampled_2d_array
            builtin type generator texture_storage_1d
            builtin type generator texture_storage_1d_array
            builtin type generator texture_storage_2d
            builtin type generator texture_storage_2d_array
            builtin type generator texture_storage_3d
            builtin function traceRay
            builtin function transpose
            builtin function trunc
            builtin constructor u32
            builtin type u32
            builtin constructor u64
            builtin type u64
            builtin enumerant uniform
            builtin function unpack2x16float
            builtin function unpack2x16snorm
            builtin function unpack2x16unorm
            builtin function unpack4x8snorm
            builtin function unpack4x8unorm
            builtin function unpack4xI8
            builtin function unpack4xU8
            builtin constructor vec2
            builtin type generator vec2
            builtin alias vec2f
            builtin constructor vec2f
            builtin alias vec2h
            builtin constructor vec2h
            builtin alias vec2i
            builtin constructor vec2i
            builtin alias vec2u
            builtin constructor vec2u
            builtin constructor vec3
            builtin type generator vec3
            builtin alias vec3f
            builtin constructor vec3f
            builtin alias vec3h
            builtin constructor vec3h
            builtin alias vec3i
            builtin constructor vec3i
            builtin alias vec3u
            builtin constructor vec3u
            builtin constructor vec4
            builtin type generator vec4
            builtin alias vec4f
            builtin constructor vec4f
            builtin alias vec4h
            builtin constructor vec4h
            builtin alias vec4i
            builtin constructor vec4i
            builtin alias vec4u
            builtin constructor vec4u
            builtin enumerant vertex_return
            builtin enumerant workgroup
            builtin function workgroupBarrier
            builtin function workgroupUniformLoad
            builtin enumerant write
        "#]],
    );
}
