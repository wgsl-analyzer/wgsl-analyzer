use expect_test::expect;

use crate::test_util::check;

#[test]
pub fn format_while_statement_empty_gets_collapsed() {
    check(
        "fn main() {
        while true {
        }


        }",
        expect![[r#"
            fn main() {
                while true {}
            }
        "#]],
    );
}

#[test]
pub fn format_while_statement_removes_needless_parentheses() {
    check(
        "fn main() {
        while (1+2==3) {}


        }",
        expect![[r#"
            fn main() {
                while 1 + 2 == 3 {}
            }
        "#]],
    );
}

#[test]
pub fn format_while_statement_indent_multiline_condition() {
    check(
        "fn main() {
        while aaaaaaaaaaaaa + bbbbbbbbbbbbbbbbbb // Force break
        < ccccccccccccccccc + ddddddddddddddd {
        a += b;
        }


        }",
        expect![[r#"
            fn main() {
                while aaaaaaaaaaaaa + bbbbbbbbbbbbbbbbbb // Force break
                    < ccccccccccccccccc + ddddddddddddddd {
                    a += b;
                }
            }
        "#]],
    );
}
