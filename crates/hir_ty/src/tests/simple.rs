use expect_test::expect;

use crate::tests::check_infer;

#[test]
fn type_alias_in_struct() {
    check_infer(
        r#"
        alias Foo = u32;
        struct S { x: Foo }

        fn foo() {
            let a = S(5);
            let b = a.x + 10u;
        }
        "#,
        expect![[r#"
            94..98 'S(5)': S
            96..97 '5': integer
            120..121 'a': S
            120..123 'a.x': ref<u32>
            120..129 'a.x + 10u': u32
            126..129 '10u': u32
        "#]],
    );
}
