use expect_test::expect;

use crate::test_util::{check, check_comments};

#[test]
pub fn format_return_statement_without_expr() {
    check(
        "fn main() {
return;


        }",
        expect![["
            fn main() {
                return;
            }
        "]],
    );
}

#[test]
pub fn format_return_statement_with_simple_expr() {
    check(
        "fn main() {
return 1;


        }",
        expect![["
            fn main() {
                return 1;
            }
        "]],
    );
}

#[test]
pub fn format_return_statement_with_needless_parens() {
    check(
        "fn main() {
return (1);


        }",
        expect![["
            fn main() {
                return 1;
            }
        "]],
    );
}

#[test]
pub fn format_return_statement_with_complex_expr() {
    check(
        "fn main() {
return 1 + 2 + (47 * get_the_number() - a ) >> 18 - 2;


        }",
        expect![["
            fn main() {
                return 1 + 2 + (47 * get_the_number() - a) >> 18 - 2;
            }
        "]],
    );
}

#[test]
pub fn format_comment_in_return_statement_with_simple_expr() {
    check_comments(
        "fn main() {
        ## return ## 1 ## ; ##
        }",
        expect![[r#"
            fn main() {
                /* 0 */
                return /* 1 */ 1 /* 2 */; /* 3 */
            }
        "#]],
        expect![[r#"
            fn main() {
                // 0
                return // 1
                1 // 2
                ; // 3
            }
        "#]],
    );
}

#[test]
pub fn format_comment_in_return_statement_without_expr() {
    check_comments(
        "fn main() {
        ## return ## ; ##
        }",
        expect![[r#"
            fn main() {
                /* 0 */
                return /* 1 */; /* 2 */
            }
        "#]],
        expect![[r#"
            fn main() {
                // 0
                return // 1
                ; // 2
            }
        "#]],
    );
}
