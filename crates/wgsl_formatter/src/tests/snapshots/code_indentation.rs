use expect_test::expect;

use crate::test_util::{check, check_tabs};

#[test]
fn format_statement_indent() {
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
fn format_statement_indent_nested() {
    check(
        "fn main() {
for() {
if(y) {
var x = 0;
}
}
}",
        expect![["
            fn main() {
                for () {
                    if y {
                        var x = 0;
                    }
                }
            }"]],
    );
}

#[test]
fn format_statements_newline() {
    check(
        "fn main() {
let x = 3;

let y = 4;
}",
        expect![["
            fn main() {
                let x = 3;

                let y = 4;
            }"]],
    );
}
