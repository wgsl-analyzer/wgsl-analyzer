use expect_test::expect;

use crate::test_util::check;

#[test]
pub fn format_for_statement_no_anything() {
    check(
        "fn main() {
        for(;;) {
        }


        }",
        expect![[r#"
            fn main() {
                for(;;) {}
            }
        "#]],
    );
}

#[test]
pub fn format_for_statement_multiline_parts() {
    check(
        "fn main() {
        for(var i = 0; // A
        i < 4; i++) {
        }


        }",
        expect![[r#"
            fn main() {
                for(
                    var i = 0; // A
                    i < 4;
                    i++
                ) {}
            }
        "#]],
    );
}
