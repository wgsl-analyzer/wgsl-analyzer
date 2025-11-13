use expect_test::expect;

use crate::test_util::{check, check_tabs};
#[test]
fn format_assignment() {
    check(
        "fn main() {
    x=0;
    y  +=  x + y;
}",
        expect![["
                fn main() {
                    x = 0;
                    y += x + y;
                }"]],
    );
}
