use expect_test::expect;

use crate::test_util::check;

#[test]
pub fn format_assert_statement_remove_parens() {
    check(
        "
        const_assert(x<y);
        const_assert
        (x
        < y
        );
        ",
        expect![["
            const_assert x < y;
            const_assert x < y;
        "]],
    );
}
