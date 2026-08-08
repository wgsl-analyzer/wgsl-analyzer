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
            builtin function abs
            builtin type generator acceleration_structure
            builtin function acos
            builtin function acosh
            builtin function all
            builtin function any
            builtin function arrayLength
            builtin function asin
            builtin function asinh
            builtin function atan
            builtin function atan2
            builtin function atanh
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
            builtin function bitcast
            builtin type generator bool
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
            builtin type generator f16
            builtin type generator f32
            builtin type generator f64
            builtin function faceForward
            builtin function firstLeadingBit
            builtin function firstTrailingBit
            builtin function floor
            builtin function fma
            builtin function fract
            builtin function frexp
            builtin function fwidth
            builtin function fwidthCoarse
            builtin function fwidthFine
            builtin function getCandidateHitVertexPositions
            builtin function getCommittedHitVertexPositions
            builtin type generator i32
            builtin type generator i64
            builtin function insertBits
            builtin function inverseSqrt
            builtin function ldexp
            builtin function length
            builtin function log
            builtin function log2
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
            builtin function quadBroadcast
            builtin function quadSwapDiagonal
            builtin function quadSwapX
            builtin function quadSwapY
            builtin function quantizeToF16
            builtin function radians
            builtin function rayQueryConfirmIntersection
            builtin function rayQueryGenerateIntersection
            builtin function rayQueryGetCandidateIntersection
            builtin function rayQueryGetCommittedIntersection
            builtin function rayQueryInitialize
            builtin function rayQueryProceed
            builtin function rayQueryTerminate
            builtin type generator ray_query
            builtin function reflect
            builtin function refract
            builtin function reverseBits
            builtin function round
            builtin type generator sampler
            builtin type generator sampler_comparison
            builtin function saturate
            builtin function select
            builtin function sign
            builtin function sin
            builtin function sinh
            builtin function smoothstep
            builtin function sqrt
            builtin function step
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
            builtin type generator texture_depth_2d
            builtin type generator texture_depth_2d_array
            builtin type generator texture_depth_cube
            builtin type generator texture_depth_cube_array
            builtin type generator texture_depth_multisampled_2d
            builtin type generator texture_external
            builtin function transpose
            builtin function trunc
            builtin type generator u32
            builtin type generator u64
            builtin function unpack2x16float
            builtin function unpack2x16snorm
            builtin function unpack2x16unorm
            builtin function unpack4x8snorm
            builtin function unpack4x8unorm
            builtin function unpack4xI8
            builtin function unpack4xU8
            builtin function workgroupBarrier
            builtin function workgroupUniformLoad
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
            builtin function abs
            builtin type generator acceleration_structure
            builtin function acos
            builtin function acosh
            builtin function all
            builtin function any
            builtin function arrayLength
            builtin function asin
            builtin function asinh
            builtin function atan
            builtin function atan2
            builtin function atanh
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
            builtin function bitcast
            builtin type generator bool
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
            builtin type generator f16
            builtin type generator f32
            builtin type generator f64
            builtin function faceForward
            builtin function firstLeadingBit
            builtin function firstTrailingBit
            builtin function floor
            builtin function fma
            builtin function fract
            builtin function frexp
            builtin function fwidth
            builtin function fwidthCoarse
            builtin function fwidthFine
            builtin function getCandidateHitVertexPositions
            builtin function getCommittedHitVertexPositions
            builtin type generator i32
            builtin type generator i64
            builtin function insertBits
            builtin function inverseSqrt
            builtin function ldexp
            builtin function length
            builtin function log
            builtin function log2
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
            builtin function quadBroadcast
            builtin function quadSwapDiagonal
            builtin function quadSwapX
            builtin function quadSwapY
            builtin function quantizeToF16
            builtin function radians
            builtin function rayQueryConfirmIntersection
            builtin function rayQueryGenerateIntersection
            builtin function rayQueryGetCandidateIntersection
            builtin function rayQueryGetCommittedIntersection
            builtin function rayQueryInitialize
            builtin function rayQueryProceed
            builtin function rayQueryTerminate
            builtin type generator ray_query
            builtin function reflect
            builtin function refract
            builtin function reverseBits
            builtin function round
            builtin type generator sampler
            builtin type generator sampler_comparison
            builtin function saturate
            builtin function select
            builtin function sign
            builtin function sin
            builtin function sinh
            builtin function smoothstep
            builtin function sqrt
            builtin function step
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
            builtin type generator texture_depth_2d
            builtin type generator texture_depth_2d_array
            builtin type generator texture_depth_cube
            builtin type generator texture_depth_cube_array
            builtin type generator texture_depth_multisampled_2d
            builtin type generator texture_external
            builtin function transpose
            builtin function trunc
            builtin type generator u32
            builtin type generator u64
            builtin function unpack2x16float
            builtin function unpack2x16snorm
            builtin function unpack2x16unorm
            builtin function unpack4x8snorm
            builtin function unpack4x8unorm
            builtin function unpack4xI8
            builtin function unpack4xU8
            builtin function workgroupBarrier
            builtin function workgroupUniformLoad
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
            builtin function abs
            builtin type generator acceleration_structure
            builtin function acos
            builtin function acosh
            builtin function all
            builtin function any
            builtin function arrayLength
            builtin function asin
            builtin function asinh
            builtin function atan
            builtin function atan2
            builtin function atanh
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
            builtin function bitcast
            builtin type generator bool
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
            builtin type generator f16
            builtin type generator f32
            builtin type generator f64
            builtin function faceForward
            builtin function firstLeadingBit
            builtin function firstTrailingBit
            builtin function floor
            builtin function fma
            builtin function fract
            builtin function frexp
            builtin function fwidth
            builtin function fwidthCoarse
            builtin function fwidthFine
            builtin function getCandidateHitVertexPositions
            builtin function getCommittedHitVertexPositions
            builtin type generator i32
            builtin type generator i64
            builtin function insertBits
            builtin function inverseSqrt
            builtin function ldexp
            builtin function length
            builtin function log
            builtin function log2
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
            builtin function quadBroadcast
            builtin function quadSwapDiagonal
            builtin function quadSwapX
            builtin function quadSwapY
            builtin function quantizeToF16
            builtin function radians
            builtin function rayQueryConfirmIntersection
            builtin function rayQueryGenerateIntersection
            builtin function rayQueryGetCandidateIntersection
            builtin function rayQueryGetCommittedIntersection
            builtin function rayQueryInitialize
            builtin function rayQueryProceed
            builtin function rayQueryTerminate
            builtin type generator ray_query
            builtin function reflect
            builtin function refract
            builtin function reverseBits
            builtin function round
            builtin type generator sampler
            builtin type generator sampler_comparison
            builtin function saturate
            builtin function select
            builtin function sign
            builtin function sin
            builtin function sinh
            builtin function smoothstep
            builtin function sqrt
            builtin function step
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
            builtin type generator texture_depth_2d
            builtin type generator texture_depth_2d_array
            builtin type generator texture_depth_cube
            builtin type generator texture_depth_cube_array
            builtin type generator texture_depth_multisampled_2d
            builtin type generator texture_external
            builtin function transpose
            builtin function trunc
            builtin type generator u32
            builtin type generator u64
            builtin function unpack2x16float
            builtin function unpack2x16snorm
            builtin function unpack2x16unorm
            builtin function unpack4x8snorm
            builtin function unpack4x8unorm
            builtin function unpack4xI8
            builtin function unpack4xU8
            builtin function workgroupBarrier
            builtin function workgroupUniformLoad
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
            builtin function abs
            builtin type generator acceleration_structure
            builtin function acos
            builtin function acosh
            builtin function all
            builtin function any
            builtin function arrayLength
            builtin function asin
            builtin function asinh
            builtin function atan
            builtin function atan2
            builtin function atanh
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
            builtin function bitcast
            builtin type generator bool
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
            builtin type generator f16
            builtin type generator f32
            builtin type generator f64
            builtin function faceForward
            builtin function firstLeadingBit
            builtin function firstTrailingBit
            builtin function floor
            builtin function fma
            builtin function fract
            builtin function frexp
            builtin function fwidth
            builtin function fwidthCoarse
            builtin function fwidthFine
            builtin function getCandidateHitVertexPositions
            builtin function getCommittedHitVertexPositions
            builtin type generator i32
            builtin type generator i64
            builtin function insertBits
            builtin function inverseSqrt
            builtin function ldexp
            builtin function length
            builtin function log
            builtin function log2
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
            builtin function quadBroadcast
            builtin function quadSwapDiagonal
            builtin function quadSwapX
            builtin function quadSwapY
            builtin function quantizeToF16
            builtin function radians
            builtin function rayQueryConfirmIntersection
            builtin function rayQueryGenerateIntersection
            builtin function rayQueryGetCandidateIntersection
            builtin function rayQueryGetCommittedIntersection
            builtin function rayQueryInitialize
            builtin function rayQueryProceed
            builtin function rayQueryTerminate
            builtin type generator ray_query
            builtin function reflect
            builtin function refract
            builtin function reverseBits
            builtin function round
            builtin type generator sampler
            builtin type generator sampler_comparison
            builtin function saturate
            builtin function select
            builtin function sign
            builtin function sin
            builtin function sinh
            builtin function smoothstep
            builtin function sqrt
            builtin function step
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
            builtin type generator texture_depth_2d
            builtin type generator texture_depth_2d_array
            builtin type generator texture_depth_cube
            builtin type generator texture_depth_cube_array
            builtin type generator texture_depth_multisampled_2d
            builtin type generator texture_external
            builtin function transpose
            builtin function trunc
            builtin type generator u32
            builtin type generator u64
            builtin function unpack2x16float
            builtin function unpack2x16snorm
            builtin function unpack2x16unorm
            builtin function unpack4x8snorm
            builtin function unpack4x8unorm
            builtin function unpack4xI8
            builtin function unpack4xU8
            builtin function workgroupBarrier
            builtin function workgroupUniformLoad
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
            builtin function abs
            builtin type generator acceleration_structure
            builtin function acos
            builtin function acosh
            builtin function all
            builtin function any
            builtin function arrayLength
            builtin function asin
            builtin function asinh
            builtin function atan
            builtin function atan2
            builtin function atanh
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
            builtin function bitcast
            builtin type generator bool
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
            builtin type generator f16
            builtin type generator f32
            builtin type generator f64
            builtin function faceForward
            builtin function firstLeadingBit
            builtin function firstTrailingBit
            builtin function floor
            builtin function fma
            builtin function fract
            builtin function frexp
            builtin function fwidth
            builtin function fwidthCoarse
            builtin function fwidthFine
            builtin function getCandidateHitVertexPositions
            builtin function getCommittedHitVertexPositions
            builtin type generator i32
            builtin type generator i64
            builtin function insertBits
            builtin function inverseSqrt
            builtin function ldexp
            builtin function length
            builtin function log
            builtin function log2
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
            builtin function quadBroadcast
            builtin function quadSwapDiagonal
            builtin function quadSwapX
            builtin function quadSwapY
            builtin function quantizeToF16
            builtin function radians
            builtin function rayQueryConfirmIntersection
            builtin function rayQueryGenerateIntersection
            builtin function rayQueryGetCandidateIntersection
            builtin function rayQueryGetCommittedIntersection
            builtin function rayQueryInitialize
            builtin function rayQueryProceed
            builtin function rayQueryTerminate
            builtin type generator ray_query
            builtin function reflect
            builtin function refract
            builtin function reverseBits
            builtin function round
            builtin type generator sampler
            builtin type generator sampler_comparison
            builtin function saturate
            builtin function select
            builtin function sign
            builtin function sin
            builtin function sinh
            builtin function smoothstep
            builtin function sqrt
            builtin function step
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
            builtin type generator texture_depth_2d
            builtin type generator texture_depth_2d_array
            builtin type generator texture_depth_cube
            builtin type generator texture_depth_cube_array
            builtin type generator texture_depth_multisampled_2d
            builtin type generator texture_external
            builtin function transpose
            builtin function trunc
            builtin type generator u32
            builtin type generator u64
            builtin function unpack2x16float
            builtin function unpack2x16snorm
            builtin function unpack2x16unorm
            builtin function unpack4x8snorm
            builtin function unpack4x8unorm
            builtin function unpack4xI8
            builtin function unpack4xU8
            builtin function workgroupBarrier
            builtin function workgroupUniformLoad
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
            builtin function abs
            builtin type generator acceleration_structure
            builtin function acos
            builtin function acosh
            builtin function all
            builtin function any
            builtin function arrayLength
            builtin function asin
            builtin function asinh
            builtin function atan
            builtin function atan2
            builtin function atanh
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
            builtin function bitcast
            builtin type generator bool
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
            builtin type generator f16
            builtin type generator f32
            builtin type generator f64
            builtin function faceForward
            builtin function firstLeadingBit
            builtin function firstTrailingBit
            builtin function floor
            builtin function fma
            builtin function fract
            builtin function frexp
            builtin function fwidth
            builtin function fwidthCoarse
            builtin function fwidthFine
            builtin function getCandidateHitVertexPositions
            builtin function getCommittedHitVertexPositions
            builtin type generator i32
            builtin type generator i64
            builtin function insertBits
            builtin function inverseSqrt
            builtin function ldexp
            builtin function length
            builtin function log
            builtin function log2
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
            builtin function quadBroadcast
            builtin function quadSwapDiagonal
            builtin function quadSwapX
            builtin function quadSwapY
            builtin function quantizeToF16
            builtin function radians
            builtin function rayQueryConfirmIntersection
            builtin function rayQueryGenerateIntersection
            builtin function rayQueryGetCandidateIntersection
            builtin function rayQueryGetCommittedIntersection
            builtin function rayQueryInitialize
            builtin function rayQueryProceed
            builtin function rayQueryTerminate
            builtin type generator ray_query
            builtin function reflect
            builtin function refract
            builtin function reverseBits
            builtin function round
            builtin type generator sampler
            builtin type generator sampler_comparison
            builtin function saturate
            builtin function select
            builtin function sign
            builtin function sin
            builtin function sinh
            builtin function smoothstep
            builtin function sqrt
            builtin function step
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
            builtin type generator texture_depth_2d
            builtin type generator texture_depth_2d_array
            builtin type generator texture_depth_cube
            builtin type generator texture_depth_cube_array
            builtin type generator texture_depth_multisampled_2d
            builtin type generator texture_external
            builtin function transpose
            builtin function trunc
            builtin type generator u32
            builtin type generator u64
            builtin function unpack2x16float
            builtin function unpack2x16snorm
            builtin function unpack2x16unorm
            builtin function unpack4x8snorm
            builtin function unpack4x8unorm
            builtin function unpack4xI8
            builtin function unpack4xU8
            builtin function workgroupBarrier
            builtin function workgroupUniformLoad
        "#]],
    );
}
