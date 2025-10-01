use expect_test::expect;

use crate::{
    FormattingOptions,
    test_util::{check, check_with_options},
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
pub fn format_expr_function_call_with_block_comments() {
    check_with_options(
        "fn main() {
        let a = /* A */
        foo /* B */
        ( /* C */
        3 /* D */
        , /* E */
        2 /* F */
        ) /* G */
        ; /* H */
        }",
        &expect![["
            fn main() {
                let a = /* A */ foo /* B */ (/* C */ 3, /* D */ /* E */ 2 /* F */) /* G */; /* H */
            }
        "]],
        &FormattingOptions {
            width: 10000,
            ..Default::default()
        },
    );
}

#[test]
pub fn format_expr_function_call_with_line_comments() {
    check_with_options(
        "fn main() {
        let a = // A
        foo // B
        ( // C
        3 // D
        , // E
        2 // F
        ) // G
        ; // H
        }",
        &expect![["
            fn main() {
                let a = // A
                    foo // B
                    (
                        // C
                        3, // D
                        // E
                        2, // F
                    ) // G
                    ; // H
            }
        "]],
        &FormattingOptions {
            width: 10000,
            ..Default::default()
        },
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
