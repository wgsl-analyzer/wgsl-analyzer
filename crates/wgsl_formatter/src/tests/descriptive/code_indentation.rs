use expect_test::expect;

use crate::test_util::check;

#[test]
fn format_statement_indent_simple() {
    check(
        "fn main() {
var x=0;
}",
        expect![["
            fn main() {
                var x = 0;
            }
            "]],
    );
}

#[test]
fn format_statement_indent_nested() {
    check(
        "fn main() {
for(let i = 0; i < 10; i++) {
if i % 2 == 0 {
var x = 0;
}
}
}",
        expect![[r#"
            fn main() {
                for(let i = 0; i < 10; i++) {
                    if i % 2 == 0 {
                        var x = 0;
                    }
                }
            }
        "#]],
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
            }
            "]],
    );
}
