use expect_test::expect;
use syntax::ExtensionsConfig;

use crate::tests::check_infer;

#[test]
fn minus() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let t = 1-1;
}
        ",
        expect![[r#"
            19..20 't': i32
            23..24 '1': integer
            23..26 '1-1': integer
            25..26 '1': integer
        "#]],
    );
}
