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

#[test]
fn spacing_between_fn_headers_1() {
    check(
        "
        fn a() {}
        fn b() {}

        fn c() {}fn d() {}


        fn e() {}
        ",
        expect![["
            fn a() {}
            fn b() {}

            fn c() {}
            fn d() {}

            fn e() {}
            "]],
    );
}

#[test]
fn no_newlines_at_at_start_of_file() {
    //Do not use expect! here, because it trims newlines and tabs and as such obscures the test case.
    check("\n\n\nfn a() {}\n", "fn a() {}\n");
}

#[test]
fn one_newline_at_end_of_file_when_missing() {
    //Do not use expect! here, because it trims newlines and tabs and as such obscures the test case.
    check("fn a() {}", "fn a() {}\n");
}

#[test]
fn one_newline_at_end_of_file_when_too_much() {
    //Do not use expect! here, because it trims newlines and tabs and as such obscures the test case.
    check("fn a() {}\n\n", "fn a() {}\n");
}
