use expect_test::expect;

use crate::test_util::check;

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
pub fn format_fn_body_empty() {
    // Following the WGSL spec, we keep @must_use inlined with the function
    check(
        "
        fn thing() {}
        ",
        expect![[r#"
            fn thing() {}
        "#]],
    );
}
