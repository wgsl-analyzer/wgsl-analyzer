use expect_test::expect;

use crate::tests::check_infer;

#[test]
fn atomicCompareExchangeWeak() {
    check_infer(
        "
var<storage, read_write> buffer: atomic<u32>;
fn foo() {
    let result = atomicCompareExchangeWeak(&buffer, 1, 2);
    let old_value = result.old_value;
    let exchanged = result.exchanged;
}
",
        expect![[r#"
            25..31 'buffer': ref<storage, atomic<u32>, read_write>
            65..71 'result': __atomic_compare_exchange_result
            74..114 'atomic... 1, 2)': __atomic_compare_exchange_result
            100..107 '&buffer': ptr<storage, atomic<u32>, read_write>
            101..107 'buffer': ref<storage, atomic<u32>, read_write>
            109..110 '1': integer
            112..113 '2': integer
            124..133 'old_value': u32
            136..142 'result': __atomic_compare_exchange_result
            136..152 'result..._value': u32
            162..171 'exchanged': bool
            174..180 'result': __atomic_compare_exchange_result
            174..190 'result...hanged': bool
        "#]],
    );
}
