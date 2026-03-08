use expect_test::expect;

use crate::test_util::check;

#[test]
pub fn format_return_statement_removes_needless_parens() {
    check(
        "fn main() {
return (1);


        }",
        expect![["
            fn main() {
                return 1;
            }
        "]],
    );
}
