use expect_test::expect;

use crate::test_util::{assert_out_of_scope, check};

#[test]
pub fn format_ident_expr_1() {
    check(
        "fn main() {
        let a = other_thing;
        }",
        expect![["
            fn main() {
                let a = other_thing;
            }
        "]],
    );
}

#[test]
pub fn format_ident_expr_unicode_fun_2() {
    check(
        "fn main() {
        let a = ΔέλταréflexionКызыл𐰓𐰏𐰇朝焼けسلام검정שָׁלוֹםगुलाփիրուզ𓃞𓃢𓆣;
        }",
        expect![["
            fn main() {
                let a = ΔέλταréflexionКызыл𐰓𐰏𐰇朝焼けسلام검정שָׁלוֹםगुलाփիրուզ𓃞𓃢𓆣;
            }
        "]],
    );
}

#[test]
pub fn format_ident_expr_int_literals() {
    check(
        "fn main() {
        let a = 0;
        let a = 123;
        let a = 123u;
        let a = 123i;
        let a = 0x1234;
        let a = 0xcaFE;
        let a = OXcaFE;
        }",
        expect![["
            fn main() {
                let a = 0;
                let a = 123;
                let a = 123u;
                let a = 123i;
                let a = 0x1234;
                let a = 0xcaFE;
                let a = OXcaFE;
            }
        "]],
    );
}

#[test]
pub fn format_ident_expr_zero_padded_int_literals() {
    // https://www.w3.org/TR/WGSL/#numeric-literals
    // "A leading zero on a non-zero integer literal (e.g. 012) is forbidden, so as to avoid confusion with other languages' leading-zero-means-octal notation."
    //
    // This should be raised as a syntax error, instead of being truncated by the parser, to more clearly communicate to the user
    // that leading zeros are not valid wgsl.
    assert_out_of_scope(
        "fn main() {
        let a = 0123;
        }",
        "Wgsl disallows a leading zero on a non-zero integer literal",
    );

    assert_out_of_scope(
        "fn main() {
        let a = 00i;
        }",
        "Wgsl disallows a leading zero on a non-zero integer literal",
    );

    assert_out_of_scope(
        "fn main() {
        let a = 00;
        }",
        "Wgsl disallows a leading zero on a non-zero integer literal",
    );
}

#[test]
pub fn format_ident_expr_namespaced_1() {
    assert_out_of_scope(
        "fn main() {
        let a = my_module::MY_CONSTANT;
        }",
        "Currently the parser supports only wgsl. This unit test needs to be replaced with a proper one, when wesl support is added.",
    );
}
