use expect_test::expect;

use crate::test_util::{check, check_comments};

#[test]
fn format_comments_in_compound_statement() {
    check_comments(
        "
        fn main() {
        ## { ## } ##
        }
        ",
        expect![[r#"
            fn main() {
                /* 0 */
                {
                    /* 1 */
                }
                /* 2 */
            }
        "#]],
        expect![[r#"
            fn main() {
                // 0
                {
                    // 1
                }
                // 2
            }
        "#]],
    );
}
