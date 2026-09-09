use expect_test::expect;

use crate::test_util::check;

#[test]
pub fn format_loop_statement_empty_gets_collapsed() {
    check(
        "fn main() {
        loop {
        }


        }",
        expect![["
            fn main() {
                loop {}
            }
        "]],
    );
}
