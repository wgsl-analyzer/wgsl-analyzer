use expect_test::expect;

use crate::test_util::{assert_out_of_scope, check, check_comments};

#[test]
pub fn format_naked_field_exprs_out_of_scope() {
    assert_out_of_scope(
        "fn main() {
        a
        .
        foo
        ;
        }",
        "Wgsl disallows expressions outside statements.",
    );
}

#[test]
pub fn format_field_expr_simple() {
    check(
        "fn main() {
        let a =
        foo
        .
        bar
        ;
        }",
        expect![["
            fn main() {
                let a = foo.bar;
            }
        "]],
    );
}

#[test]
pub fn format_field_nested_within_fields() {
    check(
        "fn main() {
        let a =
        foo
        .
        bar
        .
        baz
        .
        blub
        .
        blob
        ;
        }",
        expect![["
            fn main() {
                let a = foo.bar.baz.blub.blob;
            }
        "]],
    );
}

#[test]
pub fn format_field_nested_within_indices() {
    check(
        "fn main() {
        let a =
        foo[17]
        .
        bar[28]
        .
        baz[39]
        ;
        }",
        expect![["
            fn main() {
                let a = foo[17].bar[28].baz[39];
            }
        "]],
    );
}

#[test]
pub fn format_field_nested_within_various_things() {
    check(
        "fn main() {
        let a =
        get_my_foo()
        .
        bar
        ;
        }",
        expect![["
            fn main() {
                let a = get_my_foo().bar;
            }
        "]],
    );
}

#[test]
pub fn format_comments_in_field_expr() {
    check_comments(
        "fn main() {
        let a = ## foo ## . ## bar ## ; ##
        }",
        expect![[r#"
            fn main() {
                let a = /* 0 */ foo /* 1 */ . /* 2 */ bar /* 3 */; /* 4 */
            }
        "#]],
        expect![[r#"
            fn main() {
                let a = // 0
                    foo // 1
                    . // 2
                    bar // 3
                    ; // 4
            }
        "#]],
    );
}

#[test]
pub fn format_comments_in_nested_field_expr() {
    check_comments(
        "fn main() {
        let a = ## foo ## . ## bar ## . ## baz ## ; ##
        }",
        expect![[r#"
            fn main() {
                let a = /* 0 */ foo /* 1 */ . /* 2 */ bar /* 3 */ . /* 4 */ baz /* 5 */; /* 6 */
            }
        "#]],
        expect![[r#"
            fn main() {
                let a = // 0
                    foo // 1
                    . // 2
                    bar // 3
                    . // 4
                    baz // 5
                    ; // 6
            }
        "#]],
    );
}
