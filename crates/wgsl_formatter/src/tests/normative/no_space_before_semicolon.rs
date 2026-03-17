use expect_test::expect;

use crate::test_util::check;

/// Various tests that all make sure that there is no space before semicolons.
#[test]
pub fn format_override_declaration_without_assignment_removes_space_before_semicolon() {
    check(
        "

        override a /* a */ ;


        ",
        expect![[r#"
            override a /* a */;
        "#]],
    );
}

#[test]
pub fn format_const_declaration_has_no_space_before_semicolon() {
    check(
        "fn main() {

        const a = 1 /* A */ ;


        }",
        expect![["
            fn main() {
                const a = 1 /* A */;
            }
        "]],
    );
}

#[test]
pub fn format_let_decl_has_no_space_before_semicolon() {
    check(
        "fn main() {

        let a = 1 /* A */ ;


        }",
        expect![["
            fn main() {
                let a = 1 /* A */;
            }
        "]],
    );
}

#[test]
pub fn format_override_declaration_has_no_space_before_semicolon() {
    check(
        "

        override a = 1 /* A */ ;


        ",
        expect![[r#"
            override a = 1 /* A */;
        "#]],
    );
}

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
