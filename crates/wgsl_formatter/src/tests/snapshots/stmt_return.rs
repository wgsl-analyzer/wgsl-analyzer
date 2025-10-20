use expect_test::expect;

use crate::test_util::check;

#[test]
pub fn format_return_statement_without_expr() {
    check(
        "fn main() {
return;


        }",
        expect![["
            fn main() {
                return;
            }
        "]],
    );
}

#[test]
pub fn format_return_statement_with_simple_expr() {
    check(
        "fn main() {
return 1;


        }",
        expect![["
            fn main() {
                return 1;
            }
        "]],
    );
}

#[test]
pub fn format_return_statement_with_complex_expr() {
    check(
        "fn main() {
return 1 + 2 + (47 * get_the_number() - a ) >> 18 - 2;


        }",
        expect![["
            fn main() {
                return 1 + 2 + (47 * get_the_number() - a) >> 18 - 2;
            }
        "]],
    );
}

#[test]
pub fn format_return_statement_block_comment_with_simple_expr() {
    check(
        "fn main() {
        /* A */
return
/* B */
1
/* C */
;
/* D */


        }",
        expect![["
            fn main() {
                /* A */
                return /* B */ 1 /* C */;
                /* D */
            }
        "]],
    );
}

#[test]
pub fn format_return_statement_block_comment_without_expr() {
    check(
        "fn main() {
        /* A */
return
/* B */
;
/* C */


        }",
        expect![["
            fn main() {
                /* A */
                return /* B */;
                /* C */
            }
        "]],
    );
}

#[test]
pub fn format_return_statement_line_comment_with_simple_expr() {
    check(
        "fn main() {
        // A
return
// B
1
// C
;
// D


        }",
        expect![["
            fn main() {
                // A
                return // B
                1 // C
                ;
                // D
            }
        "]],
    );
}

#[test]
pub fn format_return_statement_line_comment_without_expr() {
    check(
        "fn main() {
        // A
return
// B
;
// C


        }",
        expect![["
            fn main() {
                // A
                return // B
                ;
                // C
            }
        "]],
    );
}
