use expect_test::expect;

use crate::test_util::check;

#[test]
pub fn format_fn_body_collapses_empty_body() {
    check(
        "fn main() {



        }",
        expect![["
            fn main() {}
        "]],
    );
}

#[test]
pub fn format_fn_body_retains_line_comment_1() {
    check(
        "fn main() {
//Hello


        }",
        expect![["
            fn main() {
                //Hello
            }
        "]],
    );
}

#[test]
pub fn format_fn_body_retains_block_comment_1() {
    check(
        "fn main() {
/* Hello */


        }",
        expect![["
            fn main() {
                /* Hello */
            }
        "]],
    );
}

/// This is debatable. For now it seems like a sane way to do it this way, and it causes less edge cases.
#[test]
pub fn format_fn_body_puts_block_comment_on_seperate_line() {
    check(
        "fn main() {/* Hello */}",
        expect![["
            fn main() {
                /* Hello */
            }
        "]],
    );
}

#[test]
pub fn format_fn_body_spacing_statements_sep_by_newline_1() {
    check(
        "fn main() {let a = 1;let b = 2;}",
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
