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
        .map(|completion_item| {
            monospace_width(&completion_item.label.primary)
                + monospace_width(
                    completion_item
                        .label
                        .detail_left
                        .as_deref()
                        .unwrap_or_default(),
                )
                + monospace_width(
                    completion_item
                        .label
                        .detail_right
                        .as_deref()
                        .unwrap_or_default(),
                )
                + usize::from(completion_item.label.detail_left.is_some())
                + usize::from(completion_item.label.detail_right.is_some())
        })
        .max()
        .unwrap_or_default();
    completions
        .into_iter()
        .map(|completion_item| {
            let tag = completion_item.kind.tag();
            let mut buffer = format!("{tag} {}", completion_item.label.primary);
            if let Some(label_detail) = &completion_item.label.detail_left {
                format_to!(buffer, " {label_detail}");
            }
            if let Some(detail_right) = completion_item.label.detail_right {
                let pad_with = label_width.saturating_sub(
                    monospace_width(&completion_item.label.primary)
                        + monospace_width(
                            completion_item
                                .label
                                .detail_left
                                .as_deref()
                                .unwrap_or_default(),
                        )
                        + monospace_width(&detail_right)
                        + usize::from(completion_item.label.detail_left.is_some()),
                );
                format_to!(buffer, "{:pad_with$}{detail_right}", "",);
            }
            if completion_item.deprecated {
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
    check(
        "
            fn test() {
                /*
                    Some multi-line comment$0
                */
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
    check(
        "
            fn test() {
                /// Some doc comment
                /// let test$0 = 1
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
