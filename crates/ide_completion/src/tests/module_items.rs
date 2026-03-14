//! Test for each completion "kind" works to provide basic coverage.

#![expect(clippy::too_many_lines, reason = "snapshot test data")]

use crate::tests::{check, completion_list};
use expect_test::expect;

#[test]
fn complete_field() {
    check(
        "
            struct Foo { bar: u32 }
            fn test() {
                let test = Foo(0);
                let x = test.$0;
            }
            ",
        expect![["field bar
"]],
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
            function abs
            function acos
            function acosh
            function all
            function any
            function arrayLength
            function asin
            function asinh
            function atan
            function atan2
            function atanh
            function atomicAdd
            function atomicAnd
            function atomicCompareExchangeWeak
            function atomicExchange
            function atomicLoad
            function atomicMax
            function atomicMin
            function atomicOr
            function atomicStore
            function atomicSub
            function atomicXor
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
            function dot4I8Packed
            function dot4U8Packed
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
            function fract
            function frexp
            function fwidth
            function fwidthCoarse
            function fwidthFine
            function insertBits
            function inverseSqrt
            function isFinite
            function isInf
            function isNan
            function isNormal
            function ldexp
            function length
            function log
            function log2
            function max
            function min
            function mix
            function modf
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
            function test               fn test()
            variable test                     i32
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
            function abs
            function acos
            function acosh
            function all
            function any
            function arrayLength
            function asin
            function asinh
            function atan
            function atan2
            function atanh
            function atomicAdd
            function atomicAnd
            function atomicCompareExchangeWeak
            function atomicExchange
            function atomicLoad
            function atomicMax
            function atomicMin
            function atomicOr
            function atomicStore
            function atomicSub
            function atomicXor
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
            function dot4I8Packed
            function dot4U8Packed
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
            function fract
            function frexp
            function fwidth
            function fwidthCoarse
            function fwidthFine
            function insertBits
            function inverseSqrt
            function isFinite
            function isInf
            function isNan
            function isNormal
            function ldexp
            function length
            function log
            function log2
            function max
            function min
            function mix
            function modf
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
            function test               fn test()
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
            function abs
            function acos
            function acosh
            function all
            function any
            function arrayLength
            function asin
            function asinh
            function atan
            function atan2
            function atanh
            function atomicAdd
            function atomicAnd
            function atomicCompareExchangeWeak
            function atomicExchange
            function atomicLoad
            function atomicMax
            function atomicMin
            function atomicOr
            function atomicStore
            function atomicSub
            function atomicXor
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
            function dot4I8Packed
            function dot4U8Packed
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
            function fract
            function frexp
            function fwidth
            function fwidthCoarse
            function fwidthFine
            function insertBits
            function inverseSqrt
            function isFinite
            function isInf
            function isNan
            function isNormal
            function ldexp
            function length
            function log
            function log2
            function max
            function min
            function mix
            function modf
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
            function test               fn test()
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
            constant Foo           const Foo: u32
            function abs
            function acos
            function acosh
            function all
            function any
            function arrayLength
            function asin
            function asinh
            function atan
            function atan2
            function atanh
            function atomicAdd
            function atomicAnd
            function atomicCompareExchangeWeak
            function atomicExchange
            function atomicLoad
            function atomicMax
            function atomicMin
            function atomicOr
            function atomicStore
            function atomicSub
            function atomicXor
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
            function dot4I8Packed
            function dot4U8Packed
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
            function fract
            function frexp
            function fwidth
            function fwidthCoarse
            function fwidthFine
            function insertBits
            function inverseSqrt
            function isFinite
            function isInf
            function isNan
            function isNormal
            function ldexp
            function length
            function log
            function log2
            function max
            function min
            function mix
            function modf
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
            function test               fn test()
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
            struct Foo               struct Foo
            function abs
            function acos
            function acosh
            function all
            function any
            function arrayLength
            function asin
            function asinh
            function atan
            function atan2
            function atanh
            function atomicAdd
            function atomicAnd
            function atomicCompareExchangeWeak
            function atomicExchange
            function atomicLoad
            function atomicMax
            function atomicMin
            function atomicOr
            function atomicStore
            function atomicSub
            function atomicXor
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
            function dot4I8Packed
            function dot4U8Packed
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
            function fract
            function frexp
            function fwidth
            function fwidthCoarse
            function fwidthFine
            function insertBits
            function inverseSqrt
            function isFinite
            function isInf
            function isNan
            function isNormal
            function ldexp
            function length
            function log
            function log2
            function max
            function min
            function mix
            function modf
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
            function test               fn test()
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
            type alias Foo                alias Foo
            function abs
            function acos
            function acosh
            function all
            function any
            function arrayLength
            function asin
            function asinh
            function atan
            function atan2
            function atanh
            function atomicAdd
            function atomicAnd
            function atomicCompareExchangeWeak
            function atomicExchange
            function atomicLoad
            function atomicMax
            function atomicMin
            function atomicOr
            function atomicStore
            function atomicSub
            function atomicXor
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
            function dot4I8Packed
            function dot4U8Packed
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
            function fract
            function frexp
            function fwidth
            function fwidthCoarse
            function fwidthFine
            function insertBits
            function inverseSqrt
            function isFinite
            function isInf
            function isNan
            function isNormal
            function ldexp
            function length
            function log
            function log2
            function max
            function min
            function mix
            function modf
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
            function test               fn test()
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
        "#]],
    );
}
