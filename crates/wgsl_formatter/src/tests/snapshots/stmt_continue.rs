use expect_test::expect;

use crate::test_util::check;

#[test]
pub fn format_continue_statement_1() {
    check(
        "fn main() {
        while(true) {

        continue;
        }


        }",
        expect![["
            fn main() {
                while(true) {
                    continue;
                }
            }
        "]],
    );
}

#[test]
pub fn format_continue_statement_with_weird_comment() {
    check(
        "fn main() {
        while(true) {


/* A */ continue /* B */; /* C */

}

        }",
        expect![["
            fn main() {
                while(true) {
                    /* A */ continue;
                    /* B */ /* C */
                }
            }
        "]],
    );
}
