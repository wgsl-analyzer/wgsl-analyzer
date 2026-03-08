use expect_test::expect;

use crate::test_util::check;

#[test]
fn format_assignment_statement_parenthesis() {
    check(
        "fn main() {
        (a) = (b + c)
        ;
        }",
        expect![[r#"
            fn main() {
                a = b + c;
            }
        "#]],
    );
}
