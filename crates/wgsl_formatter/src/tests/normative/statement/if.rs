use expect_test::expect;

use crate::test_util::check;

#[test]
fn format_if_statement_remove_parens() {
    check(
        "fn main() {
    if(x < 1){}
    if    x < 1     {}
}",
        expect![[r#"
            fn main() {
                if x < 1 {}
                if x < 1 {}
            }
        "#]],
    );
}

#[test]
pub fn format_if_else_statement_empty_gets_collapsed() {
    check(
        "fn main() {
        if
        false
        {
        }
        else
        {
        }
        }",
        expect![["
            fn main() {
                if false {} else {}
            }
        "]],
    );
}

#[test]
pub fn format_if_else_statement_empty_if_nonempy_else() {
    check(
        "fn main() {
        if
        false
        {
        }
        else
        {
        // HI
        }
        }",
        expect![[r#"
            fn main() {
                if false {} else {
                    // HI
                }
            }
        "#]],
    );
}
