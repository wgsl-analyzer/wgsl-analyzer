use expect_test::expect;

use crate::test_util::check_comments;

#[test]
pub fn format_comments_in_break() {
    check_comments(
        "fn main() { loop { ##break##;## } }",
        expect![[r#"
            fn main() {
                loop {
                    /* 0 */ break /* 1 */;
                    /* 2 */
                }
            }
        "#]],
        expect![[r#"
            fn main() {
                loop {
                    // 0
                    break // 1
                    ;
                    // 2
                }
            }
        "#]],
    );
}
