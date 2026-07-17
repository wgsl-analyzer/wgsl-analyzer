//! Test for each completion "kind" works to provide basic coverage.

#![expect(clippy::too_many_lines, reason = "snapshot test data")]

use crate::tests::completion_list;

#[test]
fn complete_field() {
    assert_eq!(
        completion_list(
            "
            struct Foo { bar: u32 }
            fn test() {
                let test = Foo(0);
                let x = test.$0;
            }
            ",
        ),
        "field bar
"
        .to_owned(),
    );
}

#[test]
fn complete_function() {
    assert_eq!(
        completion_list(
            "
            fn foo() { }
            fn bar() { $0 }
            ",
        ),
        "function abs
function acos
function all
function any
function arrayLength
function asin
function atan
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
"
        .to_owned(),
    );
}

#[test]
fn complete_variable() {
    assert_eq!(
        completion_list(
            "
            fn test() {
                let test = 0;
                let x = $0;
            }
            ",
        ),
        "function abs
function acos
function all
function any
function arrayLength
function asin
function atan
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
variable test                     i32
"
        .to_owned(),
    );
}

#[test]
// TODO https://github.com/wgsl-analyzer/wgsl-analyzer/issues/314
fn complete_keyword() {
    assert_eq!(
        completion_list(
            "
            fn test() {
                $0
            }
            ",
        ),
        "function abs
function acos
function all
function any
function arrayLength
function asin
function atan
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
"
        .to_owned(),
    );
}

#[test]
// TODO https://github.com/wgsl-analyzer/wgsl-analyzer/issues/921
fn complete_snippet() {
    assert_eq!(
        completion_list(
            "
            fn test() {
                $0
            }
            ",
        ),
        "function abs
function acos
function all
function any
function arrayLength
function asin
function atan
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
"
        .to_owned(),
    );
}

#[test]
fn complete_constant() {
    assert_eq!(
        completion_list(
            "
            const Foo: u32 = 0;
            fn test() {
                let x = $0;
            }
            ",
        ),
        "function abs
function acos
function all
function any
function arrayLength
function asin
function atan
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
constant Foo           const Foo: u32
"
        .to_owned(),
    );
}

#[test]
fn complete_struct() {
    assert_eq!(
        completion_list(
            "
            struct Foo { bar: u32 }
            fn test() {
                let x = $0;
            }
            ",
        ),
        "function abs
function acos
function all
function any
function arrayLength
function asin
function atan
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
struct Foo               struct Foo
"
        .to_owned(),
    );
}

#[test]
// TODO https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1323
fn complete_module() {
    assert_eq!(
        completion_list(
            "
            //- /shaders.wesl edition:2026_pre
            import package::$0;
            //- /shaders/foo.wesl
            alias Foo = u32;
            ",
        ),
        "".to_owned(),
    );
}

#[test]
fn complete_type_alias() {
    assert_eq!(
        completion_list(
            "
            alias Foo = u32;
            fn test() {
                let x: $0 = 1;
            }
            ",
        ),
        "function abs
function acos
function all
function any
function arrayLength
function asin
function atan
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
type alias Foo                alias Foo
"
        .to_owned(),
    );
}
