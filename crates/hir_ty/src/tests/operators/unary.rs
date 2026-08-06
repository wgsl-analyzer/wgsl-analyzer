use expect_test::expect;
use syntax::ExtensionsConfig;

use crate::tests::check_infer;

#[test]
fn negate() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    var n = 1;
    let t = -n;
}
        ",
        expect![[r#"
            19..20 'n': ref<function, i32, read_write>
            23..24 '1': integer
            34..35 't': i32
            38..40 '-n': i32
            39..40 'n': ref<function, i32, read_write>
        "#]],
    );
}
