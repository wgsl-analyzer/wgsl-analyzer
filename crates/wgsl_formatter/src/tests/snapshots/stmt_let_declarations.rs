use expect_test::expect;

use crate::test_util::{check, check_comments};

#[test]
pub fn format_let_decl_simple_literal_1() {
    check(
        "fn main() {
        let
        a
        =
        1
        ;
        }",
        expect![["
            fn main() {
                let a = 1;
            }
        "]],
    );
}

#[test]
pub fn format_let_decl_simple_statement_1() {
    check(
        "fn main() {
        let
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
                let a = 1 + 1 + 1;
            }
        "]],
    );
}

#[test]
pub fn format_let_decl_simple_statement_with_trailing_comment() {
    check(
        "fn main() {
        let a_multiline_binding = 1 // The thing
                + 1 // The other thing
                + 7 // The other thing
                // Seperate
                ;

        }",
        expect![["
            fn main() {
                let a_multiline_binding = 1 // The thing
                    + 1 // The other thing
                    + 7 // The other thing
                    // Seperate
                    ;
            }
        "]],
    );
}

#[test]
pub fn format_comment_in_let_decl() {
    check_comments(
        "fn main() {
        ## let ## a ## = ## 1 ## ; ##
        }",
        expect![[r#"
            fn main() {
                /* 0 */
                let /* 1 */ a /* 2 */ = /* 3 */ 1 /* 4 */; /* 5 */
            }
        "#]],
        expect![[r#"
            fn main() {
                // 0
                let // 1
                    a // 2
                    = // 3
                    1 // 4
                    ; // 5
            }
        "#]],
    );
}

#[test]
pub fn format_let_decl_keeps_line_comment_in_same_place_same_line() {
    check(
        "fn main() {

        let a = 1; // Assign 1


        }",
        expect![["
            fn main() {
                let a = 1; // Assign 1
            }
        "]],
    );
}

#[test]
pub fn format_let_decl_has_no_space_before_semicolon() {
    check(
        "fn main() {

        let a = 1 /* A */ ;


        }",
        expect![["
            fn main() {
                let a = 1 /* A */;
            }
        "]],
    );
}
