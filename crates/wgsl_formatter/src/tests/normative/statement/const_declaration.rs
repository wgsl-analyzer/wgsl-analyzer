use expect_test::expect;

use crate::test_util::check;

#[test]
pub fn format_const_declaration_removes_needless_parentheses() {
    check(
        "fn main() {

        const a = (1 + 1);


        }",
        expect![[r#"
            fn main() {
                const a = 1 + 1;
            }
        "#]],
    );
}
