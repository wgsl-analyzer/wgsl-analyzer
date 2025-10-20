use expect_test::expect;

use crate::test_util::check;

#[test]
pub fn format_loop_statement_empty() {
    check(
        "fn main() {
        loop {
        }


        }",
        expect![["
            fn main() {
                loop {}
            }
        "]],
    );
}

#[test]
pub fn format_loop_statement_block_comments() {
    check(
        "fn main() {
        /* A */
        loop
        /* B */
        {
        /* C */
        }
        /* D */


        }",
        expect![["
            fn main() {
                /* A */
                loop /* B */ {
                    /* C */
                }
                /* D */
            }
        "]],
    );
}

#[test]
pub fn format_loop_statement_line_comments() {
    check(
        "fn main() {
        // A
        loop
        // B
        {
        // C
        }
        // D


        }",
        expect![["
            fn main() {
                // A
                loop // B
                {
                    // C
                }
                // D
            }
        "]],
    );
}
