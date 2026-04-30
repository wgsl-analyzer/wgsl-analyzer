use expect_test::expect;

use crate::test_util::check;

#[test]
fn comment_after_statement_should_stay_on_same_line() {
    check(
        "
        fn main() {
        let a = 1; // This is one
        }
        ",
        expect![[r#"
            fn main() {
                let a = 1; // This is one
            }
        "#]],
    );
}

#[test]
pub fn format_fn_body_spacing_preserves_one_empty_line_line_2() {
    check(
        "fn main() {
            let a = 1;let b = 2;

            let c = 2;let d = 2;
        }",
        expect![["
            fn main() {
                let a = 1;
                let b = 2;

                let c = 2;
                let d = 2;
            }
        "]],
    );
}

#[test]
pub fn format_fn_body_spacing_preserves_at_most_one_empty_line_line_1() {
    check(
        "fn main() {
            let a = 1;




            let b = 2;
        }",
        expect![["
            fn main() {
                let a = 1;

                let b = 2;
            }
        "]],
    );
}

#[test]
pub fn format_fn_body_spacing_preserves_one_empty_line_line_1() {
    check(
        "fn main() {
            let a = 1;

            let b = 2;
        }",
        expect![["
            fn main() {
                let a = 1;

                let b = 2;
            }
        "]],
    );
}

#[test]
pub fn format_fn_body_spacing_statements_sep_by_newline_2() {
    check(
        "fn main() {

        let a = 1;let b = 2;
        let c = 1;let d = 2;


        }",
        expect![["
            fn main() {
                let a = 1;
                let b = 2;
                let c = 1;
                let d = 2;
            }
        "]],
    );
}

#[test]
pub fn format_fn_body_spacing_no_leading_empty_line_1() {
    check(
        "fn main() {

            let a = 1;
            let b = 2;
        }",
        expect![["
            fn main() {
                let a = 1;
                let b = 2;
            }
        "]],
    );
}

#[test]
pub fn format_fn_body_spacing_no_trailing_empty_line_1() {
    check(
        "fn main() {
            let a = 1;
            let b = 2;

        }",
        expect![["
            fn main() {
                let a = 1;
                let b = 2;
            }
        "]],
    );
}

#[test]
pub fn format_let_decl_keeps_line_comment_in_same_place_before() {
    check(
        "fn main() {

        // Assign 1
        let a = 1;


        }",
        expect![["
            fn main() {
                // Assign 1
                let a = 1;
            }
        "]],
    );
}

#[test]
pub fn format_let_decl_keeps_line_comment_in_same_place_after() {
    check(
        "fn main() {

        let a = 1;
        // Assign 1


        }",
        expect![["
            fn main() {
                let a = 1;
                // Assign 1
            }
        "]],
    );
}

#[test]
pub fn format_let_decl_keeps_line_comment_in_same_place_empty_line_before() {
    check(
        "fn main() {

        // Assign 1

        let a = 1;


        }",
        expect![["
            fn main() {
                // Assign 1

                let a = 1;
            }
        "]],
    );
}

#[test]
pub fn format_let_decl_keeps_line_comment_in_same_place_empty_line_after() {
    check(
        "fn main() {


        let a = 1;

        // Assign 1

        }",
        expect![["
            fn main() {
                let a = 1;

                // Assign 1
            }
        "]],
    );
}

#[test]
pub fn format_let_decl_keeps_block_comment_in_same_place_same_line() {
    check(
        "fn main() {

        let a = 1; /* Assign 1 */


        }",
        expect![["
            fn main() {
                let a = 1; /* Assign 1 */
            }
        "]],
    );
}

#[test]
pub fn format_let_decl_keeps_block_comment_in_same_place_before() {
    check(
        "fn main() {

        /* Assign 1 */
        let a = 1;


        }",
        expect![["
            fn main() {
                /* Assign 1 */
                let a = 1;
            }
        "]],
    );
}

#[test]
pub fn format_let_decl_keeps_block_comment_in_same_place_after() {
    check(
        "fn main() {

        let a = 1;
        /* Assign 1 */


        }",
        expect![["
            fn main() {
                let a = 1;
                /* Assign 1 */
            }
        "]],
    );
}

#[test]
pub fn format_let_decl_keeps_block_comment_in_same_place_empty_line_before() {
    check(
        "fn main() {

        /* Assign 1 */

        let a = 1;


        }",
        expect![["
            fn main() {
                /* Assign 1 */

                let a = 1;
            }
        "]],
    );
}

#[test]
pub fn format_let_decl_keeps_block_comment_in_same_place_empty_line_after() {
    check(
        "fn main() {


        let a = 1;

        /* Assign 1 */

        }",
        expect![["
            fn main() {
                let a = 1;

                /* Assign 1 */
            }
        "]],
    );
}

#[test]
pub fn format_nested_blocks_have_correct_line_breaks() {
    check(
        "fn main() {
            loop {
            loop {loop {
            a += 1;b += 1;

            }}}
        }",
        expect![[r#"
            fn main() {
                loop {
                    loop {
                        loop {
                            a += 1;
                            b += 1;
                        }
                    }
                }
            }
        "#]],
    );
}

#[test]
fn format_compound_statement_put_items_on_separate_lines() {
    check(
        "
        fn main() {
        let a = 1; let b = 2;
        }
        ",
        expect![[r#"
            fn main() {
                let a = 1;
                let b = 2;
            }
        "#]],
    );
}

#[test]
fn format_compound_statement_keep_comment_on_same_line() {
    check(
        "
        fn main() {
        let a = 1; let b = 2; // Comment
        }
        ",
        expect![[r#"
            fn main() {
                let a = 1;
                let b = 2; // Comment
            }
        "#]],
    );
}
