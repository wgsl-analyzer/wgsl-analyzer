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

use base_db::{EditionedFileId, FilePosition, SourceDatabase};
use expect_test::Expect;
use hir::database::HirDatabase;
use hir::{PrefixKind, setup_tracing};
use ide_db::{FileId, RootDatabase, SnippetCapability};
use itertools::Itertools;
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
    completion_list_with_config(TEST_CONFIG, wa_fixture, true, None)
}

pub(crate) fn completion_list_no_kw(wa_fixture: &str) -> String {
    completion_list_with_config(TEST_CONFIG, wa_fixture, false, None)
}

pub(crate) fn completion_list_no_kw_with_private_editable(wa_fixture: &str) -> String {
    let mut config = TEST_CONFIG;
    completion_list_with_config(config, wa_fixture, false, None)
}

pub(crate) fn completion_list_with_trigger_character(
    wa_fixture: &str,
    trigger_character: Option<char>,
) -> String {
    completion_list_with_config(TEST_CONFIG, wa_fixture, true, trigger_character)
}

fn completion_list_with_config_raw(
    config: CompletionConfig,
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
    config: CompletionConfig,
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

/// Creates analysis from a multi-file fixture, returns positions marked with $0.
pub(crate) fn position(wa_fixture: &str) -> (RootDatabase, FilePosition) {
    let mut database = RootDatabase::default();
    let change_fixture = ChangeFixture::parse(wa_fixture);
    database.apply_change(change_fixture.change);
    let (file_id, range_or_offset) = change_fixture
        .file_position
        .expect("expected a marker ($0)");
    let file_id = EditionedFileId::new_unchecked(&database, file_id, syntax::Edition::Wgsl);
    let offset = range_or_offset.expect_offset();
    let position = FilePosition {
        file_id: file_id.file_id(&database),
        offset,
    };
    (database, position)
}

pub(crate) fn do_completion(
    code: &str,
    kind: CompletionItemKind,
) -> Vec<CompletionItem> {
    do_completion_with_config(TEST_CONFIG, code, kind)
}

pub(crate) fn do_completion_with_config(
    config: CompletionConfig,
    code: &str,
    kind: CompletionItemKind,
) -> Vec<CompletionItem> {
    get_all_items(config, code, None)
        .into_iter()
        .filter(|c| c.kind == kind)
        .sorted_by(|l, r| l.label.cmp(&r.label))
        .collect()
}

fn render_completion_list(completions: Vec<CompletionItem>) -> String {
    fn monospace_width(s: &str) -> usize {
        s.chars().count()
    }
    let label_width = completions
        .iter()
        .map(|it| {
            monospace_width(&it.label.primary)
                + monospace_width(it.label.detail_left.as_deref().unwrap_or_default())
                + monospace_width(it.label.detail_right.as_deref().unwrap_or_default())
                + it.label.detail_left.is_some() as usize
                + it.label.detail_right.is_some() as usize
        })
        .max()
        .unwrap_or_default();
    completions
        .into_iter()
        .map(|it| {
            let tag = it.kind.tag();
            let mut buf = format!("{tag} {}", it.label.primary);
            if let Some(label_detail) = &it.label.detail_left {
                format_to!(buf, " {label_detail}");
            }
            if let Some(detail_right) = it.label.detail_right {
                let pad_with = label_width.saturating_sub(
                    monospace_width(&it.label.primary)
                        + monospace_width(it.label.detail_left.as_deref().unwrap_or_default())
                        + monospace_width(&detail_right)
                        + it.label.detail_left.is_some() as usize,
                );
                format_to!(buf, "{:pad_with$}{detail_right}", "",);
            }
            if it.deprecated {
                format_to!(buf, " DEPRECATED");
            }
            format_to!(buf, "\n");
            buf
        })
        .collect()
}

pub(crate) fn check(
    wa_fixture: &str,
    expect: Expect,
) {
    let actual = completion_list(wa_fixture);
    expect.assert_eq(&actual);
}

pub(crate) fn check_with_base_items(
    wa_fixture: &str,
    expect: Expect,
) {
    check(&format!("{BASE_ITEMS_FIXTURE}{wa_fixture}"), expect)
}

pub(crate) fn check_no_kw(
    wa_fixture: &str,
    expect: Expect,
) {
    let actual = completion_list_no_kw(wa_fixture);
    expect.assert_eq(&actual)
}

pub(crate) fn check_with_private_editable(
    wa_fixture: &str,
    expect: Expect,
) {
    let actual = completion_list_no_kw_with_private_editable(wa_fixture);
    expect.assert_eq(&actual);
}

pub(crate) fn check_with_trigger_character(
    wa_fixture: &str,
    trigger_character: Option<char>,
    expect: Expect,
) {
    let actual = completion_list_with_trigger_character(wa_fixture, trigger_character);
    expect.assert_eq(&actual)
}

pub(crate) fn get_all_items(
    config: CompletionConfig,
    code: &str,
    trigger_character: Option<char>,
) -> Vec<CompletionItem> {
    let (db, position) = position(code);
    HirDatabase::zalsa_register_downcaster(&db);
    let result = crate::completions(&db, &config, position, trigger_character)
        .map_or_else(Vec::default, Into::into);
    // validate
    result.iter().for_each(|completion_item| {
        let sr = completion_item.source_range;
        assert!(
            sr.contains_inclusive(position.offset),
            "source range {sr:?} does not contain the offset {:?} of the completion request: {completion_item:?}",
            position.offset
        );
    });
    result
}

#[test]
// TODO https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1321
fn no_completions_in_comments() {
    assert_eq!(
        completion_list(
            "
            fn test() {
                let x = 2; // A comment$0
            }
            ",
        ),
        "function abs\nfunction acos\nfunction all\nfunction any\nfunction arrayLength\nfunction asin\nfunction atan\nfunction atan2\nfunction atomicAdd\nfunction atomicAnd\nfunction atomicExchange\nfunction atomicLoad\nfunction atomicMax\nfunction atomicMin\nfunction atomicOr\nfunction atomicStore\nfunction atomicSub\nfunction atomicXor\nfunction bitcast\nfunction ceil\nfunction clamp\nfunction cos\nfunction cosh\nfunction countLeadingZeros\nfunction countOneBits\nfunction countTrailingZeros\nfunction cross\nfunction degrees\nfunction determinant\nfunction distance\nfunction dot\nfunction dpdx\nfunction dpdxCoarse\nfunction dpdxFine\nfunction dpdy\nfunction dpdyCoarse\nfunction dpdyFine\nfunction exp\nfunction exp2\nfunction extractBits\nfunction faceForward\nfunction firstLeadingBit\nfunction firstTrailingBit\nfunction floor\nfunction fma\nfunction fract\nfunction fwidth\nfunction fwidthCoarse\nfunction fwidthFine\nfunction insertBits\nfunction inverseSqrt\nfunction isFinite\nfunction isInf\nfunction isNan\nfunction isNormal\nfunction length\nfunction log\nfunction log2\nfunction max\nfunction min\nfunction mix\nfunction normalize\nfunction pack2x16float\nfunction pack2x16snorm\nfunction pack2x16unorm\nfunction pack4x8snorm\nfunction pack4x8unorm\nfunction pow\nfunction quantizeToF16\nfunction radians\nfunction reflect\nfunction refract\nfunction reverseBits\nfunction round\nfunction saturate\nfunction select\nfunction sign\nfunction sin\nfunction sinh\nfunction smoothstep\nfunction sqrt\nfunction step\nfunction storageBarrier\nfunction tan\nfunction tanh\nfunction test               fn test()\nfunction textureDimensions\nfunction textureGather\nfunction textureGatherCompare\nfunction textureLoad\nfunction textureNumLayers\nfunction textureNumLevels\nfunction textureNumSamples\nfunction textureSample\nfunction textureSampleBaseClampToEdge\nfunction textureSampleBias\nfunction textureSampleCompare\nfunction textureSampleCompareLevel\nfunction textureSampleGrad\nfunction textureSampleLevel\nfunction textureStore\nfunction transpose\nfunction trunc\nfunction unpack2x16float\nfunction unpack2x16snorm\nfunction unpack2x16unorm\nfunction unpack4x8snorm\nfunction unpack4x8unorm\nfunction workgroupBarrier\nfunction workgroupUniformLoad\n".to_owned(),
    );
    assert_eq!(
        completion_list(
            "
            fn test() {
                /*
                    Some multi-line comment$0
                */
            }
            ",
        ),
        "function abs\nfunction acos\nfunction all\nfunction any\nfunction arrayLength\nfunction asin\nfunction atan\nfunction atan2\nfunction atomicAdd\nfunction atomicAnd\nfunction atomicExchange\nfunction atomicLoad\nfunction atomicMax\nfunction atomicMin\nfunction atomicOr\nfunction atomicStore\nfunction atomicSub\nfunction atomicXor\nfunction bitcast\nfunction ceil\nfunction clamp\nfunction cos\nfunction cosh\nfunction countLeadingZeros\nfunction countOneBits\nfunction countTrailingZeros\nfunction cross\nfunction degrees\nfunction determinant\nfunction distance\nfunction dot\nfunction dpdx\nfunction dpdxCoarse\nfunction dpdxFine\nfunction dpdy\nfunction dpdyCoarse\nfunction dpdyFine\nfunction exp\nfunction exp2\nfunction extractBits\nfunction faceForward\nfunction firstLeadingBit\nfunction firstTrailingBit\nfunction floor\nfunction fma\nfunction fract\nfunction fwidth\nfunction fwidthCoarse\nfunction fwidthFine\nfunction insertBits\nfunction inverseSqrt\nfunction isFinite\nfunction isInf\nfunction isNan\nfunction isNormal\nfunction length\nfunction log\nfunction log2\nfunction max\nfunction min\nfunction mix\nfunction normalize\nfunction pack2x16float\nfunction pack2x16snorm\nfunction pack2x16unorm\nfunction pack4x8snorm\nfunction pack4x8unorm\nfunction pow\nfunction quantizeToF16\nfunction radians\nfunction reflect\nfunction refract\nfunction reverseBits\nfunction round\nfunction saturate\nfunction select\nfunction sign\nfunction sin\nfunction sinh\nfunction smoothstep\nfunction sqrt\nfunction step\nfunction storageBarrier\nfunction tan\nfunction tanh\nfunction test               fn test()\nfunction textureDimensions\nfunction textureGather\nfunction textureGatherCompare\nfunction textureLoad\nfunction textureNumLayers\nfunction textureNumLevels\nfunction textureNumSamples\nfunction textureSample\nfunction textureSampleBaseClampToEdge\nfunction textureSampleBias\nfunction textureSampleCompare\nfunction textureSampleCompareLevel\nfunction textureSampleGrad\nfunction textureSampleLevel\nfunction textureStore\nfunction transpose\nfunction trunc\nfunction unpack2x16float\nfunction unpack2x16snorm\nfunction unpack2x16unorm\nfunction unpack4x8snorm\nfunction unpack4x8unorm\nfunction workgroupBarrier\nfunction workgroupUniformLoad\n".to_owned(),
    );
    assert_eq!(
        completion_list(
            "
            fn test() {
                /// Some doc comment
                /// let test$0 = 1
            }
            ",
        ),
        "function abs\nfunction acos\nfunction all\nfunction any\nfunction arrayLength\nfunction asin\nfunction atan\nfunction atan2\nfunction atomicAdd\nfunction atomicAnd\nfunction atomicExchange\nfunction atomicLoad\nfunction atomicMax\nfunction atomicMin\nfunction atomicOr\nfunction atomicStore\nfunction atomicSub\nfunction atomicXor\nfunction bitcast\nfunction ceil\nfunction clamp\nfunction cos\nfunction cosh\nfunction countLeadingZeros\nfunction countOneBits\nfunction countTrailingZeros\nfunction cross\nfunction degrees\nfunction determinant\nfunction distance\nfunction dot\nfunction dpdx\nfunction dpdxCoarse\nfunction dpdxFine\nfunction dpdy\nfunction dpdyCoarse\nfunction dpdyFine\nfunction exp\nfunction exp2\nfunction extractBits\nfunction faceForward\nfunction firstLeadingBit\nfunction firstTrailingBit\nfunction floor\nfunction fma\nfunction fract\nfunction fwidth\nfunction fwidthCoarse\nfunction fwidthFine\nfunction insertBits\nfunction inverseSqrt\nfunction isFinite\nfunction isInf\nfunction isNan\nfunction isNormal\nfunction length\nfunction log\nfunction log2\nfunction max\nfunction min\nfunction mix\nfunction normalize\nfunction pack2x16float\nfunction pack2x16snorm\nfunction pack2x16unorm\nfunction pack4x8snorm\nfunction pack4x8unorm\nfunction pow\nfunction quantizeToF16\nfunction radians\nfunction reflect\nfunction refract\nfunction reverseBits\nfunction round\nfunction saturate\nfunction select\nfunction sign\nfunction sin\nfunction sinh\nfunction smoothstep\nfunction sqrt\nfunction step\nfunction storageBarrier\nfunction tan\nfunction tanh\nfunction test               fn test()\nfunction textureDimensions\nfunction textureGather\nfunction textureGatherCompare\nfunction textureLoad\nfunction textureNumLayers\nfunction textureNumLevels\nfunction textureNumSamples\nfunction textureSample\nfunction textureSampleBaseClampToEdge\nfunction textureSampleBias\nfunction textureSampleCompare\nfunction textureSampleCompareLevel\nfunction textureSampleGrad\nfunction textureSampleLevel\nfunction textureStore\nfunction transpose\nfunction trunc\nfunction unpack2x16float\nfunction unpack2x16snorm\nfunction unpack2x16unorm\nfunction unpack4x8snorm\nfunction unpack4x8unorm\nfunction workgroupBarrier\nfunction workgroupUniformLoad\n".to_owned(),
    );
}
