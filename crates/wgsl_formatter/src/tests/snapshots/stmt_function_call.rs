use expect_test::expect;

use crate::test_util::{check, check_comments};

#[test]
pub fn format_function_call_statement() {
    check(
        "fn main() {
        foo();
        }",
        expect![[r#"
            fn main() {
                foo();
            }
        "#]],
    );
}

#[test]
pub fn format_2_function_call_statements() {
    check(
        "fn main() {
        foo();
        bar();
        }",
        expect![[r#"
            fn main() {
                foo();
                bar();
            }
        "#]],
    );
}

#[test]
pub fn format_insanely_long_function_call_statement() {
    check(
        "fn main() {
        foo_efek_felkj_soiu_flejk_lkjef_aoieu_flkejfalk_lkjeifou_flj_lkjsieuf_flkj_Ljklllefjief();
        }",
        expect![[r#"
            fn main() {
                foo_efek_felkj_soiu_flejk_lkjef_aoieu_flkejfalk_lkjeifou_flj_lkjsieuf_flkj_Ljklllefjief(
                );
            }
        "#]],
    );
}

#[test]
pub fn format_function_call_statement_with_arguments() {
    check(
        "fn main() {
        bla(12, bar(), 1 + vubble);
        }",
        expect![[r#"
            fn main() {
                bla(12, bar(), 1 + vubble);
            }
        "#]],
    );
}

#[test]
pub fn format_function_call_statement_trailing_comma_with_multiline_arguments() {
    check(
        "fn main() {
        bla(12, // Force break
        bar(), 1 + vubble);
        }",
        expect![[r#"
            fn main() {
                bla(
                    12, // Force break
                    bar(),
                    1 + vubble,
                );
            }
        "#]],
    );
}

#[test]
pub fn format_comment_in_function_call_statement() {
    check_comments(
        "fn main() {
        ## bla ## ( ## 12 ## , ## bar ## ( ## ) ## , ## 1 ## + ## vubble ## ) ## ; ##
        }",
        expect![[r#"
            fn main() {
                /* 0 */
                bla /* 1 */ (
                    /* 2 */ 12, /* 3 */ /* 4 */
                    bar /* 5 */ (/* 6 */), /* 7 */ /* 8 */
                    1 /* 9 */ + /* 10 */ vubble, /* 11 */
                ) /* 12 */ ; /* 13 */
            }
        "#]],
        expect![[r#"
            fn main() {
                // 0
                bla // 1
                (
                    // 2
                    12, // 3
                    // 4
                    bar // 5
                    (
                        // 6
                    ), // 7
                    // 8
                    1 // 9
                    + // 10
                    vubble, // 11
                ) // 12
                ; // 13
            }
        "#]],
    );
}

#[test]
fn format_function_call() {
    check(
        "fn main() {
    min  (  x,y );
}",
        expect![[r#"
            fn main() {
                min(x, y);
            }
        "#]],
    );
}

// TODO This might be debatable - if an existing newline should be usable to quickly split a function call into multilines
// #[test]
// fn format_function_call_newline() {
//     check(
//         "fn main() {
//     min  (
//         x,y );
// }",
//         expect![[r#"
//             fn main() {
//                 min(
//                     x,
//                     y
//                 );
//             }
//         "#]],
//     );
// }
// #[test]
// fn format_function_call_newline_indent() {
//     check(
//         "fn main() {
//     if (false) {
//         min  (
//             x,y );
//     }
// }",
//         expect![["
//             fn main() {
//                 if false {
//                     min(
//                         x, y
//                     );
//                 }
//             }"]],
//     );
// }

#[test]
fn format_function_call_newline_nested() {
    check(
        "fn main() {
    min(
        min(
            1, // Force break
            2,
        )
    );
}",
        expect![[r#"
            fn main() {
                min(
                    min(
                        1, // Force break
                        2,
                    ),
                );
            }
        "#]],
    );
}
