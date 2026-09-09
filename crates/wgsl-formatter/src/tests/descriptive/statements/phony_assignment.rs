use expect_test::expect;

use crate::test_util::check;

#[test]
fn format_phony_assignment_statement_simple() {
    check(
        "fn main() {
        _
        =
        a
        ;
        }",
        expect![[r#"
            fn main() {
                _ = a;
            }
        "#]],
    );
}

#[test]
fn format_phony_assignment_statement_complex() {
    check(
        "fn main() {
        _
        =
        a +
        foo(
        27
        )
        ;
        }",
        expect![[r#"
            fn main() {
                _ = a + foo(27);
            }
        "#]],
    );
}

#[test]
fn format_phony_assignment_statement_parenthesis() {
    check(
        "fn main() {
        _ = (b + c)
        ;
        }",
        expect![[r#"
            fn main() {
                _ = b + c;
            }
        "#]],
    );
}
