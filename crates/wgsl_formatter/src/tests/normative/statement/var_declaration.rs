use expect_test::expect;

use crate::test_util::check;

#[test]
pub fn format_var_declaration_has_no_space_before_semicolon() {
    check(
        "fn main() {

        var a = 1 /* A */ ;


        }",
        expect![["
            fn main() {
                var a = 1 /* A */;
            }
        "]],
    );
}

#[test]
pub fn format_var_declaration_removes_needless_parentheses() {
    check(
        "fn main() {

        var a = (1 + 1);


        }",
        expect![[r#"
            fn main() {
                var a = 1 + 1;
            }
        "#]],
    );
}
