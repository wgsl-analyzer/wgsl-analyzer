use expect_test::expect;

use crate::test_util::{assert_out_of_scope, check, check_comments};

// https://www.w3.org/TR/WGSL/#recursive-descent-syntax-unary_expression

#[test]
pub fn format_naked_prefix_exprs_out_of_scope() {
    assert_out_of_scope(
        "fn main() {
        -
        1
        ;
        }",
        "Wgsl disallows expressions outside statements.",
    );
}

#[test]
pub fn format_prefix_expr_simple_negative() {
    check(
        "fn main() {
        let a = -

        1;
        }",
        expect![["
            fn main() {
                let a = -1;
            }
        "]],
    );
}

#[test]
pub fn format_prefix_expr_simple_ref() {
    check(
        "fn main() {
        let a = &

        1;
        }",
        expect![["
            fn main() {
                let a = &1;
            }
        "]],
    );
}

#[test]
pub fn format_prefix_expr_simple_ptr() {
    check(
        "fn main() {
        let a = *

        1;
        }",
        expect![["
            fn main() {
                let a = *1;
            }
        "]],
    );
}

#[test]
pub fn format_prefix_expr_simple_invert() {
    check(
        "fn main() {
        let a = ~

        1;
        }",
        expect![["
            fn main() {
                let a = ~1;
            }
        "]],
    );
}

#[test]
pub fn format_prefix_expr_with_block_comment() {
    check(
        "fn main() {
        let a = ~

        /* A */

        1;
        let b = ~1;
        }",
        expect![["
            fn main() {
                let a = ~ /* A */ 1;
                let b = ~1;
            }
        "]],
    );
}

#[test]
pub fn format_comments_in_prefix_expr_with() {
    check_comments(
        "fn main() {
        let a = ## ~ ## 1 ## ; ##
        }",
        expect![[r#"
            fn main() {
                let a = /* 0 */ ~ /* 1 */ 1 /* 2 */; /* 3 */
            }
        "#]],
        expect![[r#"
            fn main() {
                let a = // 0
                    ~ // 1
                    1 // 2
                    ; // 3
            }
        "#]],
    );
}

#[test]
pub fn format_comments_in_prefix_expr_in_complex_expr() {
    check_comments(
        "fn main() {
        let a = 1 + 2 - ~1;

        let a = ## 1 ## + ## 2 ## - ## ~ ## 1 ## ; ##
        }",
        expect![[r#"
            fn main() {
                let a = 1 + 2 - ~1;

                let a = /* 0 */ 1 /* 1 */ + /* 2 */ 2 /* 3 */
                    - /* 4 */ ~ /* 5 */ 1 /* 6 */; /* 7 */
            }
        "#]],
        expect![[r#"
            fn main() {
                let a = 1 + 2 - ~1;

                let a = // 0
                    1 // 1
                    + // 2
                    2 // 3
                    - // 4
                    ~ // 5
                    1 // 6
                    ; // 7
            }
        "#]],
    );
}

#[test]
pub fn format_prefix_expr_simple_positive() {
    assert_out_of_scope(
        "fn main() {
        let a = +1;
        }",
        "'+' is not a unary prefix operator",
    );
}
