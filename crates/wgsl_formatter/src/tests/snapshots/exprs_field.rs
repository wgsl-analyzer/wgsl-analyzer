use expect_test::expect;

use crate::test_util::{assert_out_of_scope, check, check_comments};

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
pub fn format_comments_in_field_expr() {
    check_comments(
        "fn main() {
        let a = ## foo ## . ## bar ## ; ##
        }",
        expect![[r#"
            fn main() {
                let a = /* 0 */ foo /* 1 */ . /* 2 */ bar /* 3 */; /* 4 */
            }
        "#]],
        expect![[r#"
            fn main() {
                let a = // 0
                    foo // 1
                    . // 2
                    bar // 3
                    ; // 4
            }
        "#]],
    );
}
