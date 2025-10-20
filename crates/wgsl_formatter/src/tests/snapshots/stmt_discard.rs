use expect_test::expect;

use crate::test_util::check;

#[test]
pub fn format_discard_statement_1() {
    check(
        "fn main() {
discard;


        }",
        expect![["
            fn main() {
                discard;
            }
        "]],
    );
}

#[test]
pub fn format_discard_statement_with_weird_comment() {
    check(
        "fn main() {
/* A */ discard /* B */; /* C */


        }",
        expect![["
            fn main() {
                /* A */ discard;
                /* B */ /* C */
            }
        "]],
    );
}
