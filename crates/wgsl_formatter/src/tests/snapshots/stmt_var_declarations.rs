use expect_test::expect;

use crate::test_util::check;

#[test]
pub fn format_var_decl_simple_literal_1() {
    check(
        "fn main() {
        var
        a
        =
        1
        ;
        }",
        expect![["
            fn main() {
                var a = 1;
            }
        "]],
    );
}

#[test]
pub fn format_var_decl_simple_statement_1() {
    check(
        "fn main() {
        var
        a
        =
        1
        +
        1
        +
        1
        ;
        }",
        expect![["
            fn main() {
                var a = 1 + 1 + 1;
            }
        "]],
    );
}

#[test]
pub fn format_var_decl_simple_statement_with_trailing_comment() {
    check(
        "fn main() {
        var a_multiline_binding = 1 // The thing
                + 1 // The other thing
                + 7 // The other thing
                // Seperate
                ;

        }",
        expect![["
            fn main() {
                var a_multiline_binding = 1 // The thing
                    + 1 // The other thing
                    + 7 // The other thing
                    // Seperate
                    ;
            }
        "]],
    );
}

#[test]
pub fn format_var_decl_line_comments() {
    check(
        "fn main() {
        // Before
        var //A
        a //B
        = //C
        1 //D
        ; //E
        // After
        }",
        expect![["
            fn main() {
                // Before
                var //A
                    a //B
                    = //C
                    1 //D
                    ; //E
                // After
            }
        "]],
    );
}

#[test]
pub fn format_var_decl_keeps_line_comment_in_same_place_same_line() {
    check(
        "fn main() {

        var a = 1; // Assign 1


        }",
        expect![["
            fn main() {
                var a = 1; // Assign 1
            }
        "]],
    );
}

#[test]
pub fn format_var_decl_has_no_space_before_semicolon() {
    check(
        "fn main() {

        var a = 1 /* A */ ;


        }",
        expect![["
            fn main() {
                var a = 1 /* A */;
            }
        "]],
    );
}

#[test]
pub fn format_var_decl_block_comments() {
    check(
        "fn main() {
        /* Before */
        var /* A */
        a /* B */
        = /* C */
        1 /* D */
        ; /* E */
        /* After */
        }",
        expect![["
            fn main() {
                /* Before */
                var /* A */ a /* B */ = /* C */ 1 /* D */; /* E */
                /* After */
            }
        "]],
    );
}
