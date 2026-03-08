use expect_test::expect;

use crate::test_util::check;

#[test]
pub fn format_override_declaration_removes_needless_parentheses() {
    check(
        "

        override a = (1 + 1);


        ",
        expect![[r#"
            override a = 1 + 1;
        "#]],
    );
}
