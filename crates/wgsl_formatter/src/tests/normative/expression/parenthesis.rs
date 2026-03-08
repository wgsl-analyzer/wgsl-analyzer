use expect_test::expect;

use crate::test_util::check;

#[test]
fn format_nested_parentheses_get_collapsed() {
    check(
        "
        fn main() {
            let a = 1 + (((((((((1+1)))))))));
        }
        ",
        expect![[r#"
            fn main() {
                let a = 1 + (1 + 1);
            }
        "#]],
    );
}

#[test]
fn format_parenthesized_literal_does_not_get_collapsed() {
    // Follow rustfmt
    check(
        "
        fn main() {
            let a = 1 + (2);
        }
        ",
        expect![[r#"
            fn main() {
                let a = 1 + (2);
            }
        "#]],
    );
}
