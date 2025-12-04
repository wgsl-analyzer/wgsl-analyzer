use expect_test::expect;

use crate::test_util::{assert_out_of_scope, check, check_comments};

#[test]
pub fn format_assert_statement_simple() {
    check(
        "
        const_assert
        x
        <
        y
        ;
        ",
        expect![["
            const_assert x < y;
        "]],
    );
}

#[test]
pub fn format_assert_statement_remove_parens() {
    check(
        "
        const_assert(x<y);
        const_assert
        (x
        < y
        );
        ",
        expect![["
            const_assert x < y;
            const_assert x < y;
        "]],
    );
}

#[test]
pub fn format_assert_statement_within_function() {
    check(
        "
        fn main() {
            const_assert
            x
            <
            y
            ;
        }
        ",
        expect![[r#"
            fn main() {
                const_assert x < y;
            }
        "#]],
    );
}

#[test]
pub fn format_comments_in_const_assert_simple() {
    check_comments(
        "
        ## const_assert ## ( ## a ## < ## b ## ) ## ; ##
        ",
        expect![[r#"
            /* 0 */
            const_assert /* 1 */ /* 2 */ a /* 3 */ < /* 4 */ b /* 5 */ /* 6 */;
            /* 7 */
        "#]],
        expect![[r#"
            // 0
            const_assert // 1
                // 2
                a // 3
                < // 4
                b // 5
                // 6
                ;
            // 7
        "#]],
    );
}
