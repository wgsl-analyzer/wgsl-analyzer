use expect_test::expect;

use crate::test_util::check;

#[test]
pub fn format_break() {
    check(
        "fn main() {
            loop {
                break;
            }
        }",
        expect![[r#"
            fn main() {
                loop {
                    break;
                }
            }
        "#]],
    );
}
