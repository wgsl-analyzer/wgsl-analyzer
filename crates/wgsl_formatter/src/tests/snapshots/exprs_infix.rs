use expect_test::expect;

use crate::test_util::{assert_out_of_scope, check};

#[test]
pub fn format_naked_infix_exprs_out_of_scope() {
    assert_out_of_scope(
        "fn main() {
        1
        +
        1
        ;
        }",
        "Wgsl disallows expressions outside statements.",
    );
}

#[test]
pub fn format_infix_expr_simple() {
    check(
        "fn main() {
        let a = 1+1;
        }",
        expect![["
            fn main() {
                let a = 1 + 1;
            }
        "]],
    );
}

#[test]
pub fn format_infix_expr_long() {
    check(
        "fn main() {
        let a = 1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1;
        }",
        expect![["
            fn main() {
                let a = 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
                     + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
                     + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
                     + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
                     + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1;
            }
        "]],
    );
}

#[test]
pub fn format_infix_expr_with_comments() {
    check(
        "fn main() {
        let a = /* A */ 1 /* B */ + /* C */ 1 /* D */;
        }",
        expect![["
            fn main() {
                let a = /* A */ 1 /* B */ + /* C */ 1 /* D */;
            }
        "]],
    );
}

#[test]
fn format_infix_expr_multiple() {
    check(
        "fn main() {
    let a=x+y*z;
}",
        expect![["
            fn main() {
                let a = x + y * z;
            }
        "]],
    );
}

#[test]
fn format_infix_expr_shr() {
    check(
        "fn main() { let x = 1u >> 3u; }",
        expect![["
            fn main() {
                let x = 1u >> 3u;
            }
        "]],
    );
}

#[test]
fn format_infix_expr_shl() {
    check(
        "fn main() { let x = 1u << 3u; }",
        expect![["
            fn main() {
                let x = 1u << 3u;
            }
        "]],
    );
}
