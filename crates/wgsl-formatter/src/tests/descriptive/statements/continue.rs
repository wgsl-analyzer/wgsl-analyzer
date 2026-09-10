use expect_test::expect;

use crate::test_util::{check, check_comments};

#[test]
pub(crate) fn format_continue_statement_1() {
    check(
        "fn main() {
        while(true) {

        continue;
        }


        }",
        expect![[r#"
            fn main() {
                while true {
                    continue;
                }
            }
        "#]],
    );
}

#[test]
pub(crate) fn format_comment_in_continue_statement() {
    check_comments(
        "fn main() {
        while(true) {

## continue ## ; ##

}

        }",
        expect![[r#"
            fn main() {
                while true {
                    /* 0 */ continue /* 1 */ ;
                    /* 2 */
                }
            }
        "#]],
        expect![[r#"
            fn main() {
                while true {
                    // 0
                    continue // 1
                    ;
                    // 2
                }
            }
        "#]],
    );
}
