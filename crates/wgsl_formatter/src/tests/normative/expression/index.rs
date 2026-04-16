use expect_test::expect;

use crate::test_util::{assert_out_of_scope, check};

#[test]
pub fn format_naked_index_exprs_out_of_scope() {
    assert_out_of_scope(
        "fn main() {
        a[0];
        }",
        "Wgsl disallows expressions outside statements.",
    );
}

#[test]
fn format_parens_in_index_expr_are_removed() {
    check(
        "
        fn main() {
            b[(1+1)]   = 0;
        }
        ",
        expect![[r#"
            fn main() {
                b[1 + 1] = 0;
            }
        "#]],
    );
}

#[test]
fn format_surrounding_spaces_in_index_expr_are_removed() {
    check(
        "
        fn main() {
            b[   1+1    ]   = 0;
        }
        ",
        expect![[r#"
            fn main() {
                b[1 + 1] = 0;
            }
        "#]],
    );
}

#[test]
fn format_index_expression_brackets_are_broken_up_if_multiline() {
    check(
        "
        fn main() {
            b[   // A
            1+1    ]   = 0;
        }
        ",
        expect![[r#"
            fn main() {
                b[
                    // A
                    1 + 1
                ] = 0;
            }
        "#]],
    );
}
