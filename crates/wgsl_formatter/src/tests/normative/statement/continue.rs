use expect_test::expect;

use crate::test_util::{assert_out_of_scope, check};

#[test]
pub fn format_continue_statement_without_loop_is_supported() {
    check(
        "fn main() {
        continue;
        }",
        expect![
            r#"
            fn main() {
                continue;
            }
            "#
        ],
    );
}
