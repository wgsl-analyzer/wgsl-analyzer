use expect_test::expect;

use crate::test_util::check;

#[test]
fn format_nested_parentheses_get_collapsed() {
    check(
        "
        fn main() {
            let a = 1 + (((((((((1+1)))))))));
        }
        ",
        expect![[r#"
            fn main() {
                let a = 1 + (1 + 1);
            }
        "#]],
    );
}

#[test]
fn format_parenthesized_literal_does_not_get_collapsed() {
    // Follow rustfmt
    check(
        "
        fn main() {
            let a = 1 + (2);
        }
        ",
        expect![[r#"
            fn main() {
                let a = 1 + (2);
            }
        "#]],
    );
}

#[test]
fn format_parentheses_do_get_removed_in_index_index() {
    check(
        "
        fn main() {
        aaaaaaaaaaaaa(bbbbbbbbbbbbb[(*ccccccccc)]);
        }
        ",
        expect![[r#"
            fn main() {
                aaaaaaaaaaaaa(bbbbbbbbbbbbb[*ccccccccc]);
            }
        "#]],
    );
}

#[test]
fn format_parentheses_do_not_get_removed_in_index_array() {
    check(
        "
        fn main() {
        aaaaaaaaaaaaa((*bbbbbbbbbbbbb)[ccccccccc]);
        }
        ",
        expect![[r#"
            fn main() {
                aaaaaaaaaaaaa((*bbbbbbbbbbbbb)[ccccccccc]);
            }
        "#]],
    );
}

#[test]
fn format_parentheses_do_not_get_removed_in_field_expression() {
    check(
        "
        fn main() {
        aaaaaaaaaaaaa((*bbbbbbbbbbbbb).aaaa);
        }
        ",
        expect![[r#"
            fn main() {
                aaaaaaaaaaaaa((*bbbbbbbbbbbbb).aaaa);
            }
        "#]],
    );
}
