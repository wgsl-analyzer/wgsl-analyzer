use expect_test::expect;

use crate::test_util::check;

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
