use expect_test::expect;

use crate::test_util::check_comments;

#[test]
fn format_comments_around_global_compdec() {
    check_comments(
        "fn a() {} ## { ## fn b() {} ## } ## fn c() {}",
        expect![[r#"
            fn a() {} /* 0 */
            { /* 1 */
            fn b() {} /* 2 */
            } /* 3 */
            fn c() {}
        "#]],
        expect![[r#"
            fn a() {} // 0
            { // 1
            fn b() {} // 2
            } // 3
            fn c() {}
        "#]],
    );
}
