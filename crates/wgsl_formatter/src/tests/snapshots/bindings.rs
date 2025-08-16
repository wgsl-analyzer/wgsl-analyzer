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

#[test]
fn format_variable() {
    check(
        "fn main() {
    var x=0;
}",
        expect![["
                fn main() {
                    var x = 0;
                }"]],
    );
}

#[test]
fn format_variable_type() {
    check(
        "fn main() {var x   : u32=0;}",
        expect!["fn main() {var x: u32 = 0;}"],
    );
}
