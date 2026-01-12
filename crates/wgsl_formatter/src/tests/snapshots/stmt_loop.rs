use expect_test::expect;

use crate::test_util::{check, check_comments};

#[test]
pub fn format_loop_statement_empty() {
    check(
        "fn main() {
        loop {
        }


        }",
        expect![["
            fn main() {
                loop {}
            }
        "]],
    );
}

#[test]
pub fn format_comment_in_loop_statement_empty() {
    check_comments(
        "fn main() {
        ## loop ## {
        ## } ##
        }",
        expect![[r#"
            fn main() {
                /* 0 */
                loop /* 1 */ {
                    /* 2 */
                }
                /* 3 */
            }
        "#]],
        expect![[r#"
            fn main() {
                // 0
                loop // 1
                {
                    // 2
                }
                // 3
            }
        "#]],
    );
}
