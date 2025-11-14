use expect_test::expect;

use crate::{
    FormattingOptions,
    test_util::{check, check_comments, check_with_options},
};

#[test]
pub fn format_expr_function_call_simple() {
    check(
        "fn main() {
        let a =
        foo
        (

        )
        ;
        }",
        expect![["
            fn main() {
                let a = foo();
            }
        "]],
    );
}

#[test]
pub fn format_expr_function_call_one_arg() {
    check(
        "fn main() {
        let a =
        foo
        (
        3

        )
        ;
        }",
        expect![["
            fn main() {
                let a = foo(3);
            }
        "]],
    );
}

#[test]
pub fn format_expr_function_call_two_arg() {
    check(
        "fn main() {
        let a =
        foo
        (
        3
        ,
        2
        )
        ;
        }",
        expect![["
            fn main() {
                let a = foo(3, 2);
            }
        "]],
    );
}

#[test]
pub fn format_expr_function_call_many_args() {
    check(
        "fn main() {
        let a = foo(1,2,3,4,5,6,7,8,9,10,1,2,3,4,5,6,7,8,9,10,1,2,3,4,5,6,7,8,9)
        ;
        }",
        expect![["
            fn main() {
                let a = foo(
                        1,
                        2,
                        3,
                        4,
                        5,
                        6,
                        7,
                        8,
                        9,
                        10,
                        1,
                        2,
                        3,
                        4,
                        5,
                        6,
                        7,
                        8,
                        9,
                        10,
                        1,
                        2,
                        3,
                        4,
                        5,
                        6,
                        7,
                        8,
                        9,
                    );
            }
        "]],
    );
}

#[test]
fn format_expr_function_call_bitcast() {
    check(
        "fn main() { let a = bitcast   <  vec4<u32>  >  ( x+5 ); }",
        expect!["fn main() { bitcast<vec4<u32>>(x + 5) }"],
    );
}

#[test]
pub fn format_comments_in_expr_function_call() {
    check_comments(
        "fn main() {
        let a = ## foo ## ( ## 3 ## , ## 2 ## ) ## ; ##
        }",
        expect![[r#"
            fn main() {
                let a = /* 0 */ foo /* 1 */ (
                        /* 2 */ 3, /* 3 */ /* 4 */
                        2, /* 5 */
                    ) /* 6 */; /* 7 */
            }
        "#]],
        expect![[r#"
            fn main() {
                let a = // 0
                    foo // 1
                    (
                        // 2
                        3, // 3
                        // 4
                        2, // 5
                    ) // 6
                    ; // 7
            }
        "#]],
    );
}

#[test]
pub fn format_expr_function_call_with_sensible_comments() {
    check_with_options(
        "fn main() {
        let a = foo(
        foo(), // Comment about foo
        bar(), // Comment about bar
        qur()
        );
        }",
        &expect![["
            fn main() {
                let a = foo(
                        foo(), // Comment about foo
                        bar(), // Comment about bar
                        qur(),
                    );
            }
        "]],
        &FormattingOptions {
            width: 10000,
            ..Default::default()
        },
    );
}
