use expect_test::expect;

use crate::test_util::{assert_out_of_scope, check};

#[test]
pub fn format_loop_continuing_break_if_statement_with_needless_parens() {
    check(
        "fn main() {
        loop {
        continuing {
        break if (false);

        }
        }


        }",
        expect![[r#"
            fn main() {
                loop {
                    continuing {
                        break if false;
                    }
                }
            }
        "#]],
    );
}

#[test]
pub fn format_break_if_statement_without_loop() {
    assert_out_of_scope(
        "fn main() {
        break if false;
        }",
        "Wgsl disallows only allows break if statements as the last statement of a continuing block",
    );
}

#[test]
pub fn format_break_if_statement_without_continuing() {
    assert_out_of_scope(
        "fn main() {
        loop{
        break if false;
        }
        }",
        "Wgsl disallows only allows break if statements as the last statement of a continuing block",
    );
}
