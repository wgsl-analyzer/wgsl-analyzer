//! Tests and test utilities for completions.
//!
//! Most tests live in this module or its submodules. The tests in these submodules are "location"
//! oriented, that is they try to check completions for something like type position, param position
//! etc.
//! Tests that are more orientated towards specific completion types like visibility checks of path
//! completions or `check_edit` tests usually live in their respective completion modules instead.
//! This gives this test module and its submodules here the main purpose of giving the developer an
//! overview of whats being completed where, not how.

mod expression;
mod module_items;
mod wesl;

use base_db::{EditionedFileId, FilePosition, SourceDatabase, change};
use expect_test::{Expect, expect};
use hir::db::HirDatabase;
use hir::setup_tracing;
use ide_db::{FileId, RootDatabase, SnippetCapability};
use itertools::Itertools as _;
use stdx::{format_to, trim_indent};
use test_fixture::ChangeFixture;
use test_utils::assert_eq_text;

use crate::{
    CallableSnippets, CompletionConfig, CompletionFieldsToResolve, CompletionItem,
    CompletionItemKind,
};

/// Basic item definitions.
const BASE_ITEMS_FIXTURE: &str = "";

pub(crate) const TEST_CONFIG: CompletionConfig = CompletionConfig {
    // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/913
    // enable_postfix_completions: true,
    // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/914
    // enable_imports_on_the_fly: true,
    // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/915
    // enable_term_search: true,
    // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/915
    // term_search_fuel: 200,
    // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/916
    // full_function_signatures: false,
    // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/917
    // callable: Some(CallableSnippets::FillArguments),
    // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/920
    // snippet_cap: SnippetCap::new(true),
    // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/914
    // insert_import: InsertImportConfig {
    //     granularity: ImportGranularity::Crate,
    //     prefix_kind: PrefixKind::Plain,
    //     enforce_granularity: true,
    //     group: true,
    //     skip_glob_imports: true,
    // },
    // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/922
    // prefer_prelude: true,
    // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/921
    // snippets: Vec::new(),
    // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/919
    limit: None,
    fields_to_resolve: CompletionFieldsToResolve::empty(),
    // TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/914
    // exclude_flyimport: vec![],
};

pub(crate) fn completion_list(wa_fixture: &str) -> String {
    completion_list_with_config(&TEST_CONFIG, wa_fixture, true, None)
}

pub(crate) fn completion_list_no_kw(wa_fixture: &str) -> String {
    completion_list_with_config(&TEST_CONFIG, wa_fixture, false, None)
}

pub(crate) fn completion_list_no_kw_with_private_editable(wa_fixture: &str) -> String {
    completion_list_with_config(&TEST_CONFIG, wa_fixture, false, None)
}

pub(crate) fn completion_list_with_trigger_character(
    wa_fixture: &str,
    trigger_character: Option<char>,
) -> String {
    completion_list_with_config(&TEST_CONFIG, wa_fixture, true, trigger_character)
}

fn completion_list_with_config_raw(
    config: &CompletionConfig,
    wa_fixture: &str,
    include_keywords: bool,
    trigger_character: Option<char>,
) -> Vec<CompletionItem> {
    let _tracing = setup_tracing();

    // filter out all but one built-in type completion for smaller test outputs
    let items = get_all_items(config, wa_fixture, trigger_character);
    items
        .into_iter()
        .filter(|it| include_keywords || it.kind != CompletionItemKind::Keyword)
        .filter(|it| include_keywords || it.kind != CompletionItemKind::Snippet)
        .sorted_by_key(|it| {
            (
                it.kind,
                it.label.primary.clone(),
                it.label.detail_left.as_ref().map(ToOwned::to_owned),
            )
        })
        .collect()
}

fn completion_list_with_config(
    config: &CompletionConfig,
    wa_fixture: &str,
    include_keywords: bool,
    trigger_character: Option<char>,
) -> String {
    render_completion_list(completion_list_with_config_raw(
        config,
        wa_fixture,
        include_keywords,
        trigger_character,
    ))
}

/// Creates analysis from a multi-file fixture and returns the position marked with $0.
pub(crate) fn position(wa_fixture: &str) -> (RootDatabase, FilePosition) {
    let mut db = RootDatabase::default();
    let change_fixture = ChangeFixture::parse(wa_fixture);
    db.apply_change(change_fixture.change);
    let (file_id, range_or_offset) = change_fixture
        .file_position
        .expect("expected a marker ($0)");
    let offset = range_or_offset.expect_offset();
    let position = FilePosition { file_id, offset };
    (db, position)
}

pub(crate) fn do_completion(
    code: &str,
    kind: CompletionItemKind,
) -> Vec<CompletionItem> {
    do_completion_with_config(&TEST_CONFIG, code, kind)
}

pub(crate) fn do_completion_with_config(
    config: &CompletionConfig,
    code: &str,
    kind: CompletionItemKind,
) -> Vec<CompletionItem> {
    get_all_items(config, code, None)
        .into_iter()
        .filter(|completion_item| completion_item.kind == kind)
        .sorted_by(|left, right| left.label.cmp(&right.label))
        .collect()
}

fn render_completion_list(mut completions: Vec<CompletionItem>) -> String {
    fn monospace_width(string: &str) -> usize {
        string.chars().count()
    }
    completions.sort_by(|first, other| first.label.cmp(&other.label));
    let label_width = completions
        .iter()
        .map(|it| {
            monospace_width(&it.label.primary)
                + monospace_width(it.label.detail_left.as_deref().unwrap_or_default())
                + monospace_width(it.label.detail_right.as_deref().unwrap_or_default())
                + usize::from(it.label.detail_left.is_some())
                + usize::from(it.label.detail_right.is_some())
        })
        .max()
        .unwrap_or_default();
    completions
        .into_iter()
        .map(|it| {
            let tag = it.kind.tag();
            let mut buffer = format!("{tag} {}", it.label.primary);
            if let Some(label_detail) = &it.label.detail_left {
                format_to!(buffer, " {label_detail}");
            }
            if let Some(detail_right) = it.label.detail_right {
                let pad_with = label_width.saturating_sub(
                    monospace_width(&it.label.primary)
                        + monospace_width(it.label.detail_left.as_deref().unwrap_or_default())
                        + monospace_width(&detail_right)
                        + usize::from(it.label.detail_left.is_some()),
                );
                format_to!(buffer, "{:pad_with$}{detail_right}", "",);
            }
            if it.deprecated {
                format_to!(buffer, " DEPRECATED");
            }
            format_to!(buffer, "\n");
            buffer
        })
        .collect()
}

#[expect(clippy::needless_pass_by_value, reason = "Matches expect! macro")]
pub(crate) fn check(
    wa_fixture: &str,
    expect: Expect,
) {
    let actual = completion_list(wa_fixture);
    expect.assert_eq(&actual);
}

#[expect(clippy::needless_pass_by_value, reason = "Matches expect! macro")]
pub(crate) fn check_no_kw(
    wa_fixture: &str,
    expect: Expect,
) {
    let actual = completion_list_no_kw(wa_fixture);
    expect.assert_eq(&actual);
}

#[expect(clippy::needless_pass_by_value, reason = "Matches expect! macro")]
pub(crate) fn check_with_private_editable(
    wa_fixture: &str,
    expect: Expect,
) {
    let actual = completion_list_no_kw_with_private_editable(wa_fixture);
    expect.assert_eq(&actual);
}

#[expect(clippy::needless_pass_by_value, reason = "Matches expect! macro")]
pub(crate) fn check_with_trigger_character(
    wa_fixture: &str,
    trigger_character: Option<char>,
    expect: Expect,
) {
    let actual = completion_list_with_trigger_character(wa_fixture, trigger_character);
    expect.assert_eq(&actual);
}

pub(crate) fn get_all_items(
    config: &CompletionConfig,
    code: &str,
    trigger_character: Option<char>,
) -> Vec<CompletionItem> {
    let (db, position) = position(code);
    HirDatabase::zalsa_register_downcaster(&db);
    let result = crate::completions(&db, config, position, trigger_character)
        .map_or_else(Vec::default, Into::into);
    // validate
    for completion_item in &result {
        let sr = completion_item.source_range;
        assert!(
            sr.contains_inclusive(position.offset),
            "source range {sr:?} does not contain the offset {:?} of the completion request: {completion_item:?}",
            position.offset
        );
    }
    result
}

#[test]
#[expect(clippy::too_many_lines, reason = "Needs fixing, see the TODO")]
// TODO https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1321
fn no_completions_in_comments() {
    check(
        "
            fn test() {
                let x = 2; // A comment$0
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
            function bitcast<f16>
            function bitcast<f32>
            function bitcast<i32>
            function bitcast<i64>
            function bitcast<u32>
            function bitcast<u64>
            function bitcast<vec2<f16>>
            function bitcast<vec2<f32>>
            function bitcast<vec2<i32>>
            function bitcast<vec2<i64>>
            function bitcast<vec2<u32>>
            function bitcast<vec2<u64>>
            function bitcast<vec3<f16>>
            function bitcast<vec3<f32>>
            function bitcast<vec3<i32>>
            function bitcast<vec3<i64>>
            function bitcast<vec3<u32>>
            function bitcast<vec3<u64>>
            function bitcast<vec4<f16>>
            function bitcast<vec4<f32>>
            function bitcast<vec4<i32>>
            function bitcast<vec4<i64>>
            function bitcast<vec4<u32>>
            function bitcast<vec4<u64>>
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
            function pack4xI8
            function pack4xI8Clamp
            function pack4xU8
            function pack4xU8Clamp
            function pow
            function quadBroadcast
            function quadSwapDiagonal
            function quadSwapX
            function quadSwapY
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
            function subgroupAdd
            function subgroupAll
            function subgroupAnd
            function subgroupAny
            function subgroupBallot
            function subgroupBroadcast
            function subgroupBroadcastFirst
            function subgroupElect
            function subgroupExclusiveAdd
            function subgroupExclusiveMul
            function subgroupInclusiveAdd
            function subgroupInclusiveMul
            function subgroupMax
            function subgroupMin
            function subgroupMul
            function subgroupOr
            function subgroupShuffle
            function subgroupShuffleDown
            function subgroupShuffleUp
            function subgroupShuffleXor
            function subgroupXor
            function tan
            function tanh
            function test               fn test()
            function textureBarrier
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
            function unpack4xI8
            function unpack4xU8
            function workgroupBarrier
            function workgroupUniformLoad
        "#]],
    );
    check(
        "
            fn test() {
                /*
                    Some multi-line comment$0
                */
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
            function bitcast<f16>
            function bitcast<f32>
            function bitcast<i32>
            function bitcast<i64>
            function bitcast<u32>
            function bitcast<u64>
            function bitcast<vec2<f16>>
            function bitcast<vec2<f32>>
            function bitcast<vec2<i32>>
            function bitcast<vec2<i64>>
            function bitcast<vec2<u32>>
            function bitcast<vec2<u64>>
            function bitcast<vec3<f16>>
            function bitcast<vec3<f32>>
            function bitcast<vec3<i32>>
            function bitcast<vec3<i64>>
            function bitcast<vec3<u32>>
            function bitcast<vec3<u64>>
            function bitcast<vec4<f16>>
            function bitcast<vec4<f32>>
            function bitcast<vec4<i32>>
            function bitcast<vec4<i64>>
            function bitcast<vec4<u32>>
            function bitcast<vec4<u64>>
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
            function pack4xI8
            function pack4xI8Clamp
            function pack4xU8
            function pack4xU8Clamp
            function pow
            function quadBroadcast
            function quadSwapDiagonal
            function quadSwapX
            function quadSwapY
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
            function subgroupAdd
            function subgroupAll
            function subgroupAnd
            function subgroupAny
            function subgroupBallot
            function subgroupBroadcast
            function subgroupBroadcastFirst
            function subgroupElect
            function subgroupExclusiveAdd
            function subgroupExclusiveMul
            function subgroupInclusiveAdd
            function subgroupInclusiveMul
            function subgroupMax
            function subgroupMin
            function subgroupMul
            function subgroupOr
            function subgroupShuffle
            function subgroupShuffleDown
            function subgroupShuffleUp
            function subgroupShuffleXor
            function subgroupXor
            function tan
            function tanh
            function test               fn test()
            function textureBarrier
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
            function unpack4xI8
            function unpack4xU8
            function workgroupBarrier
            function workgroupUniformLoad
        "#]],
    );
    check(
        "
            fn test() {
                /// Some doc comment
                /// let test$0 = 1
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
            function bitcast<f16>
            function bitcast<f32>
            function bitcast<i32>
            function bitcast<i64>
            function bitcast<u32>
            function bitcast<u64>
            function bitcast<vec2<f16>>
            function bitcast<vec2<f32>>
            function bitcast<vec2<i32>>
            function bitcast<vec2<i64>>
            function bitcast<vec2<u32>>
            function bitcast<vec2<u64>>
            function bitcast<vec3<f16>>
            function bitcast<vec3<f32>>
            function bitcast<vec3<i32>>
            function bitcast<vec3<i64>>
            function bitcast<vec3<u32>>
            function bitcast<vec3<u64>>
            function bitcast<vec4<f16>>
            function bitcast<vec4<f32>>
            function bitcast<vec4<i32>>
            function bitcast<vec4<i64>>
            function bitcast<vec4<u32>>
            function bitcast<vec4<u64>>
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
            function pack4xI8
            function pack4xI8Clamp
            function pack4xU8
            function pack4xU8Clamp
            function pow
            function quadBroadcast
            function quadSwapDiagonal
            function quadSwapX
            function quadSwapY
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
            function subgroupAdd
            function subgroupAll
            function subgroupAnd
            function subgroupAny
            function subgroupBallot
            function subgroupBroadcast
            function subgroupBroadcastFirst
            function subgroupElect
            function subgroupExclusiveAdd
            function subgroupExclusiveMul
            function subgroupInclusiveAdd
            function subgroupInclusiveMul
            function subgroupMax
            function subgroupMin
            function subgroupMul
            function subgroupOr
            function subgroupShuffle
            function subgroupShuffleDown
            function subgroupShuffleUp
            function subgroupShuffleXor
            function subgroupXor
            function tan
            function tanh
            function test               fn test()
            function textureBarrier
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
            function unpack4xI8
            function unpack4xU8
            function workgroupBarrier
            function workgroupUniformLoad
        "#]],
    );
}
