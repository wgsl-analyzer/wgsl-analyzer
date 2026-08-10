use expect_test::expect;
use syntax::ExtensionsConfig;

use crate::tests::check_infer;

#[test]
fn atomicCompareExchangeWeak() {
    check_infer(
        ExtensionsConfig::default(),
        "
var<storage, read_write> buffer: atomic<bool>;
fn foo() {
    let result = atomicCompareExchangeWeak(&buffer, true, false);
    let old_value = result.old_value;
    let exchanged = result.exchanged;
}
",
        expect![[r#"
            25..31 'buffer': ref<storage, atomic<[error]>, read_write>
            40..44 'bool': unexpected template argument, expected i32, u32, i64, or u64
            66..72 'result': [error]
            75..122 'atomic...false)': [error]
            101..108 '&buffer': ptr<storage, atomic<[error]>, read_write>
            102..108 'buffer': ref<storage, atomic<[error]>, read_write>
            110..114 'true': bool
            116..121 'false': bool
            132..141 'old_value': [error]
            144..150 'result': [error]
            144..160 'result..._value': [error]
            170..179 'exchanged': [error]
            182..188 'result': [error]
            182..198 'result...hanged': [error]
            [EditionedFileId(Id(1800))] WgslError { expression: Idx::<Expression>(4), message: "`atomicCompareExchangeWeak` 2nd and 3rd arguments are incompatible with the atomic pointer type" } in Body
        "#]],
    );
}
