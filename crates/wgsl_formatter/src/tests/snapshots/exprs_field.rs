use expect_test::expect;

use crate::test_util::{assert_out_of_scope, check};

#[test]
pub fn format_naked_field_exprs_out_of_scope() {
    assert_out_of_scope(
        "fn main() {
        a
        .
        foo
        ;
        }",
        "Wgsl disallows expressions outside statements.",
    );
}

#[test]
pub fn format_field_expr_simple() {
    check(
        "fn main() {
        let a =
        foo
        .
        bar
        ;
        }",
        expect![["
            fn main() {
                let a = foo.bar;
            }
        "]],
    );
}

#[test]
pub fn format_field_expr_with_line_comments() {
    check(
        "fn main() {
        let a = // A
        foo // B
        . // C
        bar // D
        ; // E
        }",
        expect![["
            fn main() {
                let a = // A
                    foo // B
                    . // C
                    bar // D
                    ; // E
            }
        "]],
    );
}

#[test]
pub fn format_field_expr_with_block_comments() {
    check(
        "fn main() {
        let a = /* A */
        foo /* B */
        . /* C */
        bar /* D */
        ; /* E */
        }",
        expect![["
            fn main() {
                let a = /* A */ foo /* B */ . /* C */ bar /* D */; /* E */
            }
        "]],
    );
}
