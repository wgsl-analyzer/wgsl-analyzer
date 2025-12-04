use expect_test::expect;

use crate::test_util::{assert_out_of_scope, check, check_comments};

#[test]
pub fn format_naked_index_exprs_out_of_scope() {
    assert_out_of_scope(
        "fn main() {
        a
        [
        0
        ]
        ;
        }",
        "Wgsl disallows expressions outside statements.",
    );
}

#[test]
pub fn format_index_expr_simple() {
    check(
        "fn main() {
        let a =
        foo
        [
        0
        ]
        ;
        }",
        expect![["
            fn main() {
                let a = foo[0];
            }
        "]],
    );
}

#[test]
pub fn format_index_expr_nested() {
    check(
        "fn main() {
        let a =
        foo[17]
        [28]
        [39]
        ;
        }",
        expect![["
            fn main() {
                let a = foo[17][28][39];
            }
        "]],
    );
}

#[test]
pub fn format_index_expr_with_sensible_line_comments() {
    check(
        "fn main() {
        let a =
        foo
        [
        0 //Comment about the index
        ]
        ;
        }",
        expect![["
            fn main() {
                let a = foo[
                        0 //Comment about the index
                    ];
            }
        "]],
    );
}

#[test]
pub fn format_comments_in_index_expr() {
    check_comments(
        "fn main() {
        let a = ## foo ## [ ## 0 ## ] ## ; ##
        }",
        expect![[r#"
            fn main() {
                let a = /* 0 */ foo /* 1 */ [/* 2 */ 0 /* 3 */] /* 4 */; /* 5 */
            }
        "#]],
        expect![[r#"
            fn main() {
                let a = // 0
                    foo // 1
                    [
                        // 2
                        0 // 3
                    ] // 4
                    ; // 5
            }
        "#]],
    );
}

#[test]
pub fn format_comments_in_nested_index_expr() {
    check_comments(
        "fn main() {
        let a = ## foo ## [ ## 0 ## ] ## [ ## 1 ## ] ## ; ##
        }",
        expect![[r#"
            fn main() {
                let a = /* 0 */ foo /* 1 */ [/* 2 */ 0 /* 3 */] /* 4 */ [
                        /* 5 */ 1 /* 6 */
                    ] /* 7 */; /* 8 */
            }
        "#]],
        expect![[r#"
            fn main() {
                let a = // 0
                    foo // 1
                    [
                        // 2
                        0 // 3
                    ] // 4
                    [
                        // 5
                        1 // 6
                    ] // 7
                    ; // 8
            }
        "#]],
    );
}
