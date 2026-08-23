use expect_test::expect;

use crate::{
    FormattingOptions,
    test_util::{CheckOptions, assert_out_of_scope, check, check_comments, check_with_options},
};

#[test]
pub fn format_naked_infix_exprs_out_of_scope() {
    assert_out_of_scope(
        "fn main() {
        1
        +
        1
        ;
        }",
        "Wgsl disallows expressions outside statements.",
    );
}

#[test]
pub fn format_infix_expr_simple() {
    check(
        "fn main() {
        let a = 1+1;
        }",
        expect![["
            fn main() {
                let a = 1 + 1;
            }
        "]],
    );
}

#[test]
pub fn format_infix_expr_long() {
    check(
        "fn main() {
        let a = 1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1;
        }",
        expect![[r#"
            fn main() {
                let a =
                    1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
                    + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
                    + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1;
            }
        "#]],
    );
}

#[test]
pub fn format_comments_in_infix_expr() {
    check_comments(
        "fn main() {
        let a = ## 1 ## + ## 1 ## ; ##
        }",
        expect![[r#"
            fn main() {
                let a = /* 0 */ 1 /* 1 */ + /* 2 */ 1 /* 3 */; /* 4 */
            }
        "#]],
        expect![[r#"
            fn main() {
                let a = // 0
                    1 // 1
                    + // 2
                    1 // 3
                    ; // 4
            }
        "#]],
    );
}

#[test]
fn format_infix_expr_multiple() {
    check(
        "fn main() {
    let a=x+y*z;
}",
        expect![["
            fn main() {
                let a = x + y * z;
            }
        "]],
    );
}

#[test]
fn format_infix_expr_shr() {
    check(
        "fn main() { let x = 1u >> 3u; }",
        expect![["
            fn main() {
                let x = 1u >> 3u;
            }
        "]],
    );
}

#[test]
fn format_infix_expr_shl() {
    check(
        "fn main() { let x = 1u << 3u; }",
        expect![["
            fn main() {
                let x = 1u << 3u;
            }
        "]],
    );
}

#[test]
fn format_infix_break_with_precedence_boolean_vs_arithmetic() {
    check_with_options(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        fn main() {
            let break_me = aaaaaa + bbbbbb && xxxxxxxxxx;
            let break_me = aaaaaaaaaa + bbbbbbbbbb && xxxxxxxxxx;
            let break_me = xxxxxxxxxx && aaaaaaaaaa + bbbbbbbbbb;

            let break_me = aaaaaa + bbbbbb || xxxxxxxxxx;
            let break_me = aaaaaaaaaa + bbbbbbbbbb || xxxxxxxxxx;
            let break_me = xxxxxxxxxx || aaaaaaaaaa + bbbbbbbbbb;

            let break_me = aaaaaa + bbbbbb | xxxxxxxxxx;
            let break_me = aaaaaaaaaa + bbbbbbbbbb | xxxxxxxxxx;
            let break_me = xxxxxxxxxx | aaaaaaaaaa + bbbbbbbbbb;

            let break_me = aaaaaa + bbbbbb & xxxxxxxxxx;
            let break_me = aaaaaaaaaa + bbbbbbbbbb & xxxxxxxxxx;
            let break_me = xxxxxxxxxx & aaaaaaaaaa + bbbbbbbbbb;
        }",
        expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            fn main() {
                let break_me = aaaaaa + bbbbbb
                    && xxxxxxxxxx;
                let break_me =
                    aaaaaaaaaa + bbbbbbbbbb
                    && xxxxxxxxxx;
                let break_me = xxxxxxxxxx
                    && aaaaaaaaaa + bbbbbbbbbb;

                let break_me = aaaaaa + bbbbbb
                    || xxxxxxxxxx;
                let break_me =
                    aaaaaaaaaa + bbbbbbbbbb
                    || xxxxxxxxxx;
                let break_me = xxxxxxxxxx
                    || aaaaaaaaaa + bbbbbbbbbb;

                let break_me = aaaaaa + bbbbbb
                    | xxxxxxxxxx;
                let break_me =
                    aaaaaaaaaa + bbbbbbbbbb
                    | xxxxxxxxxx;
                let break_me = xxxxxxxxxx
                    | aaaaaaaaaa + bbbbbbbbbb;

                let break_me = aaaaaa + bbbbbb
                    & xxxxxxxxxx;
                let break_me =
                    aaaaaaaaaa + bbbbbbbbbb
                    & xxxxxxxxxx;
                let break_me = xxxxxxxxxx
                    & aaaaaaaaaa + bbbbbbbbbb;
            }
        "#]],
        &CheckOptions {
            assert_line_width: None,
            formatting: FormattingOptions {
                max_line_width: 40,
                ..Default::default()
            },
        },
        parser::Edition::LATEST,
    );
}
