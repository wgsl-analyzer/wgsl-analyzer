use expect_test::expect;

use crate::name_resolution::tests::check_item_scope;

#[test]
fn item_scope_smoke_test() {
    check_item_scope(
        r#"
//- /package.wesl edition:2026_pre
const a = 3;
fn f() {}
const_assert(1 < 3);
fn g() {}
struct Bar { a: u32 }

"#,
        expect![[r#"
            - struct Bar
            - const a
            - fn f
            - fn g
        "#]],
    );
}
