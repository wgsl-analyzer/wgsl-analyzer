use expect_test::expect;

use crate::test_util::check;

#[test]
pub fn format_while_statement_empty() {
    check(
        "fn main() {
        while(true) {
        }


        }",
        expect![["
            fn main() {
                while(true) {}
            }
        "]],
    );
}

#[test]
pub fn format_while_statement_single_statement() {
    check(
        "fn main() {
        while(true) {
        let a = 3;
        }


        }",
        expect![["
            fn main() {
                while(true) {
                    let a = 3;
                }
            }
        "]],
    );
}

#[test]
pub fn format_while_statement_continue_statement() {
    // This is just a very simple smoke test for completeness, more fine grained tests are in stmt_continue.rs
    check(
        "fn main() {
        while(true) {
        let a = 3;
        continue;
        let b = 3;
        }


        }",
        expect![["
            fn main() {
                while(true) {
                    let a = 3;
                    continue;
                    let b = 3;
                }
            }
        "]],
    );
}

#[test]
pub fn format_while_statement_with_block_comments() {
    check(
        "fn main() {
        /* A */
        while
        /* B */
        (
        /* C */
        true
        /* D */
        )
        /* E */
        {
        /* F */
        }
        /* G */



        }",
        expect![["
            fn main() {
                /* A */
                while /* B */ (/* C */ true /* D */) /* E */ {
                    /* F */
                }
                /* G */
            }
        "]],
    );
}
