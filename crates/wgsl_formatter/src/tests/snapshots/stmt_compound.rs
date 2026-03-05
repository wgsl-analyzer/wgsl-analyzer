use expect_test::expect;

use crate::test_util::check;

#[test]
fn format_compound_statement_put_items_on_separate_lines() {
    check(
        "
        fn main() {
        let a = 1; let b = 2;
        }
        ",
        expect![[r#"
            fn main() {
                let a = 1;
                let b = 2;
            }
        "#]],
    );
}

#[test]
fn format_compound_statement_keep_comment_on_same_line() {
    check(
        "
        fn main() {
        let a = 1; let b = 2; // Comment
        }
        ",
        expect![[r#"
            fn main() {
                let a = 1;
                let b = 2; // Comment
            }
        "#]],
    );
}
