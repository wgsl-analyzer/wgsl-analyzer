use expect_test::expect;

use crate::test_util::{check, check_comments};

#[test]
pub fn format_discard_statement_1() {
    check(
        "fn main() {
discard;


        }",
        expect![["
            fn main() {
                discard;
            }
        "]],
    );
}

#[test]
pub fn format_discard_statement_with_weird_comment() {
    check_comments(
        "fn main() {
        ## discard ## ; ##


        }",
        expect![[r#"
            fn main() {
                /* 0 */
                discard;
                /* 1 */ /* 2 */
            }
        "#]],
        expect![[r#"
            fn main() {
                // 0
                discard;
                // 1
                // 2
            }
        "#]],
    );
}
